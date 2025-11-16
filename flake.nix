{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];

      perSystem = {
        pkgs,
        lib,
        system,
        ...
      }: (let
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        buildInputs =
          (with pkgs; [
            toolchain
            pkg-config
            cacert
          ])
          ++ runtimeLibs;

        runtimeLibs = with pkgs; [
          libGL
          vulkan-loader
          wayland
          libxkbcommon
        ];
      in {
        _module.args = {
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              (import inputs.rust-overlay)
            ];
          };
        };

        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          LD_LIBRARY_PATH =
            builtins.foldl' (a: b: "${a}/${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;

          env.RUSTFLAGS = "-C link-arg=-Wl,-rpath,${lib.makeLibraryPath buildInputs}";
        };

        packages.default =
          pkgs.runCommand "${cargoToml.package.name}-wrapped" {
            nativeBuildInputs = [pkgs.makeWrapper];
          } (let
            unwrapped = craneLib.buildPackage {
              inherit (cargoToml.package) version;
              pname = cargoToml.package.name;
              src = ./.;

              env.RUSTFLAGS = "-C link-arg=-Wl,-rpath,${lib.makeLibraryPath buildInputs}";

              inherit buildInputs;
            };
          in ''
            mkdir -p $out/bin
            makeWrapper "${unwrapped}/bin/${cargoToml.package.name}" "$out/bin/${cargoToml.package.name}-wrapped" \
              --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath runtimeLibs}
          '');
      });
    };
}
