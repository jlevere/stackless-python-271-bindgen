{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    utils,
    ...
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};

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
      packages = {
        pythonHeaders = pythonHeaders;
      };
    });
}
