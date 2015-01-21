extern crate cql_ffi;

use std::mem;
use uuid::Uuid;
use std::io::net::ip::IpAddr;

pub use cql_ffi::CassValueType;
 
use cass_error::CassError;
use cass_uuid::CassUuid;
use cass_inet::CassInet;
use cass_iterator::CassIterator;
use cass_string::CassString;
use cass_bytes::CassBytes;
use cass_decimal::CassDecimal;

pub struct CassValue<'a> {
    pub value:&'a cql_ffi::CassValue
}

impl<'a> CassValue<'a> {
    pub fn get_int32(self) -> Result<i32, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_int32(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_int64(self) -> Result<i64, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_int64(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_float(self) -> Result<f32, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_float(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_double(self) -> Result<f64, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_double(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_bool(self) -> Result<bool, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_bool(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(if *output > 0 {true} else {false}),
            err => Err(CassError{error:err})
        }
    }}

    //FIXME this should emit a uuid::Uuid instead of a CassUuid
    pub fn get_uuid(self) -> Result<CassUuid, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_uuid(self.value,output) {
            cql_ffi::CassError::CASS_OK => {
                Ok(CassUuid{uuid:*output})
            },
            err => Err(CassError{error:err})
        }
    }}

    //FIXME this should emit a uuid::Uuid instead of a CassUuid
    pub fn get_inet(self) -> Result<CassInet, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_inet(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(CassInet{inet:*output}),
            err => Err(CassError{error:err})
        }
    }}

    //FIXME this is bad api
    fn collection_iter(self) -> CassIterator<'a> {unsafe{
        CassIterator{iterator:&mut*cql_ffi::cass_iterator_from_collection(&*self.value)}
    }}

    pub fn get_string(self) -> Result<CassString, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_string(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(CassString{string:*output}),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_bytes(self) -> Result<CassBytes, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_bytes(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(CassBytes{bytes:*output}),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_decimal(self) -> Result<CassDecimal, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_decimal(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(CassDecimal{decimal:*output}),
            err => Err(CassError{error:err})
        }
    }}    

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
}
