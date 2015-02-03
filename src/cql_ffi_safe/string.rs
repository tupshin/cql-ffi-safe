extern crate cql_ffi;

use std::str::FromStr;
use std::string::ToString;
use std::slice;

#[derive(Copy,Debug)]
pub struct CassString {
    pub string:cql_ffi::CassString
}

impl ToString for CassString {
    fn to_string(&self) -> String {unsafe{
        let data = self.string.data as *const u8;
        let slice = slice::from_raw_buf(&data,self.string.length as usize);
        let vec = slice.to_vec();
        String::from_utf8(vec).unwrap()
    }}
}

impl FromStr for CassString {
    type Err = String;
    fn from_str(string:&str) -> Result<Self, String> {unsafe{
        let cass_str = cql_ffi::cass_string_init2(string.as_ptr() as *const i8,string.len() as u64);
        Ok(CassString{string:cass_str})
    }}
}
