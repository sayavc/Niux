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
        date = builtins.substring 0 8 self.lastModifiedDate;
        formatted = "${builtins.substring 0 4 date}-${builtins.substring 4 2 date}-${builtins.substring 6 2 date}";
        rev = self.rev or "dirty";
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = "${manifest.version}-${formatted}-${builtins.substring 0 7 rev}";

          nativeBuildInputs = [ pkgs.installShellFiles ];

          postInstall = ''
           installShellCompletion --zsh --name _niux ${./completions/_niux}
           installShellCompletion --bash ${./completions/niux.bash}
           installShellCompletion --fish ${./completions/niux.fish}
          '';

          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

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
