extern crate cql_ffi;
extern crate libc;

use cql_ffi_safe::iterator::CassIterator;
use cql_ffi_safe::string::CassString;
use cql_ffi_safe::value::CassValueType;
use cql_ffi_safe::row::CassRow;
use cql_ffi_safe::error::CassError;

#[allow(missing_copy_implementations)]
pub struct CassResult(pub *const cql_ffi::CassResult);

impl CassResult {
    pub fn iter(&self) -> CassIterator {
        unsafe{CassIterator(cql_ffi::cass_iterator_from_result(self.0))}
    }

    pub fn row_count(&self) -> u64 {unsafe{
        cql_ffi::cass_result_row_count(self.0)
    }}

    pub fn column_count(&self) -> u64 {unsafe{
        cql_ffi::cass_result_column_count(self.0)
    }}

    pub fn column_name(&self, index: u64) -> CassString {unsafe{
        CassString(cql_ffi::cass_result_column_name(self.0, index))
    }}

    pub fn column_type(&self, index: u64) -> CassValueType {unsafe{
        cql_ffi::cass_result_column_type(self.0, index)
    }}

    pub fn first_row(&self) -> Result<CassRow,CassError> {unsafe{
        Ok(CassRow(cql_ffi::cass_result_first_row(self.0)))
    }}

    pub fn has_more_pages(&self) -> bool {unsafe{
        if cql_ffi::cass_result_has_more_pages(self.0) > 0 {true} else {false}
    }}

}

//~ impl Drop for CassResult {
    //~ fn drop(&mut self) {unsafe{
    //~ //FIXME why does this lead to memory errors?
        //~ cql_ffi::cass_result_free(self.0)
    //~ }}
//~ }

