let
  moz_overlay = import (
    builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/c8a2ed7e614131ea1ba3d31ef9bcc9890a0df410.tar.gz
  );

  nixpkgs = import (
    builtins.fetchTarball https://github.com/NixOS/nixpkgs-channels/archive/796a8764ab85746f916e2cc8f6a9a5fc6d4d03ac.tar.gz
  ) {
    overlays = [ moz_overlay ];
  };
in with nixpkgs;
let
  rust_channel = rustChannelOf {
    date = "2019-03-20";
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

  wasm-bindgen-version = "0.2.40";
  wasm-bindgen-sha = "0s9g6bjj9gnd02iqlhda3wf7w5lc7sf22r88xm6zcyba8jacd40p";
  wasm-bindgen-lockfile = ./_scripts/wasm-bindgen-Cargo.lock;
  wasm-bindgen-cargo-sha = "0w6y31r41f1d27rfgm69hm3wnnr7n9lwzz7qn4vgd1zcc7na5bal";

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

    cargoSha256 = wasm-bindgen-cargo-sha;
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
