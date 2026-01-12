{
  description = "A Rust devshell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
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
            sqruff
            taplo
            (rust-bin.stable.latest.default.override {
              extensions = [ "llvm-tools-preview" "rust-analyzer" "rust-src" ];
            })
            (import ./scripts/init.nix { inherit pkgs; })
          ] ++ lib.optionals stdenv.isLinux [ pkgs.cargo-llvm-cov pkgs.clang pkgs.mold ];

          RUSTFLAGS = if pkgs.stdenv.isLinux then "-C linker=clang -C link-arg=-fuse-ld=${pkgs.mold}/bin/mold" else "";

          shellHook = /*bash*/ ''
            if [ "$(ulimit -Sn)" -lt "8192" ]; then
              >&2 echo "⚠️ ulimit too small. Run 'ulimit -Sn 8192' to avoid problems running tests"
            fi
          '';
        };
      }
    );
}

