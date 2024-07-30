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

    let include = dst.join("include");
    let mut cc = autocxx_build::Builder::new("src/lib.rs", [&include]).build().unwrap();
    cc.define("TI_INCLUDED", "true")
        .include(include)
        .compile("chirpy");
}
