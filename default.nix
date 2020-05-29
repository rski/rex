with import <nixpkgs>{};

rustPlatform.buildRustPackage rec {
    pname = "rex-${version}";
    version = "dev";
    src = ./.;

    cargoSha256 = "1nk56f5n9zkc2rjnzcaiis2yp00s8zx1dacrvxm74spazqfkxq5r";
}
