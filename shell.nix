let
  moz_overlay = import (
    builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/cebceca52d54c3df371c2265903f008c7a72980b.tar.gz
  );

  nixpkgs = import (
    builtins.fetchTarball https://github.com/NixOS/nixpkgs-channels/archive/5f3be9bc4b45f9da2d7c6768c0d187ef3f2643f7.tar.gz
  ) {
    overlays = [ moz_overlay ];
  };
in with nixpkgs;
let
  rust_channel = rustChannelOf {
    date = "2019-03-14";
    channel = "nightly";
  };

  rust = rust_channel.rust.override {
    targets = [ "wasm32-unknown-unknown" ];
    extensions = [ "rust-src" "rustfmt-preview" "rls-preview" ];
  };

  jekyll_env = bundlerEnv rec {
    name = "jekyll_env";
    ruby = ruby_2_5;
    gemfile = ./Gemfile;
    lockfile = ./Gemfile.lock;
    gemset = ./gemset.nix;
  };

  wasm-bindgen-version = "0.2.27";
  wasm-bindgen-sha = "0fl2gghqcfry2jkgk82rrkx3cqph9yqnkialpqh1dyhsmpkq365h";
  wasm-bindgen-lockfile = ./_scripts/wasm-bindgen-Cargo.lock;
  wasm-bindgen-cargo-sha = "09hrsz3ndzn2zbl86s1b8p2br9m1np4i5sdkgqdyfw37170ilf6m";

  wasm-bindgen-src = stdenv.mkDerivation rec {
    version = wasm-bindgen-version;
    name = "wasm-bindgen-src-${version}";

    src = fetchFromGitHub {
      owner = "rustwasm";
      repo = "wasm-bindgen";
      rev = "${version}";
      sha256 = wasm-bindgen-sha;
    };

    installPhase = ''
      mkdir -p $out
      cp -r * $out/
      cp ${wasm-bindgen-lockfile} $out/Cargo.lock
    '';
  };

  buildRustPackage = rustPlatform.buildRustPackage.override {
    rustc = rust_channel.rust;
    cargo = rust_channel.cargo;
  };

  wasm-bindgen-cli = buildRustPackage rec {
    version = wasm-bindgen-version;
    name = "wasm-bindgen-cli-${wasm-bindgen-version}";

    buildInputs = [ openssl ];
    nativeBuildInputs = [ pkgconfig ];

    src = wasm-bindgen-src;
    cargoBuildFlags = [ "-p wasm-bindgen-cli" ];

    cargoSha256 = "09hrsz3ndzn2zbl86s1b8p2br9m1np4i5sdkgqdyfw37170ilf6m";
  };
in
  mkShell rec {
    buildInputs = [
      bundler
      bundix
      jekyll_env
      rust
      wasm-bindgen-cli
      python3
    ];
  }
