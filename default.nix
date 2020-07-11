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

    cargoSha256 = "1nk56f5n9zkc2rjnzcaiis2yp00s8zx1dacrvxm74spazqfkxq5r";
}
