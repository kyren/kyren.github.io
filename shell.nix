with import <nixpkgs> {};
let
  moz_overlay = import (
    builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz
  );

  nixpkgs = import <nixpkgs> {
    overlays = [ moz_overlay ];
  };

  jekyll_env = bundlerEnv rec {
    name = "jekyll_env";
    ruby = ruby_2_5;
    gemfile = ./Gemfile;
    lockfile = ./Gemfile.lock;
    gemset = ./gemset.nix;
  };

  rust = (nixpkgs.rustChannelOf {
    date = "2018-09-24";
    channel = "nightly";
  }).rust.override {
    targets = [ "wasm32-unknown-unknown" ];
    extensions = [ "rust-src" ];
  };

  wasm-bindgen-version = "0.2.23";
  wasm-bindgen-lockfile = ./_scripts/wasm-bindgen-Cargo.lock;

  wasm-bindgen-src = stdenv.mkDerivation rec {
    version = wasm-bindgen-version;
    name = "wasm-bindgen-src-${version}";

    src = fetchFromGitHub {
      owner = "rustwasm";
      repo = "wasm-bindgen";
      rev = "${version}";
      sha256 = "0bf4iya71ikci2asmynmj7yi1y2happqif1aq8y808jmalfk9f37";
    };

    installPhase = ''
      mkdir -p $out
      cp -r * $out/
      cp ${wasm-bindgen-lockfile} $out/Cargo.lock
      touch $out/crates/cli/Cargo.lock
    '';
  };

  wasm-bindgen-cli = rustPlatform.buildRustPackage rec {
    version = wasm-bindgen-version;
    name = "wasm-bindgen-cli-${wasm-bindgen-version}";

    src = wasm-bindgen-src;
    sourceRoot = "${src.name}/crates/cli";

    cargoSha256 = "16wxnx34svkjiik70hm3gr0z8pvgarrd00hkbh8gs1kpskd87jp4";
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
      wasm-bindgen-cli
    ];
  }
