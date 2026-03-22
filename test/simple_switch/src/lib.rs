#![no_std]
#![no_main]

mod libs5_parser;
use libs5_parser::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// EtherType constants
const ETHERTYPE_IPV4: u16 = 0x0800;
const ETHERTYPE_IPV6: u16 = 0x86DD;

/// Parser entry point called from WASM runtime.
/// Returns true if packet should be forwarded, false if it should be dropped.
#[no_mangle]
pub extern "C" fn parse(parser_args_ptr: i64) -> bool {
    let pkt_len = unsafe { s5_sys_pkt_get_len(parser_args_ptr) };

    // Minimum Ethernet frame: 14 bytes (6 dst + 6 src + 2 ethertype)
    // Drop frames shorter than Ethernet header
    if pkt_len < 14 {
        unsafe {
            s5_sys_pkt_drop(parser_args_ptr);
        }
        return false;
    }

    // Read EtherType (bytes 12-13, big endian)
    let ethertype_hi = unsafe { s5_sys_pkt_read(parser_args_ptr, 12) } as u16;
    let ethertype_lo = unsafe { s5_sys_pkt_read(parser_args_ptr, 13) } as u16;
    let ethertype = (ethertype_hi << 8) | ethertype_lo;

    // Accept only IPv4 and IPv6 packets
    if ethertype == ETHERTYPE_IPV4 || ethertype == ETHERTYPE_IPV6 {
        return true;
    }

    // Drop non-IPv4/IPv6 packets
    unsafe {
        s5_sys_pkt_drop(parser_args_ptr);
    }
    false
}
