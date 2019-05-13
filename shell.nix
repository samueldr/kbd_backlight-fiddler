{
  pkgs ? import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz))
    ];
  }
}:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    latest.rustChannels.stable.rust
  ];
  RUST_BACKTRACE = 1;
}
