{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };
      in with pkgs; {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            pkg-config
            openssl
          ];
        };
      });
}

# vim: et ts=2 sw=2 sts=2 ai si ft=nix
