extern crate cql_ffi;
extern crate libc;

use cass_iterator::CassIterator;
use cass_value::CassValue;

#[derive(Copy)]
pub struct CassRow<'a> {
    pub row:&'a cql_ffi::CassRow
}


impl<'a> CassRow<'a> {
    pub fn iter(self) -> CassIterator<'a> {unsafe{
        CassIterator{iterator:&mut*cql_ffi::cass_iterator_from_row(self.row)}
    }}

    pub fn get_column(self, index: u64) -> CassValue<'a> {unsafe{
        CassValue{value:&*cql_ffi::cass_row_get_column(self.row, index)}
    }}

    pub fn get_column_by_name(row: CassRow, name: &str) -> CassValue<'a> {unsafe{
        CassValue{value:&*cql_ffi::cass_row_get_column_by_name(row.row, name.as_ptr() as *const i8)}
    }}
}
