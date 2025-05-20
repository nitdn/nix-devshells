{
  description = "A collection of minimal flakes I use";

  outputs =
    { self }:
    {
      templates = {
        rust.path = ./rust;
      };
    };
}
