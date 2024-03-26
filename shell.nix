let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  faust = nixpkgs.faust.overrideAttrs (old: {
    version = "2.72.10";
    src = nixpkgs.fetchFromGitHub {
      owner = "grame-cncm";
      repo = "faust";
      rev = "3f2d4f81446f450ee1f5fb8bc4b86e6474feb997";
      sha256 = "sha256-aUvsIKbIGDGemUqdLtaGmbaqaBn99brQicGYLukX6JE=";
      fetchSubmodules = true;
    };
    patches = [];
  });
in with nixpkgs;
  stdenv.mkDerivation {
    name = "moz_overlay_shell";
    nativeBuildInputs = [ pkg-config clang lld ];
    buildInputs = [
      (nixpkgs.rustChannelOf { date = "2024-03-17"; channel = "nightly"; }).rust
      rustup
      pkg-config

      libjack2
      alsa-lib

      libGL
      xorg.libXcursor
      xorg.libX11 # libX11-xcb.so
      xorg.xcbutilwm # libxcb-icccm.so

      faust
    ];
  }
