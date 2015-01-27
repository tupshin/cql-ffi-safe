extern crate cql_ffi;

use error::CassError;
use cass_uuid::CassUuid;
use value::CassValueType;
use string::CassString;
use bytes::CassBytes;
use decimal::CassDecimal;
use iterator::CassIterator;

use std::mem;

pub use cql_ffi::CassColumnType;

pub struct CassColumn<'a> {
    pub column:&'a cql_ffi::CassValue
}

impl<'a> CassColumn<'a> {

    pub fn get_type(&self) -> CassValueType {unsafe{
        cql_ffi::cass_value_type(self.column)
    }}

    pub fn map_iter(&self) -> CassIterator {unsafe{
        CassIterator{iterator:&mut*cql_ffi::cass_iterator_from_map(self.column)}
    }}

    pub fn collection_iter(&self) -> CassIterator<'a> {unsafe{
        CassIterator{iterator:&mut*cql_ffi::cass_iterator_from_collection(&*self.column)}
    }}

    pub fn get_int32(&self) -> Result<i32, CassError> {unsafe{
       assert!(self.get_type() == CassValueType::INT);
        let ref mut output = 0i32;
        match cql_ffi::cass_value_get_int32(self.column,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_int64(&self) -> Result<i64, CassError> {unsafe{
       assert!(self.get_type() == CassValueType::BIGINT);
        let ref mut output = 0i64;
        match cql_ffi::cass_value_get_int64(self.column,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_float(&self) -> Result<f32, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::FLOAT);
        let ref mut output = 0f32;
        match cql_ffi::cass_value_get_float(self.column,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_double(&self) -> Result<f64, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::DOUBLE);
        let ref mut output = 0f64;
        match cql_ffi::cass_value_get_double(self.column,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_bool(&self) -> Result<bool, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::BOOLEAN);
        let ref mut b_bln = 0u32;
        match cql_ffi::cass_value_get_bool(self.column,b_bln) {
            cql_ffi::CassError::CASS_OK => Ok(true),
            err => Err(CassError{error:err})
        }
    }}

    //FIXME this should emit a uuid::Uuid instead of a CassUuid
    pub fn get_uuid(&self) -> Result<CassUuid, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_uuid(self.column,output) {
            cql_ffi::CassError::CASS_OK => {
                Ok(CassUuid{uuid:*output})
            },
            err => Err(CassError{error:err})
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
        match cql_ffi::cass_value_get_string(self.column,output) {
            cql_ffi::CassError::CASS_OK => Ok(CassString{string:*output}),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_bytes(&self) -> Result<CassBytes, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_bytes(self.column,output) {
            cql_ffi::CassError::CASS_OK => Ok(CassBytes{bytes:*output}),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_decimal(self) -> Result<CassDecimal, CassError> {unsafe{
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_decimal(self.column,output) {
            cql_ffi::CassError::CASS_OK => Ok(CassDecimal{decimal:*output}),
            err => Err(CassError{error:err})
        }
    }}    

}
