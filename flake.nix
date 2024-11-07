{
  description = "A Rust devshell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            bunyan-rs
            cargo-audit
            cargo-expand
            cargo-tarpaulin
            cargo-udeps
            docker-client
            pkg-config
            postgresql
            sqlx-cli
            taplo
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-analyzer" "rust-src" ];
            })
          ];

          shellHook = /*bash*/ ''
            git pull
            if [ "$(ulimit -Sn)" -lt "8192" ]; then
              >&2 echo "⚠️ ulimit too small. Run 'ulimit -Sn 8192' to avoid problems running tests"
            fi
          '';
        };
      }
    );
}

