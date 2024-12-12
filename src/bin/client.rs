mod rsa;

use std::net::TcpStream;
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::env;
use std::str;

fn main() {
	let args: Vec<String> = env::args().collect();
	let ip_address = &args[1];

	let mut stream = TcpStream::connect(ip_address.to_owned() + ":5555").expect("Failed to bind to address");

    let (pub_key, pri_key) = rsa::gen_keys();
    send_public_key(&mut stream, &pub_key);
    let server_pub_key = receive_public_key(&mut stream); 

	let mut stream2 = stream.try_clone().expect("Failed to clone stream.");
	let handle1 = thread::spawn(move || {
		loop {
			chat_loop_write(&mut stream, &server_pub_key);
		}
	});
	let handle2 = thread::spawn(move || {
		loop {
			chat_loop_read(&mut stream2, &pri_key);
		}
	});
	handle1.join().unwrap();
	handle2.join().unwrap();
}

fn receive_public_key(stream: &TcpStream) -> rsa::PubKey {
    let mut server_buffer: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(stream);

    reader.read_until(b'\n', &mut server_buffer).expect("Failed to read public key from server");

    let public_key_str = str::from_utf8(&server_buffer).expect("Failed to convert buffer to string");

    let parts: Vec<&str> = public_key_str.trim().split_whitespace().collect();

    if parts.len() != 2 {
        panic!("Invalid public key format received from server");
    }

    let modulus: i64 = parts[0].parse().expect("Failed to parse modulus");
    let exponent: i64 = parts[1].parse().expect("Failed to parse exponent");

    rsa::PubKey {modulus, exponent}
}

fn send_public_key(mut stream: &TcpStream, pub_key: &rsa::PubKey) {
    let public_key_data = format!("{} {}\n", pub_key.modulus, pub_key.exponent);
    stream.write(public_key_data.as_bytes()).expect("Failed to send public key to server");
}

fn chat_loop_write(mut stream: &TcpStream, server_pub_key: &rsa::PubKey) {
	let mut msg: String = String::new();

	std::io::stdin().read_line(&mut msg).expect("Unable to read input");
    //println!("msg : {}", msg);
    let encrypted = rsa::rsa_encrypt(&msg, &server_pub_key);
    let encrypted_str = encrypted.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(",");
    let msg_encrypted_str = format!("{}\n", encrypted_str);
    //println!("msg : {:?}", msg_encrypted_str);
	stream.write(msg_encrypted_str.as_bytes()).expect("Couldn't write to server");
}


fn chat_loop_read(mut stream: &TcpStream, client_pri_key: &rsa::PriKey) {
	let mut server_buffer: Vec<u8> = Vec::new();
	let mut reader = BufReader::new(&mut stream);
	reader.read_until(b'\n', &mut server_buffer).expect("Couldn't read from server");

	//println!("\x1b[92m{}\x1b[0m", std::str::from_utf8(&server_buffer).expect("Could not write buffer as string"));
    let encrypted_msg: &str = std::str::from_utf8(&server_buffer).expect("");
    let msg: Vec<i64> = encrypted_msg.trim().split(',').map(|s| s.parse::<i64>().expect("Invalid number")).collect();
    //println!("{:?}", msg);
    println!("\x1b[92m{}\x1b[0m", rsa::rsa_decrypt(&msg, client_pri_key));
}

