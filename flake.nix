{
  description = "A collection of minimal flakes I use";

  outputs =
    { ... }:
    {
      templates.rust = {
        path = ./rust;
        description = "Rust templates";
        welcomeText = ''
          To use this template, change packages.default
          to your project name, like:
          `packages.default = self'.packages.<project_name>`.
          Also change it on Cargo.toml if you haven't already.
          This comes with jj-vcs so you should use `jj git init`.
        '';
      };
      templates.ocaml = {
        path = ./ocaml;
        description = "Ocaml templates";
      };
    };
}
