{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-flake = {
      url = "github:juspay/rust-flake";
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
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
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
        in
        {
          # Per-system attributes can be defined here. The self' and inputs'
          # module parameters provide easy access to attributes of the same
          # system.
          packages.default = self'.packages.hello;
          # An example configuration
          rust-project.crates."hello".crane = {
            args = {
              inherit buildInputs;
              nativeBuildInputs = with pkgs; [
                # makeWrapper
                pkg-config
              ];
            };
            #   extraBuildArgs = {
            #     postInstall = ''
            #       # The Space between LD_LIBRARY_PATH and : is very important
            #       wrapProgram $out/bin/template --prefix LD_LIBRARY_PATH : \
            #       ${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}
            #     '';
            #   };
          };

          treefmt.programs = {
            nixfmt.enable = true;
            rustfmt.enable = true;
            shfmt.enable = true;
            sqlfluff.enable = true;
            sqlfluff.dialect = "postgres";
            typstyle.enable = true;
            taplo.enable = true;
          };

          devShells.default = pkgs.mkShell {
            inputsFrom = [
              self'.devShells.rust
            ];
            packages = [
              pkgs.bacon
              pkgs.vscode-extensions.vadimcn.vscode-lldb.adapter
              pkgs.jujutsu
            ];
            # An example enviroment setup
            # LD_LIBRARY_PATH = builtins.toString (pkgs.lib.makeLibraryPath buildInputs);
          };
        };
    };
}
