with import <nixpkgs> { };
rustPlatform.buildRustPackage rec {
  name = "rex";
  src = nix-gitignore.gitignoreSource [ ] ./.;

  propagatedBuildInputs = [ rustfmt ];
  cargoLock = { lockFile = ./Cargo.lock; };
}
