{
  self,
  pkgs,
  craneLib,
  system,
  ...
}:
let
  # construct the shell provided by pre-commit for running hooks
  pre-commit = pkgs.mkShell {
    inherit (self.checks.${system}.pre-commit-check) shellHook;
    buildInputs = self.checks.${system}.pre-commit-check.enabledPackages;
  };

  #construct a shell from crane for rust checks
  craneShell = craneLib.devShell {

    # Inherit inputs from checks.
    checks = self.checks.${system};
    # Additional dev-shell environment variables can be set directly
    # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

    # Extra inputs can be added here; cargo and rustc are provided by default.
    packages = [
      # pkgs.ripgrep
      pkgs.openssl
      pkgs.pkg-config
    ];
  };
in
{
  default = pkgs.mkShell {
    inputsFrom = [
      pre-commit
      craneShell
    ];
  };
}
