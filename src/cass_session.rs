extern crate cql_ffi;

use cass_future::CassFuture;
use cass_cluster::CassCluster;
use cass_statement::CassStatement;
use cass_string::CassString;

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

    pub fn prepare(&mut self, query:CassString) -> CassFuture<'static> {unsafe{
        CassFuture{future:&mut*cql_ffi::cass_session_prepare(&mut*self.session, query.string)}
    }}

    pub fn execute(&mut self, statement:CassStatement) -> CassFuture<'static> {unsafe{
        CassFuture{future:&mut*cql_ffi::cass_session_execute(&mut*self.session, statement.statement)}
    }}

    //FIXME to mut or not to mut?
    //~ pub fn get_schema(&mut self, ) -> CassSchema {unsafe{
        //~ CassSchema{schema:&mut*cql_ffi::cass_session_get_schema(&mut*self.session).clone()}
    //~ }}

    pub fn connect_keyspace(&mut self, cluster:CassCluster, keyspace: &str) -> CassFuture {unsafe{
        CassFuture{future:&mut*cql_ffi::cass_session_connect_keyspace(&mut*self.session, cluster.cluster, keyspace.as_ptr() as *const i8)}
    }}
}
