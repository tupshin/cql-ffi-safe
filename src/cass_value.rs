extern crate cql_ffi;

use std::mem;
//~ use uuid::Uuid;
//~ use std::io::net::ip::IpAddr;

pub use cql_ffi::CassValueType;
 
use cass_error::CassError;
use cass_uuid::CassUuid;
use cass_inet::CassInet;
use cass_iterator::CassIterator;
use cass_string::CassString;
use cass_bytes::CassBytes;
use cass_decimal::CassDecimal;

#[derive(Copy)]
pub struct CassValue<'a> {
    pub value:&'a cql_ffi::CassValue
}

impl<'a> CassValue<'a> {
   

    //FIXME this is bad api


    pub fn get_type(self) -> CassValueType {unsafe{
        cql_ffi::cass_value_type(self.value)
    }}

    pub fn is_null(self) -> bool {unsafe{
        match cql_ffi::cass_value_is_null(self.value) > 0 {
            true  => true,
            false => false
        }
    }}

    pub fn is_collection(self) -> bool {unsafe{
        match cql_ffi::cass_value_is_collection(self.value) > 0 {
            true  => true,
            false => false
        }
    }}

    pub fn item_count(self) -> u64 {unsafe{
        cql_ffi::cass_value_item_count(self.value)
    }}

    pub fn primary_sub_type(self) -> CassValueType {unsafe{
        cql_ffi::cass_value_primary_sub_type(self.value)
    }}

    pub fn secondary_sub_type(self) -> CassValueType {unsafe{
        cql_ffi::cass_value_secondary_sub_type(self.value)
    }}

    pub fn map_iter(&self) -> CassIterator {unsafe{
        CassIterator{iterator:&mut*cql_ffi::cass_iterator_from_map(self.value)}
    }}
}
