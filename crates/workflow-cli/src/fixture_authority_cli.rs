use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use serde::{de::DeserializeOwned, Deserialize};
use workflow_core::local_fixture_replacement_authority::LocalFixturePrincipalVerifier;
use workflow_core::{
    ActorId, CapabilityGrant, CapabilityGrantId, HarnessContractId, SqliteStateBackend,
    WorkflowOsError,
};

use super::{
    github_pr_comment_continuation_database, parse_github_pr_comment_fixture_continuation_input,
    print_github_pr_comment_fixture_continuation_result, usage, with_fixture_continuation_request,
    Invocation,
};

const MAX_INPUT_BYTES: u64 = 64 * 1024;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DelegatedFixtureInput {
    request: serde_json::Value,
    fixture: serde_json::Value,
}

impl DelegatedFixtureInput {
    fn into_value(self) -> serde_json::Value {
        serde_json::json!({"request": self.request, "fixture": self.fixture})
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum FixtureAuthorityCommand {
    Validate {
        grant: CapabilityGrantId,
    },
    Upgrade,
    Register {
        actor: ActorId,
        harness: HarnessContractId,
    },
    Issue {
        request: PathBuf,
        grant: PathBuf,
        operator: ActorId,
    },
    RevokeGrant(CapabilityGrantId),
    RevokePrincipal(ActorId),
}

pub(super) fn parse(args: &[String]) -> Result<FixtureAuthorityCommand, WorkflowOsError> {
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let command = match args.as_slice() {
        ["continue-sandbox-delegated", "--grant", grant, "--credential-and-request-stdin"] => {
            FixtureAuthorityCommand::Validate {
                grant: CapabilityGrantId::new(*grant)?,
            }
        }
        ["fixture-authority", "upgrade"] => FixtureAuthorityCommand::Upgrade,
        ["fixture-authority", "register", "--actor", actor, "--harness", harness, "--credential-stdin"] => {
            FixtureAuthorityCommand::Register {
                actor: ActorId::new(*actor)?,
                harness: HarnessContractId::new(*harness)?,
            }
        }
        ["fixture-authority", "issue", "--request", request, "--grant-file", grant, "--operator", operator] => {
            FixtureAuthorityCommand::Issue {
                request: PathBuf::from(request),
                grant: PathBuf::from(grant),
                operator: ActorId::new(*operator)?,
            }
        }
        ["fixture-authority", "revoke-grant", "--grant", grant] => {
            FixtureAuthorityCommand::RevokeGrant(CapabilityGrantId::new(*grant)?)
        }
        ["fixture-authority", "revoke-principal", "--actor", actor] => {
            FixtureAuthorityCommand::RevokePrincipal(ActorId::new(*actor)?)
        }
        _ => {
            return Err(usage(
                "invalid GitHub PR-comment fixture authority command; see --help",
            ))
        }
    };
    Ok(command)
}

pub(super) fn execute(
    invocation: &Invocation,
    command: &FixtureAuthorityCommand,
) -> Result<(), WorkflowOsError> {
    let database = github_pr_comment_continuation_database(invocation);
    if let FixtureAuthorityCommand::Validate { grant } = command {
        return validate(invocation, &database, grant, &mut io::stdin().lock());
    }
    if matches!(command, FixtureAuthorityCommand::Upgrade) {
        SqliteStateBackend::upgrade_fixture_replacement_authority_v9_to_v10(database)?;
        print_operator_result(invocation, "schema_ready");
        return Ok(());
    }
    // Operator commands never initialize a missing store or implicitly upgrade V9.
    let _existing = SqliteStateBackend::open_read_only(&database)?;
    let backend = SqliteStateBackend::open(database)?;
    let status = match command {
        FixtureAuthorityCommand::Register { actor, harness } => {
            let mut input = io::stdin().lock();
            let credential = read_credential(&mut input)?;
            let mut extra = [0_u8];
            if input.read(&mut extra).map_err(|_| invalid_input())? != 0 {
                return Err(invalid_input());
            }
            backend.register_local_fixture_principal(&LocalFixturePrincipalVerifier::new(
                actor.clone(),
                harness.clone(),
                &credential,
            ))?;
            "principal_registered"
        }
        FixtureAuthorityCommand::Issue {
            request,
            grant,
            operator,
        } => {
            let grant: CapabilityGrant = read_file(invocation, grant)?;
            let value: DelegatedFixtureInput = read_file(invocation, request)?;
            let parsed = parse_github_pr_comment_fixture_continuation_input(
                &value.into_value(),
                Some(grant.subject().clone()),
            )?;
            with_fixture_continuation_request(&backend, parsed, |executor, request| {
                workflow_core::issue_local_fixture_replacement_grant(
                    executor, &request, operator, &grant,
                )
            })?;
            "grant_issued"
        }
        FixtureAuthorityCommand::RevokeGrant(grant) => {
            backend.revoke_local_fixture_grant(grant)?;
            "grant_revoked"
        }
        FixtureAuthorityCommand::RevokePrincipal(actor) => {
            backend.revoke_local_fixture_principal(actor)?;
            "principal_revoked"
        }
        FixtureAuthorityCommand::Validate { .. } | FixtureAuthorityCommand::Upgrade => {
            unreachable!("handled above")
        }
    };
    print_operator_result(invocation, status);
    Ok(())
}

fn validate(
    invocation: &Invocation,
    database: &Path,
    grant: &CapabilityGrantId,
    input: &mut impl Read,
) -> Result<(), WorkflowOsError> {
    let backend = SqliteStateBackend::open_read_only(database)?;
    let credential = read_credential(input)?;
    let principal = backend.authenticate_local_fixture_grant_principal(grant, &credential)?;
    let value: DelegatedFixtureInput = read_json(input)?;
    let parsed =
        parse_github_pr_comment_fixture_continuation_input(&value.into_value(), Some(principal))?;
    let result = with_fixture_continuation_request(&backend, parsed, |executor, request| {
        workflow_core::validate_local_fixture_replacement_continuation(
            executor,
            &request,
            grant,
            &credential,
        )
    })?;
    print_github_pr_comment_fixture_continuation_result(invocation, &result, "verified_delegation");
    Ok(())
}

fn read_credential(input: &mut impl Read) -> Result<[u8; 32], WorkflowOsError> {
    let mut credential = [0_u8; 32];
    input
        .read_exact(&mut credential)
        .map_err(|_| invalid_input())?;
    Ok(credential)
}

fn read_file<T: DeserializeOwned>(
    invocation: &Invocation,
    path: &Path,
) -> Result<T, WorkflowOsError> {
    let file = File::open(invocation.project_dir.join(path)).map_err(|_| invalid_input())?;
    read_json(file)
}

fn read_json<T: DeserializeOwned>(input: impl Read) -> Result<T, WorkflowOsError> {
    let mut bytes = Vec::new();
    let length = input
        .take(MAX_INPUT_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| invalid_input())?;
    if length as u64 > MAX_INPUT_BYTES {
        return Err(invalid_input());
    }
    serde_json::from_slice(&bytes).map_err(|_| invalid_input())
}

fn invalid_input() -> WorkflowOsError {
    WorkflowOsError::validation(
        "github_pr_comment_fixture_authority_cli.input_invalid",
        "fixture authority input is missing, malformed, or too large",
    )
}

fn print_operator_result(invocation: &Invocation, status: &str) {
    if invocation.json {
        println!(
            "{}",
            serde_json::json!({"status": status, "mode": "fixture", "provider_call_performed": false})
        );
    } else {
        println!("status: {status}\nmode: fixture\nprovider_call_performed: false");
    }
}

pub(super) fn print_help() {
    println!("  provider github-pr-comment continue-sandbox-delegated --grant <id> --credential-and-request-stdin");
    println!("  provider github-pr-comment fixture-authority upgrade");
    println!("  provider github-pr-comment fixture-authority register --actor <id> --harness <id> --credential-stdin");
    println!("  provider github-pr-comment fixture-authority issue --request <path> --grant-file <path> --operator <actor-id>");
    println!("  provider github-pr-comment fixture-authority revoke-grant --grant <id>");
    println!("  provider github-pr-comment fixture-authority revoke-principal --actor <id>");
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::{read_credential, read_json, DelegatedFixtureInput, MAX_INPUT_BYTES};

    #[test]
    fn delegated_input_is_bounded_and_rejects_actor_or_duplicate_fields() {
        assert!(read_credential(&mut &[0_u8; 31][..]).is_err());
        assert!(
            read_json::<DelegatedFixtureInput>(&b"{\"request\":{},\"fixture\":{}}"[..]).is_ok()
        );
        assert!(read_json::<DelegatedFixtureInput>(
            &b"{\"request\":{},\"fixture\":{},\"replacement_actor\":\"agent/forged\"}"[..]
        )
        .is_err());
        assert!(read_json::<DelegatedFixtureInput>(
            &b"{\"request\":{},\"request\":{},\"fixture\":{}}"[..]
        )
        .is_err());
        assert!(read_json::<DelegatedFixtureInput>(
            std::io::repeat(b' ').take(MAX_INPUT_BYTES + 1)
        )
        .is_err());
    }
}
