{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.SDL2
    pkgs.pkg-config
  ];
}
