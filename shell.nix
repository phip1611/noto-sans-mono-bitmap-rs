{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell rec {
  packages = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    xorg.libX11
    xorg.libXcursor
    libxkbcommon
  ];

  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
}
