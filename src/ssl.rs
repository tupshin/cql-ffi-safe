extern crate cql_ffi;

use cql_ffi::CassError::CASS_OK;

use string::CassString;
use error::CassError;

pub struct CassSsl<'a> {
    pub ssl:&'a mut cql_ffi::CassSsl
}

#[unsafe_destructor]
impl<'a> Drop for CassSsl<'a> {
    fn drop(&mut self) {unsafe{
        cql_ffi::cass_ssl_free(self.ssl)
    }}
}

impl<'a> CassSsl<'a> {
    pub fn new() -> CassSsl<'a> {unsafe{
        CassSsl{ssl:&mut*cql_ffi::cass_ssl_new()}
    }}

    //FIXME make this take a &str
    pub fn add_trusted_cert(&mut self, cert: CassString) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_ssl_add_trusted_cert(self.ssl, cert.string) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    pub fn set_verify_flags(&mut self, flags: i32) {unsafe{
        cql_ffi::cass_ssl_set_verify_flags(self.ssl, flags) 
    }}

    pub fn set_cert(&mut self, cert: CassString) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_ssl_set_cert(self.ssl, cert.string) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}

    //FIXME have this take another &str for the key?
    pub fn set_private_key(&mut self, key: CassString, password: &str) -> Result<(),CassError> {unsafe{
        match cql_ffi::cass_ssl_set_private_key(self.ssl, key.string, password.as_ptr() as *const i8) {
            CASS_OK => Ok(()),
            rc => Err(CassError{error:rc})
        }
    }}
}
