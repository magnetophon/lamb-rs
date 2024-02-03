let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in with nixpkgs;
  stdenv.mkDerivation {
    name = "moz_overlay_shell";
    nativeBuildInputs = [ pkg-config clang lld ];
    buildInputs = [
      (nixpkgs.rustChannelOf { date = "2024-02-01"; channel = "nightly"; }).rust
      rustup
      pkg-config

      libjack2
      alsa-lib

      libGL
      xorg.libXcursor
      xorg.libX11 # libX11-xcb.so
      xorg.xcbutilwm # libxcb-icccm.so

      # python3

      faust
    ];

  }
