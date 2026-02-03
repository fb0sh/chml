use crate::schema::PingNode;

use super::color;
use chml_api::schema;
pub fn print_user_info(user: &schema::UserInfo) {
    println!(
        "(UserInfo)\t{}:{}, expiration_date={}, credits={}, bandwidth={}m, tunnel={}, created_tunnel={}",
        user.username,
        user.usergroup,
        user.term,
        user.integral,
        user.bandwidth,
        user.tunnel,
        user.tunnelCount,
    );
}

pub fn print_user_domains(domains: &[schema::UserDomain]) {
    for domain in domains {
        println!(
            "(UserDomain)\t{} {} type={} ttl={}",
            format!("{}.{}", domain.record, domain.domain),
            domain.target,
            domain.r#type,
            domain.ttl
        )
    }
}

pub fn print_user_tunnels(tunnels: &[schema::Tunnel]) {
    for tunnel in tunnels {
        let connected_str = if tunnel.state {
            color(&tunnel.state.to_string(), 32) // 绿色
        } else {
            tunnel.state.to_string()
        };

        let item_name = if tunnel.state {
            color("(UserTunnel)", 32)
        } else {
            "(UserTunnel)".to_string()
        };

        let name_str = color(&tunnel.name, 31); // 红色
        let lport_str = color(&tunnel.nport.to_string(), 34); // 蓝色
        let rport_str = color(&tunnel.dorp, 35); // 紫色
        let rip_str = color(tunnel.ip.as_deref().unwrap_or("N/A"), 31); // 红色
        println!(
            "{}\ttunnel_id={}, name={}, connected={}, node={}, type={}, lhost={}, lport={}, rip={}, rpot={}, cur_conns={}",
            item_name,
            tunnel.id.unwrap_or(0),
            name_str,
            connected_str,
            tunnel.node,
            tunnel.r#type,
            tunnel.localip,
            lport_str,
            rip_str,
            rport_str,
            tunnel.cur_conns.unwrap_or(0),
        );
    }
}

pub fn print_ping_nodes(ping_nodes: &[PingNode]) {
    fn inline_print_node(ping_node: &PingNode) {
        println!(
            "(Node:{})\trtt={}ms,name={},area={},real_ip={}, china={}, web={}, udp={}, notes={}",
            ping_node.node.nodegroup,
            ping_node.rtt,
            ping_node.node.name,
            ping_node.node.area,
            ping_node.node_info.real_IP,
            ping_node.node.china,
            ping_node.node.web,
            ping_node.node.udp,
            ping_node.node.notes
        );
    }

    let (vip_nodes, other_nodes): (Vec<_>, Vec<_>) =
        ping_nodes.iter().partition(|n| n.node.nodegroup == "vip");

    other_nodes.iter().map(|n| inline_print_node(n)).count();
    println!();
    vip_nodes.iter().map(|n| inline_print_node(n)).count();
}
