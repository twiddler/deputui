{
  description = "Provides the rust toolchain and a nix package";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11"; # You can pin a version here
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "outdatui";
        version = "0.0.1";
        src = pkgs.lib.cleanSource ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
      };

      devShells.${system}.default = pkgs.mkShell {
        name = "rust-dev-shell";

        packages = with pkgs; [
          cargo
          rustc
          rustfmt
        ];
      };
    };
}
