extern crate cql_ffi;

use cql_ffi_safe::statement::CassStatement;

pub struct CassPrepared(pub cql_ffi::CassPrepared);

#[unsafe_destructor]
impl Drop for CassPrepared {
    fn drop(&mut self) {unsafe{
        cql_ffi::cass_prepared_free(&mut self.0)
    }}
}

impl CassPrepared {
    pub fn bind(&self) -> CassStatement {unsafe{
        CassStatement(*cql_ffi::cass_prepared_bind(&self.0))
    }}
}

