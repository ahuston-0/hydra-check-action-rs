{ inputs, outputs }:
let
  inherit (inputs.nixpkgs) lib;

  notBroken = pkg: !(pkg.meta.broken or false);
  isDistributable = pkg: (pkg.meta.license or { redistributable = true; }).redistributable;
  hasPlatform = sys: pkg: lib.elem sys (pkg.meta.platforms or [ sys ]);
  filterValidPkgs =
    sys: pkgs:
    lib.filterAttrs (
      _: pkg: lib.isDerivation pkg && hasPlatform sys pkg && notBroken pkg && isDistributable pkg
    ) pkgs;
in
{
  formatter = outputs.formatter.x86_64-linux;
  devShells = outputs.devShells.x86_64-linux;
  checks = outputs.checks.x86_64-linux;
  pkgs = lib.mapAttrs filterValidPkgs outputs.packages.x86_64-linux;
}
