{
  description = "A rust library project got inspiration from bilibili-api.";

  inputs = {
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = pkgs.rustPlatform;
        deps = with pkgs; [
          openssl
          sqlite
        ];
      in
      {
        # Used by `nix develop`
        devShells.default = pkgs.mkShell {
          buildInputs =
            with pkgs;
            [
              (with toolchain; [
                cargo
                rustc
                rustLibSrc
              ])
              clippy
              rustfmt
              pkg-config
            ]
            ++ deps;

          # Specify the rust-src path (many editors rely on this)
          RUST_SRC_PATH = "${toolchain.rustLibSrc}";

          shellHook = ''
            alias x='cargo xtask'
            if [ -f Session.vim ]; then
              exec nvim -S Session.vim
            fi
            cargo --version
            exec $SHELL
          '';
        };
      }
    );
}
