use std::net::IpAddr;

use crate::schema::PingNode;
use chml_api::ChmlApi;

pub async fn get_ping_nodes(chml: &ChmlApi) -> anyhow::Result<Vec<PingNode>> {
    let mut ping_nodes = Vec::new();
    let mut ping_tasks = Vec::new();

    let nodes = chml.node().await?.into_result()?;
    for node in nodes {
        let node_info = chml.nodeinfo(&node.name).await?.into_result()?;
        let ip = node_info.real_IP.parse::<IpAddr>();
        if ip.is_err() {
            continue;
        }

        let task = tokio::spawn(async move {
            if let Ok((_, rtt)) = surge_ping::ping(ip.unwrap(), &[]).await {
                Some(PingNode {
                    node,
                    node_info,
                    rtt: rtt.as_millis(),
                })
            } else {
                None
            }
        });
        ping_tasks.push(task);
    }

    for task in ping_tasks {
        if let Some(ping_node) = task.await? {
            ping_nodes.push(ping_node);
        }
    }

    ping_nodes.sort_by(|a, b| a.rtt.cmp(&b.rtt));

    Ok(ping_nodes)
}
