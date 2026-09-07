{
  description = "Noxide, a D project built with Nox";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nox.url = "github:playfairs/nox";
  };

  outputs =
    { self, nixpkgs, nox }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems =
        function: nixpkgs.lib.genAttrs systems (system: function nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (
        pkgs:
        let
          noxBinary = nox.packages.${pkgs.system}.default;
        in
        {
          default = pkgs.stdenv.mkDerivation {
            pname = "noxide";
            version = pkgs.lib.strings.trim (builtins.readFile ./VERSION);
            src = ./.;
            nativeBuildInputs = [
              noxBinary
              pkgs.ldc
            ];
            dontConfigure = true;
            buildPhase = ''
              nox setup build
              nox build build
            '';
            installPhase = ''
              install -Dm755 build/debug/noxide/noxide $out/bin/noxide
            '';
          };
        }
      );

      devShells = forAllSystems (
        pkgs:
        {
          default = pkgs.mkShell {
            packages = [
              nox.packages.${pkgs.system}.default
              pkgs.ldc
              pkgs.dub
            ];
          };
        }
      );

      checks = forAllSystems (pkgs: {
        package = self.packages.${pkgs.system}.default;
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt);
    };
}
