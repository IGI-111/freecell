{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
        libPath = with pkgs;
          lib.makeLibraryPath [
            libGL
            libxkbcommon
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            alsa-lib
            udev
          ];

      in rec {
        # `nix build`
        packages.freecell = naersk-lib.buildPackage {
          pname = "freecell";
          root = ./.;
          buildInputs = with pkgs; [ pkg-config alsa-lib udev ];
          nativeBuildInputs = with pkgs; [ makeWrapper ];
          postInstall = ''
            wrapProgram "$out/bin/freecell" --prefix LD_LIBRARY_PATH : "${libPath}"
          '';

        };
        defaultPackage = packages.freecell;

        # `nix run`
        apps.freecell = flake-utils.lib.mkApp { drv = packages.freecell; };
        defaultApp = apps.freecell;

        # `nix develop`
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            rustfmt
            rustPackages.clippy
            rust-analyzer

            pkg-config
            alsa-lib
            udev
          ];
          LD_LIBRARY_PATH = libPath;
        };
      });
}
