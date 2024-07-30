
#[cxx::bridge(namespace = "taichi::lang")]
pub mod bridge {
    unsafe extern "C++" {
        include!("taichi/ir/shim.h");
        pub type IRBuilder;
        pub fn ir_builder_new() -> UniquePtr<IRBuilder>;
    }
}