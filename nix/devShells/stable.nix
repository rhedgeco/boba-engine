{
  pkgs,
  inputs,
  ...
}:
pkgs.callPackage ./build.nix {
  inherit inputs;
  name = "rust-stable";
}
