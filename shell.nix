{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell rec {
  packages = with pkgs; [
  ];

  buildInputs = with pkgs; [
    xorg.libX11
    xorg.libXcursor
  ];

  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
}
