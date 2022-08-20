{
    pkgs ? import <nixpkgs> { },
    pkgsLinux ? import <nixpkgs> { system = "x86_64-linux"; },
    imageTag ? "latest"
}:

let 
    cargo_nix = import ./Cargo.nix {inherit pkgs; };
    nixos = pkgs.dockerTools.pullImage {
        imageName = "nixos/nix";
        imageDigest =
            "sha256:1ec5b4a6bee82fc5bb93f782d08fc58029715dde166139c7164c39fa5db75d23";
        finalImageName = "nix";
        finalImageTag = "2.10.3";
        sha256 = "5b5c41dd457865f273896a5b901fe306b15b1e036239c849179432720e8d85d4";
        os = "linux";
        arch = "x86_64";
    };
in
    pkgs.dockerTools.buildImage {
        name = "ghcr.io/yuiyukihira/romanticise";
        tag = imageTag;
        
        fromImage = nixos;
        fromImageName = "nix";
        fromImageTag = "2.10.3";
        config = {
            Cmd = [ "${cargo_nix.rootCrate.build}/bin/romanticise"];
        };
    }
