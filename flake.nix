{
  description = "JPCS NUD Portal";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    sqlx-cli-pin = {
      url = "github:NixOS/nixpkgs/1d4c88323ac36805d09657d13a5273aea1b34f0c";
      flake = false;
    };
  };

  outputs = inputs @ { self, flake-parts, fenix, ... }: flake-parts.lib.mkFlake { inherit inputs; } {
    systems = import inputs.systems;

    perSystem = { system, ... }: let
      pinned-pkgs-overlay = final: prev: {
        sqlx-cli = (import inputs.sqlx-cli-pin { inherit system; }).sqlx-cli;
      };

      pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [ pinned-pkgs-overlay ];
      };

      fenix-lib = fenix.packages.${system};
      rust-stable = fenix-lib.fromToolchainName {
        name = "1.91.1";
        sha256 = "sha256-SDu4snEWjuZU475PERvu+iO50Mi39KVjqCeJeNvpguU=";
      };

      rust-toolchain = fenix-lib.combine [
        rust-stable.cargo
        rust-stable.rustc
        fenix-lib.targets."wasm32-unknown-unknown".stable.rust-std
      ];

      dioxus-version = "0.6.3";

      dioxus-cli-asset-map = {
        "x86_64-linux" = "dx-x86_64-unknown-linux-gnu-v${dioxus-version}.tar.gz";
        "x86_64-darwin" = "dx-x86_64-unknown-linux-gnu-v${dioxus-version}.tar.gz"; 
        "aarch64-darwin" = "dx-aarch64-apple-darwin-v${dioxus-version}.tar.gz";
        "x86_64-windows" = "dx-x86_64-pc-windows-msvc-v${dioxus-version}.zip";
        "aarch64-windows" = "dx-aarch64-pc-windows-msvc-v${dioxus-version}.zip";
      };

      dioxus-asset = dioxus-cli-asset-map.${system} or (abort "Unsupported system: ${system}");
      dioxus-url = "https://github.com/DioxusLabs/dioxus/releases/download/v${dioxus-version}/${dioxus-asset}";
    in {
      packages.dioxus-cli = pkgs.stdenv.mkDerivation {
        pname = "dx";
        version = dioxus-version;

        src = pkgs.fetchurl {
          url = dioxus-url;
          sha256 = "sha256-LS4gW62XFRQQGexVjhmHTTkix4A2VumLpFGMGKDiIZY=";
        };

        unpackPhase = ''
          case "${dioxus-asset}" in
            *.zip)
              unzip $src
              ;;
            *.tar.gz|*.tgz)
              tar -xvf $src
              ;;
            *)
              echo "Unknown archive format: ${dioxus-asset}"
              exit 1
              ;;
          esac
        '';

        installPhase = ''
          mkdir -p $out/bin
          cp dx $out/bin/
        '';

        meta = with pkgs.lib; {
          description = "Dioxus CLI";
          longDescription = "Dioxus CLI pinned to v0.6.3";
          homepage = "https://dioxuslabs.com";
          license = licenses.mit;
          platforms = platforms.all;
        };
      };

      devShells.frontend = pkgs.mkShell {
        name = "jpcsnud-portal-frontend";
        packages = [
          pkgs.tailwindcss_4
          self.packages.${system}.dioxus-cli
          rust-toolchain
        ];
        shellHook = ''
          echo "Starting frontend dev environment..."
          tailwindcss -i tailwind.css --config tailwind.config.js -o assets/tailwind.css --watch >/dev/null 2>&1 &
          dx serve
        '';
      };

      devShells.backend = pkgs.mkShell {
        name = "jpcsnud-portal-backend";
        packages = [
          pkgs.openssl
          pkgs.pkg-config
          pkgs.sqlx-cli
          pkgs.postgresql_17
          rust-toolchain
          (pkgs.writeShellScriptBin "sql" (builtins.readFile ./backend/scripts/sql.sh))
        ];

        shellHook = ''
          BACKEND_DIR=$(pwd)/backend
          POSTGRES_STARTED=0

          on_exit() {
            echo "Leaving dev shell"
            if [ $POSTGRES_STARTED -eq 0 ]; then
              exit 0
            fi

            pg_ctl -D $BACKEND_DIR/pgdata stop

            if [ $? -ne 0 ]; then
              echo "Error: pg_ctl stop failed" >&2
              exit $?
            fi
          }

          trap on_exit EXIT

          mkdir -p $BACKEND_DIR/migrations
          mkdir -p $BACKEND_DIR/pg{log,data,socket}
          if [ ! -f "$BACKEND_DIR/pgdata/PG_VERSION" ]; then
              echo "Initializing database..."
              initdb -D "$BACKEND_DIR/pgdata"
              if [ $? -ne 0 ]; then
                  echo "Error: initdb failed" >&2
                  exit 1
              fi
          fi

          echo "Starting PostgreSQL..."
          pg_ctl -D $BACKEND_DIR/pgdata status
          if [ $? -ne 0 ]; then
            pg_ctl \
              -D $BACKEND_DIR/pgdata \
              -o "-k $BACKEND_DIR/pgsocket -h 127.0.0.1" \
              -l $BACKEND_DIR/pglog/server.log \
              start

            if [ $? -ne 0 ]; then
              echo "Error: pg_ctl start failed" >&2
              exit $?
            fi

            POSTGRES_STARTED=1
          fi

          echo "Starting backend dev environment..."
          echo '  - run `sql` to enter postgresql environment'
          echo '  - run `cargo run -p backend` to run backend'
          echo '  - run `sqlx migrate run --source backend/migrations` to run migrations'
        '';

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.openssl.out
        ];
      };
    };
  };
}
