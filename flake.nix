{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    flake-utils.url = "github:numtide/flake-utils";

    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, naersk, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
          ];
        };

        rust = pkgs.rust-bin.stable.latest.default;
        rust-dev = rust.override {
          extensions = [ "rust-src" ];
        };

        naersk-lib = pkgs.callPackage naersk {
          cargo = rust;
          rustc = rust;
        };

        name = "five-letter-words";
        src = ./.;
      in
      rec {
        packages."${name}" = naersk-lib.buildPackage {
          inherit src;
          pname = name;
        };
        packages.default = packages."${name}";

        apps."${name}" = flake-utils.lib.mkApp {
          drv = packages."${name}";
        };
        apps.default = apps."${name}";

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [ ];

          nativeBuildInputs = with pkgs; [
            rust-dev
            rust-analyzer

            rnix-lsp
          ];
        };
      }
    );
}
