use clap::Parser;
use nix::sys::socket::{
    bind, sendto, socket, AddressFamily, LinkAddr, MsgFlags, SockFlag, SockType, SockaddrLike,
};
use std::ffi::CString;
use std::os::fd::{AsRawFd, OwnedFd};

#[derive(Parser, Debug)]
#[command(name = "e_pktgen")]
#[command(about = "Simple L2 frame generator")]
struct Args {
    #[arg(short, long, help = "Interface name to send packets")]
    interface: String,

    #[arg(short, long, default_value = "1", help = "Number of packets to send")]
    count: u64,

    #[arg(long, default_value = "ff:ff:ff:ff:ff:ff", help = "Destination MAC address")]
    dst_mac: String,

    #[arg(long, default_value = "00:00:00:00:00:01", help = "Source MAC address")]
    src_mac: String,
}

fn parse_mac(s: &str) -> [u8; 6] {
    let parts: Vec<u8> = s
        .split(':')
        .map(|p| u8::from_str_radix(p, 16).expect("Invalid MAC address"))
        .collect();
    if parts.len() != 6 {
        panic!("Invalid MAC address format");
    }
    [parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]]
}

fn get_interface_index(name: &str) -> i32 {
    let c_name = CString::new(name).expect("Invalid interface name");
    unsafe {
        let index = libc::if_nametoindex(c_name.as_ptr());
        if index == 0 {
            panic!("Interface {} not found", name);
        }
        index as i32
    }
}

fn create_raw_socket() -> OwnedFd {
    socket(
        AddressFamily::Packet,
        SockType::Raw,
        SockFlag::empty(),
        Some(nix::sys::socket::SockProtocol::EthAll),
    )
    .expect("Failed to create raw socket")
}

fn main() {
    let args = Args::parse();

    let dst_mac = parse_mac(&args.dst_mac);
    let src_mac = parse_mac(&args.src_mac);

    println!(
        "Sending {} packets to interface {} (src: {}, dst: {})",
        args.count, args.interface, args.src_mac, args.dst_mac
    );

    let if_index = get_interface_index(&args.interface);
    println!("Interface {} index: {}", args.interface, if_index);

    let sock = create_raw_socket();

    // Bind to interface
    let mut bind_addr: libc::sockaddr_ll = unsafe { std::mem::zeroed() };
    bind_addr.sll_family = libc::AF_PACKET as u16;
    bind_addr.sll_protocol = (libc::ETH_P_ALL as u16).to_be();
    bind_addr.sll_ifindex = if_index;

    let addr = unsafe {
        LinkAddr::from_raw(
            &bind_addr as *const libc::sockaddr_ll as *const libc::sockaddr,
            Some(std::mem::size_of::<libc::sockaddr_ll>() as u32),
        )
        .expect("Failed to create LinkAddr")
    };

    bind(sock.as_raw_fd(), &addr).expect("Failed to bind socket");

    // Minimum Ethernet frame: 14 bytes header + 46 bytes payload = 60 bytes
    let frame_size = 60;

    for i in 0..args.count {
        let mut buffer = vec![0u8; frame_size];

        // Build Ethernet header
        // Destination MAC (6 bytes)
        buffer[0..6].copy_from_slice(&dst_mac);
        // Source MAC (6 bytes)
        buffer[6..12].copy_from_slice(&src_mac);
        // EtherType: IPv4 (0x0800)
        buffer[12] = 0x08;
        buffer[13] = 0x00;

        // Fill payload with sequence number pattern
        let seq_bytes = (i as u32).to_be_bytes();
        for j in 14..frame_size {
            buffer[j] = seq_bytes[(j - 14) % 4];
        }

        // Create destination address
        let mut dest_addr: libc::sockaddr_ll = unsafe { std::mem::zeroed() };
        dest_addr.sll_family = libc::AF_PACKET as u16;
        dest_addr.sll_protocol = (libc::ETH_P_ALL as u16).to_be();
        dest_addr.sll_ifindex = if_index;
        dest_addr.sll_halen = 6;
        dest_addr.sll_addr[..6].copy_from_slice(&dst_mac);

        let dest = unsafe {
            LinkAddr::from_raw(
                &dest_addr as *const libc::sockaddr_ll as *const libc::sockaddr,
                Some(std::mem::size_of::<libc::sockaddr_ll>() as u32),
            )
            .expect("Failed to create dest LinkAddr")
        };

        match sendto(sock.as_raw_fd(), &buffer, &dest, MsgFlags::empty()) {
            Ok(_) => println!("Sent packet {}/{}", i + 1, args.count),
            Err(e) => eprintln!("Error sending packet {}: {}", i + 1, e),
        }
    }

    println!("Done.");
}
