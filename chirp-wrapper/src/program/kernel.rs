use std::borrow::Borrow;
use std::pin::Pin;

use autocxx::c_int;
use autocxx::WithinUniquePtr;
use cxx::UniquePtr;
use chirp_sys::taichi::lang;
use crate::Program;
use crate::AutodiffMode;
use crate::DataType;
pub struct Kernel { }

pub struct Callable {
    inner: UniquePtr<lang::Callable>
}

impl Kernel {
    pub fn from_ir(prog: &mut Program , ir: Pin<Box<UniquePtr<lang::IRNode>>>, name: &str, autodiff_mode: AutodiffMode) -> *mut lang::Kernel {
        cxx::let_cxx_string!(ident = name);
        lang::Kernel::new2(prog.origin(), ir, &ident, autodiff_mode.to_sys()).within_unique_ptr().into_raw()
    }
}

impl Callable {
    pub fn new(kernel: *mut lang::Kernel) -> Self {
        let inner = unsafe {
            UniquePtr::from_raw(kernel.cast::<lang::Callable>())
        };
        Callable { inner }
    }

    pub fn insert_ret(&mut self, dt: DataType) -> c_int {
        self.inner.as_mut().unwrap().insert_ret(dt.uni_ptr().borrow())
    }

    pub fn finalize_rets(&mut self) {
        self.inner.as_mut().unwrap().finalize_rets()
    }
}