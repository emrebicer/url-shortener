# url-shortener
A web server that shorten long urls

## Usage
Make a post request to the server with the ***long url*** in the request payload, it should return you the shortened url.
```bash
# Run the server for local testing
$ cargo run

# Shorten a url by making a post request
$ curl -X POST 127.0.0.1:3000 -d "https://github.com"
127.0.0.1:3000/z6 # Response from the server
```
Use the --host and --port arguments as needed.

## Web interface
The web interface is accessible at ***/web***. Navigate to http://{host}:{port}/web to use it. By default it is http://0.0.0.0:3000/web

## Use with NixOS flakes
Add the git repository as an input to your flake and use the NixOS module;
```nix
{
  inputs = {
    url-shortener.url = "github:emrebicer/url-shortener";
  };
}
```

and enable url-shortener in your system configuration;
```nix
{ inputs, ... }: # Make sure the flake inputs are in your system's config
{
  imports = [ inputs.url-shortener.nixosModules.url-shortener];

  url-shortener = {
    enable = true;
    host = "127.0.0.1";
    port = 3000;
  };

}
```
