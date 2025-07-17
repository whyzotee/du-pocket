use std::{io::Result, net::UdpSocket, str::from_utf8};

pub fn server() -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:4399")?;

    println!("[Server] socket bind to port 4399");

    let mut buf = [0; 100];

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;

        let buf = &mut buf[..amt];

        let word = from_utf8(&buf).unwrap();

        println!("[Server] Recive data from client: {}", word);

        buf.reverse();

        socket.send_to(buf, &src)?;
    }
}
