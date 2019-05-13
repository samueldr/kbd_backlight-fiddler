{ stdenv, lib, cargo, rustc, ... }:

stdenv.mkDerivation {
  pname = "kbd_backlight-fiddler";
  version = "0.0.1";

  src = lib.cleanSource ./.;

  buildInputs = [
    cargo
  ];

  buildPhase = ''
    cargo build --release
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp target/release/kbd_backlight-fiddler $out/bin/
  '';
}
