extern crate cql_ffi;

use cql_ffi_safe::error::CassError;
use cql_ffi_safe::uuid::CassUuid;
use cql_ffi_safe::value::CassValueType;
use cql_ffi_safe::string::CassString;
use cql_ffi_safe::bytes::CassBytes;
use cql_ffi_safe::decimal::CassDecimal;
use cql_ffi_safe::iterator::CassIterator;

use std::mem;

pub use cql_ffi::CassColumnType;

#[derive(Copy)]
pub struct CassColumn(pub cql_ffi::CassValue);

impl CassColumn {

    pub fn get_type(&self) -> CassValueType {unsafe{
        cql_ffi::cass_value_type(&self.0)
    }}

    pub fn map_iter(&self) -> CassIterator {unsafe{
        CassIterator(*cql_ffi::cass_iterator_from_map(&self.0))
    }}

    pub fn collection_iter(&self) -> CassIterator {unsafe{
        CassIterator(*cql_ffi::cass_iterator_from_collection(&self.0))
    }}

    pub fn get_int32(&self) -> Result<i32, CassError> {unsafe{
       assert!(self.get_type() == CassValueType::INT);
        let ref mut output = 0i32;
        match cql_ffi::cass_value_get_int32(&self.0,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError(err))
        }
    }}

    pub fn get_int64(&self) -> Result<i64, CassError> {unsafe{
       assert!(self.get_type() == CassValueType::BIGINT);
        let ref mut output = 0i64;
        match cql_ffi::cass_value_get_int64(&self.0,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError(err))
        }
    }}

    pub fn get_float(&self) -> Result<f32, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::FLOAT);
        let ref mut output = 0f32;
        match cql_ffi::cass_value_get_float(&self.0,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError(err))
        }
    }}

    pub fn get_double(&self) -> Result<f64, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::DOUBLE);
        let ref mut output = 0f64;
        match cql_ffi::cass_value_get_double(&self.0,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError(err))
        }
    }}

    pub fn get_bool(&self) -> Result<bool, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::BOOLEAN);
        let ref mut b_bln = 0u32;
        match cql_ffi::cass_value_get_bool(&self.0,b_bln) {
            cql_ffi::CassError::CASS_OK => Ok(true),
            err => Err(CassError(err))
        }
    }}

    //FIXME this should emit a uuid::Uuid instead of a CassUuid
    pub fn get_uuid(&self) -> Result<CassUuid, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_uuid(&self.0,output) {
            cql_ffi::CassError::CASS_OK => {
                Ok(CassUuid(*output))
            },
            err => Err(CassError(err))
        }
    }}

    //FIXME this should emit a uuid::Uuid instead of a CassUuid
    //~ pub fn get_inet(&self) -> Result<CassInet, CassError> {
        //~ match self.get_inet() {
            //~ Ok(inet) => Ok(inet),
            //~ Err(err) => Err(err)
        //~ }
    //~ }

    pub fn get_string(&self) -> Result<CassString, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::BOOLEAN);
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_string(&self.0,output) {
            cql_ffi::CassError::CASS_OK => Ok(CassString(*output)),
            err => Err(CassError(err))
        }
    }}

    pub fn get_bytes(&self) -> Result<CassBytes, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_bytes(&self.0,output) {
            cql_ffi::CassError::CASS_OK => Ok(CassBytes(*output)),
            err => Err(CassError(err))
        }
    }}

    pub fn get_decimal(&self) -> Result<CassDecimal, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_decimal(&self.0,output) {
            cql_ffi::CassError::CASS_OK => Ok(CassDecimal(*output)),
            err => Err(CassError(err))
        }
    }}    

}
