{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    helix = {
      url = "github:helix-editor/helix/master";
      inputs.rust-overlay.follows = "rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        # To import a flake module
        # 1. Add foo to inputs
        # 2. Add foo as a parameter to the outputs function
        # 3. Add here: foo.flakeModule
        inputs.treefmt-nix.flakeModule
      ];
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
          inherit (inputs) nixpkgs rust-overlay;

          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };

          buildRustCrateForPkgs =
            crate:
            pkgs.buildRustCrate.override {
              rustc = pkgs.rust-bin.stable.latest.default;
              cargo = pkgs.rust-bin.stable.latest.default;
            };

          buildInputs = with pkgs; [
            expat
            fontconfig
            freetype
            freetype.dev
            libGL
            pkg-config
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            wayland
            libxkbcommon
          ];

          cargo_nix = pkgs.callPackage ./Cargo.nix {
            inherit pkgs buildRustCrateForPkgs;
          };
        in
        {
          # Per-system attributes can be defined here. The self' and inputs'
          # module parameters provide easy access to attributes of the same
          # system.
          packages.default = config.packages.hello;
          packages.hello = cargo_nix.rootCrate.build;

          checks.rustnix = cargo_nix.rootCrate.build.override {
            runTests = true;
          };

          # An example configuration
          treefmt.programs = {
            nixfmt.enable = true;
            rustfmt.enable = true;
            shfmt.enable = true;
            sqlfluff.enable = true;
            sqlfluff.dialect = "postgres";
            typstyle.enable = true;
            taplo.enable = true;
          };

          treefmt.settings.excludes = [ "Cargo.nix" ];

          devShells.default = pkgs.mkShell {
            packages = [
              pkgs.bacon
              pkgs.cargo
              pkgs.clippy
              pkgs.rustfmt
              pkgs.rustc
              pkgs.rust-analyzer
              pkgs.crate2nix
              pkgs.llvmPackages.lldb
              inputs'.helix.packages.default
            ];
            # An example enviroment setup
            # LD_LIBRARY_PATH = builtins.toString (pkgs.lib.makeLibraryPath buildInputs);
          };
        };
    };
  nixConfig = {
    extra-substituters = [ "https://helix.cachix.org" ];
    extra-trusted-public-keys = [ "helix.cachix.org-1:ejp9KQpR1FBI2onstMQ34yogDm4OgU2ru6lIwPvuCVs=" ];
  };
}
