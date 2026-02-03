use chml_api::node::schema::{Node, NodeInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PingNode {
    pub node: Node,
    pub node_info: NodeInfo,
    pub rtt: u128,
}
