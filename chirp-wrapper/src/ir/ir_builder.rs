use autocxx::prelude::*;
use std::pin::Pin;
use chirp_sys::taichi::lang;
use crate::types;
use crate::Block;

pub struct IRBuilder {
    inner: Pin<Box<lang::IRBuilder>>,
}

impl IRBuilder {
    pub fn new() -> Self {
        let inner = lang::IRBuilder::new().within_box();
        IRBuilder { inner }
    }

    pub fn create_local_var(&mut self, dt: types::DataType) -> *mut lang::AllocaStmt {
        self.origin().create_local_var(dt.into_inner())
    }

    pub fn create_local_store(&mut self, ptr: *mut lang::AllocaStmt, data: *mut lang::Stmt) {
        unsafe { self.origin().create_local_store(ptr, data); }
    }

    pub fn create_local_load(&mut self, ptr: *mut lang::AllocaStmt) -> *mut lang::LocalLoadStmt {
        unsafe { self.origin().create_local_load(ptr) }
    }

    pub fn create_return(&mut self, value: *mut lang::Stmt) -> *mut lang::ReturnStmt {
        unsafe { self.origin().create_return(value) }
    }

    pub fn extract_ir(&mut self) -> Block {
        let blk_ptr = self.origin().extract_ir();
        Block::new(blk_ptr)
    }

    pub fn origin(&mut self) -> Pin<&mut lang::IRBuilder> {
        self.inner.as_mut()
    }
}
