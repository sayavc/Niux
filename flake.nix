{
  description = "niux — declarative NixOS package manager";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;

          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = [ pkgs.installShellFiles ];

          postInstall = ''
           installShellCompletion --cmd niux \
           --bash <($out/bin/niux --completions bash) \
           --zsh <($out/bin/niux --completions zsh) \
           --fish <($out/bin/niux --completions fish)
          '';

          meta = {
            description = "Declarative NixOS package manager";
            homepage = "https://github.com/sayavc/niux";
            license = pkgs.lib.licenses.gpl3Only;
            mainProgram = "niux";
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            rust-analyzer
            clippy
            rustfmt
          ];
        };
      });
}
