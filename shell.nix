{ utils ? import ./utils.nix
, pkgs ? import <nixpkgs> {
    overlays = [
      (utils.importRepo { user = "oxalica"; repo = "rust-overlay"; branch = "master"; })
    ];
  }
}:

let
  thorConfig = {
    # Enable rust
    rust.enable = true;
    # Shell to use for `nix-shell`
    # Note that the shell used for
    # `nix-build` will be bash,
    # regardless of this config value.
    shell = "zsh";
  };
  thor = utils.importRepo { user = "dblanovschi"; repo = "thor"; }
    { inherit pkgs; config = thorConfig; };
in
with thor.rust.toolchainCommons;
thor.rust.mkRustDerivation {
  action = "dev";

  name = "xaoc-shell";

  toolchain = {
    toolchain = nightly;
    targets = [
      targets.x86_64-linux-gnu
      targets.x86_64-linux-musl
    ];
    defaultTarget = targets.x86_64-linux-musl;
  };

  buildInputs = with pkgs; [ nixpkgs-fmt ];

  cargoAliases = { };

  enableIncremental = true;

  shellAliases =
    {
      # cargo run
      cr = "cargo run";
      crr = "cargo run --release";

      # cargo fmt
      cf = "cargo fmt -- --emit=files";
    };

  phases.build = false;
}
