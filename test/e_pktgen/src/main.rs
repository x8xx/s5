use clap::Parser;
use std::process::ExitCode;

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

    #[arg(long, default_value = "60", help = "Frame size in bytes (min: 14)")]
    frame_size: usize,

    #[arg(long, help = "Expected receive count (default: same as send count)")]
    expect: Option<u64>,

    #[arg(long, default_value = "0800", help = "EtherType in hex (e.g., 0800=IPv4, 86dd=IPv6, 0806=ARP)")]
    ethertype: String,
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

fn parse_ethertype(s: &str) -> [u8; 2] {
    let val = u16::from_str_radix(s, 16).expect("Invalid EtherType (use hex, e.g., 0800)");
    val.to_be_bytes()
}

#[cfg(target_os = "linux")]
fn main() -> ExitCode {
    use nix::sys::socket::{
        bind, recvfrom, sendto, setsockopt, socket, sockopt, AddressFamily, LinkAddr, MsgFlags,
        SockFlag, SockType, SockaddrLike,
    };
    use nix::sys::time::TimeVal;
    use std::ffi::CString;
    use std::os::fd::{AsRawFd, OwnedFd};
    use std::time::{Duration, Instant};

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

    let args = Args::parse();

    let dst_mac = parse_mac(&args.dst_mac);
    let src_mac = parse_mac(&args.src_mac);
    let ethertype = parse_ethertype(&args.ethertype);

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
            (args.timeout / 1000) as libc::time_t,
            ((args.timeout % 1000) * 1000) as libc::suseconds_t,
        );
        setsockopt(&sock, sockopt::ReceiveTimeout, &timeout).expect("Failed to set SO_RCVTIMEO");

        sock
    });

    let frame_size = args.frame_size;
    let expected_recv = args.expect.unwrap_or(args.count);

    println!(
        "Sending {} packets to {} (src: {}, dst: {}, size: {}, ethertype: 0x{})",
        args.count, args.interface, args.src_mac, args.dst_mac, frame_size, args.ethertype
    );
    if let Some(ref rx_if) = args.rx_interface {
        println!(
            "Receiving on {} (timeout: {}ms, expect: {})",
            rx_if, args.timeout, expected_recv
        );
    }

    let mut sent_count = 0u64;

    for i in 0..args.count {
        let mut buffer = vec![0u8; frame_size];

        // Build Ethernet header if frame is large enough
        if frame_size >= 6 {
            buffer[0..6.min(frame_size)].copy_from_slice(&dst_mac[..6.min(frame_size)]);
        }
        if frame_size >= 12 {
            buffer[6..12].copy_from_slice(&src_mac);
        }
        if frame_size >= 14 {
            buffer[12] = ethertype[0];
            buffer[13] = ethertype[1];
        }

        // Fill payload with sequence number pattern
        if frame_size > 14 {
            let seq_bytes = (i as u32).to_be_bytes();
            for j in 14..frame_size {
                buffer[j] = seq_bytes[(j - 14) % 4];
            }
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

        while recv_count < expected_recv && start.elapsed() < timeout_duration {
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

        println!("Received: {}/{}", recv_count, expected_recv);

        if recv_count == expected_recv {
            println!("PASS: Expected packets received");
            ExitCode::SUCCESS
        } else {
            println!("FAIL: Unexpected packet count");
            ExitCode::FAILURE
        }
    } else {
        println!("Done.");
        ExitCode::SUCCESS
    }
}

#[cfg(not(target_os = "linux"))]
fn main() -> ExitCode {
    eprintln!("e_pktgen requires Linux (AF_PACKET sockets)");
    eprintln!("Please build and run in a Linux environment or VM");
    ExitCode::FAILURE
}
