#![doc = "External-consumer compile proof for the unstable local-host composition API."]

use workflow_core::TrustedHostLocalApplicationPreparedSession;
use workflow_local_host::LocalHostPreparedOperation;

#[allow(dead_code)]
fn consume_core_prepared_pair(
    prepared: TrustedHostLocalApplicationPreparedSession<'_>,
) -> LocalHostPreparedOperation<'_> {
    LocalHostPreparedOperation::from_prepared(prepared)
}

#[test]
fn external_consumer_can_name_only_the_reviewed_composition_surface() {
    let operation_type = std::any::type_name::<LocalHostPreparedOperation<'static>>();
    assert!(operation_type.contains("LocalHostPreparedOperation"));
}
