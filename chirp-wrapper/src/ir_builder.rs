use autocxx::prelude::*;
use std::pin::Pin;
use std::ops::{Deref, DerefMut};
use chirp_sys::taichi::lang;
use crate::types;

pub struct IRBuilder {
    inner: Pin<Box<lang::IRBuilder>>,
}

impl IRBuilder {
    pub fn new() -> Self {
        let inner = lang::IRBuilder::new().within_box();
        IRBuilder { inner }
    }

    pub fn create_local_var(&mut self, dt: types::DataType) -> *mut lang::AllocaStmt {
        self.inner.as_mut().create_local_var(dt.as_value_param())
    }

    pub fn create_local_store(&mut self, ptr: *mut lang::AllocaStmt, data: *mut lang::Stmt) {
        unsafe { self.inner.as_mut().create_local_store(ptr, data); }
    }

    pub fn create_local_load(&mut self, ptr: *mut lang::AllocaStmt) -> *mut lang::LocalLoadStmt {
        unsafe { self.inner.as_mut().create_local_load(ptr) }
    }

    fn inner_mut(&mut self) -> &mut lang::IRBuilder {
        unsafe{ self.inner.as_mut().get_unchecked_mut() }
    }
}

impl Deref for IRBuilder {
    type Target = lang::IRBuilder;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for IRBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}
