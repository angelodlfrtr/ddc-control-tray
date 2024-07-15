{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};
      in rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;

          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            pkg-config
          ];

          buildInputs = with pkgs; [
            gtk3
            systemd
            libayatana-appindicator
            makeWrapper
          ];

          postInstall = ''
            wrapProgram $out/bin/ddc-control-tray --set LD_LIBRARY_PATH ${pkgs.libayatana-appindicator}/lib
          '';
        };

        # For `nix develop` (optional, can be skipped):
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            pkg-config
          ];

          buildInputs = with pkgs; [
            gtk3
            systemd
            libayatana-appindicator
          ];

          LD_LIBRARY_PATH = "${pkgs.libayatana-appindicator}/lib";
          shellHook = ''
            LD_LIBRARY_PATH="${pkgs.libayatana-appindicator}/lib"
          '';
        };
      }
    );
}
