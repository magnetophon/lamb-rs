let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  faust = nixpkgs.faust.overrideAttrs (old: {
    version = "2.72.15";
    src = nixpkgs.fetchFromGitHub {
      owner = "grame-cncm";
      repo = "faust";
      rev = "bde0c9e3168a6da9e953367856099100e9537490";
      sha256 = "sha256-/fcxdOPm6KxVSoMpTAR2cfQgxW8bVOOp8xnN1MIRf44=";
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
      python3
    ];
  }
