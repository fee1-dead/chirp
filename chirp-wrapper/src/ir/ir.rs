use cxx::UniquePtr;
use chirp_sys::taichi::lang;
use std::pin::Pin;

pub struct Block {
    inner: UniquePtr<lang::Block>
}

pub struct Callable {
    inner: UniquePtr<lang::Callable>
}
trait IRNode {
}

impl Block {
    pub fn new(inner: UniquePtr<lang::Block>) -> Self {
        Block { inner }
    }

    pub fn cast(self) -> Pin<Box<UniquePtr<lang::IRNode>>> {
        let raw = self.inner.into_raw();
        unsafe { Box::pin(UniquePtr::from_raw(raw.cast::<lang::IRNode>())) }
    }
}
