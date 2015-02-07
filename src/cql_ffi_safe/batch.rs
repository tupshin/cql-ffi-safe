extern crate cql_ffi;

use cql_ffi_safe::consistency::CassConsistency;
use cql_ffi_safe::error::CassError;
use cql_ffi_safe::statement::CassStatement;

pub use cql_ffi::CassBatchType;

pub struct CassBatch(pub *mut cql_ffi::CassBatch);

#[unsafe_destructor]
impl Drop for CassBatch {
    fn drop(&mut self) {
        unsafe{cql_ffi::cass_batch_free(self.0)}
    }
}

impl CassBatch {
    pub fn new(_type:CassBatchType) -> CassBatch {unsafe{
        CassBatch(cql_ffi::cass_batch_new(_type))
    }}

    pub fn set_consistency(&mut self, consistency: CassConsistency) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_batch_set_consistency(self.0, consistency)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }
    
    pub fn add_statement(&mut self, statement: CassStatement) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_batch_add_statement(self.0, &mut*statement.0)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }
}
