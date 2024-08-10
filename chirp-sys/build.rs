fn main() {
    println!("cargo:rerun-if-changed=taichi");
    println!("cargo:rerun-if-changed=src/main.rs");

    let dst = cmake::Config::new("taichi")
        .define("CMAKE_CXX_COMPILER", "clang++")
        .define("TI_WITH_PYTHON", "off")
        .define("TI_WITH_STATIC_C_API", "off")
        .define("USE_LLD", "off")
        .define("USE_MOLD", "on")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=taichi_core");
    println!("cargo:rustc-link-lib=static=taichi_common");
    println!("cargo:rustc-link-lib=static=taichi_util");
    println!("cargo:rustc-link-lib=static=ti_device_api");
    println!("cargo:rustc-link-lib=static=opengl_rhi");
    println!("cargo:rustc-link-lib=static=opengl_program_impl");
    println!("cargo:rustc-link-lib=static=cuda_rhi");


    // println!("cargo:rustc-link-lib=static=vulkan_rhi");
    // println!("cargo:rustc-link-lib=static=amdgpu_rhi");
    println!("cargo:rustc-link-lib=static=cpu_codegen");
    // println!("cargo:rustc-link-lib=static=interop_rhi");
    println!("cargo:rustc-link-lib=static=cpu_runtime");
    println!("cargo:rustc-link-lib=static=gfx_runtime");
    // println!("cargo:rustc-link-lib=static=cuda_codegen");
    // println!("cargo:rustc-link-lib=static=dx12_runtime");
    // println!("cargo:rustc-link-lib=static=dx12_codegen");
    println!("cargo:rustc-link-lib=static=llvm_codegen");
    println!("cargo:rustc-link-lib=static=llvm_runtime");
    println!("cargo:rustc-link-lib=static=cpu_rhi");
    println!("cargo:rustc-link-lib=static=spirv_codegen");

    // println!("cargo:rustc-link-lib=static=amdgpu_codegen");
    println!("cargo:rustc-link-lib=static=gfx_program_impl");
    // println!("cargo:rustc-link-lib=static=dx_program_impl");
    // println!("cargo:rustc-link-lib=static=dx12_program_impl");

    // println!("cargo:rustc-link-lib=static=vulkan_program_impl");
    // println!("cargo:rustc-link-lib=static=metal_rhi");
    // println!("cargo:rustc-link-lib=static=metal_program_impl");
    println!("cargo:rustc-link-lib=static=compilation_manager");
    // println!("cargo:rustc-link-lib=static=dx_rhi");
    
    // println!("cargo:rustc-link-lib=static=dx12_rhi");
    // println!("cargo:rustc-link-lib=static=llvm");
    //glfw
    let include = dst.join("include");

    let mut cc = autocxx_build::Builder::new("src/lib.rs", [&include])
        .extra_clang_args(&["-std=c++17", "-DTI_INCLUDED=true"])
        .build()
        .unwrap();
    cc.define("TI_INCLUDED", "true")
        .flag_if_supported("-std=c++17")
        .compile("chirpy");
}
