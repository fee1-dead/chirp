use std::borrow::Borrow;
use std::pin::Pin;

use autocxx::c_int;
use autocxx::WithinUniquePtr;
use cxx::UniquePtr;
use chirp_sys::taichi::lang;
use crate::Program;
use crate::AutodiffMode;
use crate::DataType;
pub struct Kernel {
    inner: UniquePtr<lang::Kernel>
}

pub struct Callable {
    inner: UniquePtr<lang::Callable>
}

impl Kernel {
    pub fn from_ir(mut prog: Program, ir: Pin<Box<UniquePtr<lang::IRNode>>>, name: &str, autodiff_mode: AutodiffMode) -> Self {
        cxx::let_cxx_string!(iden = name);
        let inner = lang::Kernel::new2(prog.origin(), ir, &iden, autodiff_mode.to_sys()).within_unique_ptr();
        Kernel { inner }
    }

    pub fn into_raw(self) -> *mut lang::Kernel {
        self.inner.into_raw()
    }
}

impl Callable {
    pub fn new(inner: UniquePtr<lang::Callable>) -> Self {
        Callable { inner }
    }

    pub fn insert_ret(&mut self, dt: DataType) -> c_int {
        self.inner.as_mut().unwrap().insert_ret(dt.into_inner().borrow())
    }

    pub fn finalize_rets(&mut self) {
        self.inner.as_mut().unwrap().finalize_rets()
    }
}