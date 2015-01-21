extern crate cql_ffi;

use cass_bytes::CassBytes;
use std::default::Default;

#[derive(Copy)]
pub struct CassDecimal {
    pub decimal:cql_ffi::CassDecimal
}

impl Default for CassDecimal {
    fn default() -> CassDecimal { CassDecimal{decimal:Default::default() } }
}


impl CassDecimal {
    //FIXME there needs to be a rust-friendlier way to create these
    pub fn init(scale: i32, varint: CassBytes) -> CassDecimal {unsafe{
        CassDecimal{decimal:cql_ffi::cass_decimal_init(scale,varint.bytes)}
    }}
}

