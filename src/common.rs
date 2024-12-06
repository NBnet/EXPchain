use chaindev::{beacon_based::common::NodePorts, NodeID};
use ruc::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{
    collections::{BTreeMap, BTreeSet},
    env, thread,
};

pub const EL_DIR: &str = "el";
pub const CL_BN_DIR: &str = "cl/bn";
pub const CL_VC_DIR: &str = "cl/vc";

pub const EL_LOG_NAME: &str = "el.log";
pub const CL_BN_LOG_NAME: &str = "cl.bn.log";
pub const CL_VC_LOG_NAME: &str = "cl.vc.log";

pub const MGMT_LOG_NAME: &str = "mgmt.log";

// pub const EL_ERR_NAME: &str = "el.err";
// pub const CL_BN_ERR_NAME: &str = "cl.bn.err";
// pub const CL_VC_ERR_NAME: &str = "cl.vc.err";

pub type MnemonicWords = String;

pub fn json_el_kind(v: &Option<JsonValue>) -> Result<Eth1Kind> {
    if let Some(v) = v {
        serde_json::from_value::<NodeCustomData>(v.clone())
            .c(d!())
            .map(|d| d.el_kind)
    } else {
        Ok(Eth1Kind::default())
    }
}

pub fn json_el_kind_set(jv: &mut Option<JsonValue>, k: Eth1Kind) -> Result<()> {
    let v = if let Some(v) = jv {
        let mut v = serde_json::from_value::<NodeCustomData>(v.clone()).c(d!())?;
        v.el_kind = k;
        v
    } else {
        NodeCustomData {
            el_kind: k,
            ..Default::default()
        }
    };

    jv.replace(v.to_json_value());

    Ok(())
}

pub fn json_deposits_append(
    jv: &mut Option<JsonValue>,
    mut deposits: BTreeMap<MnemonicWords, BTreeSet<u16>>,
) -> Result<()> {
    let v = if let Some(v) = jv {
        let mut v = serde_json::from_value::<NodeCustomData>(v.clone()).c(d!())?;
        v.deposits.append(&mut deposits);
        v
    } else {
        NodeCustomData {
            el_kind: Eth1Kind::default(),
            deposits,
        }
    };

    jv.replace(v.to_json_value());

    Ok(())
}

pub fn json_deposits_remove(
    jv: &mut Option<JsonValue>,
    mnemonic: &str,
    idx: u16,
) -> Result<bool> {
    let mut ret = false;
    let v = if let Some(v) = jv {
        let mut v = serde_json::from_value::<NodeCustomData>(v.clone()).c(d!())?;
        let hdr = v.deposits.get_mut(mnemonic).c(d!())?;
        if hdr.remove(&idx) {
            ret = true;
        }
        v
    } else {
        NodeCustomData::default()
    };

    jv.replace(v.to_json_value());

    Ok(ret)
}

pub fn json_deposits_clean_up(jv: &mut Option<JsonValue>) -> Result<()> {
    if let Some(v) = jv {
        let mut v = serde_json::from_value::<NodeCustomData>(v.clone()).c(d!())?;

        let old_len = v.deposits.len();

        v.deposits.retain(|_, v| !v.is_empty());

        if v.deposits.len() < old_len {
            jv.replace(v.to_json_value());
        }
    }

    Ok(())
}

pub fn json_el_kind_matched(v: &Option<JsonValue>, k: Eth1Kind) -> Result<bool> {
    json_el_kind(v).map(|i| i == k).c(d!())
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct NodeCustomData {
    pub el_kind: Eth1Kind,

    /// Mnemonic => deposited validator number
    pub deposits: BTreeMap<MnemonicWords, BTreeSet<u16>>,
}

impl NodeCustomData {
    pub fn new_with_geth() -> Self {
        Self {
            el_kind: Eth1Kind::Geth,
            deposits: map! {B},
        }
    }

    pub fn new_with_reth() -> Self {
        Self {
            el_kind: Eth1Kind::Reth,
            deposits: map! {B},
        }
    }

    pub fn to_json_value(&self) -> JsonValue {
        serde_json::to_value(self).c(d!()).unwrap()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Eth1Kind {
    Geth = 0,
    Reth = 1,
}

impl Default for Eth1Kind {
    fn default() -> Self {
        Self::Geth
    }
}

// **FIX ME**
//
// Secret Key:
//     - '0xbcdf20249abf0ed6d944c0288fad489e33f66b3960d9e6229c1cd214ed3bbe31'
pub const FEE_RECIPIENT: &str = "0x8943545177806ED17B9F23F0a21ee5948eCaa776";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomInfo {
    pub el_geth_bin: String,
    pub el_reth_bin: String,
    pub cl_bin: String,
}

impl Default for CustomInfo {
    fn default() -> Self {
        Self {
            el_geth_bin: String::from("geth"),
            el_reth_bin: String::from("reth"),
            cl_bin: String::from("lighthouse"),
        }
    }
}

// impl CustomInfo {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }

/// Active ports of a node
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Ports {
    pub el_discovery: u16,
    pub el_discovery_v5: u16, // reth only
    pub el_engine_api: u16,
    pub el_rpc: u16,
    pub el_rpc_ws: u16,
    pub el_metric: u16,

    pub cl_discovery: u16,
    pub cl_discovery_quic: u16,
    pub cl_bn_rpc: u16,
    pub cl_vc_rpc: u16,
    pub cl_bn_metric: u16,
    pub cl_vc_metric: u16,
}

impl NodePorts for Ports {
    // Reserve wide-used ports for the default node
    //
    // - lighthouse bn(discovery port): 9000
    // - lighthouse bn(quic port): 9001
    // - lighthouse bn(http rpc): 5052
    // - lighthouse vc(http rpc): 5062
    // - lighthouse bn(prometheus metrics): 5054
    // - lighthouse vc(prometheus metrics): 5064
    fn cl_reserved() -> Vec<u16> {
        vec![9000, 9001, 5052, 5062, 5054, 5064]
    }

    // Reserved ports defined by the Execution Client
    //
    // - geth/reth(discovery port): 30303
    // - reth(discovery v5 port): 9200
    // - geth/reth(engine api): 8551
    // - geth/reth(web3 rpc): 8545, 8546
    // - geth(prometheus metrics): 6060
    fn el_reserved() -> Vec<u16> {
        vec![30303, 9200, 8551, 8545, 8546, 6060]
    }

    fn try_create(ports: &[u16]) -> Result<Self> {
        if ports.len() != Self::reserved().len() {
            return Err(eg!("invalid length"));
        }

        let i = Self {
            el_discovery: ports[0],
            el_discovery_v5: ports[1], // reth only
            el_engine_api: ports[2],
            el_rpc: ports[3],
            el_rpc_ws: ports[4],
            el_metric: ports[5],

            cl_discovery: ports[6],
            cl_discovery_quic: ports[7],
            cl_bn_rpc: ports[8],
            cl_vc_rpc: ports[9],
            cl_bn_metric: ports[10],
            cl_vc_metric: ports[11],
        };

        Ok(i)
    }

    /// Get all actual ports from the instance,
    /// all: <sys ports> + <app ports>
    fn get_port_list(&self) -> Vec<u16> {
        vec![
            self.el_discovery,
            self.el_discovery_v5,
            self.el_engine_api,
            self.el_rpc,
            self.el_rpc_ws,
            self.el_metric,
            self.cl_discovery,
            self.cl_discovery_quic,
            self.cl_bn_rpc,
            self.cl_vc_rpc,
            self.cl_bn_metric,
            self.cl_vc_metric,
        ]
    }

    /// The p2p listening port in the execution side,
    /// may be used in generating the enode address for an execution node
    fn get_el_p2p(&self) -> u16 {
        self.el_discovery
    }

    /// The engine API listening port in the execution side
    /// usage(beacon): `--execution-endpoints="http://localhost:8551"`
    fn get_el_engine_api(&self) -> u16 {
        self.el_engine_api
    }

    /// The rpc listening port in the app side,
    /// eg. ETH el(geth/reth) web3 http API rpc
    fn get_el_rpc(&self) -> u16 {
        self.el_rpc
    }

    /// The rpc listening port in the app side,
    /// eg. ETH el(geth/reth) web3 websocket API rpc
    fn get_el_rpc_ws(&self) -> u16 {
        self.el_rpc_ws
    }

    /// The p2p(tcp/udp protocol) listening port in the beacon side
    /// may be used in generating the ENR address for a beacon node
    fn get_cl_p2p_bn(&self) -> u16 {
        self.cl_discovery
    }

    /// The p2p(quic protocol) listening port in the beacon side
    /// may be used in generating the ENR address for a beacon node
    fn get_cl_p2p_bn_quic(&self) -> u16 {
        self.cl_discovery_quic
    }

    /// The rpc listening port in the beacon side,
    /// usage(beacon): `--checkpoint-sync-url="http://${peer_ip}:5052"`
    fn get_cl_rpc_bn(&self) -> u16 {
        self.cl_bn_rpc
    }

    /// The rpc listening port in the vc side
    fn get_cl_rpc_vc(&self) -> u16 {
        self.cl_vc_rpc
    }
}

/// Return: "enode,enode,enode"
pub fn el_get_boot_nodes(rpc_endpoints: &[&str]) -> Result<String> {
    let ret = thread::scope(|s| {
        rpc_endpoints
            .iter()
            .map(|addr| {
                s.spawn(move || {
                    let body =
                        r#"{"jsonrpc":"2.0","method":"admin_nodeInfo","params":[],"id":1}"#;

                    ruc::http::post(
                        addr,
                        body.as_bytes(),
                        Some(&[("Content-Type", "application/json")]),
                    )
                    .c(d!())
                    .and_then(|(_code, resp)| serde_json::from_slice::<JsonValue>(&resp).c(d!()))
                    .map(|v| pnk!(v["result"]["enode"].as_str()).to_owned())
                    })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|hdr| hdr.join())
            .filter_map(|i| i.ok())
            .flat_map(|i| i.ok())
            .collect::<Vec<_>>()
    });

    if ret.is_empty() {
        return Err(eg!("No valid data return"));
    }

    Ok(ret.join(","))
}

/// Return: "(<enr,enr,enr>,<peer_id,peer_id,peer_id>)"
pub fn cl_get_boot_nodes(
    rpc_endpoints: &[&str],
) -> Result<(Vec<String>, String, String)> {
    let ret: (Vec<_>, (Vec<_>, Vec<_>)) = thread::scope(|s| {
        rpc_endpoints
            .iter()
            .map(|url| {
                s.spawn(move || {
                    ruc::http::get(
                        &format!("{url}/eth/v1/node/identity"),
                        Some(&[("Content-Type", "application/json")]),
                    )
                    .c(d!())
                    .and_then(|(_code, resp)| {
                        serde_json::from_slice::<JsonValue>(&resp).c(d!())
                    })
                    .map(|v| {
                        (
                            url.to_string(),
                            (
                                pnk!(v["data"]["enr"].as_str()).to_owned(),
                                pnk!(v["data"]["peer_id"].as_str()).to_owned(),
                            ),
                        )
                    })
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|hdr| hdr.join())
            .filter_map(|i| i.ok())
            .flat_map(|i| i.ok())
            .unzip()
    });

    if ret.0.is_empty() {
        return Err(eg!("No valid data return"));
    }

    Ok((ret.0, ret.1 .0.join(","), ret.1 .1.join(",")))
}

pub fn node_sync_from_genesis() -> bool {
    env::var("EXPCHAIN_NODE_SYNC_FROM_GENESIS").is_ok()
}

pub fn new_sb_runtime() -> sb::runtime::Runtime {
    sb::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap()
}

////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////

pub fn parse_nodes_to_vec(nodes_expr: &str) -> Result<Vec<NodeID>> {
    parse_nodes(nodes_expr).map(|ns| ns.into_iter().collect())
}

pub fn parse_nodes(nodes_expr: &str) -> Result<BTreeSet<NodeID>> {
    if nodes_expr.is_empty() {
        return Err(eg!("The nodes expression is empty!"));
    }

    let mut ret = set! {B};

    for expr in nodes_expr.split(',') {
        if let Some((l, h)) = expr.split_once('-') {
            let l_id = l.parse::<NodeID>().c(d!(expr))?;
            let h_id = h.parse::<NodeID>().c(d!(expr))?;
            if l_id > h_id {
                return Err(eg!("Incorrect range: {}~{}", l_id, h_id));
            }
            for id in l_id..=h_id {
                ret.insert(id);
            }
        } else {
            let id = expr.parse::<NodeID>().c(d!(expr))?;
            ret.insert(id);
        }
    }

    Ok(ret)
}

#[macro_export]
macro_rules! def_select_nodes {
    () => {
        fn select_nodes(
            env_name: &chaindev::EnvName,
            nodes_expr: Option<&str>,
            filter_geth: bool,
            filter_reth: bool,
            include_fuhrer_nodes: bool,
        ) -> Result<Option<std::collections::BTreeSet<chaindev::NodeID>>> {
            if nodes_expr.is_none() && !filter_geth && !filter_reth {
                Ok(None)
            } else if nodes_expr.is_some() && !filter_geth && !filter_reth {
                $crate::common::parse_nodes(nodes_expr.unwrap())
                    .map(Some)
                    .c(d!())
            } else {
                let env = load_sysenv(env_name).c(d!())?;
                let get_ids = |nodes: &std::collections::BTreeMap<
                    chaindev::NodeID,
                    Node<Ports>, /*USE RELATIVE PATH*/
                >| {
                    nodes
                        .values()
                        .filter(|n| {
                            if filter_geth && filter_reth {
                                true
                            } else if filter_geth {
                                pnk!(json_el_kind_matched(
                                    &n.custom_data,
                                    Eth1Kind::Geth
                                ))
                            } else if filter_reth {
                                pnk!(json_el_kind_matched(
                                    &n.custom_data,
                                    Eth1Kind::Reth
                                ))
                            } else {
                                true
                            }
                        })
                        .map(|n| n.id)
                        .collect::<BTreeSet<_>>()
                };

                let mut ids = get_ids(&env.meta.nodes);

                if include_fuhrer_nodes {
                    ids.append(&mut get_ids(&env.meta.fuhrers));
                }

                if let Some(expr) = nodes_expr {
                    let parsed = $crate::common::parse_nodes(expr).c(d!())?;
                    ids = ids.intersection(&parsed).copied().collect();
                }

                Ok(Some(ids))
            }
        }
    };
}

#[macro_export]
macro_rules! select_nodes_by_el_kind {
    ($nodes_expr: expr, $filter_geth: expr, $filter_reth: expr, $env_name: expr, $include_fuhrer_nodes: expr) => {{
        pnk!(select_nodes(
            &$env_name,
            $nodes_expr.as_deref(),
            $filter_geth,
            $filter_reth,
            $include_fuhrer_nodes
        ))
    }};
    ($nodes_expr: expr, $filter_geth: expr, $filter_reth: expr, $env_name: expr) => {{
        select_nodes_by_el_kind!(
            $nodes_expr,
            $filter_geth,
            $filter_reth,
            $env_name,
            true
        )
    }};
}
