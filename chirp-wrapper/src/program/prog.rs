use crate::Arch;
use crate::ir::snodes::SNode;
use crate::AotModuleBuilder;
use chirp_sys::taichi::lang;
use autocxx::prelude::*;
use std::pin::Pin;
use cxx::{CxxVector, CxxString};

pub struct Program {
    inner: Pin<Box<lang::Program>>,
}

impl Program {
    pub fn new(arch: Arch) -> Self {
        let inner = lang::Program::new1(arch.to_sys()).within_box();
        Program { inner }
    }

    pub fn add_snode_tree(&mut self, root: SNode, compile_only: bool) -> *mut lang::SNodeTree {
        self.origin().add_snode_tree(root.into_inner(), compile_only)
    }

    pub fn make_aot_module_builder(&mut self, arch: Arch, caps: &CxxVector<CxxString>) -> AotModuleBuilder {
        AotModuleBuilder::new(self.origin().make_aot_module_builder(arch.to_sys(), caps))
    }

    pub fn origin(&mut self) -> Pin<&mut lang::Program> {
        self.inner.as_mut()
    }
}
