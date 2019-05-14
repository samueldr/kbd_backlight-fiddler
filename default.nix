{ rustPlatform, lib, ... }:

rustPlatform.buildRustPackage {
  pname = "kbd_backlight-fiddler";
  version = "0.2.0";

  src = lib.cleanSource ./.;

  cargoSha256 = "1s57iy3bp3d88r5bwz73j2g4izm7gqk1wzr9239zxf927gk942f7";

  meta = with lib; {
  };
}
