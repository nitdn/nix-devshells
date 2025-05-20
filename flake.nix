{
  description = "A collection of minimal flakes I use";

  outputs =
    { self }:
    {
      templates = {
        rust.path = ./rust;
        rust.welcomeText = ''
          To use this template, change packages.default
          to your project name, like:
          `packages.default = self'.packages.<project_name>`.
          Also change it on Cargo.toml if you haven't already
        '';
      };
    };
}
