extern crate zmq;
extern crate protobuf;

use protobuf::*;

mod pingpong;

fn run_client(ctx: &mut zmq::Context, addr: &str) -> Result<(), zmq::Error> {
    let mut socket = try!(ctx.socket(zmq::REQ));
    try!(socket.connect(addr));
    let payload = "PING";
    loop {
        let mut p = pingpong::PingPong::new();
        p.set_action(payload.to_string());
        p.set_name("Nolan".to_string());
        println!("Sending -> {:?}", p.get_action());
        let mut msg = try!(zmq::Message::new());
        try!(socket.send(&p.write_to_bytes().unwrap(), 0));
        try!(socket.recv(&mut msg, 0));
        let contents = msg.as_str().unwrap();
        println!("Received <- {:?}", contents);
    }
    Ok(())
}

fn run_server(ctx: &mut zmq::Context, addr: &str) -> Result<(), zmq::Error> {
    let mut socket = try!(ctx.socket(zmq::REP));
    try!(socket.bind(addr));
    let mut msg = try!(zmq::Message::new());
    loop {
        if let Ok(_) = socket.recv(&mut msg, 0) {
            let p: pingpong::PingPong = parse_from_bytes(&msg).unwrap();
            let mut r: pingpong::PingPong = pingpong::PingPong::new();
            if p.get_action() == "PING" {
                try!(socket.send_str("PONG", 0));
            }
        }
    }
    Ok(())
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let mut ctx = zmq::Context::new();
    let addr = "tcp://127.0.0.1:5555";
    if args[1] == "client" {
        println!("ZeroMQ client connecting to {}", addr);
        run_client(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
    } else {
        println!("ZeroMQ server listening on {}", addr);
        run_server(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
    }
}
