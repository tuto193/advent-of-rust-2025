{ pkgs ? import <nixpkgs> {} }:
let
  # rustVersion = "1.86.0";
  rustupToolchain = "stable";

  rustBuildTargetTriple = "x86_64-unknown-linux-musl";
  rustBuildHostTriple = "x86_64-unknown-linux-gnu";
  libPath = with pkgs; lib.makeLibraryPath [
    # load external libraries that you need in your rust project here
  ];
  # rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
  #   extensions = [
  #     "rust-src" # for rust-analyzer
  #     "rust-analyzer"
  #   ];
  # };
  # Our windows cross package set.
  # pkgs-cross-mingw = import pkgs.path {
  #   crossSystem = {
  #       config = "x86_64-w64-mingw32";
  #     };
  # };

  # Our windows cross compiler plus
  # the required libraries and headers.
  # mingw_w64_cc = pkgs-cross-mingw.stdenv.cc;
  # mingw_w64 = pkgs-cross-mingw.windows.mingw_w64;
  # mingw_w64_pthreads_w_static = pkgs-cross-mingw.windows.mingw_w64_pthreads.overrideAttrs (oldAttrs: {
  #   # TODO: Remove once / if changed successfully upstreamed.
  #   configureFlags = (oldAttrs.configureFlags or []) ++ [
  #     # Rustc require 'libpthread.a' when targeting 'x86_64-pc-windows-gnu'.
  #     # Enabling this makes it work out of the box instead of failing.
  #     "--enable-static"
  #   ];
  # });

  # wine = pkgs.wineWowPackages.stable;

in

pkgs.mkShell rec {
  buildInputs =
  # [
  #   rust
  # ] ++ (
    with pkgs; [
    clang
    llvmPackages.bintools
    # Testing / running produced executables and for `winedump`.
    mold
    # Easier toml file manipulations via `tomlq` for quick
    # experiments when needed.
    rustup
  ];
  # Avoid polluting home dir with local project stuff.
  RUSTUP_HOME = toString ./.rustup;
  CARGO_HOME = toString ./.cargo;

  RUSTUP_TOOLCHAIN = rustupToolchain;

  # Set windows as the default cargo target so that we don't
  # have use the `--target` argument on every `cargo` invocation.
  CARGO_BUILD_TARGET = rustBuildTargetTriple;
  # Set wine as our cargo runner to allow the `run` and `test`
  # command to work.
  # CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUNNER = "${wine}/bin/wine64";

  LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
  shellHook = ''
    export PATH=$PATH:${CARGO_HOME}/bin
    export PATH=$PATH:${RUSTUP_HOME}/toolchains/${rustupToolchain}-${rustBuildHostTriple}/bin/

    # Ensures our windows target is added via rustup.
    rustup target add "${rustBuildTargetTriple}"
    rustup default stable
    cargo install aoc-cli --version 0.12.0
    '';
  RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
  ]);
  LD_LIBRARY_PATH = libPath;
  # Add glibc, clang, glib, and other headers to bindgen search path
  BINDGEN_EXTRA_CLANG_ARGS =
  # Includes normal include path
  (builtins.map (a: ''-I"${a}/include"'') [
    # add dev libraries here (e.g. pkgs.libvmi.dev)
    pkgs.glibc.dev
  ]);
}
