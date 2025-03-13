{
  name,
  system,
  inputs,
  version ? "stable",
  profile ? "default",
  packages ? [],
  ...
}: let
  # apply the rust overlay to nixpkgs
  pkgs = import inputs.nixpkgs {
    inherit system;
    overlays = [(import inputs.rust-overlay)];
  };

  # get rust binary version and profile
  rust-bin = pkgs.rust-bin.${version}.latest.${profile}.override {
    extensions = ["rust-src"];
  };
in
  pkgs.mkShell {
    name = "${name}";
    RUST_SRC_PATH = "${rust-bin}/lib/rustlib/src/rust/library";
    buildInputs =
      packages
      ++ [
        pkgs.cargo-expand
        pkgs.rust-analyzer
        pkgs.just
        rust-bin
      ];
  }
