extern crate cql_ffi;
extern crate libc;


use cql_ffi_safe::error::CassError;
use cql_ffi_safe::ssl::CassSsl;

use cql_ffi::CassError::CASS_OK;

use std::ffi::CString;
pub struct CassCluster(pub *mut cql_ffi::CassCluster);

impl CassCluster {
    pub fn new() -> CassCluster {
        CassCluster(unsafe{cql_ffi::cass_cluster_new()})
    }

    pub fn set_contact_points(&mut self, contact_points: &str) -> Result<&mut Self,CassError> {
        println!("contact points: {:?}",contact_points);
       // let mut cluster = &mut CassCluster::new();

        let result = unsafe{cql_ffi::cass_cluster_set_contact_points(self.0, CString::from_slice(contact_points.as_bytes()).as_ptr() as *const i8)};
        match result {
            CASS_OK => {println!("foo");Ok(self)},
            err => Err(CassError(err))
        }
    }

    pub fn set_port(&mut self, port: i32) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_port(self.0, port)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }

    pub fn set_ssl(&mut self, ssl: CassSsl) {
        unsafe{cql_ffi::cass_cluster_set_ssl(self.0, ssl.0)};
    }

    pub fn set_protocol_version(&mut self, protocol_version: i32) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_protocol_version(self.0, protocol_version)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }

    pub fn set_num_threads_io(&mut self, num_threads: i32) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_protocol_version(self.0, num_threads)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }
    
    pub fn set_queue_size_io(&mut self, queue_size: u32) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_queue_size_io(self.0, queue_size)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }

    pub fn set_queue_size_event(&mut self, queue_size: u32) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_queue_size_event(self.0, queue_size)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }

    pub fn set_queue_size_log(&mut self, queue_size: u32) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_queue_size_log(self.0, queue_size)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }    

    pub fn set_core_connections_per_host(&mut self, num_connections: u32) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_core_connections_per_host(self.0, num_connections)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }    

    pub fn set_max_connections_per_host(&mut self, num_connections: u32) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_max_connections_per_host(self.0, num_connections)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }

    pub fn set_reconnect_wait_time(&mut self, wait_time: u32)  {
        unsafe{cql_ffi::cass_cluster_set_reconnect_wait_time(self.0, wait_time)};
    }

    pub fn set_max_concurrent_creation(&mut self, num_connections: u32) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_max_concurrent_creation(self.0, num_connections)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }

    pub fn set_max_concurrent_requests_threshold(&mut self, num_requests: u32) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_max_concurrent_requests_threshold(self.0, num_requests)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }

    pub fn set_max_requests_per_flush(&mut self, num_requests: u32) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_max_requests_per_flush(self.0, num_requests)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }        

    pub fn set_write_bytes_high_water_mark(&mut self, num_bytes: u32) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_write_bytes_high_water_mark(self.0, num_bytes)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    } 

    pub fn set_write_bytes_low_water_mark(&mut self, num_bytes: u32) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_write_bytes_low_water_mark(self.0, num_bytes)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    } 

    pub fn set_pending_requests_high_water_mark(&mut self, num_requests: u32) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_pending_requests_high_water_mark(self.0, num_requests)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }

    pub fn set_pending_requests_low_water_mark(&mut self, num_requests: u32) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_pending_requests_low_water_mark(self.0, num_requests)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }

    pub fn set_connect_timeout(&mut self, timeout_ms: u32) {
        unsafe{cql_ffi::cass_cluster_set_connect_timeout(self.0, timeout_ms)};
    }

    pub fn set_request_timeout(&mut self, timeout_ms: u32) {
        unsafe{cql_ffi::cass_cluster_set_request_timeout(self.0, timeout_ms)};
    }

    pub fn set_credentials(&mut self, username: &str, password: &str) {
        unsafe{cql_ffi::cass_cluster_set_credentials(self.0, username.as_ptr() as *const i8, password.as_ptr() as *const i8)};
    }    

   pub fn set_load_balance_round_robin(&mut self) -> Result<(),CassError> {
        match unsafe{cql_ffi::cass_cluster_set_load_balance_round_robin(self.0)} {
            CASS_OK => Ok(()),
            err => Err(CassError(err))
        }
    }

   pub fn set_load_balance_dc_aware(&mut self, local_dc: &str, used_hosts_per_remote_dc: u32, allow_remote_dcs_for_local_cl: bool) {
        unsafe{cql_ffi::cass_cluster_set_load_balance_dc_aware(self.0, local_dc.as_ptr() as *const i8, used_hosts_per_remote_dc, if allow_remote_dcs_for_local_cl {1} else {0})};
    }

   pub fn set_token_aware_routing(&mut self, enabled: bool) {
        unsafe{cql_ffi::cass_cluster_set_token_aware_routing(self.0, if enabled {1} else {0})}
    }    

   pub fn set_tcp_nodelay(&mut self, enable: bool) {
        unsafe{cql_ffi::cass_cluster_set_token_aware_routing(self.0, if enable {1} else {0})}
    }

   pub fn set_tcp_keepalive(&mut self, enable: bool, delay_secs: u32) {
        unsafe{cql_ffi::cass_cluster_set_tcp_keepalive(self.0, if enable {1} else {0}, delay_secs)}
    }    
}

impl Drop for CassCluster {
    fn drop(&mut self) {
        unsafe{cql_ffi::cass_cluster_free(self.0)}
    }
}
