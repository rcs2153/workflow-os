# Locking

Workflow OS workers are intended to be stateless over durable state. Locks are a coordination aid, not the source of truth.

## Store Contract

`LockStore` supports:

- Acquiring a lock by key and owner.
- Releasing a lock by lease.
- Returning a deterministic contention error when a lock is already held.

Locks must not be used to hide invalid event sequences. Event append and replay still enforce sequence, identity, and transition invariants.

## Local Backend Semantics

The local filesystem backend represents locks as directories under `locks/`. Acquiring a lock creates a directory. Releasing a lock removes that directory.

This provides practical local-process and local-filesystem contention behavior for development. It is not distributed locking and must not be used as a production coordination guarantee across machines or network filesystems.

## Limitations

The v0 local lock implementation:

- Has no lease expiration.
- Has no fencing token.
- Has no distributed consensus.
- May require manual cleanup if a process exits while holding a lock.

Future production backends must document lease expiration, fencing, ownership, and failure semantics explicitly.
