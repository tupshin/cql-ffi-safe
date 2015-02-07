extern crate cql_ffi;

use cql_ffi_safe::string::CassString;
use cql_ffi_safe::value::CassValue;
use cql_ffi_safe::iterator::CassIterator;

pub use cql_ffi::CassSchemaMetaType;

#[allow(missing_copy_implementations)]
pub struct CassSchemaMetaField(pub *const cql_ffi::CassSchemaMetaField);

impl CassSchemaMetaField {
    pub fn meta_field_name(&mut self) -> CassString {unsafe{
        CassString(cql_ffi::cass_schema_meta_field_name(self.0))
    }}

    pub fn meta_field_value(field: CassSchemaMetaField) -> CassValue {unsafe{
        CassValue(cql_ffi::cass_schema_meta_field_value(field.0))
    }}
}

pub struct CassSchema(pub *const cql_ffi::CassSchema);

#[allow(missing_copy_implementations)]
pub struct CassSchemaMeta(pub *const cql_ffi::CassSchemaMeta);

impl CassSchemaMeta {
    pub fn meta_type(&self) -> CassSchemaMetaType {unsafe{
        cql_ffi::cass_schema_meta_type(self.0)
    }}

    pub fn get_entry(&self, name: &str) -> Self {unsafe{
        CassSchemaMeta(cql_ffi::cass_schema_meta_get_entry(self.0, name.as_ptr() as *const i8))
    }}

    pub fn get_field(&self, name: &str) -> CassSchemaMetaField {unsafe{
        CassSchemaMetaField(cql_ffi::cass_schema_meta_get_field(self.0, name.as_ptr() as *const i8))
    }}

    pub fn iter(&self) -> CassIterator {unsafe{
        CassIterator(cql_ffi::cass_iterator_from_schema_meta(self.0))
    }}

    pub fn iter_fields(&self) -> CassIterator {unsafe{
        CassIterator(cql_ffi::cass_iterator_fields_from_schema_meta(self.0))
    }}
}

#[unsafe_destructor]
impl Drop for CassSchema {
    fn drop(&mut self) {unsafe{
        cql_ffi::cass_schema_free(self.0)
    }}
}

impl CassSchema {
    pub fn get_keyspace(&self, keyspace_name: &str) -> CassSchemaMeta {unsafe{
        CassSchemaMeta(cql_ffi::cass_schema_get_keyspace(self.0, keyspace_name.as_ptr() as *const i8))
    }}

    pub fn iter(&self) -> CassIterator {unsafe{
        CassIterator(cql_ffi::cass_iterator_from_schema(self.0))
    }}

}
