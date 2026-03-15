{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    naersk,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages."${system}";
    naerskLib = pkgs.callPackage naersk {};
  in {
    devShells."${system}".default = pkgs.mkShell {
      buildInputs = with pkgs; [
        cargo
        rustc
        rustfmt
        clippy
        rust-analyzer
        openssl
      ];
      nativeBuildInputs = [pkgs.pkg-config];
      env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };
    packages."${system}".default = naerskLib.buildPackage {
      src = ./.;
    };
  };
}
