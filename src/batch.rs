extern crate cql_ffi;

use consistency::CassConsistency;
use error::CassError;
use statement::CassStatement;

pub use cql_ffi::CassBatchType;

pub struct CassBatch<'a> {
    pub batch:&'a mut cql_ffi::CassBatch
}

#[unsafe_destructor]
impl<'a> Drop for CassBatch<'a> {
    fn drop(&mut self) {
        unsafe{cql_ffi::cass_batch_free(self.batch)}
    }
}

impl<'a> CassBatch<'a> {
    pub fn new(_type:CassBatchType) -> CassBatch<'a> {unsafe{
        CassBatch{batch:&mut*cql_ffi::cass_batch_new(_type)}
    }}

    pub fn set_consistency(&self, consistency: CassConsistency) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_batch_set_consistency(self.batch, consistency)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }
    
    pub fn add_statement(&self, statement: CassStatement) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_batch_add_statement(self.batch, statement.statement)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }
}
