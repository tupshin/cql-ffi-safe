extern crate cql_ffi;

use std::ffi::CString;

use std::str::FromStr;
use std::string::ToString;
use std::slice;

#[derive(Copy,Show)]
pub struct CassString {
    pub string:cql_ffi::CassString
}

impl ToString for CassString {
    fn to_string(&self) -> String {unsafe{
        if self.string.length > 1000000 {panic!("wtf: {}", self.string.length)};
        let data = self.string.data as *const u8;
        let slice = slice::from_raw_buf(&data,self.string.length as usize);
        let vec = slice.to_vec();
        String::from_utf8(vec).unwrap()
    }}
}

impl FromStr for CassString {
    fn from_str(string:&str) -> Option<Self> {unsafe{
        let cass_str = cql_ffi::cass_string_init(CString::from_slice(string.as_bytes()).as_ptr());
        Some(CassString{string:cass_str})
    }}
}
