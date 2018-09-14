with (import <nixpkgs> {});
let
  env = bundlerEnv {
    name = "kyren-blog";
    inherit ruby;
    gemfile = ./Gemfile;
    lockfile = ./Gemfile.lock;
    gemset = ./gemset.nix;
  };
in stdenv.mkDerivation {
  name = "kyren-blog";
  buildInputs = [env ruby];
}

