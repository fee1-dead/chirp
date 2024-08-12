use std::env;
use std::ptr::null_mut;

// temp for testing
use autocxx::prelude::*;
use chirp_sys::taichi::lang::{Callable, IRNode, Kernel, SNode, SNodeType};
use chirp_wrapper::*;

fn main() {
    unsafe {
        env::set_var("TI_LIB_DIR", chirp_sys::TAICHI_LIBDIR)
    }
/*
    let arch = chirp_sys::taichi::Arch::x64;
    let mut prog = chirp_sys::taichi::lang::Program::new1(arch.clone()).within_box();
    let snode = unsafe {
        SNode::new1(c_int(0), SNodeType::root, null_mut(), null_mut())
    };
    prog.as_mut().add_snode_tree(snode.within_unique_ptr(), true);
    let mut aot_builder = prog.as_mut().make_aot_module_builder(arch, &cxx::CxxVector::new());
*/
    let arch = Arch::X64;
    let mut prog = Program::new(arch.clone());
    let snode = chirp_wrapper::SNode::new(0, chirp_wrapper::SNodeType::Root);
    prog.add_snode_tree(snode, true);
    let mut aot_builder = prog.make_aot_module_builder(arch, &cxx::CxxVector::new());



/*
    let mut builder = chirp_sys::taichi::lang::IRBuilder::new().within_box();
    let ty = chirp_sys::taichi::lang::PrimitiveType::get(chirp_sys::taichi::lang::PrimitiveTypeID::u32_);
    let v = builder.as_mut().create_local_var(ty.within_unique_ptr());
    let value = builder.as_mut().get_uint32(42);

    unsafe { 
        builder.as_mut().create_local_store(v, value.cast());
        let loaded = builder.as_mut().create_local_load(v);
        builder.as_mut().create_return(loaded.cast());
    }
 */
    let mut builder = IRBuilder::new();
    let ty = DataType::from(PrimTy::U32);
    let v = builder.create_local_var(ty);
    let value = builder.origin().get_uint32(42);

    builder.create_local_store(v, value.cast());
    let loaded = builder.create_local_load(v);
    builder.create_return(loaded.cast());
    


    cxx::let_cxx_string!(ident = "foo");
    let ir = unsafe {
        UniquePtr::from_raw(builder.origin().extract_ir().into_raw().cast::<IRNode>())
    };

    let ir2 = Box::pin(ir);
    let kernel = Kernel::new2(prog.origin(), ir2, &ident, chirp_sys::AutodiffMode::kNone).within_unique_ptr();
    let kernel = kernel.into_raw();
    let mut callable = unsafe {
        UniquePtr::from_raw(kernel.cast::<Callable>())
    };
    let ty = chirp_sys::taichi::lang::PrimitiveType::get(chirp_sys::taichi::lang::PrimitiveTypeID::u32_);
    callable.as_mut().unwrap().insert_ret(&*ty.within_box().as_ref());
    callable.as_mut().unwrap().finalize_rets();

    unsafe {
        aot_builder.as_mut().unwrap().add(&ident, kernel);
    }
    cxx::let_cxx_string!(out_dir = "./chirp-output/");
    cxx::let_cxx_string!(filename = "");
    aot_builder.dump(&out_dir, &filename);
}