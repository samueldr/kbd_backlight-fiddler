{ pkgs, ... }:

{
  security.wrappers.kbd_backlight-fiddler =
    let
      kbd_backlight-fiddler = pkgs.callPackage ./package.nix {};
    in
    {
      source = "${kbd_backlight-fiddler}/bin/kbd_backlight-fiddler";
    };
}
