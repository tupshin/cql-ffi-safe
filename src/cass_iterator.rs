extern crate cql_ffi;

pub use cql_ffi::CassIteratorType;

use cass_value::CassValue;
use cass_row::CassRow;
use cass_column::CassColumn;
use cass_schema::CassSchemaMeta;

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

    pub fn get_row(&mut self) -> CassRow {unsafe{
        CassRow{row:&*cql_ffi::cass_iterator_get_row(self.iterator)}
    }}

    pub fn get_column(&mut self) -> CassColumn {unsafe{
        CassColumn{column:&*cql_ffi::cass_iterator_get_column(self.iterator)}
    }}
    
    pub fn get_value(&mut self) -> CassValue {unsafe{
        CassValue{value:&*cql_ffi::cass_iterator_get_value(self.iterator)}
    }}

    //~ fn get_map_key(&mut self) -> CassValue {unsafe{
        //~ CassValue{value:&*cql_ffi::cass_iterator_get_map_key(self.iterator)}
    //~ }}

    //~ fn get_map_value(&mut self) -> CassValue {unsafe{
         //~ CassValue{value:&*cql_ffi::cass_iterator_get_map_value(self.iterator)}
    //~ }}

    pub fn get_map_pair(&mut self) -> (CassValue,CassValue) {unsafe{
        (CassValue{value:&*cql_ffi::cass_iterator_get_map_key(self.iterator)},CassValue{value:&*cql_ffi::cass_iterator_get_map_value(self.iterator)})
    }}

    pub fn get_schema_meta(&mut self) -> CassSchemaMeta {unsafe{
        CassSchemaMeta{schema_meta:&*cql_ffi::cass_iterator_get_schema_meta(self.iterator)}
    }}

    pub fn has_next(&mut self) -> bool {unsafe{
        if cql_ffi::cass_iterator_next(self.iterator) > 0 {true} else {false}
    }}

}

//~ enum CassIteratee<'a> {
    //~ Row(CassRow<'a>),
    //~ Column(CassValue<'a>),
//~ }

//~ #[old_impl_check]
//~ impl<'a, Self> Iterator for CassIterator<'a> {
    //~ type Item = CassRow;

    //~ pub fn next(&mut self) -> Option<<Self as Iterator>::Item> {unsafe{
        //~ match cql_ffi::cass_iterator_next(self.iterator) > 0 {
            //~ true => Some(1u8),
            //~ false => Some(2u8),
        //~ }}
    //~ }
//~ }


    
