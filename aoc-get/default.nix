{ lib
, curl
, fish
, makeWrapper
, stdenvNoCC }:
stdenvNoCC.mkDerivation rec {
  pname = "aoc-get";
  version = "0.1.0";

  src = ./aoc-get;

  buildInputs = [
    makeWrapper
    fish
  ];

  dontUnpack = true;
  dontBuild = true;

  wrapperPath = lib.makeBinPath [
    curl
  ];

  fixupPhase = ''
    patchShebangs $out/bin/${pname}
    wrapProgram $out/bin/${pname} --prefix PATH : "${wrapperPath}"
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp $src $out/bin/${pname}
    chmod a+x $out/bin/${pname}
  '';
}
