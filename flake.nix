{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      eachSystem = fn: nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed fn;
    in
    {
      devShells = eachSystem (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustc
              rustfmt
              rust-analyzer
              cargo
            ];
          };
        }
      );
    };
}
