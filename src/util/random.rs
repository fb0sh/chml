use chml_api::{ChmlApi, schema::Node};
use rand::prelude::IndexedRandom;
use rand::{Rng, distr::Alphanumeric};
pub fn random_string(len: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn random_port() -> u16 {
    rand::rng().random_range(10000..=65535)
}

pub async fn random_node(
    chml: &ChmlApi,
    web: Option<bool>,
    udp: Option<bool>,
    china: Option<bool>,
) -> anyhow::Result<Node> {
    let nodes = chml.node().await?.into_result()?;

    let is_vip = chml
        .user_info()
        .await?
        .into_result()?
        .usergroup
        .contains("超级会员");

    // 按条件筛选节点，只有 Some(true/false) 才参与过滤
    let filtered: Vec<Node> = nodes
        .into_iter()
        .filter(|n| {
            web.map_or(true, |w| n.web == w)
                && udp.map_or(true, |u| n.udp == u)
                && china.map_or(true, |c| n.china == c)
        })
        .collect();

    if filtered.is_empty() {
        return Err(anyhow::anyhow!("no nodes match given conditions"));
    }

    // VIP 优先 + fallback
    if is_vip {
        let vip_nodes: Vec<Node> = filtered
            .iter()
            .filter(|n| n.nodegroup == "vip")
            .cloned()
            .collect();

        if let Some(node) = vip_nodes.choose(&mut rand::rng()) {
            return Ok(node.clone());
        }
        // 如果没有 VIP 节点，fallback 到普通节点
    }

    let node = filtered
        .choose(&mut rand::rng())
        .ok_or_else(|| anyhow::anyhow!("no available node matches given conditions"))?
        .clone();

    Ok(node)
}
