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
            cargoHash = "sha256-gMKARLNvd52yWzA4tUAFAp85SZtMo/a0uF+jpnEArxU=";
            patchPhase = ''
              substituteInPlace src/main.rs --replace-fail x86_64-linux "${system}"
            '';
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
