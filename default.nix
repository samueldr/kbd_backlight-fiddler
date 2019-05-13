{ rustPlatform, lib, ... }:

rustPlatform.buildRustPackage {
  pname = "kbd_backlight-fiddler";
  version = "0.0.1";

  src = lib.cleanSource ./.;

  cargoSha256 = "0jacm96l1gw9nxwavqi1x4669cg6lzy9hr18zjpwlcyb3qkw9z7f";

  meta = with lib; {
  };
}
