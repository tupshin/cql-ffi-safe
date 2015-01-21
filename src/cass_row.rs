extern crate cql_ffi;
extern crate libc;

use cass_iterator::CassIterator;


pub struct CassRow<'a> {
    pub row:&'a cql_ffi::CassRow
}


impl<'a> CassRow<'a> {
    fn iter(self) -> CassIterator<'a> {unsafe{
        CassIterator{iterator:&mut*cql_ffi::cass_iterator_from_row(self.row)}
    }}
}
