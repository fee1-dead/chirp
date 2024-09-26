use crate::Kernel;
use chirp_sys::taichi::lang;
use cxx::UniquePtr;

pub struct AotModuleBuilder {
    inner: UniquePtr<lang::AotModuleBuilder>,
}

impl AotModuleBuilder {
    pub fn new(aot_builder_ptr: UniquePtr<lang::AotModuleBuilder>) -> Self {
        let inner = aot_builder_ptr;
        AotModuleBuilder { inner }
    }

    pub fn add(&mut self, name: &str, kernel: &Kernel) {
        cxx::let_cxx_string!(ident = name);
        unsafe { self.inner.as_mut().unwrap().add(&ident, *kernel.raw_ptr()) }
    }

    pub fn dump(&mut self, output_dir: &str, filename: &str) {
        cxx::let_cxx_string!(opd = output_dir);
        cxx::let_cxx_string!(fnm = filename);
        self.inner.dump(&opd, &fnm);
    }
}
