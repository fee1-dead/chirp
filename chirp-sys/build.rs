use std::process::Command;

fn llvm_config(cfg: impl FnOnce(&mut Command) -> &mut Command) -> String {
    let mut cmd = Command::new("llvm-config");
    cfg(&mut cmd);
    String::from_utf8_lossy(&cmd.output().unwrap().stdout).into_owned()
}

fn main() {
    // println!("cargo:rerun-if-changed=taichi");
    // println!("cargo:rerun-if-changed=src/lib.rs");

    let dst = cmake::Config::new("taichi")
        .define("CMAKE_CXX_COMPILER", "clang++")
        .define("TI_WITH_PYTHON", "off")
        .define("TI_WITH_STATIC_C_API", "off")
        .define("TI_WITH_CUDA", "off")
        .define("USE_LLD", "off")
        .define("USE_MOLD", "on")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-search=native={}", dst.join("lib64").display());
    let libdir = llvm_config(|cmd| cmd.arg("--libdir"));
    println!("cargo:rustc-link-search=native={libdir}"); 

    println!("cargo:rustc-env=CHIRP_TAICHI_LIB_DIR={}", dst.join("python/taichi/_lib/runtime").display()); 

    println!("cargo:rustc-link-lib=static=taichi_core");
    println!("cargo:rustc-link-lib=static=taichi_common");
    println!("cargo:rustc-link-lib=static=taichi_util");
    println!("cargo:rustc-link-lib=static=ti_device_api");
    println!("cargo:rustc-link-lib=static=common_rhi");
    println!("cargo:rustc-link-lib=static=opengl_rhi");
    println!("cargo:rustc-link-lib=static=opengl_program_impl");
    println!("cargo:rustc-link-lib=static=llvm_rhi");


    // println!("cargo:rustc-link-lib=static=vulkan_rhi");
    // println!("cargo:rustc-link-lib=static=amdgpu_rhi");
    println!("cargo:rustc-link-lib=static=cpu_codegen");
    // println!("cargo:rustc-link-lib=static=interop_rhi");
    println!("cargo:rustc-link-lib=static=cpu_runtime");
    println!("cargo:rustc-link-lib=static=gfx_runtime");
    // println!("cargo:rustc-link-lib=static=cuda_runtime");
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
    println!("cargo:rustc-link-lib=static=llvm_program_impl");

    // println!("cargo:rustc-link-lib=static=vulkan_program_impl");
    // println!("cargo:rustc-link-lib=static=metal_rhi");
    // println!("cargo:rustc-link-lib=static=metal_program_impl");
    println!("cargo:rustc-link-lib=static=compilation_manager");
    // println!("cargo:rustc-link-lib=static=dx_rhi");
    
    println!("cargo:rustc-link-lib=static=glfw3");
    println!("cargo:rustc-link-lib=static=spirv-cross-core");
    println!("cargo:rustc-link-lib=static=spirv-cross-glsl");
    println!("cargo:rustc-link-lib=static=SPIRV-Tools");
    println!("cargo:rustc-link-lib=static=SPIRV-Tools-opt");
    let llvm_libnames = llvm_config(|cmd| cmd.args(["--link-static", "--libnames"]));
    let llvm_libnames = llvm_libnames.split([' ', '\n']).filter_map(|s| s.strip_prefix("lib")).filter_map(|s| s.strip_suffix(".a"));
    for lib in llvm_libnames {
        println!("cargo:rustc-link-lib=static={lib}");
    }

    let llvm_systemlibnames = llvm_config(|cmd| cmd.args(["--link-static", "--system-libs"]));
    let llvm_systemlibnames = llvm_systemlibnames.split([' ', '\n']).filter_map(|s| s.strip_prefix("-l"));
    for lib in llvm_systemlibnames {
        println!("cargo:rustc-link-lib=dylib={lib}");
    }

    let include = dst.join("include");

    let mut cc = autocxx_build::Builder::new("src/lib.rs", [&include])
        .extra_clang_args(&["-std=c++17", "-DTI_INCLUDED=true", "-w"])
        .build()
        .unwrap();
    cc.define("TI_INCLUDED", "true")
        .flag("-w")
        .flag_if_supported("-std=c++17")
        .compile("chirpy");
}
