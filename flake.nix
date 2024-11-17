{
  description = "A web application that shortens the urls";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    lib = nixpkgs.lib;
          pkgs = nixpkgs.legacyPackages.${system};
  in {

      devShells.${system}.default = pkgs.mkShellNoCC {
        buildInputs = with pkgs; [
          gcc9
          cargo
          rustc
        ];
      };


    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "url_shortener";
        version = "0.1.0";
        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
      };

    apps.${system}.default = {
        type = "app";
        program = "${self.packages.x86_64-linux}/bin/url_shortener";
    };

    nixosModules = {
      url-shortener = { config, lib, pkgs, ... }: with lib; {
        options.url-shortener = {
          enable = mkOption {
            type = types.bool;
            default = false;
            description = "Enable the URL shortener systemd service.";
          };

          host = mkOption {
            type = types.str;
            default = "127.0.0.1";
            description = "The host the URL shortener will bind to.";
          };

          port = mkOption {
            type = types.port;
            default = 8000;
            description = "The port the URL shortener will listen on.";
          };
        };

        config = mkIf config.url-shortener.enable {
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
              }}/bin/url_shortener --host ${config.url-shortener.host} --port ${toString config.url-shortener.port}";
              Restart = "always";
            };
          };
        };
      };
    };
  };
}

