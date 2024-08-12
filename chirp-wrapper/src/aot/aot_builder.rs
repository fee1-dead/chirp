use cxx::UniquePtr;
use chirp_sys::taichi::lang;
pub struct AotModuleBuilder {
    inner: UniquePtr<lang::AotModuleBuilder>,
}

impl AotModuleBuilder {
    pub fn new(aot_builder_ptr: UniquePtr<lang::AotModuleBuilder>) -> Self {
        let inner = aot_builder_ptr;
        AotModuleBuilder { inner }
    }
}