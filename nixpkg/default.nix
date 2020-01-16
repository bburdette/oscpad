{ stdenv, openssl, fetchFromGitHub, rustPlatform, Security, pkgconfig }:

rustPlatform.buildRustPackage rec {
  pname = "oscpad";
  version = "1.0";

  src = fetchFromGitHub {
    owner = "bburdette";
    repo = pname;
    rev = "v0.2.3";
    sha256 = "1h23wsn4ckny4shh5i5dl5mgsvcdyphssh1q2ws7ai5aq5ni7ngn";
  };

  cargoSha256 = "15189k48qkj9nwr24861hg10p2xi8mgs42j0v60h0c41w6m1zki8";
  # dontMakeSourcesWritable=1;

  nativeBuildInputs = [ pkgconfig ];

  buildInputs = [(stdenv.lib.optional stdenv.isDarwin Security)
    openssl];

  meta = with stdenv.lib; {
    description = "osc touch screen gadget";
    homepage = https://github.com/bburdette/oscpad;
    license = with licenses; [ bsd3 ];
    maintainers = [ ];
    platforms = platforms.all;
  };
}

