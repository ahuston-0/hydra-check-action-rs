{
  inputs,
  system,
  formatter,
  my-crate,
  craneLib,
  commonArgs,
  cargoArtifacts,
  src,
  ...
}:
{
  pre-commit-check = inputs.pre-commit-hooks.lib.${system}.run {
    src = ./.;
    hooks = {
      # nix checks
      # Example custom hook for nix formatting
      fmt-check = {
        enable = true;

        # The command to execute (mandatory):
        entry = "${formatter}/bin/nixfmt --check";

        # The pattern of files to run on (default: "" (all))
        # see also https://pre-commit.com/#hooks-files
        files = "\\.nix$";
      };
      ## static analysis checks for nix
      nil.enable = true;
      statix.enable = true;
      deadnix = {
        enable = true;
        settings = {
          noUnderscore = true; # ignore variables starting with underscore
          # ignore lambda patterns (useful for passing args from ({}@args)
          # to other functions)
          noLambdaPatternNames = true;
        };
      };

      # toml hooks
      check-toml.enable = true;

      # git hooks
      check-merge-conflicts.enable = true;
      ## prevents committing to main
      no-commit-to-branch.enable = true;

      # rust hooks
      # https://github.com/cachix/git-hooks.nix/pull/396
      cargo-check.enable = false;
      clippy.enable = false;

      # misc hooks
      check-added-large-files.enable = true;
      ## prevents two similarly named files for case sensitive systems
      check-case-conflicts.enable = true;
      detect-private-keys.enable = true;
    };
  };
  # Build the crate as part of `nix flake check` for convenience
  inherit my-crate;

  # Run clippy (and deny all warnings) on the crate source,
  # again, reusing the dependency artifacts from above.
  #
  # Note that this is done as a separate derivation so that
  # we can block the CI if there are issues here, but not
  # prevent downstream consumers from building our crate by itself.
  my-crate-clippy = craneLib.cargoClippy (
    commonArgs
    // {
      inherit cargoArtifacts;
      cargoClippyExtraArgs = "--all-targets -- --deny warnings";
    }
  );

  my-crate-doc = craneLib.cargoDoc (commonArgs // { inherit cargoArtifacts; });

  # Check formatting
  my-crate-fmt = craneLib.cargoFmt { inherit src; };

  # Audit dependencies
  my-crate-audit = craneLib.cargoAudit {
    inherit src;
    inherit (inputs) advisory-db;
  };

  # Audit licenses
  my-crate-deny = craneLib.cargoDeny { inherit src; };

  # Run tests with cargo-nextest
  # Consider setting `doCheck = false` on `my-crate` if you do not want
  # the tests to run twice
  my-crate-nextest = craneLib.cargoNextest (
    commonArgs
    // {
      inherit cargoArtifacts;
      partitions = 1;
      partitionType = "count";
    }
  );
}
