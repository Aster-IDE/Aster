{
  description = "A Simpler Text Editor written in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
        ];

        buildInputs = with pkgs; [
          # probably gonna have something for AsterIDE here
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
          RUST_BACKTRACE = "full";
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "aster";
          inherit ((pkgs.lib.importTOML ./Cargo.toml).workspace.package) version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          inherit buildInputs nativeBuildInputs;
        };
      });
}