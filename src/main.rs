use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 128];
    stream.read(&mut buffer).unwrap();

    // Read the contents of the file
    let contents = fs::read_to_string("src/hello.html").unwrap();

    // Calculate the content length
    let content_length = contents.len();

    // Format the HTTP response with headers
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Type: text/html; charset=UTF-8\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\
        \r\n{}",
        content_length,
        contents
    );

    

    // Write the response to the stream and flush it
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        handle_connection(stream);
    }
}


// #0 
// I am trying to create a web server using vanilla rust (or as few crates as possible)
//
// #1 ---------------------------------------------------
// let listener = TcpListener
// Create an instance of a `TcpListener`` and bind it to a variable `listener`
// Call the `bind` function associated w/ the TcpListener struct - but where is STRUCT? Is it implicitly a struct?
// In stadard library's std::net it is explicitly defined as a struct: pub struct TcpListener(net_imp::TcpListener);
// Call the bind function. This f tries to create a new `TcpListener` struct, that will then be stored in the `listener` var.
// `unwrap()` f is used to extract the value contained within a `Result` of `Option` type, assuming that the val is present
// If value is not pres. (if the Result is an `Err` or hte `Option` is `None`), unwrap will cause prog.
// to "panic" (terminate unexpectedly) and print error msg.
// TODO: see Result and Option types

// #2 ---------------------------------------------------
// fn handle_connection
// reads incoming data (up to 512 bytes???) from the stream and prints it as a string.
// let mut buffer = [0; 512]; // A buffer is an array of bytes that will tempor. hold the data received from the client.
// ---
// stream.read(&mut buffer) // reads data from the TcpStream into the buffer.
// read method attempts to fill the buffer with data from the stream.
// read returns a Result<usize, std::io::Error>, where usize == the number of bytes read.
// unwrap() handles Result if success. If fail => panic.
// ---
// Processing the data:
// println!("Request: {}", String::from_utf8_lossy(&buffer[..])); -> processes data
// (println! in Rust is a macro, NOT a function)
// the data in buffer is usally raw bytes, so we need to convert them into strings.
// string::from_utf8_lossy - converts the buffer. Any invalid utf-8 sequence -> "�"
// -------------------

// !!! The chronic problem I have is that in Chrome (for example) the response takes too long to get, 
// !!! that the browser times out.
