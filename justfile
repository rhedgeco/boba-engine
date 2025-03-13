default:
    just --list

dev VERSION="default":
    nix develop .#{{VERSION}} -c bash -c "SHELL=$SHELL $SHELL"
