#![feature(core,collections)]

extern crate cql_ffi_safe;

use cql_ffi_safe::CassStatement;
use cql_ffi_safe::CassSession;
use cql_ffi_safe::CassPrepared;
use cql_ffi_safe::CassError;
use cql_ffi_safe::CassBatchType;
use cql_ffi_safe::CassBatch;
use cql_ffi_safe::CassCluster;

struct Pair<'a> {
    key:&'a str,
    value:&'a str
}

static INSERT_QUERY_CMD: &'static str = "INSERT INTO examples.pairs (key, value) VALUES (?, ?)";
static CREATE_KEYSPACE_CMD: &'static str = "CREATE KEYSPACE examples WITH replication = { 'class': 'SimpleStrategy', 'replication_factor': '3' };";
static CREATE_TABLE_CMD: &'static str = "CREATE TABLE examples.pairs (key text, value text, PRIMARY KEY (key));";

static CONTACT_POINTS: &'static str = "127.0.0.1,127.0.0.2,127.0.0.3";

fn insert_into_batch_with_prepared<'a>(session:&mut CassSession, pairs:Vec<Pair>, prepared:CassPrepared<'a>)-> Result<CassPrepared<'a>,CassError> {
    let batch = CassBatch::new(CassBatchType::LOGGED);
    for pair in pairs.iter() {
        let mut statement = prepared.bind();
        try!(statement.bind_string(0, pair.key.as_slice()));
        try!(statement.bind_string(1, pair.value.as_slice()));
        try!(batch.add_statement(statement));
    }
    let mut statement = CassStatement::new(INSERT_QUERY_CMD,2);
    try!(statement.bind_string(0, "d"));
    try!(statement.bind_string(1, "4"));
    try!(batch.add_statement(statement));

    let future = session.execute_batch(batch);
    future.wait();
    Ok(prepared)
}

fn prepare_insert_into_batch<'a>(session:&mut CassSession<'a>) -> Result<CassPrepared<'a>,CassError> {
    let future = session.prepare(INSERT_QUERY_CMD.to_string());
    future.wait();
    Ok(future.get_prepared())
}


fn main() {
    match CassCluster::new().set_contact_points(CONTACT_POINTS) {
        Ok(cluster) => {
            let mut session = CassSession::new();
            let pairs = vec!(Pair{key:"a", value:"1"}, Pair{key:"b", value:"2"});
            session.connect(cluster).wait();
            session.execute(CassStatement::new(CREATE_KEYSPACE_CMD,0));
            session.execute(CassStatement::new(CREATE_TABLE_CMD,0));
            let prepared = prepare_insert_into_batch(&mut session).unwrap();    
            match insert_into_batch_with_prepared(&mut session, pairs, prepared) {
                Ok(_) => {}
                Err(err) => panic!("err: {:?}", err),
            }
            let close_future = session.close();
            close_future.wait();
            close_future.wait();
        },
        Err(err) => panic!("err: {:?}", err)
    }
}
