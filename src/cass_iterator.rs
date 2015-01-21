extern crate cql_ffi;

pub use cql_ffi::CassIteratorType;

use cass_result::CassResult;
use cass_value::CassValue;
use cass_row::CassRow;
use cass_column::CassColumn;

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

    pub fn get_row(&mut self) -> CassRow {
        CassRow{row:&*cql_ffi::cass_iterator_get_row(self.iterator)}
    }

    pub fn get_column(&mut self) -> CassColumn {
        CassColumn{column:cql_ffi::cass_iterator_get_column(self.iterator)}
    }
    
    pub fn get_value(&mut self) -> CassValue {
        cql_ffi::cass_iterator_get_value(self.iterator)
    }
    
}

//~ enum CassIteratees {
    //~ row(CassRow),
    //~ column(CassValue),
    
//~ }

//~ impl<'a, CassIteratees> Iterator for CassIterator<'a> {
    //~ pub fn next(&mut self) -> Option<<Self as Iterator>::Item> {unsafe{
        //~ match cql_ffi::cass_iterator_next(self.iterator) > 0 {
            //~ true => Some(1u8),
            //~ false => Some(2u8),
        //~ }}
    //~ }

    //~ type Item = T ;
//~ }


    
    //~ pub fn cass_iterator_get_column(iterator: *mut CassIterator) -> *const CassValue;
    //~ pub fn cass_iterator_get_value(iterator: *mut CassIterator)-> *const CassValue;
    //~ pub fn cass_iterator_get_map_key(iterator: *mut CassIterator) -> *const CassValue;
    //~ pub fn cass_iterator_get_map_value(iterator: *mut CassIterator) -> *const CassValue;
    //~ pub fn cass_iterator_get_schema_meta(iterator: *mut CassIterator) -> *const CassSchemaMeta;
    //~ pub fn cass_iterator_get_schema_meta_field(iterator: *mut CassIterator) -> *const CassSchemaMetaField;
