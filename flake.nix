{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        myRust = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analysis"];
        };
      in
        {
          devShell = pkgs.mkShell {
            buildInputs = with pkgs; [
              nixpkgs-fmt
              rust-analyzer
              myRust

              self.packages.${system}.aoc-get
            ];

            RUST_SRC_PATH = "${pkgs.rust-bin.stable.latest.rust-src}/lib/rustlib/src/rust/library";
          };

          packages = {
            aoc-get = pkgs.callPackage ./aoc-get {};
          };
        });
}
