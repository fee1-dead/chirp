use cxx::UniquePtr;
use chirp_sys::taichi::lang;
use std::pin::Pin;

pub struct Block {
    inner: UniquePtr<lang::Block>
}

impl Block {
    pub fn new(inner: UniquePtr<lang::Block>) -> Self {
        Block { inner }
    }

    pub fn cast(self) -> Pin<Box<UniquePtr<lang::IRNode>>> {
        unsafe { Box::pin(UniquePtr::from_raw(self.inner.into_raw().cast::<lang::IRNode>())) }
    }
}
