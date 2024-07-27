{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, rust-overlay, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        osxDependencies = with pkgs;
          lib.optionals stdenv.isDarwin
          [ darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.CoreServices
          ];

        cargoTomlContents = builtins.readFile ./Cargo.toml;
        version = (builtins.fromTOML cargoTomlContents).package.version;

        mainPkg = pkgs.rustPlatform.buildRustPackage {
          inherit version;

          name = "cldb";

          buildInputs = with pkgs; [ openssl ] ++ osxDependencies;
          nativeBuildInputs = with pkgs; [ pkg-config openssl.dev ];

          src = pkgs.lib.cleanSourceWith { src = self; };

          cargoLock.lockFile = ./Cargo.lock;


          meta = with pkgs.lib; {
            description = "Cardano Lightning Demo Backend";
            homepage = "https://cardano-lightning.org";
            # license = licenses.mit;
          };
        };

        packages = {
          cldb = mainPkg;
          default = packages.cldb;
        };

        overlays.default = final: prev: { cldb = packages.cldb; };

        gitRev = if (builtins.hasAttr "rev" self) then self.rev else "dirty";
      in {
        inherit packages overlays;

        devShell = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              pkg-config
              openssl
              cargo-insta
              (pkgs.rust-bin.stable.latest.default.override {
                extensions = [ "rust-src" "clippy" "rustfmt" "rust-analyzer"];
                targets = [ "wasm32-unknown-unknown" ];
              })
              # nodePackages_latest.nodejs
              # nodePackages_latest.typescript-language-server
              cmake
              wasm-pack
              protobuf
              sqlite
            ] ++ osxDependencies;

          shellHook = ''
            export GIT_REVISION=${gitRev}
          '';
        };
      });
}
