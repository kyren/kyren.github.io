let
  moz_overlay = import (
    builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz
  );

  nixpkgs = import <nixpkgs> {
    overlays = [ moz_overlay ];
  };
in with nixpkgs;
let
  rust_channel = rustChannelOf {
    date = "2018-09-24";
    channel = "nightly";
  };

  rust = rust_channel.rust.override {
    targets = [ "wasm32-unknown-unknown" ];
    extensions = [ "rust-src" ];
  };

  jekyll_env = bundlerEnv rec {
    name = "jekyll_env";
    ruby = ruby_2_5;
    gemfile = ./Gemfile;
    lockfile = ./Gemfile.lock;
    gemset = ./gemset.nix;
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
    '';
  };

  buildRustPackage = callPackage (import <nixpkgs/pkgs/build-support/rust>) {
    rust = {
      rustc = rust_channel.rust;
      cargo = rust_channel.cargo;
    };
  };

  wasm-bindgen-cli = buildRustPackage rec {
    version = wasm-bindgen-version;
    name = "wasm-bindgen-cli-${wasm-bindgen-version}";

    buildInputs = [ openssl ];
    nativeBuildInputs = [ pkgconfig ];

    src = wasm-bindgen-src;
    cargoBuildFlags = [ "-p wasm-bindgen-cli" ];

    cargoSha256 = "16wxnx34svkjiik70hm3gr0z8pvgarrd00hkbh8gs1kpskd87jp4";
  };
in
  mkShell rec {
    buildInputs = [
      bundler
      bundix
      jekyll_env
      rust
      rustracer
      wasm-bindgen-cli
      nodejs
    ];
  }
