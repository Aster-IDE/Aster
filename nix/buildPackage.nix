{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage {
  pname = "aster";
  version = "0.1.0";
  
  src = ./.;
  
  cargoLock.lockFile = ./Cargo.lock;
  
  nativeBuildInputs = with pkgs; [
    pkg-config
  ];
  
  buildInputs = with pkgs; [
  ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Cocoa
    darwin.apple_sdk.frameworks.CoreGraphics
  ];
  
  meta = with pkgs.lib; {
    description = "A Simpler Text Editor written in Rust";
    homepage = "https://github.com/Aster-IDE/Aster";
    license = licenses.gpl3;
    platforms = platforms.all;
  };
}
