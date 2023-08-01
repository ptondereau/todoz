use tezos_smart_rollup::storage::{
    accounts::{storage::Storage, StorageError},
    path::{concat, OwnedPath, RefPath},
};
use tezos_smart_rollup_host::{path::Path, runtime::Runtime};
use thiserror::Error;

use crate::todo::TodoItem;

#[derive(Error, Debug)]
pub enum TodoStorageError {
    /// Some error happened when constructing the path to some resource
    /// associated with a todo.
    #[error("Path error")]
    PathError(#[from] tezos_smart_rollup_host::path::PathError),

    #[error("Runtime error")]
    RuntimeError(#[from] tezos_smart_rollup_host::runtime::RuntimeError),

    #[error("Serialization error")]
    SerializationFailed(#[from] serde_json_wasm::ser::Error),
    #[error("Deserialization error")]
    DeserializationFailed(#[from] serde_json_wasm::de::Error),

    #[error("Todo not found: {0}")]
    TodoNotFound(u64),

    #[error("Storage error")]
    StorageError(#[from] StorageError),

    #[error("Creation of new todos storage failed")]
    NewTodosStorageFailed,
}

pub type TodoStorageResult<T> = Result<T, TodoStorageError>;

pub const TODOS: RefPath = RefPath::assert_from(b"/todos");
pub const TODO_ITEMS: RefPath = RefPath::assert_from(b"/items");

#[derive(Debug)]
pub struct Todo {
    path: OwnedPath,
}

impl Todo {
    fn todo_path(&self, todo_id: u64) -> TodoStorageResult<OwnedPath> {
        let todo_path = OwnedPath::try_from(format!("/{todo_id}"))?;

        concat(&self.path, &todo_path).map_err(TodoStorageError::from)
    }

    fn todo_item_path(&self, todo_id: u64, todo_item_id: u64) -> TodoStorageResult<OwnedPath> {
        let todo_item_path = OwnedPath::try_from(format!("/{todo_item_id}"))?;

        concat(&self.todo_path(todo_id)?, &todo_item_path).map_err(TodoStorageError::from)
    }

    pub fn save_item(
        &self,
        host: &mut impl Runtime,
        todo_id: u64,
        todo_item_id: u64,
        todo_item: &TodoItem,
    ) -> TodoStorageResult<()> {
        let todo_path = self.todo_path(todo_id)?;
        let is_exists = exists(host, &todo_path)?;
        if !is_exists {
            return Err(TodoStorageError::TodoNotFound(todo_id));
        }

        let todo_item_path = self.todo_item_path(todo_id, todo_item_id)?;
        store_json(host, &todo_item_path, todo_item)
    }

    pub fn save(
        &self,
        host: &mut impl Runtime,
        todo_id: u64,
        todos: &Vec<TodoItem>,
    ) -> TodoStorageResult<()> {
        let todo_path = self.todo_path(todo_id)?;

        todos.iter().enumerate().try_for_each(|(i, todo)| {
            let todo_item_path = self.todo_item_path(todo_id, i as u64)?;
            self.save_item(host, todo_id, i as u64, todo)
        })?;

        store_json(host, &todo_path, todos)
    }

    pub fn get(&self, host: &mut impl Runtime, todo_id: u64) -> TodoStorageResult<Vec<TodoItem>> {
        let todo_path = self.todo_path(todo_id)?;

        let is_exists = exists(host, &todo_path)?;

        if !is_exists {
            return Err(TodoStorageError::TodoNotFound(todo_id));
        }
        read_json(host, &todo_path)
    }
}

impl From<OwnedPath> for Todo {
    fn from(path: OwnedPath) -> Self {
        Self { path }
    }
}

/// Get the initial storage interface for todos
pub fn init_todo_storage(host: &mut impl Runtime) -> TodoStorageResult<Todo> {
    let mut internal_storage = Storage::<Todo>::init(&TODOS).map_err(TodoStorageError::from)?;
    let todos_storage = internal_storage.create_new(host, &TODOS)?;

    if let Some(todos) = todos_storage {
        Ok(todos)
    } else {
        Err(TodoStorageError::NewTodosStorageFailed)
    }
}

/// Stores a serde seriliazed value at a given path
fn store_json<'a, R: Runtime, T>(
    host: &mut R,
    path: &OwnedPath,
    data: &'a T,
) -> TodoStorageResult<()>
where
    T: serde::Serialize,
{
    let bytes = serde_json_wasm::to_vec(data).map_err(TodoStorageError::from)?;
    host.store_write(path, &bytes, 0)
        .map_err(TodoStorageError::from)
}

/// Read a serde deserialized value from a given path
fn read_json<'a, R: Runtime, T>(host: &mut R, path: &OwnedPath) -> TodoStorageResult<T>
where
    T: serde::de::DeserializeOwned,
{
    let size = host
        .store_value_size(path)
        .map_err(TodoStorageError::from)?;

    let bytes = host
        .store_read(path, 0, size)
        .map_err(TodoStorageError::from)?;

    serde_json_wasm::from_slice(bytes.as_slice()).map_err(TodoStorageError::from)
}

///  Check if a path exists
pub fn exists<R: Runtime>(host: &mut R, path: &impl Path) -> TodoStorageResult<bool> {
    let exists = Runtime::store_has(host, path)?
        .map(|_| true)
        .unwrap_or_default();
    Ok(exists)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::todo::TodoItem;
    use tezos_smart_rollup_mock::MockHost;

    #[test]
    fn test_todo_storage() {
        let mut runtime = MockHost::default();
        let todos = init_todo_storage(&mut runtime).expect("Failed to init todos storage");

        let expected_todo_items = vec![TodoItem::new("Buy milk".to_string())];
        todos
            .save(&mut runtime, 1, &expected_todo_items)
            .expect("Failed to save todo");

        let todo_items = todos.get(&mut runtime, 1).expect("Failed to get todo");

        assert_eq!(expected_todo_items, todo_items);

        let not_foud_todo = todos.get(&mut runtime, 2);
        assert_eq!(
            not_foud_todo.unwrap_err().to_string(),
            TodoStorageError::TodoNotFound(2).to_string()
        );
    }
}
