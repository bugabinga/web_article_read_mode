extern crate hyper;

use hyper::client::Client;

fn main() {
    //TODO: download a simple article and sanitaize it using the default ammonia
    // settings.
    // Start testing with lobste.rs
    //TODO find simpler request library, do socket shit or find hyper sll thing...
    let http_client = Client::new();

    let http_download_request = http_client.get("http://www.google.com").send().unwrap();

    println!("The status is: {:?}", http_download_request.status);
}
