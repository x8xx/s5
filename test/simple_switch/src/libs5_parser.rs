extern "C" {
    pub fn s5_sys_pkt_get_len(parser_args_ptr: i64) -> i32;
    pub fn s5_sys_pkt_read(parser_args_ptr: i64, offset: i32) -> i32;
    pub fn s5_sys_pkt_drop(parser_args_ptr: i64);
}
