extern crate cql_ffi;

use std::ffi::c_str_to_bytes;
use std::str;

#[derive(Copy)]
pub struct CassError {
    pub error:cql_ffi::CassError
}

impl CassError {
    pub fn new(error:cql_ffi::CassError) -> CassError {
        CassError{error:error}
    }

    pub fn desc(self) -> String {unsafe{
        str::from_utf8(c_str_to_bytes(&cql_ffi::cass_error_desc(self.error))).unwrap().to_string()
    }}
}
