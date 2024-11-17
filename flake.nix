{
  description = "A web application that shortens the urls";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    let 
      lib = nixpkgs.lib;
      pkgs = import nixpkgs;
    in 
    flake-utils.lib.eachDefaultSystem (system:
      let 
        pkgs = import nixpkgs { inherit system; };
      in {
        devShells.default = pkgs.mkShellNoCC {
          buildInputs = with pkgs; [
            gcc9
            cargo
            rustc
          ];
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "url_shortener";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/url_shortener";
        };
      })
    // {
      # NixOS module to enable the service (system-independent)
      nixosModules.url-shortener = {
        options.url-shortener = {
          enable = lib.mkOption {
            type = lib.types.bool;
            default = false;
            description = "Enable the URL shortener systemd service.";
          };

          host = lib.mkOption {
            type = lib.types.str;
            default = "127.0.0.1";
            description = "The host the URL shortener will bind to.";
          };

          port = lib.mkOption {
            type = lib.types.port;
            default = 8000;
            description = "The port the URL shortener will listen on.";
          };
        };

        config = lib.mkIf self.config.url-shortener.enable {
          systemd.services.url-shortener = {
            description = "URL Shortener Service";
            after = [ "network.target" ];
            wantedBy = [ "multi-user.target" ];

            serviceConfig = {
              ExecStart = "${pkgs.rustPlatform.buildRustPackage {
                pname = "url_shortener";
                version = "0.1.0";
                src = ./.;
                cargoLock = { lockFile = ./Cargo.lock; };
              }}/bin/url_shortener --host ${self.config.url-shortener.host} --port ${toString self.config.url-shortener.port}";
              Restart = "always";
            };
          };
        };
      };
    };
}

