extern crate cql_ffi;
extern crate libc;

use cql_ffi_safe::iterator::CassIterator;
use cql_ffi_safe::string::CassString;
use cql_ffi_safe::value::CassValueType;
use cql_ffi_safe::row::CassRow;
use cql_ffi_safe::error::CassError;

pub struct CassResult<'a> {
    pub result:&'a cql_ffi::CassResult
}

impl<'a> CassResult<'a> {
    pub fn iter(self) -> CassIterator<'a> {
        unsafe{CassIterator{iterator:&mut*cql_ffi::cass_iterator_from_result(self.result)}}
    }

    pub fn row_count(&self) -> u64 {unsafe{
        cql_ffi::cass_result_row_count(self.result)
    }}

    pub fn column_count(&self) -> u64 {unsafe{
        cql_ffi::cass_result_column_count(self.result)
    }}

    pub fn column_name(&self, index: u64) -> CassString {unsafe{
        CassString{string:cql_ffi::cass_result_column_name(self.result, index)}
    }}

    pub fn column_type(&self, index: u64) -> CassValueType {unsafe{
        cql_ffi::cass_result_column_type(self.result, index)
    }}

    pub fn first_row(&self) -> Result<CassRow<'a>,CassError> {unsafe{
        Ok(CassRow{row:&*cql_ffi::cass_result_first_row(self.result)})
    }}

    pub fn has_more_pages(&self) -> bool {unsafe{
        if cql_ffi::cass_result_has_more_pages(self.result) > 0 {true} else {false}
    }}

}

#[unsafe_destructor]
impl<'a> Drop for CassResult<'a> {
    fn drop(&mut self) {unsafe{
    //FIXME why does this lead to memory errors?
    //    cql_ffi::cass_result_free(self.result)
    }}
}

