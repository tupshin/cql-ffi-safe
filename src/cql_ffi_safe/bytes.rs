extern crate cql_ffi;

use std::slice;

#[allow(missing_copy_implementations)]
pub struct CassBytes(pub *const cql_ffi::CassBytes);


impl CassBytes {
    pub fn new(data: &[u8]) -> CassBytes {
        //FIXME review and carefully test this code
        CassBytes(unsafe{&cql_ffi::cass_bytes_init(data.as_ptr() as *const u8, data.len() as u64)})
    }

    pub fn as_bytes(&mut self) -> Vec<u8> {unsafe{
        let mut vec = Vec::<u8>::new();
        vec.push_all(slice::from_raw_buf(&(*self.0).data,(*self.0).size as usize));
        vec
    }}
}
