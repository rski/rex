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

    cargoSha256 = "0s5kq8ga7i8q3pj5cdyx4ipinw3ljvkxxr8wg5w8ld6hpwvc0hhm";
}
