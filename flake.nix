{
  description = "A collection of minimal flakes I use";

  outputs =
    { self }:
    {
      templates = {
        rust.path = ./rust;
        rust.description = "Rust templates";
        rust.welcomeText = ''
          To use this template, change packages.default
          to your project name, like:
          `packages.default = self'.packages.<project_name>`.
          Also change it on Cargo.toml if you haven't already
        '';
        rust-advanced.path = ./rust-advanced;
        rust-advanced.description = "Advanced Rust template";
        rust-advanced.welcomeText = ''
          This is a special template which uses wrappers.
          It is highly recommended to run `git-init` afterwards,
          and add your files.
        '';
      };
    };
}
