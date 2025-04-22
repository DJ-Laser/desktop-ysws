{
  description = "Desktop devShell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};

      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = ["rust-src"];
      };

      nativeLibraries = with pkgs; [
        # Rust
        rustToolchain

        # misc. libraries
        openssl
        pkg-config

        # GUI libs
        libxkbcommon
        libGL
        fontconfig

        # wayland libraries
        wayland

        # x11 libraries
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi
        xorg.libX11
      ];
    in
      with pkgs; {
        devShells.default = mkShell {
          buildInputs =
            nativeLibraries
            ++ [
              alejandra
            ];

          LD_LIBRARY_PATH = "${lib.makeLibraryPath nativeLibraries}";
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      });
}
