extern crate cql_ffi;

#[derive(Copy)]
pub struct CassBytes {
    pub bytes:cql_ffi::CassBytes
}

impl CassBytes {
    pub fn new(data: &[u8]) -> CassBytes {
        //FIXME review and carefully test this code
        CassBytes{bytes:unsafe{cql_ffi::cass_bytes_init(data.as_ptr() as *const u8, data.len() as u64)}}
    }
}
