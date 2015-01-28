extern crate cql_ffi;

use cql_ffi_safe::future::CassFuture;
use cql_ffi_safe::cluster::CassCluster;
use cql_ffi_safe::statement::CassStatement;
use cql_ffi_safe::batch::CassBatch;
use cql_ffi_safe::string::CassString;
use cql_ffi_safe::error::CassError;
use cql_ffi_safe::prepared::CassPrepared;

use std::str::FromStr;

pub struct CassSession<'a> {
    session:&'a mut cql_ffi::CassSession
}

#[unsafe_destructor]
impl<'a> Drop for CassSession<'a> {
    fn drop(&mut self) {unsafe{
        cql_ffi::cass_session_free(self.session)
    }}
}

impl<'a> CassSession<'a> {
    pub fn new() -> CassSession<'static> {unsafe{
        CassSession{session:&mut*cql_ffi::cass_session_new()}
    }}

    pub fn close(&mut self) -> CassFuture {unsafe{
        CassFuture{future:&mut*cql_ffi::cass_session_close(&mut*self.session)}
    }}

    pub fn connect(&mut self, cluster:&CassCluster) -> CassFuture<'static> {unsafe{
        CassFuture{future:&mut*cql_ffi::cass_session_connect(&mut*self.session, cluster.cluster)}
    }}

    pub fn prepare(&mut self, query:String) -> CassFuture<'static> {unsafe{
        let string:CassString = FromStr::from_str(query.as_slice()).unwrap();
        CassFuture{future:&mut*cql_ffi::cass_session_prepare(&mut*self.session, string.string)}
    }}

    pub fn execute(&mut self, statement:CassStatement) -> CassFuture<'static> {unsafe{
        CassFuture{future:&mut*cql_ffi::cass_session_execute(&mut*self.session, statement.statement)}
    }}

    pub fn execute_batch(&mut self, batch:CassBatch) -> CassFuture<'static> {unsafe{
        CassFuture{future:&mut*cql_ffi::cass_session_execute_batch(&mut*self.session, batch.batch)}
    }}

    pub fn prepare_insert_into_batch(&mut self, query:&str) -> Result<CassPrepared<'a>,CassError> {
        let future:CassFuture = self.prepare(query.to_string());
        future.wait();
        let prepared = future.get_prepared();
        Ok(prepared)
    }



    //FIXME to mut or not to mut?
    //~ pub fn get_schema(&mut self, ) -> CassSchema {unsafe{
        //~ CassSchema{schema:&mut*cql_ffi::cass_session_get_schema(&mut*self.session).clone()}
    //~ }}

    pub fn connect_keyspace(&mut self, cluster:CassCluster, keyspace: &str) -> CassFuture {unsafe{
        CassFuture{future:&mut*cql_ffi::cass_session_connect_keyspace(&mut*self.session, cluster.cluster, keyspace.as_ptr() as *const i8)}
    }}
}
