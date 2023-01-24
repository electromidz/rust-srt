use std::io;
use std::collections::HashMap;
use std::net::Ipv4Addr;

mod tcp;

// #![derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad , tcp::State> = Default::default();
        let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
        let mut buf = vec![0u8; 1504]; // MTU + 4 for the header
    loop {
    let nbytes = nic.recv(&mut buf[..])?;
        let eth_flag = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x0800 {
            // not ipV4
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) { 
            Ok(iph) => {
            let src = iph.source_addr();
            let dst = iph.destination_addr();
                if iph.protocol() != 0x06  && iph.protocol() != 0x11 {
                    // not TCP or UDP
                    continue;
                }
        match etherparse::TcpHeaderSlice::from_slice(&buf[4+iph.slice().len()..]) {
            Ok(p) => {
                        connections.entry(Quad {
                            src:(src, p.source_port()),
                           dst:(dst, p.destination_port())
                        }).or_default();
                println!("{} -> {} {}b of tcp to port {}",src, dst, p.slice().len(),p.destination_port());
            }
            Err(e) => {
                eprintln!("ignoring weired tcp packet {:?}", e);
            }
        }

        match etherparse::UdpHeaderSlice::from_slice(&buf[4+iph.slice().len()..]){
                    Ok(up) => {
                println!("{} -> {} {}b of udp to port {}",src, dst, up.slice().len(),up.destination_port());
                    }
                    Err(e) => {
                eprintln!("ignoring weired udp packet {:?}", e);
            }
                    }
            } 
           
            Err(e) => {
                eprintln!("ignoring weired packet {:?}", e);
            }
        }
    }
    Ok(())
}
