#[macro_use]
extern crate nickel;

use nickel::{Nickel, JsonBody, HttpRouter, Request, Response, Middleware, MediaType};

fn main() {

    let mut server = Nickel::new();
    let mut router = Nickel::router();


    router.get("/users", middleware! { |request, response|
    
        format!("Hello world!")
    
    });

    server.utilize(router);

    server.listen("127.0.0.1:9000");
}
