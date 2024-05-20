#[cfg(test)]
mod socket_test {
    use std::io::{Read, Write};
    use std::os::unix::net::{UnixListener, UnixStream};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_unix_socket() {
        let socket_path = "/tmp/test_socket";

        // Create a Unix socket listener in a separate thread
        let listener_thread = thread::spawn(move || {
            let listener = UnixListener::bind(socket_path).expect("Failed to bind socket");
            let (mut stream, _) = listener.accept().expect("Failed to accept connection");

            let mut buffer = [0; 1024];
            let bytes_read = stream
                .read(&mut buffer)
                .expect("Failed to read from socket");
            let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);

            assert_eq!(received_data, "Hello, server!");

            stream
                .write_all(b"Hello, client!")
                .expect("Failed to write to socket");
        });

        // Wait for a short duration to ensure the listener is ready
        thread::sleep(Duration::from_millis(100));

        // Connect to the Unix socket
        let mut stream = UnixStream::connect(socket_path).expect("Failed to connect to socket");

        // Send data to the server
        stream
            .write_all(b"Hello, server!")
            .expect("Failed to write to socket");

        // Read the response from the server
        let mut buffer = [0; 1024];
        let bytes_read = stream
            .read(&mut buffer)
            .expect("Failed to read from socket");
        let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);

        assert_eq!(received_data, "Hello, client!");

        // Wait for the listener thread to finish
        listener_thread
            .join()
            .expect("Failed to join listener thread");
    }
}
