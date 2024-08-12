use cxx::UniquePtr;
use chirp_sys::taichi::lang;
use crate::Program;
pub struct Kernel {
    inner: UniquePtr<lang::Kernel>
}
