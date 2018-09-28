with import <nixpkgs> {};

let
  jekyll_env = bundlerEnv rec {
    name = "jekyll_env";
    ruby = ruby_2_5;
    gemfile = ./Gemfile;
    lockfile = ./Gemfile.lock;
    gemset = ./gemset.nix;
  };
  moz_overlay = import (
    builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz
  );
  nixpkgs = import <nixpkgs> {
    overlays = [ moz_overlay ];
  };
  rust = (nixpkgs.rustChannelOf {
    date = "2018-09-24";
    channel = "nightly";
  }).rust.override {
    targets = [ "wasm32-unknown-unknown" ];
    extensions = [ "rust-src" ];
  };
in
  with nixpkgs;
  mkShell rec {
    buildInputs = [
      bundler
      bundix
      jekyll_env
      rust
      rustracer
      cargo-web
    ];
  }
