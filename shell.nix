{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    nativeBuildInputs = [
        pkgs.rustup
        pkgs.gnumake
        pkgs.cargo-edit
        pkgs.bacon
        pkgs.crate2nix
        pkgs.openssl
        pkgs.cachix
        pkgs.sqlx-cli
        pkgs.yq
	    pkgs.docker
	    pkgs.nix
    ];
}
