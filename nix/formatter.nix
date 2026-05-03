{ pkgs ? import <nixpkgs> {} }:

pkgs.writeShellScriptBin "format" ''
  set -e
  echo "formatting"
  cargo fmt
  echo "formatting complete"
''
