extern crate cql_ffi;
extern crate libc;

//~ use libc::types::common::c95::c_void;
 
//use cql_ffi::CassFutureCallback;
use cql_ffi_safe::error::CassError;
use cql_ffi_safe::result::CassResult;
use cql_ffi_safe::prepared::CassPrepared;
use cql_ffi_safe::string::CassString;

pub struct CassFuture(pub *mut cql_ffi::CassFuture);

//~ impl Future for CassFuture {

//}

impl CassFuture {
    //FIXME. How do I work with c_voids and callbacks?
    //~ pub fn set_callback(self, callback: CassFutureCallback, data: c_void) -> Result<(),CassError> {
        //~ let cl_result = unsafe{cql_ffi::cass_future_set_callback(self.future, callback, data)};
        //~ match cl_result {
            //~ cql_ffi::CassError::CASS_OK => Ok(()),
            //~ _=> Err(CassError::new(cl_result))
        //~ }
    //~ }

    pub fn ready(&mut self) -> bool {
        if unsafe{cql_ffi::cass_future_ready(self.0)} > 0 {true} else {false}
    }

    pub fn wait(&mut self) -> &mut Self {
        unsafe{cql_ffi::cass_future_wait(self.0)};
        self
    }

    pub fn wait_timed(&mut self, timeout_us: u64) -> bool {
        if unsafe{cql_ffi::cass_future_wait_timed(self.0, timeout_us)} > 0 {true} else {false}
    }

    pub fn get_result(&mut self) -> Option<CassResult> {unsafe{
        Some(CassResult(cql_ffi::cass_future_get_result(self.0)))
    }}

    pub fn get_prepared(&mut self) -> CassPrepared {unsafe{
        CassPrepared(cql_ffi::cass_future_get_prepared(self.0),"".to_string())
    }}

    pub fn error_code(&mut self) -> Result<(),CassError> {unsafe{
        let rc = cql_ffi::cass_future_error_code(self.0);
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }}

    pub fn is_error(&mut self) -> bool {
        match self.error_code() {
            Ok(_) => false,
            Err(_) => true
        }
    }

    pub fn error_message(&mut self) -> CassString {unsafe{
        CassString(cql_ffi::cass_future_error_message(self.0))
    }}
}

impl Drop for CassFuture {
    fn drop(&mut self){unsafe{
        cql_ffi::cass_future_free(self.0)
    }}
}
