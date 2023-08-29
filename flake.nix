{
  description = "Xv6";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; 
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils,... }: let
    lib = {
      inherit (flake-utils.lib) defaultSystems eachSystem;
    };
    supportedSystems = [ "x86_64-linux" ];
  in lib.eachSystem supportedSystems (system: let
    nightlyVersion = "2023-01-15";
    pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (import rust-overlay)
        ];
      };
    pinnedRust = pkgs.rust-bin.nightly.${nightlyVersion}.default.override {
      extensions = ["rustc-dev" "rust-src" "rust-analyzer-preview" "llvm-tools-preview"];
      targets = [ "riscv64gc-unknown-none-elf" ];
    };
    # rustPlatform = pkgs.makeRustPlatform {
    #   rustc = pinnedRust;
    #   cargo = pinnedRust;
    # };
    #cargoPlay = pkgs.cargo-feature.override { inherit rustPlatform; };
  in {
    
devShell = pkgs.pkgsCross.riscv64.mkShell {
  nativeBuildInputs = with pkgs; [
    qemu
    gdb
    cargo-binutils
  ] ++ [pinnedRust ];

  shellHook = ''
  '';
};

  });
}
