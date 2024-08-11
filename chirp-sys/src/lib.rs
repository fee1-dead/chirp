pub const TAICHI_LIBDIR: &'static str = env!("CHIRP_TAICHI_LIB_DIR");

autocxx::include_cpp! {
    #include "taichi/aot/module_builder.h"
    #include "taichi/ir/ir.h"
    #include "taichi/ir/ir_builder.h"
    #include "taichi/ir/type.h"
    #include "taichi/ir/snode.h"
    // #include "taichi/ir/shim.h"
    #include "taichi/program/program.h"
    #include "taichi/program/kernel.h"
    #include "taichi/program/callable.h"
    name!(ffi)
    safety!(unsafe_ffi)
    generate!("taichi::lang::AotModuleBuilder")
    generate!("taichi::lang::IRBuilder")
    generate!("taichi::lang::IRNode")
    generate!("taichi::lang::PrimitiveType")
    generate!("taichi::lang::Program")
    generate!("taichi::lang::Kernel")
    generate!("taichi::lang::Callable")
    // extern_cpp_opaque_type!("taichi::lang::Kernel", ffi2::Kernel)
    generate!("taichi::lang::SNode")
    block!("taichi::lang::aot::Kernel")
    // generate!("taichi::lang::new_kernel")
}
pub use ffi::*;

/*#[cxx::bridge(namespace = "taichi::lang")]
pub mod ffi2 {
    unsafe extern "C++" {
        include!("taichi/program/kernel.h");
        type Kernel;
    }
}
*/
