use super::stmts::*;
use crate::types;
use crate::Block;
use autocxx::WithinBox;
use chirp_sys::taichi::lang;
use std::pin::Pin;

pub struct IRBuilder {
    inner: Pin<Box<lang::IRBuilder>>,
}

impl IRBuilder {
    pub fn new() -> Self {
        let inner = lang::IRBuilder::new().within_box();
        IRBuilder { inner }
    }

    pub fn create_local_var(&mut self, dt: types::DataType) -> AllocaStmt {
        AllocaStmt::new(self.pin().create_local_var(dt.uni_ptr()))
    }

    pub fn create_local_store(&mut self, ptr: &AllocaStmt, mut data: impl Stmt) {
        unsafe {
            self.pin().create_local_store(*ptr.into_raw(), data.cast());
        }
    }

    pub fn create_local_load(&mut self, ptr: &AllocaStmt) -> LocalLoadStmt {
        unsafe { LocalLoadStmt::new(self.pin().create_local_load(*ptr.into_raw())) }
    }

    pub fn create_return(&mut self, mut value: impl Stmt) -> ReturnStmt {
        unsafe { ReturnStmt::new(self.pin().create_return(value.cast())) }
    }

    pub fn extract_ir(&mut self) -> Block {
        Block::new(self.pin().extract_ir().into_raw())
    }

    pub fn get_float32(&mut self, value: f32) -> ConstStmt {
        ConstStmt::new(self.pin().get_float32(value))
    }

    pub fn pin(&mut self) -> Pin<&mut lang::IRBuilder> {
        self.inner.as_mut()
    }
}
