{
  pkgs,
  inputs,
  ...
}: rec {
  # make default shell an alias for stable
  default = stable;

  # build all other devShells
  stable = pkgs.callPackage ./stable.nix {inherit inputs;};
  nightly = pkgs.callPackage ./nightly.nix {inherit inputs;};
  vscode = pkgs.callPackage ./vscode.nix {inherit inputs;};
}
