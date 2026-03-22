mod core;
mod parser;

use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[command(name = "s5-dplane")]
#[command(about = "Simple packet forwarder from rx interface to tx interface")]
struct Args {
    #[arg(long, help = "RX interface name")]
    rx: String,

    #[arg(long, help = "TX interface name")]
    tx: String,

    #[arg(long, help = "Path to parser WASM file")]
    parser: Option<String>,
}

#[cfg(target_os = "linux")]
fn main() {
    use core::network::interface::{bind_to_interface, create_raw_socket, get_interface_index};
    use nix::sys::socket::{recvfrom, sendto, LinkAddr, MsgFlags, SockaddrLike};
    use parser::parser::Parser;
    use std::os::fd::AsRawFd;

    let args = Args::parse();

    println!("Starting dplane: {} -> {}", args.rx, args.tx);

    // Load parser if specified
    let mut parser_instance = args.parser.map(|path| {
        println!("Loading parser WASM: {}", path);
        let wasm_bytes = std::fs::read(&path).expect("Failed to read WASM file");
        Parser::new(&wasm_bytes)
    });

    let rx_index = get_interface_index(&args.rx);
    let tx_index = get_interface_index(&args.tx);

    println!("RX interface {} index: {}", args.rx, rx_index);
    println!("TX interface {} index: {}", args.tx, tx_index);

    let rx_socket = create_raw_socket();
    let tx_socket = create_raw_socket();

    bind_to_interface(&rx_socket, rx_index);
    bind_to_interface(&tx_socket, tx_index);

    println!("Forwarding packets...");

    let mut buf = [0u8; 65535];

    loop {
        match recvfrom::<LinkAddr>(rx_socket.as_raw_fd(), &mut buf) {
            Ok((len, _addr)) => {
                if len > 0 {
                    // Run parser if configured
                    let should_forward = match &mut parser_instance {
                        Some(p) => p.parse(buf.as_mut_ptr(), len),
                        None => true,
                    };

                    if !should_forward {
                        continue;
                    }

                    // Create destination address for tx interface
                    let mut dest_addr: libc::sockaddr_ll = unsafe { std::mem::zeroed() };
                    dest_addr.sll_family = libc::AF_PACKET as u16;
                    dest_addr.sll_protocol = (libc::ETH_P_ALL as u16).to_be();
                    dest_addr.sll_ifindex = tx_index;
                    dest_addr.sll_halen = 6;

                    // Copy destination MAC from packet
                    if len >= 6 {
                        dest_addr.sll_addr[..6].copy_from_slice(&buf[..6]);
                    }

                    let dest = unsafe {
                        LinkAddr::from_raw(
                            &dest_addr as *const libc::sockaddr_ll as *const libc::sockaddr,
                            Some(std::mem::size_of::<libc::sockaddr_ll>() as u32),
                        )
                        .expect("Failed to create dest LinkAddr")
                    };

                    if let Err(e) =
                        sendto(tx_socket.as_raw_fd(), &buf[..len], &dest, MsgFlags::empty())
                    {
                        eprintln!("Error sending packet: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error receiving packet: {}", e);
            }
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("s5-dplane requires Linux (AF_PACKET sockets)");
    eprintln!("Please build and run in a Linux environment or VM");
    std::process::exit(1);
}
