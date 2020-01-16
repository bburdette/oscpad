nix-build -E 'with import <nixpkgs> { }; callPackage ./default.nix {
  inherit (darwin.apple_sdk.frameworks) Security; }'
