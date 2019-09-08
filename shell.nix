let 
  nixpkgs = import <nixpkgs> {};
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "oscpad-env";
    buildInputs = [ 
      cargo
      rustc
      pkgconfig
      openssl.dev 
      nix
      ];
    OPENSSL_DEV=openssl.dev;
  }
