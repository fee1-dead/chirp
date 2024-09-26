use chirp_sys::taichi::lang;
use cxx::UniquePtr;
use std::pin::Pin;

pub trait IRNode {
    fn cast(&mut self) -> Pin<Box<UniquePtr<lang::IRNode>>>;
}

pub struct Block {
    inner: *mut lang::Block,
}

impl Block {
    pub fn new(inner: *mut lang::Block) -> Self {
        Block { inner }
    }
}

impl IRNode for Block {
    fn cast(&mut self) -> Pin<Box<UniquePtr<lang::IRNode>>> {
        unsafe { Box::pin(UniquePtr::from_raw(self.inner.cast::<lang::IRNode>())) }
    }
}
