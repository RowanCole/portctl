use std::collections::HashMap;
use std::net::IpAddr;

use netstat2::{
    get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, SocketInfo,
};

use crate::cli::{Proto, Target};
use crate::i18n;
use crate::process::process_name;

pub fn all_sockets() -> Result<Vec<SocketInfo>, String> {
    get_sockets_info(
        AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6,
        ProtocolFlags::TCP | ProtocolFlags::UDP,
    )
    .map_err(|e| i18n::fetch_sockets_failed(&e.to_string()))
}

pub fn socket_matches(target: Target, proto: Proto, s: &SocketInfo) -> bool {
    let proto_ok = match &s.protocol_socket_info {
        ProtocolSocketInfo::Tcp(_) => proto == Proto::TCP || proto == Proto::Any,
        ProtocolSocketInfo::Udp(_) => proto == Proto::UDP || proto == Proto::Any,
    };
    if !proto_ok {
        return false;
    }
    target.ip.map_or(true, |ip| ip == s.local_addr())
        && target.port.map_or(true, |p| p == s.local_port())
}

fn addr_str(ip: IpAddr, port: u16) -> String {
    if ip.is_ipv6() {
        format!("[{ip}]:{port}")
    } else {
        format!("{ip}:{port}")
    }
}

pub fn socket_row(s: &SocketInfo, names: &mut HashMap<u32, String>) -> Vec<String> {
    let pids = &s.associated_pids;
    let pid_str = if pids.is_empty() {
        "-".to_string()
    } else {
        pids.iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(",")
    };
    let name = match pids.first() {
        None => "-".to_string(),
        Some(&pid) => names
            .entry(pid)
            .or_insert_with(|| process_name(pid))
            .clone(),
    };
    match &s.protocol_socket_info {
        ProtocolSocketInfo::Tcp(t) => vec![
            pid_str,
            name,
            "TCP".to_string(),
            addr_str(t.local_addr, t.local_port),
            addr_str(t.remote_addr, t.remote_port),
            t.state.to_string(),
        ],
        ProtocolSocketInfo::Udp(u) => vec![
            pid_str,
            name,
            "UDP".to_string(),
            addr_str(u.local_addr, u.local_port),
            "-".to_string(),
            "-".to_string(),
        ],
    }
}
