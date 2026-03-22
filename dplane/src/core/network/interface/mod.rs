use nix::sys::socket::{bind, socket, AddressFamily, LinkAddr, SockFlag, SockType, SockaddrLike};
use std::ffi::CString;
use std::os::fd::{AsRawFd, OwnedFd};

pub fn get_interface_index(name: &str) -> i32 {
    let c_name = CString::new(name).expect("Invalid interface name");
    unsafe {
        let index = libc::if_nametoindex(c_name.as_ptr());
        if index == 0 {
            panic!("Interface {} not found", name);
        }
        index as i32
    }
}

pub fn create_raw_socket() -> OwnedFd {
    socket(
        AddressFamily::Packet,
        SockType::Raw,
        SockFlag::empty(),
        Some(nix::sys::socket::SockProtocol::EthAll),
    )
    .expect("Failed to create raw socket")
}

pub fn bind_to_interface(fd: &OwnedFd, if_index: i32) {
    let mut addr_storage: libc::sockaddr_ll = unsafe { std::mem::zeroed() };
    addr_storage.sll_family = libc::AF_PACKET as u16;
    addr_storage.sll_protocol = (libc::ETH_P_ALL as u16).to_be();
    addr_storage.sll_ifindex = if_index;

    let addr = unsafe {
        LinkAddr::from_raw(
            &addr_storage as *const libc::sockaddr_ll as *const libc::sockaddr,
            Some(std::mem::size_of::<libc::sockaddr_ll>() as u32),
        )
        .expect("Failed to create LinkAddr")
    };

    bind(fd.as_raw_fd(), &addr).expect("Failed to bind socket");
}
