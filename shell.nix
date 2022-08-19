{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    nativeBuildInputs = [ pkgs.rustup pkgs.gnumake pkgs.cargo-edit pkgs.bacon pkgs.crate2nix ];
}
