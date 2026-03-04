{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";

    package = pkgs.callPackage ./default.nix {};
  in {
    packages.x86_64-linux.default = package;

    devShells.x86_64-linux.default = pkgs.mkShell {
      inputsFrom = [package];

      packages = with pkgs; [
        rust-analyzer
        rustfmt
        clippy
      ];
    };
  };
}
