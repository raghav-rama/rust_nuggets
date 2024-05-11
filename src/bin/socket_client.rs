use reqwest::blocking::Client;
use reqwest::Url;
use std::os::unix::net::UnixStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Unix stream connected to the Docker socket
    let socket_path = "/var/run/docker.sock";
    let stream = match UnixStream::connect(socket_path) {
        Ok(sock) => sock,
        Err(e) => {
            println!("Couldn't connect: {e:?}");
            return Ok(());
        }
    };
    let addr = stream.local_addr()?;
    println!("Connected to {addr:?}.");
    let peer_addr = stream.peer_addr()?;
    println!("Peer address: {peer_addr:?}.");

    // Emulate this
    // curl --unix-socket /var/run/docker.sock -X GET http:/v1.45/containers/json

    // Create a reqwest client with the Unix stream as the connector
    let client = Client::builder().build().unwrap();

    // Make the GET request to the specified endpoint
    let url = Url::parse("http://localhost/v1.45/containers/json")?;
    let response = client.get(url).send()?;

    // Check the response status and print the response body
    if response.status().is_success() {
        let body = response.text()?;
        println!("Response Body: {}", body);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}
