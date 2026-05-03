{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rust-bin.stable.latest.default
    pkg-config
  ];
  
  shellHook = ''
    export RUST_BACKTRACE="full"
  '';
}
