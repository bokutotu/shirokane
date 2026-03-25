{
  description = "shirokane - A blank Haskell project";

  nixConfig = {
    extra-substituters = [
      "https://cache.nixos.org"
      "https://cache.iog.io"
    ];
    extra-trusted-public-keys = [
      "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
      "hydra.iohk.io:f/Ea+s+dFdN+3Y/G+FDgSq+a5NEWhJGzdjvKNGv0/EQ="
    ];
    experimental-features = [ "nix-command" "flakes" ];
    allow-import-from-derivation = true;
  };

  inputs = {
    haskellNix.url = "github:input-output-hk/haskell.nix";
    nixpkgs.follows = "haskellNix/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, haskellNix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ haskellNix.overlay ];
          inherit (haskellNix) config;
        };

        src = pkgs.runCommand "shirokane-hpack-src" {
          nativeBuildInputs = [ pkgs.haskellPackages.hpack ];
        } ''
          mkdir -p $out
          cp -r ${./.}/. $out
          chmod -R +w $out
          cd $out
          hpack .
        '';

        project = pkgs.haskell-nix.project' {
          inherit src;
          compiler-nix-name = "ghc9123";
        };

        exe = project.hsPkgs.shirokane.components.exes.shirokane;
      in {
        devShells.default = project.shellFor {
          buildInputs = [
            pkgs.cabal-install
            pkgs.haskellPackages.hpack
            pkgs.git
            pkgs.nixpkgs-fmt
          ];
        };

        apps.default = {
          type = "app";
          program = "${exe}/bin/shirokane";
        };

        formatter = pkgs.nixpkgs-fmt;

        packages = {
          default = exe;
        };
      });
}
