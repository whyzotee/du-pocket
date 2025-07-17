use std::{io::Result, net::UdpSocket, str::from_utf8};

pub fn client() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    println!("[Client] socket bind to port 4399");

    let mut buf = [0; 100];

    socket.send_to("Hello World".as_bytes(), "127.0.0.1:4399")?;

    let (amt, _) = socket.recv_from(&mut buf)?;

    let buf = &mut buf[..amt];

    let word = from_utf8(&buf).unwrap();

    println!("[Client] Recive data from server: {}", word);

    Ok(())
}
