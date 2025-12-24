{
  description = "wyattavilla.dev flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self, nixpkgs, ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        wasmTarget = "wasm32-unknown-unknown";
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import inputs.rust-overlay) ];
        };
        inherit (pkgs) lib;

        rustBin = with pkgs; [
          (rust-bin.nightly.latest.default.override {
            extensions = [
              "clippy"
              "rust-src"
            ];
            targets = [ wasmTarget ];
          })
        ];
      in
      {

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = rustBin ++ (with pkgs; [ rust-analyzer ]);

          shellHook = ''
            export CARGO_BUILD_TARGET="${wasmTarget}"
          '';
        };
        packages.default =
          let
            cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
            pname = cargoToml.package.name;
          in
          pkgs.rustPlatform.buildRustPackage {
            inherit pname;
            name = pname;
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            buildPhase = ''
              cargo build -j $(nproc) --offline --release --target=${wasmTarget}
              mv target/stylers target/stylers-release
            '';

            checkPhase = ''
              ${lib.getExe' pkgs.wabt "wasm-validate"} target/${wasmTarget}/release/${pname}.wasm
            '';

            installPhase = ''
              mkdir -p $out/pkg

              cp target/${wasmTarget}/release/${pname}.wasm $out/pkg/

              ${lib.getExe pkgs.wasm-bindgen-cli} \
              target/${wasmTarget}/release/${pname}.wasm \
              --out-dir $out \
              --target web \
              --no-typescript

              ${lib.getExe' pkgs.binaryen "wasm-opt"} \
              $out/${pname}_bg.wasm \
              -o $out/${pname}_bg.wasm \
              -Oz

              cp -r assets $out/

              cp target/stylers-release/main.css $out/

              cat > $out/index.html << EOF
              <!DOCTYPE html>
              <html>
              <head>
                <meta charset="utf-8">
                <title>wyattavilla.dev</title>
                <link rel="modulepreload" href="./${pname}.js">
                <link rel="stylesheet" href="./main.css">
              </head>
              <body>
                <script type="module">
                  import init from './${pname}.js';
                  init();
                </script>
              </body>
              </html>
            '';

            nativeBuildInputs = rustBin;
          };

        apps.default = {
          type = "app";
          program = toString (
            pkgs.writeShellScript "serve-wasm" ''
              ${lib.getExe pkgs.python3} -m http.server --bind 127.0.0.1 8000 -d ${
                self.packages.${system}.default
              }
            ''
          );
        };

        checks =
          let
            mkCheck =
              {
                name,
                cmds,
                src ? self,
                inputs ? [ ],
              }:
              pkgs.runCommand name { buildInputs = inputs; } ''
                cd ${src}
                ${pkgs.lib.strings.concatLines cmds}
                touch $out
              '';

            checkArgs = {
              rustFormatting = {
                inputs = rustBin ++ [ pkgs.leptosfmt ];
                cmds = [
                  "leptosfmt --check ${self}"
                  "cargo fmt --check"
                ];
              };

              nixFormatting = {
                inputs = [ pkgs.nixfmt-rfc-style ];
                cmds = [ "nixfmt --check ${self}" ];
              };

              nixCheck = {
                inputs = with pkgs; [
                  deadnix
                  statix
                ];
                cmds = [
                  "statix check"
                  "deadnix check"
                ];
              };
            };
          in
          builtins.mapAttrs (name: args: mkCheck (args // { inherit name; })) checkArgs;

      }
    );
}
