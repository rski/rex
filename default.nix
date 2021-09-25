with import <nixpkgs> { };
rustPlatform.buildRustPackage rec {
  name = "rex";
  src = nix-gitignore.gitignoreSource [ ] ./.;
  cargoLock = { lockFile = ./Cargo.lock; };
}
