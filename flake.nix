{
  description = "A simple tile climbing game using bevy.";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        manifest = (pkgs.lib.importTOML ./Cargo.toml);
        # Pinning rustc to this version as future versions break backwards compatibility,
        # and don't let the game compile. More info here:
        # https://www.flypig.co.uk/gecko?&list_id=1181&list=gecko
        rustVersion = "1.68.0";
        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.rust-bin.stable.${rustVersion}.minimal;
          rustc = pkgs.rust-bin.stable.${rustVersion}.minimal;
        };
      in {
        packages.default = rustPlatform.buildRustPackage rec {
          pname = manifest.package.name;
          version = manifest.package.version;
          src = pkgs.lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;
          cargoLock.allowBuiltinFetchGit = true;
          buildType = "release";

          nativeBuildInputs = with pkgs; [
            makeWrapper
            pkg-config
          ];

          buildInputs = with pkgs; [
            alsa-lib
            udev
            xorg.libxcb
          ];

          postInstall = ''
            wrapProgram $out/bin/${pname} \
              --prefix LD_LIBRARY_PATH : "${pkgs.lib.makeLibraryPath (with pkgs; [
                libGL
                libxkbcommon
                wayland
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                xorg.libXrandr
                vulkan-loader
              ])}"
          '';
        };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            (pkgs.rust-bin.stable.${rustVersion}.default)
            pkg-config
          ];

          buildInputs = with pkgs; [
            alsa-lib
            udev
            xorg.libxcb
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
            libGL
            libxkbcommon
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            vulkan-loader
          ]);
        };
      }
    );
}
