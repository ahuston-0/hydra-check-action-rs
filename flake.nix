{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable-small";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      fenix,
      flake-utils,
      advisory-db,
      ...
    }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        inherit (pkgs) lib;

        craneLib = crane.mkLib pkgs;
        src = craneLib.cleanCargoSource (craneLib.path ./.);

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;
          strictDeps = true;

          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.openssl
          ];

          buildInputs =
            [
              # Add additional build inputs here
              pkgs.pkg-config
              pkgs.openssl
            ]
            ++ lib.optionals pkgs.stdenv.isDarwin [
              # Additional darwin specific inputs can be set here
              pkgs.libiconv
            ];

          # Additional environment variables can be set directly
          # MY_CUSTOM_VAR = "some value";
        };

        craneLibLLvmTools = craneLib.overrideToolchain (
          fenix.packages.${system}.complete.withComponents [
            "cargo"
            "llvm-tools"
            "rustc"
          ]
        );

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        my-crate = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
      in
      rec {
        formatter = pkgs.nixfmt-rfc-style;
        checks = import ./checks.nix {
          inherit
            my-crate
            craneLib
            commonArgs
            cargoArtifacts
            src
            inputs
            system
            formatter
            ;
        };

        packages =
          {
            default = my-crate;
          }
          // lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
            my-crate-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (commonArgs // { inherit cargoArtifacts; });
          };

        apps.default = flake-utils.lib.mkApp { drv = my-crate; };

        devShells = import ./shell.nix {
          inherit
            self
            system
            pkgs
            craneLib
            ;
        };

      }
    )
    // {
      hydraJobs = import ./hydra/jobs.nix {
        inherit inputs;
        inherit (self) outputs;
      };
    };
}
