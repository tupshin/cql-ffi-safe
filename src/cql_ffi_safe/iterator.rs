extern crate cql_ffi;

pub use cql_ffi::CassIteratorType;

use cql_ffi_safe::value::CassValue;
use cql_ffi_safe::row::CassRow;
use cql_ffi_safe::column::CassColumn;
use cql_ffi_safe::schema::CassSchemaMeta;

use std::ptr;

pub struct CassIterator(pub *mut cql_ffi::CassIterator);

pub trait ToCassIterator {
    fn to_cass_iterator(&self) -> CassIterator;
}

impl CassIterator {
    pub fn free(&mut self) {
        unsafe{cql_ffi::cass_iterator_free(self.0)};
    }

    pub fn get_type(&mut self) -> CassIteratorType {
        unsafe{cql_ffi::cass_iterator_type(self.0)}
    }

    pub fn get_row(&mut self) -> CassRow {unsafe{
        CassRow(cql_ffi::cass_iterator_get_row(self.0))
    }}

    pub fn get_column(&mut self) -> CassColumn {unsafe{
        CassColumn(cql_ffi::cass_iterator_get_column(self.0))
    }}
    
    pub fn get_value(&mut self) -> CassValue {unsafe{
        CassValue(cql_ffi::cass_iterator_get_value(self.0))
    }}

    //~ fn get_map_key(&mut self) -> CassValue {unsafe{
        //~ CassValue{value:&*cql_ffi::cass_iterator_get_map_key(self.iterator)}
    //~ }}

    //~ fn get_map_value(&mut self) -> CassValue {unsafe{
         //~ CassValue{value:&*cql_ffi::cass_iterator_get_map_value(self.iterator)}
    //~ }}

    pub fn get_map_pair(&mut self) -> (CassValue,CassValue) {unsafe{
        (
            CassValue(cql_ffi::cass_iterator_get_map_key(self.0)),
            CassValue(cql_ffi::cass_iterator_get_map_value(self.0))
        )
    }}

    pub fn get_schema_meta(&mut self) -> CassSchemaMeta {unsafe{
        CassSchemaMeta(ptr::read(cql_ffi::cass_iterator_get_schema_meta(self.0)))
    }}

    pub fn has_next(&mut self) -> bool {unsafe{
        if cql_ffi::cass_iterator_next(self.0) > 0 {true} else {false}
    }}

}

//~ enum CassIteratee {
    //~ Row(CassRow),
    //~ Column(CassValue),
//~ }


//~ impl<Self> Iterator for CassIterator {
    //~ type Item = CassRow;

    //~ pub fn next(&mut self) -> Option<<Self as Iterator>::Item> {unsafe{
        //~ match cql_ffi::cass_iterator_next(self.iterator) > 0 {
            //~ true => Some(1u8),
            //~ false => Some(2u8),
        //~ }}
    //~ }
//~ }


    
