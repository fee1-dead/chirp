// temp for testing
use chirp_wrapper::*;
use std::env;

fn main() {
    unsafe { env::set_var("TI_LIB_DIR", chirp_sys::TAICHI_LIBDIR) }
    /*
    // let arch = chirp_sys::taichi::Arch::x64;
    // let mut prog = chirp_sys::taichi::lang::Program::new1(arch.clone()).within_box();
    // let snode = unsafe {
    //     SNode::new1(c_int(0), SNodeType::root, null_mut(), null_mut())
    // };
    // prog.as_mut().add_snode_tree(snode.within_unique_ptr(), true);
    // let mut aot_builder = prog.as_mut().make_aot_module_builder(arch, &cxx::CxxVector::new());
     */
    // let arch = Arch::X64;
    let mut prog = Program::new(Arch::X64);
    let mut snode = SNode::new(0, SNodeType::Root);
    prog.add_snode_tree(&mut snode, true);
    let mut aot_builder = prog.make_aot_module_builder(Arch::X64, &CVec::new());

    /*
    //    let mut builder = chirp_sys::taichi::lang::IRBuilder::new().within_box();
    //    let ty = chirp_sys::taichi::lang::PrimitiveType::get(chirp_sys::taichi::lang::PrimitiveTypeID::u32_);
    //    let v = builder.as_mut().create_local_var(ty.within_unique_ptr());
    //    let value = builder.as_mut().get_uint32(42);

    //    unsafe {
    //        builder.as_mut().create_local_store(v, value.cast());
    //        let loaded = builder.as_mut().create_local_load(v);
    //        builder.as_mut().create_return(loaded.cast());
    //    }
    */
    let ident1 = "simple_ret";
    let mut kernel_simple_ret: Kernel;
    let mut kernel_init: Kernel;
    let mut kernel_ret: Kernel;
    /*
    @ti.kernel
    def ret() -> ti.f32:
      sum = 0.2
      return sum
     */
    {
        let mut builder = IRBuilder::new();
        let sum = builder.create_local_var(DataType::from(PrimTy::F32));
        let value = builder.get_float32(0.2);
        builder.create_local_store(&sum, value);
        let loaded = builder.create_local_load(&sum);
        builder.create_return(loaded);

        kernel_simple_ret =
            Kernel::from_ir(&mut prog, builder.extract_ir(), ident1, AutodiffMode::KNone);
        kernel_simple_ret.insert_ret(DataType::from(PrimTy::F32));
        kernel_simple_ret.finalize_rets();
    }

    {
        let mut builder = IRBuilder::new();
        let zero = builder.get_int32(0);
        let n_stmt = builder.get_int32(10);
        let looop = builder.create_range_for(zero, n_stmt, true, 1, 0, false);
        {}
    }
    /*
    // cxx::let_cxx_string!(ident = "foo");
    // let ir = unsafe {
    //     UniquePtr::from_raw(builder.origin().extract_ir().into_raw().cast::<IRNode>())
    // };
    // let ir2 = Box::pin(ir);
     */
    /*
        // let kernel = Kernel::new2(prog.origin(), ir2, &ident, chirp_sys::AutodiffMode::kNone).within_unique_ptr();
        // let kernel = kernel.into_raw();
        // let mut callable = unsafe {
        //     UniquePtr::from_raw(kernel.cast::<Callable>())
        // };
        // let ty = chirp_sys::taichi::lang::PrimitiveType::get(chirp_sys::taichi::lang::PrimitiveTypeID::u32_);
        // callable.as_mut().unwrap().insert_ret(&*ty.within_box().as_ref());
        // callable.as_mut().unwrap().finalize_rets();

        // unsafe {
        //     aot_builder.as_mut().unwrap().add(&ident, kernel);
        // }
        // aot_builder.dump(&out_dir, &filename);
    */
    let out_dir = "./chirp-output/";
    let filename = "cmcmmmma";

    aot_builder.add(ident1, &kernel_simple_ret);
    aot_builder.dump(out_dir, filename);
}
