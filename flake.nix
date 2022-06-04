{
  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in {
    packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
      pname = "ws-write";
      version = "0.0.1";
      src = ./.;
      cargoHash = "sha256-lRhNqbdYNl/mNP518SIx592q1OZHQzH7NgG/30jZsJo=";
      meta = {
        description = "Toy app providing websocket on all paths on localhost:7777 and writes messages it receives to a file.";
        homepage = "https://github.com/typetetris/ws-write";
        license = pkgs.lib.licenses.mit;
        maintainers = [pkgs.lib.maintainers.typetetris];
      };
    };
  };
}
