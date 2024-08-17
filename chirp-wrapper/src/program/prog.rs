use crate::ir::snode::SNode;
use crate::AotModuleBuilder;
use crate::{Arch, CVec};
use autocxx::WithinBox;
use chirp_sys::taichi::lang;
use cxx::UniquePtr;
use std::pin::Pin;

pub struct Program {
    inner: Pin<Box<lang::Program>>,
}

impl Program {
    pub fn new(arch: Arch) -> Self {
        let inner = lang::Program::new1(arch.to_sys()).within_box();
        Program { inner }
    }

    pub fn add_snode_tree(&mut self, root: &mut SNode, compile_only: bool) -> *mut lang::SNodeTree {
        self.pin()
            .add_snode_tree(unsafe { UniquePtr::from_raw(*root.raw()) }, compile_only)
    }

    pub fn make_aot_module_builder(&mut self, arch: Arch, caps: &CVec) -> AotModuleBuilder {
        AotModuleBuilder::new(
            self.pin()
                .make_aot_module_builder(arch.to_sys(), caps.as_ref()),
        )
    }

    pub fn pin(&mut self) -> Pin<&mut lang::Program> {
        self.inner.as_mut()
    }
}
