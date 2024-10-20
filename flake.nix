{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        commonAttrs = {
          name = "nixpkgs-failed";
          src = self;
          cargoHash = "sha256-0Wn1WZLettqEsVd9rz+/WWro3tOo6D8gaNVKXMH0ao0=";
          patchPhase = ''
            substituteInPlace src/main.rs --replace-fail x86_64-linux "${system}"
          '';
          doCheck = false;
          meta.mainProgram = "nixpkgs-failed";
        };
      in rec {
        packages.default =
          pkgs.pkgsStatic.rustPlatform.buildRustPackage commonAttrs;
        devShells.default = pkgs.rustPlatform.buildRustPackage (commonAttrs // {
          nativeBuildInputs = with pkgs; [ clippy rustfmt cargo-audit ];
        });
        formatter = pkgs.nixfmt-classic;
      });
}
