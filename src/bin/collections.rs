#![feature(core)]

extern crate cql_ffi_safe;

use cql_ffi_safe::CassStatement;
use cql_ffi_safe::CassSession;
use cql_ffi_safe::CassError;
use cql_ffi_safe::CassCluster;
use cql_ffi_safe::CassCollection;
use cql_ffi_safe::CassCollectionType;

static CREATE_KEYSPACE_CMD:&'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };";
static CREATE_TABLE_CMD:&'static str = "CREATE TABLE examples.collections (key text, items set<text>, PRIMARY KEY (key))";
static INSERT_QUERY_CMD:&'static str ="INSERT INTO examples.collections (key, items) VALUES (?, ?);";
static SELECT_QUERY_CMD:&'static str = "SELECT items FROM examples.collections WHERE key = ?";

fn insert_into_collections(session:&mut CassSession, key:&str, items:Vec<String>) -> Result<(),CassError> {
    let mut statement = CassStatement::new(INSERT_QUERY_CMD, 2);    
    let mut collection = CassCollection::new(CassCollectionType::SET, items.len() as u64).unwrap();
    for item in items.iter() {
        try!(collection.append_string(item.as_slice()));
    }
    try!(statement.bind_string(0, key));
    try!(statement.bind_collection(1, &collection));
    let mut future = session.execute(statement);
    future.wait();
    try!(future.error_code());
    Ok(())
}

fn select_from_collections(session: &mut CassSession, key:&str) -> Result<(),CassError> {
    let mut statement = CassStatement::new(SELECT_QUERY_CMD, 1);
    try!(statement.bind_string(0, key));
    let mut future = session.execute(statement);
    future.wait();
    try!(future.error_code());
    let result = future.get_result().unwrap();
    let row = try!(result.first_row());
    let column = row.get_column(0);
    let mut iterator = column.collection_iter();
    while iterator.has_next() {
        let value = iterator.get_value();
        println!("value:{:?}",value.get_string());
    }
    Ok(())
}    

fn main() {
    match CassCluster::new().set_contact_points("127.0.0.1,127.0.0.2,127.0.0.3") {
        Ok(cluster) => {
            let items = vec!("apple".to_string(), "orange".to_string(), "banana".to_string(), "mango".to_string());
            let mut session = CassSession::new();
            session.connect(cluster).wait();
            session.execute(CassStatement::new(CREATE_KEYSPACE_CMD,0));
            session.execute(CassStatement::new(CREATE_TABLE_CMD,0));
            insert_into_collections(&mut session, "test", items).unwrap();
            select_from_collections(&mut session, "test").unwrap();
            let mut close_future = session.close();
            close_future.wait();
        },
        Err(err) => panic!("{:?}",err)
    }
}
