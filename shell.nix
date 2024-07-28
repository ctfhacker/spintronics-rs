let

  orig_pkgs = import (fetchTarball("channel:nixpkgs-23.11-darwin")) {};

  rust-overlay = import(orig_pkgs.fetchFromGitHub {
    owner = "oxalica";
    repo = "rust-overlay";
    rev = "56baac5e6b2743d4730e664ea64f6d8a2aad0fbb";
    sha256 = "sha256-G7/gHz8ORRvHd1/RIURrdcswKRPe9K0FsIYR4v5jSWo=";
  });

  pkgs = orig_pkgs.extend rust-overlay;

in pkgs.mkShell {
  buildInputs = with pkgs; [ 
    rust-bin.nightly.latest.default
    rust-analyzer
    pkg-config
    jless
  ];
}
