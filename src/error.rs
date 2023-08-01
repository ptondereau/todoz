use tezos_smart_rollup::storage::accounts::StorageError;
use thiserror::Error;

use crate::storage::TodoStorageError;

pub type KernelResult<T> = Result<T, KernelError>;

#[derive(Debug, Error)]
pub enum KernelError {
    #[error("Todo storage error")]
    TodoStorageError(#[from] TodoStorageError),

    #[error("Storage error")]
    StorageError(#[from] StorageError),
}
