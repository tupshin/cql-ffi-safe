extern crate cql_ffi;

use cql_ffi_safe::future::CassFuture;
use cql_ffi_safe::cluster::CassCluster;
use cql_ffi_safe::statement::CassStatement;
use cql_ffi_safe::batch::CassBatch;
use cql_ffi_safe::string::CassString;
use cql_ffi_safe::error::CassError;
use cql_ffi_safe::prepared::CassPrepared;

use std::str::FromStr;

pub struct CassSession(pub cql_ffi::CassSession);

impl Drop for CassSession {
    fn drop(&mut self) {unsafe{
        cql_ffi::cass_session_free(&mut self.0)
    }}
}

impl CassSession {
    pub fn new() -> CassSession {unsafe{
        CassSession(*cql_ffi::cass_session_new())
    }}

    pub fn close(&mut self) -> CassFuture {unsafe{
        CassFuture(*cql_ffi::cass_session_close(&mut self.0))
    }}

    pub fn connect(&mut self, cluster:&mut CassCluster) -> CassFuture {unsafe{
        CassFuture(*cql_ffi::cass_session_connect(&mut self.0, &mut cluster.0))
    }}

    pub fn prepare(&mut self, query:String) -> CassFuture {unsafe{
        let string:CassString = FromStr::from_str(query.as_slice()).unwrap();
        CassFuture(*cql_ffi::cass_session_prepare(&mut self.0, string.0))
    }}

    pub fn execute(&mut self, mut statement:CassStatement) -> CassFuture {unsafe{
        CassFuture(*cql_ffi::cass_session_execute(&mut self.0, &mut statement.0))
    }}

    pub fn execute_batch(&mut self, batch:CassBatch) -> CassFuture {unsafe{
        CassFuture(*cql_ffi::cass_session_execute_batch(&mut self.0, &batch.0))
    }}

    pub fn prepare_insert_into_batch(&mut self, query:&str) -> Result<CassPrepared,CassError> {
        let future = self.prepare(query.to_string());
        future.wait();
        match future.error_code() {
            Ok(_) => Ok(future.get_prepared()),
            err => panic!("{:?}",err)
        }
    }



    //FIXME to mut or not to mut?
    //~ pub fn get_schema(&mut self, ) -> CassSchema {unsafe{
        //~ CassSchema{schema:&mut*cql_ffi::cass_session_get_schema(&mut*self.session).clone()}
    //~ }}

    pub fn connect_keyspace(&mut self, cluster:CassCluster, keyspace: &str) -> CassFuture {unsafe{
        CassFuture(*cql_ffi::cass_session_connect_keyspace(&mut self.0, &cluster.0, keyspace.as_ptr() as *const i8))
    }}
}
