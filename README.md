# niux

Declarative NixOS/home-manager CLI package manager written in Rust.

## Features

- Fast and lightweight command-line interface
- Manage home and system packages declaratively
- Built with Rust for performance and reliability
- Simple and intuitive command syntax
- Supports both standalone and module home-manager
- Supports NixOS with and without flakes

## Requirements
- NixOS

## Installation
Add to your `flake.nix` inputs:

```nix
inputs.niux = {
    url = "github:sayavc/niux";
    inputs.nixpkgs.follows = "nixpkgs";
};
```

Then add to your `home.nix`:

```nix
{ inputs, pkgs, ... }: {
    home.packages = [
        inputs.niux.packages.${pkgs.system}.default
    ];
}
```

Run `home-manager switch` to apply.

## Configuration

First, generate the default config:
```bash
niux --gen-config
```

Or at a custom path:
```bash
niux --gen-config --default-path-config ~/my/path/niux.kdl
```

> **Note:** `--default-path-config` requires an existing `.kdl` file. Always run `--gen-config` first.

## Usage

### Quick Start
```bash
niux -Hi firefox        # Install firefox for home
niux -Si vim            # Install vim for system
niux -HSia firefox vim  # Install both + rebuild

niux -Hr firefox        # Remove firefox from home

niux -Hl                # List home packages
niux -l firefox         # Search everywhere

niux -U                 # Update all flakes
niux -USHa              # Update + rebuild everything

niux -HSa               # Rebuild both configs
```

### Installation & Removal
```bash
niux -Hi firefox            # Install firefox for home
niux -Hia firefox           # Install and rebuild home
niux -Si vim                # Install vim for system
niux -Sia vim               # Install and rebuild system
niux -Hi firefox vim        # Install multiple packages for home
niux -Si firefox vim        # Install multiple packages for system
niux -HSia firefox vim      # Install for both + rebuild

niux -Hr firefox            # Remove firefox from home
niux -Hra firefox           # Remove and rebuild home
niux -Sr vim                # Remove vim from system
niux -Sra vim               # Remove and rebuild system
niux -Hr firefox vim        # Remove multiple from home
niux -Sr firefox vim        # Remove multiple from system
```

### Listing & Search
```bash
niux -l                     # List all packages
niux -Hl                    # List home packages
niux -Sl                    # List system packages
niux -l firefox             # Search everywhere
niux -Hl firefox            # Search in home
niux -Sl firefox            # Search in system
niux -l firefox vim         # Search multiple
```

### Updates
```bash
niux -U                     # Update all flakes
niux -U nixpkgs             # Update specific flake input
niux -HUa                   # Update + rebuild home
niux -SUa                   # Update + rebuild system
niux -USHa                  # Update + rebuild everything
niux -HUa nixpkgs           # Update nixpkgs + rebuild home
niux -SUa nixpkgs           # Update nixpkgs + rebuild system
```

### Build & Apply
```bash
niux -Ha                    # Rebuild home config
niux -Sa                    # Rebuild system config
niux -HSa                   # Rebuild both configs
```

### Cleanup
```bash
niux --clear                # Run nix-collect-garbage
```

## Commands Reference

| Flag | Description |
|------|-------------|
| `-H, --home` | Target home packages |
| `-S, --system` | Target system packages |
| `-i, --install` | Install packages |
| `-r, --remove` | Remove packages |
| `-a, --apply` | Apply and rebuild configuration |
| `-l, --list` | List or search packages |
| `-U, --update` | Update flakes |
| `--gen-config` | Generate default configuration |
| `--default-path-config` | Use custom config path |
| `--clear` | Run garbage collection |

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Author

Created by [sayavc](https://github.com/sayavc)
