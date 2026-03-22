pub struct ParserArgs<'a> {
    pub pkt: *mut u8,
    pub pkt_len: usize,
    pub is_accept: &'a mut bool,
}

pub fn s5_sys_pkt_get_len(parser_args_ptr: i64) -> i32 {
    unsafe { (*(parser_args_ptr as *const ParserArgs)).pkt_len as i32 }
}

pub fn s5_sys_pkt_read(parser_args_ptr: i64, offset: i32) -> i32 {
    unsafe { *(*(parser_args_ptr as *const ParserArgs)).pkt.offset(offset as isize) as i32 }
}

pub fn s5_sys_pkt_drop(parser_args_ptr: i64) {
    let parser_args = unsafe { &mut *(parser_args_ptr as *mut ParserArgs) };
    *parser_args.is_accept = false;
}
