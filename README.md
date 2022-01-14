# url-shortener
A web server that shorten long urls

## Usage
Make a post request to the server with the ***long url*** in the request payload, it should return you the shortened url.
```bash
# Run the server for local testing
$ cargo run

# Shorten a url by making a post request
$ curl -X POST 127.0.0.1:3000 -d "https://github.com"
127.0.0.1:3000/z6a6 # Response from the server
```

Or you can directly use https://url-rs.herokuapp.com/ to test the project.
## Web interface
The web interface is accessible at ***/web***, you can try it out <a href=https://url-rs.herokuapp.com/web> here</a>.
