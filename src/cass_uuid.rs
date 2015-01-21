extern crate cql_ffi;
extern crate collections;

use std::mem;
use std::ffi;
use std::str::FromStr;
use std::string::ToString;

#[derive(Copy)]
pub struct CassUuid {
    pub uuid:cql_ffi::CassUuid
}

pub struct CassUuidGen<'a> {
    pub uuid_gen:&'a mut cql_ffi::CassUuidGen
}

impl<'a> CassUuidGen<'a> {
    fn new() -> Self {unsafe{
        CassUuidGen{uuid_gen:&mut*cql_ffi::cass_uuid_gen_new()}
    }}

    fn new_with_node(node: u64) -> Self {unsafe{
        CassUuidGen{uuid_gen:&mut*cql_ffi::cass_uuid_gen_new_with_node(node)}
    }}

   fn time(self) -> CassUuid {unsafe{
        let output =  mem::zeroed();
        cql_ffi::cass_uuid_gen_time(self.uuid_gen, output);
        CassUuid{uuid:*output}
    }}

   fn gen_time(self) -> CassUuid {unsafe{
        let output =  mem::zeroed();
        cql_ffi::cass_uuid_gen_time(self.uuid_gen, output);
        CassUuid{uuid:*output}
    }}

    fn gen_random(self) -> CassUuid {unsafe{
        let output =  mem::zeroed();
        cql_ffi::cass_uuid_gen_random(self.uuid_gen, output);
        CassUuid{uuid:*output}
    }}

    fn gen_from_time(self, timestamp: u64) -> CassUuid {unsafe{
        let output =  mem::zeroed();
        cql_ffi::cass_uuid_gen_from_time(self.uuid_gen, timestamp, output);
        CassUuid{uuid:*output}
    }}
}

#[unsafe_destructor]
impl<'a> Drop for CassUuidGen<'a> {
 fn drop(&mut self) {unsafe{
        cql_ffi::cass_uuid_gen_free(self.uuid_gen);
    }}
}

impl CassUuid {
    pub fn min_from_time(timestamp: u64) -> Self {unsafe{
        let output =  mem::zeroed();
        cql_ffi::cass_uuid_min_from_time(timestamp, output);
        CassUuid{uuid:*output}
    }}

    pub fn max_from_time(timestamp: u64) -> Self {unsafe{
        let output =  mem::zeroed();
        cql_ffi::cass_uuid_max_from_time(timestamp, output);
        CassUuid{uuid:*output}
    }}

    pub fn timestamp(uuid: CassUuid) -> u64 {unsafe{
        cql_ffi::cass_uuid_timestamp(uuid.uuid)
    }}

    pub fn version(uuid: CassUuid) -> u8 {unsafe{
        cql_ffi::cass_uuid_version(uuid.uuid)
    }}
}

impl ToString for CassUuid {
    fn to_string(&self) -> String {unsafe{
        let output:*mut i8 =  mem::zeroed();
        cql_ffi::cass_uuid_string(self.uuid, output);
        let output = output as *const i8;
        String::from_utf8_lossy(ffi::c_str_to_bytes(&output)).into_owned()
    }}
}

impl FromStr for CassUuid {
    fn from_str(string:&str) -> Option<Self> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_uuid_from_string(string.as_ptr() as *const i8,output) {
            cql_ffi::CassError::CASS_OK => Some(CassUuid{uuid:*output}),
            _=> None
        }
    }}
}
