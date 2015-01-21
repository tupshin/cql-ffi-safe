extern crate cql_ffi;
extern crate libc;

use cass_iterator::CassIterator;
use cass_value::CassValue;


pub struct CassRow<'a> {
    pub row:&'a cql_ffi::CassRow
}


impl<'a> CassRow<'a> {
    fn iter(self) -> CassIterator<'a> {unsafe{
        CassIterator{iterator:&mut*cql_ffi::cass_iterator_from_row(self.row)}
    }}

    fn get_column(row: CassRow, index: u64) -> CassValue {unsafe{
        CassValue{value:&*cql_ffi::cass_row_get_column(row.row, index)}
    }}

    fn get_column_by_name(row: CassRow, name: &str) -> CassValue<'a> {unsafe{
        CassValue{value:&*cql_ffi::cass_row_get_column_by_name(row.row, name.as_ptr() as *const i8)}
    }}
}
