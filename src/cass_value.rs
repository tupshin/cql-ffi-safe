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
    pub fn get_int32(self) -> Result<i32, CassError> {unsafe{
       assert!(self.get_type() == CassValueType::INT);
        let ref mut output = 0i32;
        match cql_ffi::cass_value_get_int32(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_int64(self) -> Result<i64, CassError> {unsafe{
       assert!(self.get_type() == CassValueType::BIGINT);
        let ref mut output = 0i64;
        match cql_ffi::cass_value_get_int64(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_float(self) -> Result<f32, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::FLOAT);
        let ref mut output = 0f32;
        match cql_ffi::cass_value_get_float(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_double(self) -> Result<f64, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::DOUBLE);
        let ref mut output = 0f64;
        match cql_ffi::cass_value_get_double(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_bool(self) -> Result<bool, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::BOOLEAN);
        let ref mut b_bln = 0u32;
        match cql_ffi::cass_value_get_bool(self.value,b_bln) {
            cql_ffi::CassError::CASS_OK => Ok(true),
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
    pub fn collection_iter(self) -> CassIterator<'a> {unsafe{
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

    pub fn map_iter(&self) -> CassIterator {unsafe{
        CassIterator{iterator:&mut*cql_ffi::cass_iterator_from_map(self.value)}
    }}
}
