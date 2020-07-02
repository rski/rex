with import <nixpkgs>{};

rustPlatform.buildRustPackage rec {
    name= "rex";
    src = builtins.path { path = ./.; name = "rex"; };

    cargoSha256 = "1nk56f5n9zkc2rjnzcaiis2yp00s8zx1dacrvxm74spazqfkxq5r";
}
