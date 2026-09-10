{
  description = "NoxIDE, a Rust eframe editor built with Nox";

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
            src = builtins.path {
              path = ./.;
              name = "noxide-source";
            };
            nativeBuildInputs = [
                noxBinary
                pkgs.rustc
                pkgs.cargo
            ];
            dontConfigure = true;
            buildPhase = ''
              nox setup build
              nox build build
            '';
            installPhase = ''
              install -Dm755 build/debug/noxide/noxide $out/bin/noxide
              install -Dm644 assets/icon.icns $out/share/icons/noxide.icns
                ${pkgs.lib.optionalString pkgs.stdenv.hostPlatform.isDarwin ''
                mkdir -p $out/Applications/NoxIDE.app/Contents/MacOS
                mkdir -p $out/Applications/NoxIDE.app/Contents/Resources
                install -Dm755 $out/bin/noxide $out/Applications/NoxIDE.app/Contents/MacOS/NoxIDE
                install -Dm644 assets/icon.icns $out/Applications/NoxIDE.app/Contents/Resources/icon.icns
                printf '%s\n' '<?xml version="1.0" encoding="UTF-8"?>' '<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">' '<plist version="1.0"><dict><key>CFBundleDisplayName</key><string>NoxIDE</string><key>CFBundleExecutable</key><string>NoxIDE</string><key>CFBundleIconFile</key><string>icon.icns</string><key>CFBundleIdentifier</key><string>dev.noxide.editor</string><key>CFBundleName</key><string>NoxIDE</string><key>CFBundlePackageType</key><string>APPL</string><key>CFBundleVersion</key><string>${builtins.readFile ./VERSION}</string></dict></plist>' > $out/Applications/NoxIDE.app/Contents/Info.plist
              ''}
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
              pkgs.rustc
              pkgs.cargo
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
