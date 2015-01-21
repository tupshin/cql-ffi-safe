extern crate cql_ffi;

use std::ffi::CString;

pub use cql_ffi::CassCollectionType;
use cass_error::CassError;
use cass_bytes::CassBytes;
use cass_uuid::CassUuid;
use cass_inet::CassInet;
use cass_decimal::CassDecimal;
use cass_iterator::ToCassIterator;
use cass_iterator::CassIterator;

pub struct CassCollection<'a> {
    pub collection:&'a mut cql_ffi::CassCollection
}

impl<'a> CassCollection<'a> {
    pub fn new(_type: CassCollectionType, item_count: u64) -> Option<CassCollection<'a>> {unsafe{
        let collection = cql_ffi::cass_collection_new(_type, item_count);
        match collection.is_null() {
            true => None,
            false => Some(CassCollection{collection:&mut*collection})
        }
    }}

    pub fn append_int32(self, value: i32) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_int32(self.collection, value)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }

    pub fn append_int64(self, value: i64) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_int64(self.collection, value)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }

    pub fn append_float(self, value: f32) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_float(self.collection, value)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }    

    pub fn append_double(self, value: f64) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_double(self.collection, value)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }    
    
    pub fn cass_collection_append_bool(self, value: bool) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_bool(self.collection, if value==true {1} else {0})};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }    

    pub fn append_string(self, value: &str) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_string(self.collection,cql_ffi::cass_string_init(CString::from_slice(value.as_bytes()).as_ptr()))};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }

    pub fn append_bytes(self, value: CassBytes) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_bytes(self.collection, value.bytes)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }

    pub fn append_uuid(self, value: CassUuid) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_uuid(self.collection, value.uuid)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }

    pub fn append_inet(self, value: CassInet) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_inet(self.collection, value.inet)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }

    pub fn append_decimal(self, value: CassDecimal) -> Result<(),CassError> {
        let cl_result = unsafe{cql_ffi::cass_collection_append_decimal(self.collection, value.decimal)};
        match cl_result {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _=> Err(CassError::new(cl_result))
        }
    }


}

#[unsafe_destructor]
impl<'a> Drop for CassCollection<'a> {
    fn drop(&mut self) {
        unsafe{cql_ffi::cass_collection_free(self.collection)}
    }
}
