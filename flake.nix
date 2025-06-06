{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    utils,
    rust-overlay,
    ...
  }:
    utils.lib.eachDefaultSystem (system: let
      name = "stackless-python-2.7.1-bindgen";

      overlays = [
        rust-overlay.overlays.default
      ];

      pkgs = import nixpkgs {
        inherit system overlays;
      };

      stacklessSrc = pkgs.fetchFromGitHub {
        owner = "stackless-dev";
        repo = "stackless";
        rev = "v2.7.1-slp";
        sha256 = "sha256-KwWkgXPC0apqTpW0Zc82uJMWzutyIJSvOs5RO+MFt9g=";
      };

      pythonHeaders = pkgs.stdenv.mkDerivation {
        pname = "stackless-python-headers";
        version = "2.7.1-slp";
        src = stacklessSrc;
        nativeBuildInputs = [pkgs.autoconf pkgs.automake pkgs.libtool];
        configurePhase = "./configure --prefix=$out";
        installPhase = ''
          mkdir -p $out/include/python2.7
          cp -r Include/* $out/include/python2.7
          cp -r Stackless/core/ $out/include/python2.7
          cp -r Stackless/module/ $out/include/python2.7
          cp Stackless/stackless*.h $out/include/python2.7
          cp -r Stackless/platf $out/include/python2.7
          cp PC/pyconfig.h $out/include/python2.7
        '';
        dontBuild = true;
      };
    in {
      devShells.default = let
        rust-sources = pkgs.rust-bin.stable.latest.rust-src;
        rustToolchain = pkgs.rust-bin.stable.latest.default;
      in
        pkgs.mkShell {
          name = "${name}-devshell";

          packages = [
            rustToolchain
            rust-sources
            pkgs.clippy
            pkgs.rustfmt
            pkgs.rust-analyzer
            pkgs.alejandra
            pkgs.vhs
            pkgs.clang
            pkgs.llvmPackages.libclang
            pkgs.rust-bindgen
            pythonHeaders
          ];

          shellHook = ''
            export PYTHON_HEADER_PATH=${pythonHeaders}/include/python2.7
            export BINDGEN_EXTRA_CLANG_ARGS="-I$PYTHON_HEADER_PATH"
            export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
            export RUST_SRC_PATH=${rust-sources}
          '';
        };
    });
}
