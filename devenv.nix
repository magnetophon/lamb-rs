{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  # https://devenv.sh/basics/
  env.GREET = "magnetophon";

  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    # lldb
    # cargo
    # rustc
    # rustfmt
    # rust-analyzer
    # clippy
    # cargo-watch
    # cargo-nextest
    # cargo-expand # expand macros and inspect the output
    # cargo-llvm-lines # count number of lines of LLVM IR of a generic function
    # cargo-inspect
    # cargo-criterion
    # evcxr # make sure repl is in a gc-root
    # cargo-play # quickly run a rust file that has a maint function

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

  # https://devenv.sh/scripts/
  scripts.hello.exec = "echo hello from $GREET";

  enterShell = ''
    hello
    # git --version
    # fish
  '';

  # https://devenv.sh/tests/
  # enterTest = ''
  # echo "Running tests"
  # git --version | grep "2.42.0"
  # '';

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/languages/
  # https://devenv.sh/reference/options/#languagesrustchannel
  languages.rust = {
    enable = true;
    channel = "nightly";
  };

  # https://devenv.sh/pre-commit-hooks/
  pre-commit.hooks = {
    shellcheck.enable = true;
    clippy.enable = true;
    # hunspell.enable = true;
    alejandra.enable = true;
    rustfmt.enable = true;
    typos.enable = true;
  };

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping example.com";

  # See full reference at https://devenv.sh/reference/options/
}
