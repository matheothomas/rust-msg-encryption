mod rsa;

use std::net::TcpStream;
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let ip_address = &args[1];

	let mut stream = TcpStream::connect(ip_address.to_owned() + ":5555").expect("Failed to bind to address");


    let (pub_key, pri_key) = rsa::gen_keys();
    send_public_key(&mut stream, &pub_key);

	let mut stream2 = stream.try_clone().expect("Failed to clone stream.");
	let handle1 = thread::spawn(move || {
		loop {
			chat_loop_write(&mut stream);
		}
	});
	let handle2 = thread::spawn(move || {
		loop {
			chat_loop_read(&mut stream2);
		}
	});
	handle1.join().unwrap();
	handle2.join().unwrap();
}


fn send_public_key(mut stream: &TcpStream, pub_key: &rsa::PubKey) {
    let public_key_data = format!("{} {}\n", pub_key.modulus, pub_key.exponent);
    stream.write(public_key_data.as_bytes()).expect("Failed to send public key to server");
}

fn chat_loop_write(mut stream: &TcpStream) {
	let mut msg: String = String::new();

	// println!("You : ");
	std::io::stdin().read_line(&mut msg).expect("Unable to read input");
	stream.write(msg.as_bytes()).expect("Couldn't write to server");
}


fn chat_loop_read(mut stream: &TcpStream) {
	let mut server_buffer: Vec<u8> = Vec::new();
	let mut reader = BufReader::new(&mut stream);
	reader.read_until(b'\n', &mut server_buffer).expect("Couldn't read from server");

	println!("\x1b[92m{}\x1b[0m", std::str::from_utf8(&server_buffer).expect("Could not write buffer as string"));
}

