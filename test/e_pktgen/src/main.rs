use clap::Parser;
use nix::sys::socket::{
    bind, recvfrom, sendto, setsockopt, socket, sockopt, AddressFamily, LinkAddr, MsgFlags,
    SockFlag, SockType, SockaddrLike,
};
use nix::sys::time::TimeVal;
use std::ffi::CString;
use std::os::fd::{AsRawFd, OwnedFd};
use std::process::ExitCode;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(name = "e_pktgen")]
#[command(about = "Simple L2 frame generator and receiver")]
struct Args {
    #[arg(short, long, help = "Interface name to send packets")]
    interface: String,

    #[arg(short, long, default_value = "1", help = "Number of packets to send")]
    count: u64,

    #[arg(long, default_value = "ff:ff:ff:ff:ff:ff", help = "Destination MAC address")]
    dst_mac: String,

    #[arg(long, default_value = "00:00:00:00:00:01", help = "Source MAC address")]
    src_mac: String,

    #[arg(long, help = "Interface name to receive packets (optional)")]
    rx_interface: Option<String>,

    #[arg(long, default_value = "1000", help = "Receive timeout in milliseconds")]
    timeout: u64,
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

fn bind_to_interface(sock: &OwnedFd, if_index: i32) {
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
}

fn main() -> ExitCode {
    let args = Args::parse();

    let dst_mac = parse_mac(&args.dst_mac);
    let src_mac = parse_mac(&args.src_mac);

    let tx_if_index = get_interface_index(&args.interface);
    let tx_sock = create_raw_socket();
    bind_to_interface(&tx_sock, tx_if_index);

    // Setup RX socket if rx_interface is specified
    let rx_sock = args.rx_interface.as_ref().map(|rx_if| {
        let rx_if_index = get_interface_index(rx_if);
        let sock = create_raw_socket();
        bind_to_interface(&sock, rx_if_index);

        // Set receive timeout
        let timeout = TimeVal::new(
            (args.timeout / 1000) as i64,
            ((args.timeout % 1000) * 1000) as i64,
        );
        setsockopt(&sock, sockopt::ReceiveTimeout, &timeout).expect("Failed to set SO_RCVTIMEO");

        sock
    });

    println!(
        "Sending {} packets to {} (src: {}, dst: {})",
        args.count, args.interface, args.src_mac, args.dst_mac
    );
    if let Some(ref rx_if) = args.rx_interface {
        println!("Receiving on {} (timeout: {}ms)", rx_if, args.timeout);
    }

    // Minimum Ethernet frame: 14 bytes header + 46 bytes payload = 60 bytes
    let frame_size = 60;
    let mut sent_count = 0u64;

    for i in 0..args.count {
        let mut buffer = vec![0u8; frame_size];

        // Build Ethernet header
        buffer[0..6].copy_from_slice(&dst_mac);
        buffer[6..12].copy_from_slice(&src_mac);
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
        dest_addr.sll_ifindex = tx_if_index;
        dest_addr.sll_halen = 6;
        dest_addr.sll_addr[..6].copy_from_slice(&dst_mac);

        let dest = unsafe {
            LinkAddr::from_raw(
                &dest_addr as *const libc::sockaddr_ll as *const libc::sockaddr,
                Some(std::mem::size_of::<libc::sockaddr_ll>() as u32),
            )
            .expect("Failed to create dest LinkAddr")
        };

        match sendto(tx_sock.as_raw_fd(), &buffer, &dest, MsgFlags::empty()) {
            Ok(_) => {
                sent_count += 1;
            }
            Err(e) => eprintln!("Error sending packet {}: {}", i + 1, e),
        }
    }

    println!("Sent: {}/{}", sent_count, args.count);

    // Receive packets if rx_interface is specified
    if let Some(rx_sock) = rx_sock {
        let mut recv_count = 0u64;
        let mut buf = [0u8; 65535];
        let start = Instant::now();
        let timeout_duration = Duration::from_millis(args.timeout);

        while recv_count < args.count && start.elapsed() < timeout_duration {
            match recvfrom::<LinkAddr>(rx_sock.as_raw_fd(), &mut buf) {
                Ok((len, _)) => {
                    if len >= 14 {
                        // Check if packet matches our src MAC
                        if buf[6..12] == src_mac {
                            recv_count += 1;
                        }
                    }
                }
                Err(nix::errno::Errno::EAGAIN) => {
                    // Timeout
                    break;
                }
                Err(e) => {
                    eprintln!("Error receiving: {}", e);
                    break;
                }
            }
        }

        println!("Received: {}/{}", recv_count, args.count);

        if recv_count == args.count {
            println!("PASS: All packets received");
            ExitCode::SUCCESS
        } else {
            println!("FAIL: Packet loss detected");
            ExitCode::FAILURE
        }
    } else {
        println!("Done.");
        ExitCode::SUCCESS
    }
}
