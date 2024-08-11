use crate::Arch;
use chirp_sys::taichi::lang;
use autocxx::prelude::*;
use std::pin::Pin;

pub struct Program {
    inner: Pin<Box<lang::Program>>,
}

impl Program {
    pub fn new(arch: Arch) -> Self {
        let inner = lang::Program::new1(arch.to_sys()).within_box();
        Program { inner }
    }
}
