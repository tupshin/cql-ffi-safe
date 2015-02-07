extern crate cql_ffi;

use std::ffi::c_str_to_bytes;
use std::str;
use std::fmt::Formatter;
use std::fmt;
use std::fmt::Debug;

#[derive(Copy)]
pub struct CassError(pub cql_ffi::CassError);

impl CassError {
    pub fn new(error:cql_ffi::CassError) -> CassError {
        CassError(error)
    }

    pub fn desc(&self) -> String {unsafe{
        let c_str = &cql_ffi::cass_error_desc(self.0);
        str::from_utf8(c_str_to_bytes(c_str)).unwrap().to_string()
    }}
}

impl Debug for CassError {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        write!(f, "{:?} - {:?}", self.0,self.desc())
    }
}
