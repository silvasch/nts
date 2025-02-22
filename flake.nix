{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
  };
  outputs =
    {
      flake-utils,
      naersk,
      nixpkgs,
      self,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk { };
      in
      {
        devShell = pkgs.mkShell {
          packages = with pkgs; [
            just

            cargo
            rustc
            rust-analyzer
            clippy
            rustfmt

            git-cliff
          ];
        };

        packages.nts = naersk'.buildPackage {
          pname = "nts";
          src = ./.;
        };

        packages.nts_set_pwd = naersk'.buildPackage {
          pname = "nts_set_pwd";
          src = ./.;
        };

        packages.default = self.packages.${system}.nts;
      }
    );
}
