mod shortener;


use simple_server::{Method, Server, StatusCode};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    start_server()?;
    Ok(())
}




fn start_server() -> Result<(), Box<dyn std::error::Error>>{
    let host = "127.0.0.1";
    let port = "7878";

    let shorty = shortener::Shortener::new();

    let server = Server::new(|request, mut response| {

        match (request.method(), request.uri().path()) {
            (&Method::GET, "") => {
                // TODO: redirect to /web
                Ok(response.body("<h1>Hi!</h1><p>Hello Rust!</p>".as_bytes().to_vec())?)
            }
            (&Method::POST, "") => {
                // TODO: get the body, and generate a shorted url, return it
                Ok(response.body("<h1>Hi!</h1><p>Hello Rust!</p>".as_bytes().to_vec())?)
            }
            (&Method::GET, _) => {
                // TODO: Check if the path exists in the shortened urls, if so
                // return it, otherwise return 404 not found
                Ok(response.body("<h1>Hi!</h1><p>Hello Rust!</p>".as_bytes().to_vec())?)
            }
            (_, _) => {
                response.status(StatusCode::NOT_FOUND);
                Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
            }
        }
    });

    server.listen(host, port);
    Ok(())
}


//fn not_found() -> Result<Response<T>, Error>) {
//}
