use autocxx::prelude::*;

pub mod ir_builder {
    autocxx::include_cpp! {
        #include "taichi/ir/ir_builder.h"
        // #include "taichi/ir/type.h"
        // #include "taichi/ir/ir.h"
        name!(ffi)
        safety!(unsafe_ffi)
        generate!("taichi::lang::IRBuilder")
    }
    pub use ffi::*;
}

#[cxx::bridge(namespace = "taichi::lang")]
pub mod bridge {
    #[repr(u16)]
    enum PrimTy {
        f16,
        f32,
        f64,
        i8,
        i16,
        i32,
        i64,
        u1,
        u8,
        u16,
        u32,
        u64,
        gen,
        unknown,
    }
    unsafe extern "C++" {
        include!("taichi/ir/shim.h");
        pub type IRBuilder;
        pub fn ir_builder_new() -> UniquePtr<IRBuilder>;
        pub type AllocaStmt;
        pub type PrimTy;
        pub unsafe fn ir_builder_create_local_var(builder: *mut IRBuilder, ty: PrimTy) -> *mut AllocaStmt;
    }
}
