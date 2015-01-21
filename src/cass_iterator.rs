extern crate cql_ffi;

pub use cql_ffi::CassIteratorType;

use cass_result::CassResult;

pub struct CassIterator<'a> {
    pub iterator:&'a mut cql_ffi::CassIterator
}

pub trait ToCassIterator {
    fn to_cass_iterator(&self) -> CassIterator;
}

impl<'a> CassIterator<'a> {
    pub fn free(&mut self) {
        unsafe{cql_ffi::cass_iterator_free(self.iterator)};
    }

    pub fn get_type(&mut self) -> CassIteratorType {
        unsafe{cql_ffi::cass_iterator_type(self.iterator)}
    }
}


    //~ pub fn cass_iterator_from_map(value: *const CassValue) -> *mut CassIterator;
    //~ pub fn cass_iterator_from_schema(schema: *const CassSchema) -> *mut CassIterator;
    //~ pub fn cass_iterator_from_schema_meta(meta: *const CassSchemaMeta) -> *mut CassIterator;
    //~ pub fn cass_iterator_fields_from_schema_meta(meta: *const CassSchemaMeta) -> *mut CassIterator;
    
    //~ pub fn cass_iterator_next(iterator: *mut CassIterator) -> cass_bool_t;
    //~ pub fn cass_iterator_get_row(iterator: *mut CassIterator) -> *const CassRow;
    //~ pub fn cass_iterator_get_column(iterator: *mut CassIterator) -> *const CassValue;
    //~ pub fn cass_iterator_get_value(iterator: *mut CassIterator)-> *const CassValue;
    //~ pub fn cass_iterator_get_map_key(iterator: *mut CassIterator) -> *const CassValue;
    //~ pub fn cass_iterator_get_map_value(iterator: *mut CassIterator) -> *const CassValue;
    //~ pub fn cass_iterator_get_schema_meta(iterator: *mut CassIterator) -> *const CassSchemaMeta;
    //~ pub fn cass_iterator_get_schema_meta_field(iterator: *mut CassIterator) -> *const CassSchemaMetaField;
