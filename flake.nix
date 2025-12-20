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

        nativeRustToolchain = with pkgs; [
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
          nativeBuildInputs = nativeRustToolchain ++ (with pkgs; [ rust-analyzer ]);

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
              cargo fmt --check
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

              cp target/stylers-release/main.css $out/

              cat > $out/index.html << EOF
              <!DOCTYPE html>
              <html>
              <head>
                <meta charset="utf-8">
                <title>wyattavilla.dev</title>
                <link rel="modulepreload" href="/${pname}.js">
                <link rel="stylesheet" href="/main.css">
              </head>
              <body>
                <script type="module">
                  import init from './${pname}.js';
                  init();
                </script>
              </body>
              </html>
              EOF
            '';

            nativeBuildInputs = nativeRustToolchain;
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
      }
    );
}
