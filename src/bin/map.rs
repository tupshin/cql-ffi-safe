#![feature(core)]

extern crate cql_ffi_safe;

use cql_ffi_safe::CassStatement;
use cql_ffi_safe::CassSession;
use cql_ffi_safe::CassError;
use cql_ffi_safe::CassCollection;
use cql_ffi_safe::CassCluster;
use cql_ffi_safe::CassCollectionType;

static CREATE_KEYSPACE_CMD:&'static str = "CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };";
static CREATE_TABLE_CMD:&'static str = "CREATE TABLE IF NOT EXISTS examples.maps (key text, items map<text, int>, PRIMARY KEY (key))";
static INSERT_QUERY_CMD:&'static str = "INSERT INTO examples.maps (key, items) VALUES (?, ?);";
static SELECT_QUERY_CMD:&'static str = "SELECT items FROM examples.maps WHERE key = ?";

static CONTACT_POINTS:&'static str = "127.0.0.1,127.0.0.2,127.0.0.3";

fn insert_into_maps(session:&mut CassSession, key:String, items:Vec<(String,i32)>) -> Result<(),CassError> {
    use cql_ffi_safe::CassBindable::*;
    let mut statement = CassStatement::new(INSERT_QUERY_CMD, 2);    
    let collection = CassCollection::new(CassCollectionType::MAP, items.len() as u64).unwrap();
    for item in items.iter() {
        let item = item.clone();
        try!(collection.append(STR(item.0)));
        try!(collection.append(I32(item.1)));
    }
    try!(statement.bind_string(0, key.as_slice()));
    try!(statement.bind_collection(1, collection));
    let future = session.execute(statement);
    future.wait();
    try!(future.error_code());
    Ok(())
}

fn select_from_maps(session: &mut CassSession, key:&str) -> Result<(),CassError> {
    let mut statement = CassStatement::new(SELECT_QUERY_CMD, 1);
    try!(statement.bind_string(0, key));
    let mut future = session.execute(statement);
    future.wait();
    try!(future.error_code());
    let result = future.get_result().unwrap();
    let row = try!(result.first_row());
    let column = row.get_column(0);
    let mut iterator = column.map_iter();
    while iterator.has_next() {
        let (key,value) = iterator.get_map_pair();
        println!("key:{:?}\tvalue:{:?}",try!(key.get()),try!(value.get()));
    }
    Ok(())
}    

fn main() {
    match CassCluster::new().set_contact_points(CONTACT_POINTS) {
        Err(err) => panic!("{:?}",err),
        Ok(cluster) => {
            let mut session = CassSession::new();
            session.connect(cluster).wait();
            let items:Vec<(String,i32)> = vec!(
                ("apple".to_string(), 1),
                ("orange".to_string(), 2),
                ("banana".to_string(), 3),
                ("mango".to_string(), 4)
            );
            
            session.execute(CassStatement::new(CREATE_KEYSPACE_CMD,0));
            session.execute(CassStatement::new(CREATE_TABLE_CMD,0));
            insert_into_maps(&mut session, "test".to_string(), items).unwrap();
            select_from_maps(&mut session, "test").unwrap();
            let close_future = session.close();
            close_future.wait();
        }
    }
}
