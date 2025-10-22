{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      crane,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;
        cargoMetas = (builtins.fromTOML (builtins.readFile (self + "/Cargo.toml")));
        version = cargoMetas.package.version;
        pname = cargoMetas.package.name;

        commonArgs = {
          strictDeps = true;
          src = ./.;

          nativeBuildInputs = with pkgs; [
            pkg-config
            makeWrapper
          ];

          buildInputs = with pkgs; [
            gtk3
            systemd
            libayatana-appindicator
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly (
          commonArgs
          // {
            inherit pname version;
          }
        );

        package = craneLib.buildPackage (
          commonArgs
          // {
            inherit
              pname
              version
              cargoArtifacts
              ;

            postInstall = ''
              wrapProgram $out/bin/ddc-control-tray --set LD_LIBRARY_PATH ${pkgs.libayatana-appindicator}/lib
            '';
          }
        );
      in
      {
        formatter = pkgs.nixfmt-rfc-style;

        devShell = craneLib.devShell {
          inputsFrom = [ package ];

          packages = with pkgs; [
            gnumake
            rust-analyzer
          ];

          LD_LIBRARY_PATH = "${pkgs.libayatana-appindicator}/lib";
        };

        # Default package
        packages.default = package;
      }
    );
}
