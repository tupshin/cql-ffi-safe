extern crate cql_ffi;

use cql_ffi_safe::statement::CassStatement;

pub struct CassPrepared<'a> {
    pub prepared:&'a cql_ffi::CassPrepared
}

#[unsafe_destructor]
impl<'a> Drop for CassPrepared<'a> {
    fn drop(&mut self) {unsafe{
        cql_ffi::cass_prepared_free(self.prepared)
    }}
}

impl<'a> CassPrepared<'a> {
    pub fn bind(&self) -> CassStatement {unsafe{
        CassStatement{statement:&mut*cql_ffi::cass_prepared_bind(self.prepared)}
    }}
}

