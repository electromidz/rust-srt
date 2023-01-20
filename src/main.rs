use std::io;

fn main() -> io::Result<()> {
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
            Ok(p) => {
            let src = p.source_addr();
            let dst = p.destination_addr();
            let proto = p.protocol();
                if proto != 0x06 {
                    // not TCP
                    continue;
                }
 match etherparse::Ipv4HeaderSlice::from_slice(&buf[4+p.slice().len()..]) {
            Ok(p) => {
                println!("{} -> {} {}b of tcp to port {}",src, dst, p.slice().len(), p.destination_port());
            }
            Err(e) => {
                eprintln!("ignoring weired tcp packet {:?}", e);
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
