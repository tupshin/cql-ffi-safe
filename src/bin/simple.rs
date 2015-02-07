#![feature(core)]
extern crate cql_ffi_safe;

use cql_ffi_safe::*;

static SELECT_QUERY_CMD:&'static str = "SELECT keyspace_name,durable_writes FROM system.schema_keyspaces;";
static CONTACT_POINTS:&'static str = "127.0.0.1,127.0.0.2,127.0.0.3";

fn main() {
    match CassCluster::new().set_contact_points(CONTACT_POINTS) {
        Err(err) => println!("{:?}",err),
        Ok(cluster) => {
            let mut session = CassSession::new();
            session.connect(cluster).wait();
            let result_iter = session.execute(&CassStatement::new(SELECT_QUERY_CMD,0))
                .wait()
                .get_result()
                .unwrap()
                .iter();
            for row in result_iter {
                let value:Result<String,CassError> = FromCol::from_col(row.get_column(0));
                match value {
                    Err(err) => println!("Error: {:?}", err),
                    Ok(ks) => println!("ks: {:?}",ks)
                }
            }
        }
    }
}
