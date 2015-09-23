{ stdenv, openssl }:

stdenv.mkDerivation rec {
  name = "meh-1.0";
  version = "1.0";
  rev = "1";
  src = ".";
  buildInputs = [openssl];
  meta = with stdenv.lib; {
    description = "An interactive Gtk canvas widget for graph-based interfaces";
    homepage = http://drobilla.net;
    license = licenses.gpl3;
    maintainers = [ maintainers.goibhniu ];
    platforms = platforms.linux;
  };
}
