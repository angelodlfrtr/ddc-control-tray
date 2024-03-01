{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.gtk3
    pkgs.systemd
    pkgs.libayatana-appindicator

    # keep this line if you use bash
    pkgs.bashInteractive
  ];

  # lib app indicator not use pkg-config ?
  shellHook = ''
    export LD_LIBRARY_PATH="${pkgs.libayatana-appindicator}/lib"
  '';

  nativeBuildInputs = [
    pkgs.pkg-config
    pkgs.cargo
    pkgs.rustc
  ];
}
