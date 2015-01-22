extern crate cql_ffi;
extern crate libc;

//~ use libc::types::common::c95::c_void;
 
//use cql_ffi::CassFutureCallback;
use cass_error::CassError;
use cass_result::CassResult;
use cass_prepared::CassPrepared;
use cass_string::CassString;


pub struct CassFuture<'a> {
    pub future:&'a mut cql_ffi::CassFuture
}

//~ impl<'a> Future for CassFuture<'a> {

//}

impl<'a> CassFuture<'a> {
    //FIXME. How do I work with c_voids and callbacks?
    //~ pub fn set_callback(self, callback: CassFutureCallback, data: c_void) -> Result<(),CassError> {
        //~ let cl_result = unsafe{cql_ffi::cass_future_set_callback(self.future, callback, data)};
        //~ match cl_result {
            //~ cql_ffi::CassError::CASS_OK => Ok(()),
            //~ _=> Err(CassError::new(cl_result))
        //~ }
    //~ }

    pub fn ready(&mut self) -> bool {
        if unsafe{cql_ffi::cass_future_ready(&mut*self.future)} > 0 {true} else {false}
    }

    pub fn wait(&self) {
        unsafe{cql_ffi::cass_future_wait(self.future)}
    }

    pub fn wait_timed(&mut self, timeout_us: u64) -> bool {
        if unsafe{cql_ffi::cass_future_wait_timed(&mut*self.future, timeout_us)} > 0 {true} else {false}
    }

    pub fn get_result(&mut self) -> CassResult {unsafe{
        CassResult{result:&*cql_ffi::cass_future_get_result(self.future)}
    }}

    pub fn get_prepared<'b>(&self) -> CassPrepared<'b> {unsafe{
        CassPrepared{prepared:&*cql_ffi::cass_future_get_prepared(self.future)}
    }}

    pub fn error_code(&self) -> Result<(),CassError> {unsafe{
        let rc = cql_ffi::cass_future_error_code(self.future);
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }}

    pub fn is_error(&self) -> bool {
        match self.error_code() {
            Ok(_) => false,
            Err(_) => true
        }
    }

    pub fn error_message(&self) -> CassString {unsafe{
        CassString{string:cql_ffi::cass_future_error_message(self.future)}
    }}
}


#[unsafe_destructor]
impl<'a> Drop for CassFuture<'a> {
    fn drop(&mut self){unsafe{
        cql_ffi::cass_future_free(&mut*self.future)
    }}
}
