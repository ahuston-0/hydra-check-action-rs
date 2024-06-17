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
  inherit (outputs) formatter devShells checks;
  pkgs = lib.mapAttrs filterValidPkgs outputs.packages;
}
