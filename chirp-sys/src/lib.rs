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
    #include "taichi/ir/statements.h"
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

    generate!("taichi::lang::DecorationStmt")
    generate!("taichi::lang::ArgLoadStmt")
    generate!("taichi::lang::ExternalPtrStmt")
    generate!("taichi::lang::GlobalPtrStmt")
    generate!("taichi::lang::MatrixOfGlobalPtrStmt")
    generate!("taichi::lang::MatrixOfMatrixPtrStmt")
    generate!("taichi::lang::MatrixPtrStmt")
    generate!("taichi::lang::SNodeOpStmt")
    generate!("taichi::lang::ExternalTensorShapeAlongAxisStmt")
    generate!("taichi::lang::ExternalTensorBasePtrStmt")
    generate!("taichi::lang::AssertStmt")
    generate!("taichi::lang::ExternalFuncCallStmt")
    generate!("taichi::lang::RangeAssumptionStmt")
    generate!("taichi::lang::LoopUniqueStmt")
    generate!("taichi::lang::GlobalLoadStmt")
    generate!("taichi::lang::GlobalStoreStmt")
    generate!("taichi::lang::LocalStoreStmt")

    generate!("taichi::lang::PrintStmt")

    generate!("taichi::lang::FuncCallStmt")
    generate!("taichi::lang::ReferenceStmt")
    generate!("taichi::lang::GetElementStmt")

    generate!("taichi::lang::IntegerOffsetStmt")
    generate!("taichi::lang::LinearizeStmt")
    generate!("taichi::lang::GetRootStmt")
    generate!("taichi::lang::SNodeLookupStmt")
    generate!("taichi::lang::GetChStmt")
    generate!("taichi::lang::OffloadedStmt")

    generate!("taichi::lang::LoopLinearIndexStmt")
    generate!("taichi::lang::GlobalThreadIndexStmt")
    generate!("taichi::lang::BlockCornerIndexStmt")
    generate!("taichi::lang::GlobalTemporaryStmt")
    generate!("taichi::lang::ThreadLocalPtrStmt")
    generate!("taichi::lang::BlockLocalPtrStmt")
    generate!("taichi::lang::ClearListStmt")
    generate!("taichi::lang::InternalFuncStmt")
    generate!("taichi::lang::TexturePtrStmt")
    generate!("taichi::lang::TextureOpStmt")


    generate!("taichi::lang::AdStackPopStmt")
    generate!("taichi::lang::AdStackPushStmt")
    generate!("taichi::lang::AdStackAccAdjointStmt")

    generate!("taichi::lang::BitStructStoreStmt")

    generate!("taichi::lang::MeshIndexConversionStmt")

    generate!("taichi::lang::MatrixInitStmt")


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
