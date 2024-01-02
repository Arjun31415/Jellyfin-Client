{
  description = "jellyfin client in rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {nixpkgs, ...} @ inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      perSystem = {
        config,
        system,
        lib,
        ...
      }: let
        overlays = [(import inputs.rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            vscode-extensions.llvm-org.lldb-vscode
            taplo
            openssl.dev
            openssl
            pkg-config
            rust-bin.stable.latest.default
          ];
        };
        formatter = pkgs.alejandra;
      };
    };
}
