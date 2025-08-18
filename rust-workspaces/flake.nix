{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-flake.url = "github:juspay/rust-flake";
    rust-flake.inputs.nixpkgs.follows = "nixpkgs";
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
            pkg-config
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
          packages.default = self'.packages.mandelbrot;
          # Per-system attributes can be defined here. The self' and inputs'
          # module parameters provide easy access to attributes of the same
          # system.
          rust-project.crates."mandelbrot".path = ./mandelbrot;
          # rust-project.crates."subcrate-example".crane = {
          # args = {
          #   inherit buildInputs;
          #   nativeBuildInputs = with pkgs; [
          #     makeWrapper
          #     pkg-config
          #   ];
          # };
          # extraBuildArgs = {
          #   postInstall = ''
          #     # The Space between LD_LIBRARY_PATH and : is very important
          #     wrapProgram $out/bin/subcrate-example --prefix LD_LIBRARY_PATH : \
          #     ${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}
          #   '';
          # };
          # };

          devShells.default = pkgs.mkShell {
            inputsFrom = [
              self'.devShells.rust
            ];
            packages = with pkgs; [
              self'.packages.graphing
              bacon
              just
              meld
              watchexec
              jj
            ];

            LD_LIBRARY_PATH = builtins.toString (pkgs.lib.makeLibraryPath buildInputs);
          };
        };
      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.

      };
    };
}
