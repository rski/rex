let
   pkgs = import <nixpkgs>{};
in
pkgs.rustPlatform.buildRustPackage rec {
    name= "rex";
    src = builtins.path { path = ./.; name = "rex"; };

    propagatedBuildInputs = [ pkgs.rustfmt ];

    cargoSha256 = "1nk56f5n9zkc2rjnzcaiis2yp00s8zx1dacrvxm74spazqfkxq5r";
}
