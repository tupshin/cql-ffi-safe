extern crate cql_ffi;

use std::num::Int;
use libc::types;

struct CassLogLevel {
    log_level:cql_ffi::CassLogLevel
}

struct CassLogCallBack {
    callback:cql_ffi::CassLogCallback
}

impl CassLogCallBack {
    //FIXME don't force passing in a void
    fn set_callback(&self, data:  &mut types::common::c95::c_void) {unsafe{
        cql_ffi::cass_log_set_callback(self.callback, data)
    }}
}

impl ToString for CassLogLevel {
    fn to_string(&self) -> String {
       panic!("FIXME");
    }
}

impl CassLogLevel {
    pub fn cleanup(&self) {unsafe{
        cql_ffi::cass_log_cleanup()
    }}

    pub fn set_level(&self) {unsafe{
        cql_ffi::cass_log_set_level(self.log_level)
    }}

    pub fn set_queue_size(queue_size: u64) {unsafe{
        cql_ffi::cass_log_set_queue_size(queue_size)
    }}

}
