extern crate cql_ffi;
extern crate collections;

use cql_ffi_safe::error::CassError;

use std::mem;
use std::ffi;
use std::str::FromStr;
use std::string::ToString;
use std::ptr;

#[derive(Copy,Debug)]
pub struct CassUuid(pub *const cql_ffi::CassUuid);

pub struct CassUuidGen(cql_ffi::CassUuidGen);

impl CassUuidGen {
    pub fn new() -> Self {unsafe{
        CassUuidGen(ptr::read(cql_ffi::cass_uuid_gen_new()))
    }}

    pub fn new_with_node(node: u64) -> Self {unsafe{
        CassUuidGen(ptr::read(cql_ffi::cass_uuid_gen_new_with_node(node)))
    }}

   pub fn time(&mut self) -> CassUuid {unsafe{
        let output =  mem::zeroed();
        cql_ffi::cass_uuid_gen_time(&mut self.0, output);
        CassUuid(output)
    }}

   pub fn gen_time(&mut self) -> CassUuid {unsafe{
        let output =  mem::zeroed();
        cql_ffi::cass_uuid_gen_time(&mut self.0, output);
        CassUuid(output)
    }}

    pub fn gen_random(&mut self) -> CassUuid {unsafe{
        let output =  mem::zeroed();
        cql_ffi::cass_uuid_gen_random(&mut self.0, output);
        CassUuid(output)
    }}

    pub fn gen_from_time(&mut self, timestamp: u64) -> CassUuid {unsafe{
        let output =  mem::zeroed();
        cql_ffi::cass_uuid_gen_from_time(&mut self.0, timestamp, output);
        CassUuid(output)
    }}
}

impl Drop for CassUuidGen {
 fn drop(&mut self) {unsafe{
        cql_ffi::cass_uuid_gen_free(&mut self.0);
    }}
}

impl CassUuid {
    pub fn min_from_time(timestamp: u64) -> Self {unsafe{
        let output =  mem::zeroed();
        cql_ffi::cass_uuid_min_from_time(timestamp, output);
        CassUuid(output)
    }}

    pub fn max_from_time(timestamp: u64) -> Self {unsafe{
        let output =  mem::zeroed();
        cql_ffi::cass_uuid_max_from_time(timestamp, output);
        CassUuid(output)
    }}

    pub fn timestamp(uuid: CassUuid) -> u64 {unsafe{
        cql_ffi::cass_uuid_timestamp(*uuid.0)
    }}

    pub fn version(uuid: CassUuid) -> u8 {unsafe{
        cql_ffi::cass_uuid_version(*uuid.0)
    }}
}

impl ToString for CassUuid {
    fn to_string(&self) -> String {unsafe{
        let output:*mut i8 =  mem::zeroed();
        cql_ffi::cass_uuid_string(*self.0, output);
        let output = output as *const i8;
        String::from_utf8_lossy(ffi::c_str_to_bytes(&output)).into_owned()
    }}
}

impl FromStr for CassUuid {
    type Err = CassError;
    fn from_str(string:&str) -> Result<Self,CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_uuid_from_string(string.as_ptr() as *const i8,output) {
            cql_ffi::CassError::CASS_OK => Ok(CassUuid(output)),
            err => Err(CassError(&err))
        }
    }}
}
