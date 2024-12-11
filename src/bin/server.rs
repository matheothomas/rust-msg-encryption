mod rsa;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::env;
use std::str;

fn main() {
	let args: Vec<String> = env::args().collect();

	let ip_addr_collection: Vec<&str> = (&args[1]).split(".").collect();
	let mut values: Vec<u8> = Vec::new();
	for val in ip_addr_collection {
		values.push(val.parse::<u8>().unwrap());
	}

	let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(values[0], values[1], values[2], values[3])), 5555);
	let listener = TcpListener::bind(socket).expect("Failed to bind to address");
	println!("Server listening on {}:{}", socket.ip(), socket.port());

	let (mut stream, _) = listener.accept().unwrap();

    let (pub_key, pri_key) = rsa::gen_keys();
    send_public_key(&mut stream, &pub_key);
    let client_pub_key = receive_public_key(&mut stream); 

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


fn receive_public_key(mut stream: &TcpStream) -> rsa::PubKey {
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

/*
   fn handle_client_connection(mut stream: TcpStream) -> std::io::Result<()> {
   println!("Incoming connection from client {}", stream.peer_addr()?);
   let mut buffer: [u8; 1024] = [0; 1024];
   loop {
   let bytes_read: usize = stream.read(&mut buffer)?;
   if bytes_read == 0 {
   return Ok(());
   }
   let msg = match std::str::from_utf8(&buffer[..bytes_read]) {
   Ok(s) => s,
   Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
   };

   println!("Other : {}", msg);
   let mut msg: String = String::new();
   println!("You : ");
   std::io::stdin().read_line(&mut msg).expect("Unable to read input");

   stream.write(msg.as_bytes()).expect("Couldn't write to client");

   }
   }
   */
