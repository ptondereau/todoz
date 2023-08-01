extern crate alloc;

mod error;
mod storage;
mod todo;

use error::KernelResult;
use tezos_smart_rollup::{kernel_entry, prelude::*};

use crate::{storage::init_todo_storage, todo::TodoItem};

pub fn run(host: &mut impl Runtime) -> KernelResult<()> {
    let todos_storage = init_todo_storage(host)?;

    todos_storage
        .save(host, 1, &vec![TodoItem::new("Buy milk".to_string())])
        .expect("Failed to save todo");

    Ok(())
}

pub fn entry(host: &mut impl Runtime) {
    debug_msg!(host, "Todo kernel started!\n");

    match run(host) {
        Ok(_) => {}
        Err(err) => debug_msg!(host, "{}", &err.to_string()),
    }
}

kernel_entry!(entry);
