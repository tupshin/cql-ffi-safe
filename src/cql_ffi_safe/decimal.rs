extern crate cql_ffi;

use cql_ffi_safe::bytes::CassBytes;

use std::default::Default;

#[derive(Copy)]
pub struct CassDecimal(pub cql_ffi::CassDecimal);

impl Default for CassDecimal {
    fn default() -> CassDecimal { CassDecimal(Default::default() ) }
}


impl CassDecimal {
    //FIXME there needs to be a rust-friendlier way to create these
    pub fn init(scale: i32, varint: CassBytes) -> CassDecimal {unsafe{
        CassDecimal(cql_ffi::cass_decimal_init(scale,varint.0))
    }}
}

