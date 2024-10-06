{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in rec
      {
        packages = {
          default = pkgs.rustPlatform.buildRustPackage {
            name = "nixpkgs-failed";
            src = self;
            cargoHash = "sha256-SwwbYXiP8O4l/WM3GIbscVbexEtyvUmN7Xv/DNdWfLs=";
            doCheck = false;
            meta.mainProgram = "nixpkgs-failed";
          };
        };
        devShells.default = packages.default.overrideAttrs (final: old: {
          nativeBuildInputs = old.nativeBuildInputs ++ (with pkgs; [ clippy rustfmt ]);
        });
      }
    );
}
