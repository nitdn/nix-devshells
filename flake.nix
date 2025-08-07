{
  description = "A collection of minimal flakes I use";

  outputs =
    { self }:
    {
      templates.rust = {
        path = ./rust;
        description = "Rust templates";
        welcomeText = ''
          To use this template, change packages.default
          to your project name, like:
          `packages.default = self'.packages.<project_name>`.
          Also change it on Cargo.toml if you haven't already
        '';
      };
      templates.rust-advanced = {
        path = ./rust-advanced;
        description = "Advanced Rust template";
        welcomeText = ''
          This is a special template which uses wrappers.
          It is highly recommended to run `git-init` afterwards,
          and add your files.
        '';
      };
      templates.rust-workspaces = {
        path = ./rust-workspaces;
        description = "Rust template with workspaces";
        welcomeText = ''
          This is a template with workspaces and jj vcs in the
          devshell. You can use `jj git init`.
        '';
      };
    };
}
