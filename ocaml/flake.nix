{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        let
          nativeBuildInputs = with pkgs.ocamlPackages; [
            findlib
            dune_3
            batteries
          ];
        in
        {
          devShells.default = pkgs.mkShell {
            packages = [
              pkgs.ocamlPackages.ocaml-lsp
              pkgs.ocamlPackages.utop
            ];
            inherit nativeBuildInputs;
          };
        };
    };
}
