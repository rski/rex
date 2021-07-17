let
  pkgs = import <nixpkgs>{};
  srcNoTarget = dir:
    builtins.filterSource
    (path: type: type != "directory" || builtins.baseNameOf path != "target")
    dir;
in
pkgs.rustPlatform.buildRustPackage rec {
    name= "rex";
    src = srcNoTarget ./.;

    propagatedBuildInputs = [ pkgs.rustfmt ];
    cargoLock = { lockFile = ./Cargo.lock; };
}
