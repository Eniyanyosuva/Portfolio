{
  description = "A very basic flake";

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
            frontend = cargoToml.package.name;
          in
          pkgs.rustPlatform.buildRustPackage {
            name = frontend;
            pname = frontend;
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            buildPhase = ''
              cargo build -j $(nproc) -p ${frontend} --offline --release --target=${wasmTarget}
              mv target/stylers target/stylers-release
            '';

            checkPhase = ''
              # TODO: wasm-validate?
              cargo clippy --package ${frontend} --all-features -- -W clippy::pedantic -D warnings
              cargo fmt --package ${frontend} --check
            '';

            installPhase = ''
              mkdir -p $out/pkg

              cp target/${wasmTarget}/release/${frontend}.wasm $out/pkg/

              ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen \
              target/${wasmTarget}/release/${frontend}.wasm \
              --out-dir $out \
              --target web \
              --no-typescript

              ${pkgs.binaryen}/bin/wasm-opt \
              $out/${frontend}_bg.wasm \
              -o $out/${frontend}_bg.wasm \
              -Oz

              cp target/stylers-release/main.css $out/

              cat > $out/index.html << EOF
              <!DOCTYPE html>
              <html>
              <head>
                <meta charset="utf-8">
                <title>wyattavilla.dev</title>
                <link rel="modulepreload" href="/${frontend}.js">
                <link rel="stylesheet" href="/main.css">
              </head>
              <body>
                <script type="module">
                  import init from './${frontend}.js';
                  init();
                </script>
              </body>
              </html>
              EOF
            '';

            nativeBuildInputs = nativeRustToolchain;
          };
      }
    );
}
