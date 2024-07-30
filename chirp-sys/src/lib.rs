

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