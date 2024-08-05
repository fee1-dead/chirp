use std::ptr::null_mut;

// temp for testing
use autocxx::prelude::*;
use chirp_sys::taichi::lang::{IRNode, Kernel, SNode, SNodeType};

fn main() {
    let arch = chirp_sys::taichi::Arch::x64;
    let mut prog = chirp_sys::taichi::lang::Program::new1(arch.clone()).within_box();

    let snode = unsafe {
        SNode::new1(c_int(0), SNodeType::root, null_mut(), null_mut())
    };
    prog.as_mut().add_snode_tree(snode.within_unique_ptr(), true);
    let mut aot_builder = prog.as_mut().make_aot_module_builder(arch, &cxx::CxxVector::new());
    //C++ Heap: within_unique_ptr()
    //Rust Heap: within_box()
    //Rust Stack: just new()
    let mut builder = chirp_sys::taichi::lang::IRBuilder::new().within_box();
    let ty = chirp_sys::taichi::lang::PrimitiveType::get(chirp_sys::taichi::lang::PrimitiveTypeID::u32_);
    let v = builder.as_mut().create_local_var(ty.within_unique_ptr());
    let value = builder.as_mut().get_uint32(42);

    unsafe { 
        builder.as_mut().create_local_store(v, value.cast());
        let loaded = builder.as_mut().create_local_load(v);
        builder.as_mut().create_return(loaded.cast());
    }

    cxx::let_cxx_string!(ident = "foo");
    let ir = unsafe {
        UniquePtr::from_raw(builder.as_mut().extract_ir().into_raw().cast::<IRNode>())
    };
    let ir2 = Box::pin(ir);
    let kernel = Kernel::new2(prog.as_mut(), ir2, &ident, chirp_sys::AutodiffMode::kNone).within_unique_ptr();

    unsafe {
        aot_builder.as_mut().unwrap().add(&ident, kernel.into_raw());
    }
    cxx::let_cxx_string!(out_dir = ".");
    cxx::let_cxx_string!(filename = "");
    aot_builder.dump(&out_dir, &filename);
    // let mut sum = builder.as_mut().create_local_var();
    // let mut sum_ptr = &sum;
    // unsafe {
    //     builder.as_mut().create_local_store(*sum_ptr, builder.as_mut().get_float32(0.2));
    // }
}