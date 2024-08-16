use std::net::{TcpListener, TcpStream};
use std::io::Read;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

    // This will block and wait for incoming connections
    for stream in listener.incoming() {
        // For now, just print out a message when a connection is received
        println!("Connection established!");
    }
}



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
// println!("Request: {}", String::from_utf8_lossy(&buffer[..])); -> processes data
//
