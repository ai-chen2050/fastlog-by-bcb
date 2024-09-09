use crate::error::{ProposerConfigError, ProposerConfigResult};
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;
use tools::helper::validate_addr;
use tools::helper::validate_key;

/// Proposer Node Config
#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct ProposerConfig {
    pub db: DbConfig,
    pub net: NetworkConfig,
    pub node: NodeConfig,
    pub chain: ChainConfig,
    pub api: ApiConfig,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct DbConfig {
    pub storage_root_path: Option<StorageRootPath>,
    pub pg_db_url: String,
    pub pg_db_name: String,
    pub max_connect_pool: u32,
    pub min_connect_pool: u32,
    pub connect_timeout: u64, // seconds
    pub acquire_timeout: u64,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct NetworkConfig {
    pub rest_url: String,
    pub txs_commit_udp: String,
    pub tee_vlc_cid: u32,
    pub tee_vlc_port: u32,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct NodeConfig {
    pub node_id: String,
    pub signer_key: String,
    pub cache_msg_maximum: u64,
    pub proposal_interval_ms: u64,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct ChainConfig {
    pub chain_rpc_url: String,
    pub vrf_range_contract: String,
    pub vrf_sort_precision: u16,
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct ApiConfig {
    pub read_maximum: u64,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct StorageRootPath(PathBuf);

impl StorageRootPath {
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

impl ProposerConfig {
    pub fn load_config(path: PathBuf) -> ProposerConfigResult<ProposerConfig> {
        let p: &Path = path.as_ref();
        let config_yaml = std::fs::read_to_string(p).map_err(|err| match err {
            e @ std::io::Error { .. } if e.kind() == std::io::ErrorKind::NotFound => {
                ProposerConfigError::ConfigMissing(path)
            }
            _ => err.into(),
        })?;

        let config: ProposerConfig =
            serde_yaml::from_str(&config_yaml).map_err(ProposerConfigError::SerializationError)?;
        ProposerConfig::validate_config(&config)
    }

    pub fn validate_config(config: &ProposerConfig) -> ProposerConfigResult<ProposerConfig> {
        if !validate_addr(&config.node.node_id.clone()) {
            return Err(ProposerConfigError::IllegalNodeId);
        }

        if !validate_key(&config.node.signer_key.clone()) {
            return Err(ProposerConfigError::IllegalSignerKey);
        }

        Ok(config.clone())
    }
}
