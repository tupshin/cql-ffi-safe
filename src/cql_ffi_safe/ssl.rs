extern crate cql_ffi;

use cql_ffi::CassError::CASS_OK;

use cql_ffi_safe::string::CassString;
use cql_ffi_safe::error::CassError;

pub struct CassSsl(pub cql_ffi::CassSsl);

#[unsafe_destructor]
impl Drop for CassSsl {
    fn drop(&mut self) {unsafe{
        cql_ffi::cass_ssl_free(&mut self.0)
    }}
}

impl CassSsl {
    pub fn new() -> CassSsl {unsafe{
        CassSsl(*cql_ffi::cass_ssl_new())
    }}

    //FIXME make this take a &str
    pub fn add_trusted_cert(&mut self, cert: CassString) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_ssl_add_trusted_cert(&mut self.0, cert.0) {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }}

    pub fn set_verify_flags(&mut self, flags: i32) {unsafe{
        cql_ffi::cass_ssl_set_verify_flags(&mut self.0, flags) 
    }}

    pub fn set_cert(&mut self, cert: CassString) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_ssl_set_cert(&mut self.0, cert.0) {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }}

    //FIXME have this take another &str for the key?
    pub fn set_private_key(&mut self, key: CassString, password: &str) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_ssl_set_private_key(&mut self.0, key.0, password.as_ptr() as *const i8) {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }}
}
