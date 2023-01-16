extern crate tun_tap;
use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = vec![0u8; 1504]; // MTU + 4 for the header

    let nbytes = nic.recv(&mut buf[..])?;
    println!("Read {} from {:x?}", nbytes , &buf[..nbytes]);

    Ok(())
}
