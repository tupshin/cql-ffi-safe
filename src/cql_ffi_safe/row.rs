extern crate cql_ffi;
extern crate libc;

use cql_ffi_safe::iterator::CassIterator;
use cql_ffi_safe::column::CassColumn;

#[allow(missing_copy_implementations)]
pub struct CassRow(pub *const cql_ffi::CassRow);

impl CassRow {
    pub fn iter(&self) -> CassIterator {unsafe{
        CassIterator(cql_ffi::cass_iterator_from_row(self.0))
    }}

    pub fn get_column(&self, index: u64) -> CassColumn {unsafe{
        CassColumn(cql_ffi::cass_row_get_column(self.0, index))
    }}

    pub fn get_column_by_name(&self, name: &str) -> CassColumn {unsafe{
        CassColumn(cql_ffi::cass_row_get_column_by_name(self.0, name.as_ptr() as *const i8))
    }}
}
