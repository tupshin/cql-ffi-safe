extern crate cql_ffi;
extern crate uuid;

pub use cql_ffi::CassValueType;
 
use cql_ffi_safe::error::CassError;
use cql_ffi_safe::decimal::CassDecimal;
use cql_ffi_safe::bytes::CassBytes;
use cql_ffi_safe::string::CassString;
use cql_ffi_safe::uuid::CassUuid;

use std::mem;

#[derive(Copy)]
pub struct CassValue<'a> {
    pub value:&'a cql_ffi::CassValue
}

#[derive(Debug)]
pub enum CassBindable {
    BOOL(bool),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    STR(String),
    BLOB(Vec<u8>),
    UUID(CassUuid) //FIXME this shoud return a Rust uuid once I figure out conversion
}

impl<'a> CassValue<'a> {
   

    //FIXME this is bad api


    pub fn get_type(&self) -> CassValueType {unsafe{
        cql_ffi::cass_value_type(self.value)
    }}

    pub fn is_null(&self) -> bool {unsafe{
        match cql_ffi::cass_value_is_null(self.value) > 0 {
            true  => true,
            false => false
        }
    }}

    pub fn is_collection(&self) -> bool {unsafe{
        match cql_ffi::cass_value_is_collection(self.value) > 0 {
            true  => true,
            false => false
        }
    }}

    pub fn item_count(&self) -> u64 {unsafe{
        cql_ffi::cass_value_item_count(self.value)
    }}

    pub fn primary_sub_type(&self) -> CassValueType {unsafe{
        cql_ffi::cass_value_primary_sub_type(self.value)
    }}

    pub fn secondary_sub_type(&self) -> CassValueType {unsafe{
        cql_ffi::cass_value_secondary_sub_type(self.value)
    }}

    pub fn get(&self) -> Result<CassBindable, CassError> {
        use cql_ffi::CassValueType::*;
        match self.get_type() {
            UNKNOWN => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            CUSTOM => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            VARCHAR|ASCII|TEXT => Ok(CassBindable::STR(try!(self.get_string()).to_string())),
            INT => Ok(CassBindable::I32(try!(self.get_int32()))),
            BIGINT => Ok(CassBindable::I64(try!(self.get_int64()))),
            BLOB => Ok(CassBindable::BLOB(try!(self.get_bytes()))),
            BOOLEAN => Ok(CassBindable::BOOL(try!(self.get_bool()))),
            COUNTER => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            DECIMAL => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            VARINT => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            TIMESTAMP => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            TIMEUUID => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            LIST => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            MAP => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            SET => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            INET => Err(CassError::new(cql_ffi::CassError::LIB_INVALID_VALUE_TYPE)),
            DOUBLE => Ok(CassBindable::I64(try!(self.get_int64()))),
            FLOAT => Ok(CassBindable::F32(try!(self.get_float()))),
            UUID => Ok(CassBindable::UUID(try!(self.get_uuid()))),
        }
    }

    pub fn get_int32(&self) -> Result<i32, CassError> {unsafe{
       assert!(self.get_type() == CassValueType::INT);
        let ref mut output = 0i32;
        match cql_ffi::cass_value_get_int32(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_int64(&self) -> Result<i64, CassError> {unsafe{
       assert!(self.get_type() == CassValueType::BIGINT);
        let ref mut output = 0i64;
        match cql_ffi::cass_value_get_int64(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_float(&self) -> Result<f32, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::FLOAT);
        let ref mut output = 0f32;
        match cql_ffi::cass_value_get_float(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_double(&self) -> Result<f64, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::DOUBLE);
        let ref mut output = 0f64;
        match cql_ffi::cass_value_get_double(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(*output),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_bool(&self) -> Result<bool, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::BOOLEAN);
        let ref mut b_bln = 0u32;
        match cql_ffi::cass_value_get_bool(self.value,b_bln) {
            cql_ffi::CassError::CASS_OK => Ok(true),
            err => Err(CassError{error:err})
        }
    }}

    //FIXME this should emit a uuid::Uuid instead of a CassUuid
    pub fn get_uuid(&self) -> Result<CassUuid, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::UUID);
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_uuid(self.value,output) {
            cql_ffi::CassError::CASS_OK => {
                Ok(CassUuid{uuid:*output})
            },
            err => Err(CassError{error:err})
        }
    }}

    //FIXME this should emit a uuid::Uuid instead of a CassUuid
    //~ pub fn get_inet(&self) -> Result<CassInet, CassError> {
        //~ assert!(self.get_type() == CassValueType::INET);
        //~ match self.get_inet() {
            //~ Ok(inet) => Ok(inet),
            //~ Err(err) => Err(err)
        //~ }
    //~ }

    pub fn get_string(&self) -> Result<CassString, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::VARCHAR);
        let mut output =  mem::zeroed::<cql_ffi::CassString>();
        match cql_ffi::cass_value_get_string(self.value,&mut output) {
            cql_ffi::CassError::CASS_OK => Ok(CassString{string:output}),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_bytes(&self) -> Result<Vec<u8>, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::BLOB);
        let mut output:cql_ffi::CassBytes =  mem::zeroed();
        let mut bytes = CassBytes(output);
        match cql_ffi::cass_value_get_bytes(self.value,&mut output) {
            cql_ffi::CassError::CASS_OK => Ok(bytes.as_bytes()),
            err => Err(CassError{error:err})
        }
    }}

    pub fn get_decimal(self) -> Result<CassDecimal, CassError> {unsafe{
        assert!(self.get_type() == CassValueType::DECIMAL);
        let output =  mem::zeroed();
        match cql_ffi::cass_value_get_decimal(self.value,output) {
            cql_ffi::CassError::CASS_OK => Ok(CassDecimal{decimal:*output}),
            err => Err(CassError{error:err})
        }
    }}    
    
}
