use crate::AutodiffMode;
use crate::DataType;
use crate::IRNode;
use crate::Program;
use autocxx::c_int;
use autocxx::WithinUniquePtr;
use chirp_sys::taichi::lang;
use cxx::UniquePtr;
pub struct Kernel {
    inner: *mut lang::Kernel,
    callable: UniquePtr<lang::Callable>,
}

impl Kernel {
    pub fn from_ir(
        prog: &mut Program,
        mut ir: impl IRNode,
        name: &str,
        autodiff_mode: AutodiffMode,
    ) -> Self {
        cxx::let_cxx_string!(ident = name);
        let inner = lang::Kernel::new2(prog.pin(), ir.cast(), &ident, autodiff_mode.to_sys())
            .within_unique_ptr()
            .into_raw();
        let callable = unsafe { UniquePtr::from_raw(inner.cast::<lang::Callable>()) };
        Kernel { inner, callable }
    }

    pub fn insert_ret(&mut self, dt: DataType) -> c_int {
        self.callable.as_mut().unwrap().insert_ret(&dt.uni_ptr())
    }

    pub fn finalize_rets(&mut self) {
        self.callable.as_mut().unwrap().finalize_rets();
    }

    pub fn raw_ptr(&self) -> &*mut lang::Kernel {
        &self.inner
    }
}
