extern crate cql_ffi;

use cass_error::CassError;
use cass_ssl::CassSsl;

pub struct CassCluster<'a> {
    cluster:&'a mut cql_ffi::CassCluster
}

impl<'a> CassCluster<'a> {
    pub fn new() -> CassCluster<'a> {
        CassCluster{cluster:unsafe{&mut*cql_ffi::cass_cluster_new()}}
    } 

    pub fn set_contact_points(cluster:CassCluster, contact_points: &str) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_contact_points(cluster.cluster, cql_ffi::str2ref(contact_points))};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }

    pub fn set_port(self, port: i32) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_port(self.cluster, port)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }

    pub fn set_ssl(self, ssl: CassSsl) {
        unsafe{cql_ffi::cass_cluster_set_ssl(self.cluster, ssl.ssl)};
    }

    pub fn set_protocol_version(self, protocol_version: i32) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_protocol_version(self.cluster, protocol_version)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }

    pub fn set_num_threads_io(self, num_threads: i32) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_protocol_version(self.cluster, num_threads)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }
    
    pub fn set_queue_size_io(self, queue_size: u32) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_queue_size_io(self.cluster, queue_size)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }

    pub fn set_queue_size_event(self, queue_size: u32) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_queue_size_event(self.cluster, queue_size)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }

    pub fn set_queue_size_log(self, queue_size: u32) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_queue_size_log(self.cluster, queue_size)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }    

    pub fn set_core_connections_per_host(self, num_connections: u32) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_core_connections_per_host(self.cluster, num_connections)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }    

    pub fn set_max_connections_per_host(self, num_connections: u32) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_max_connections_per_host(self.cluster, num_connections)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }

    pub fn set_reconnect_wait_time(self, wait_time: u32)  {
        unsafe{cql_ffi::cass_cluster_set_reconnect_wait_time(self.cluster, wait_time)};
    }

    pub fn set_max_concurrent_creation(self, num_connections: u32) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_max_concurrent_creation(self.cluster, num_connections)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }

    pub fn set_max_concurrent_requests_threshold(self, num_requests: u32) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_max_concurrent_requests_threshold(self.cluster, num_requests)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }

    pub fn set_max_requests_per_flush(self, num_requests: u32) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_max_requests_per_flush(self.cluster, num_requests)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }        

    pub fn set_write_bytes_high_water_mark(self, num_bytes: u32) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_write_bytes_high_water_mark(self.cluster, num_bytes)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    } 

    pub fn set_write_bytes_low_water_mark(self, num_bytes: u32) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_write_bytes_low_water_mark(self.cluster, num_bytes)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    } 

    pub fn set_pending_requests_high_water_mark(self, num_requests: u32) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_pending_requests_high_water_mark(self.cluster, num_requests)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }

    pub fn set_pending_requests_low_water_mark(self, num_requests: u32) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_pending_requests_low_water_mark(self.cluster, num_requests)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }

    pub fn set_connect_timeout(self, timeout_ms: u32) {
        unsafe{cql_ffi::cass_cluster_set_connect_timeout(self.cluster, timeout_ms)};
    }

    pub fn set_request_timeout(self, timeout_ms: u32) {
        unsafe{cql_ffi::cass_cluster_set_request_timeout(self.cluster, timeout_ms)};
    }

    pub fn set_credentials(self, username: &str, password: &str) {
        unsafe{cql_ffi::cass_cluster_set_credentials(self.cluster, username.as_ptr() as *const i8, password.as_ptr() as *const i8)};
    }    

   pub fn set_load_balance_round_robin(self) -> Result<(),CassError> {
        let rc = unsafe{cql_ffi::cass_cluster_set_load_balance_round_robin(self.cluster)};
        match rc {
            cql_ffi::CassError::CASS_OK => Ok(()),
            _ => Err(CassError{error:rc})
        }
    }

   pub fn set_load_balance_dc_aware(self, local_dc: &str, used_hosts_per_remote_dc: u32, allow_remote_dcs_for_local_cl: bool) {
        unsafe{cql_ffi::cass_cluster_set_load_balance_dc_aware(self.cluster, local_dc.as_ptr() as *const i8, used_hosts_per_remote_dc, if allow_remote_dcs_for_local_cl {1} else {0})};
    }

   pub fn set_token_aware_routing(self, enabled: bool) {
        unsafe{cql_ffi::cass_cluster_set_token_aware_routing(self.cluster, if enabled {1} else {0})}
    }    

   pub fn set_tcp_nodelay(self, enable: bool) {
        unsafe{cql_ffi::cass_cluster_set_token_aware_routing(self.cluster, if enable {1} else {0})}
    }

   pub fn set_tcp_keepalive(self, enable: bool, delay_secs: u32) {
        unsafe{cql_ffi::cass_cluster_set_tcp_keepalive(self.cluster, if enable {1} else {0}, delay_secs)}
    }    
}

#[unsafe_destructor]
impl<'a> Drop for CassCluster<'a> {
    fn drop(&mut self) {
        unsafe{cql_ffi::cass_cluster_free(self.cluster)}
    }
}
