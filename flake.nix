{
  description = "A web application that shortens the urls";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:

    let 
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {

      devShells.${system}.default = pkgs.mkShellNoCC {
        buildInputs = with pkgs; [
          gcc9
          cargo
          rustc
        ];

      };

      default = pkgs.rustPlatform.buildRustPackage {
        pname = "url_shortener";
        version = "0.1.0";
        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
      };
      
      # Expose your program
      apps.default = {
        type = "app";
        program = "${self.packages.default}/bin/url_shortener";
      };
      
    };
}
