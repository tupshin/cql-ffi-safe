extern crate cql_ffi;

use cass_string::CassString;
use cass_value::CassValue;
use cass_iterator::CassIterator;

pub use cql_ffi::CassSchemaMetaType;

pub struct CassSchemaMetaField<'a> {
    pub meta_field:&'a cql_ffi::CassSchemaMetaField
}

impl<'a> CassSchemaMetaField<'a> {
    pub fn meta_field_name(&self) -> CassString {unsafe{
        CassString{string:cql_ffi::cass_schema_meta_field_name(self.meta_field)}
    }}

    pub fn meta_field_value(field: CassSchemaMetaField) -> CassValue {unsafe{
        CassValue{value:&*cql_ffi::cass_schema_meta_field_value(field.meta_field)}
    }}
}

pub struct CassSchema<'a> {
    pub schema:&'a mut cql_ffi::CassSchema
}

pub struct CassSchemaMeta<'a> {
    pub schema_meta:&'a cql_ffi::CassSchemaMeta
}

impl<'a> CassSchemaMeta<'a> {
    pub fn meta_type(&self) -> CassSchemaMetaType {unsafe{
        cql_ffi::cass_schema_meta_type(self.schema_meta)
    }}

    pub fn get_entry(&self, name: &str) -> Self {unsafe{
        CassSchemaMeta{schema_meta:&*cql_ffi::cass_schema_meta_get_entry(self.schema_meta, name.as_ptr() as *const i8)}
    }}

    pub fn get_field(&self, name: &str) -> CassSchemaMetaField {unsafe{
        CassSchemaMetaField{meta_field:&*cql_ffi::cass_schema_meta_get_field(self.schema_meta, name.as_ptr() as *const i8)}
    }}

    pub fn iter(&self) -> CassIterator {unsafe{
        CassIterator{iterator:&mut*cql_ffi::cass_iterator_from_schema_meta(self.schema_meta)}
    }}

    pub fn iter_fields(&self) -> CassIterator {unsafe{
        CassIterator{iterator:&mut*cql_ffi::cass_iterator_fields_from_schema_meta(self.schema_meta)}
    }}
}

#[unsafe_destructor]
impl<'a> Drop for CassSchema<'a> {
    fn drop(&mut self) {unsafe{
        cql_ffi::cass_schema_free(self.schema)
    }}
}

impl<'a> CassSchema<'a> {
    pub fn get_keyspace(&self, keyspace_name: &str) -> CassSchemaMeta<'a> {unsafe{
        CassSchemaMeta{schema_meta:&*cql_ffi::cass_schema_get_keyspace(self.schema, keyspace_name.as_ptr() as *const i8)}
    }}

    pub fn iter(&self) -> CassIterator {unsafe{
        CassIterator{iterator:&mut*cql_ffi::cass_iterator_from_schema(self.schema)}
    }}

}
