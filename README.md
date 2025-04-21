# nescrow
blockchain part for need of escrow project

[https://testnet.needofescrow.com](https://testnet.needofescrow.com)

# How to migrate state

1. Change your task_legacy.rs to current task.rs
2. Perform modifications in task according to your business needs
3. Go to state_migration.rs and change NescrowV1 and NescrowV2 according to your needs. NescrowV1 should be your current contract's state.
4. Change migrate_state method in state_migration.rs if needed.
5. Change StateVersion::V2 inside match current_version to newer one.
6. If you create new collection it should use new entry from StorageKeys.
7. Deploy the contract without state initialization.
8. Find migrate state command in contract-rs/README.md and run it.