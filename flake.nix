{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.default = with pkgs; (pkgs.mkShell.override { stdenv = clang18Stdenv; }) {
        buildInputs = 
          with python311Packages; [
            pkg-config
            openssl
            cmake
            pybind11
            llvm_15 # specifically required by taichi
            xorg.libX11
            xorg.libXrandr
            xorg.libXinerama
            xorg.libXcursor
            xorg.libXi
            xorg.libpthreadstubs
            libGL
            mold
          ];
        LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
        shellHook = ''
            export BINDGEN_EXTRA_CLANG_ARGS="$(< ${stdenv.cc}/nix-support/libc-crt1-cflags) \
            $(< ${stdenv.cc}/nix-support/libc-cflags) \
            $(< ${stdenv.cc}/nix-support/cc-cflags) \
            $(< ${stdenv.cc}/nix-support/libcxx-cxxflags) \
            ${lib.optionalString stdenv.cc.isClang "-idirafter ${stdenv.cc.cc}/lib/clang/${lib.getVersion stdenv.cc.cc}/include"} \
            ${lib.optionalString stdenv.cc.isGNU "-isystem ${stdenv.cc.cc}/include/c++/${lib.getVersion stdenv.cc.cc} -isystem ${stdenv.cc.cc}/include/c++/${lib.getVersion stdenv.cc.cc}/${stdenv.hostPlatform.config} -idirafter ${stdenv.cc.cc}/lib/gcc/${stdenv.hostPlatform.config}/${lib.getVersion stdenv.cc.cc}/include"} \
          "
        '';
      };
    });
}