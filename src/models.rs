use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub computed: Computed,
    pub raw_in: RawIn,
    pub raw_out: RawOut,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Computed {
    pub compute_data: Vec<ComputeDaum>,
    pub performance_data: Vec<PerformanceDaum>,
    pub object_storage_data: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputeDaum {
    pub site_id: String,
    pub component: String,
    pub site: String,
    pub name: String,
    pub cores: String,
    pub ram: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceDaum {
    pub site: String,
    pub repo_name: String,
    pub capacity: String,
    pub first_transactions: String,
    pub second_transactions: String,
    pub last_transactions: String,
    pub first_immut_transactions: String,
    pub second_immut_transactions: String,
    pub last_immut_transactions: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawIn {
    pub arch_tier_repos: Vec<ArchTierRepo>,
    pub backup_windows: Vec<BackupWindow>,
    pub cap_tier_repos: Vec<CapTierRepo>,
    pub data_properties: Vec<DataProperty>,
    pub perf_tier_repos: Vec<PerfTierRepo>,
    pub project_length: f32,
    pub retentions: Vec<Retention>,
    pub show_points: bool,
    pub show_workloads: bool,
    pub sites: Vec<Site>,
    pub workloads: Vec<Workload>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchTierRepo {
    pub archive_tier_repo_id: String,
    pub archive_tier_repo_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupWindow {
    pub backup_window_id: String,
    pub backup_window_name: String,
    pub full_window: f32,
    pub incremental_window: f32,
    pub default: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapTierRepo {
    pub cap_tier_repo_id: String,
    pub cap_tier_repo_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataProperty {
    pub data_property_id: String,
    pub data_property_name: String,
    pub change_rate: f32,
    pub compression: f32,
    pub growth_factor: f32,
    pub default: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerfTierRepo {
    pub repo_id: String,
    pub repo_name: String,
    pub site_id: String,
    pub copy_capacity_tier_enabled: bool,
    pub move_capacity_tier_enabled: bool,
    pub archive_tier_enabled: bool,
    pub capacity_tier_days: f32,
    pub archive_tier_days: f32,
    pub capacity_tier_repo_id: String,
    pub archive_tier_repo_id: String,
    pub storage_type: String,
    pub immutable_perf: bool,
    pub immutable_cap: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Retention {
    pub retention_id: String,
    pub retention_name: String,
    pub simple: f32,
    pub weekly: f32,
    pub monthly: f32,
    pub yearly: f32,
    pub default: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub site_id: String,
    pub site_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workload {
    pub backup: Backup,
    pub copies: Vec<Value>,
    pub copies_enabled: bool,
    pub data_property_id: String,
    pub enabled: bool,
    pub site_id: String,
    #[serde(rename = "sourceTB")]
    pub source_tb: f32,
    pub units: f32,
    pub workload_id: String,
    pub workload_name: String,
    pub workload_type: String,
    pub large_block: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backup {
    pub retention_id: String,
    pub repo_id: String,
    pub backup_window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawOut {
    pub global: Global,
    pub sites: Vec<Site2>,
    pub capacity_tier: Vec<Value>,
    pub archive_tier: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Global {
    pub vbr_server: VbrServer,
    pub db_server: DbServer,
    pub enterprise_manager: EnterpriseManager,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VbrServer {
    pub vbr_cores: f32,
    pub vbr_ram: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbServer {
    pub db_cores: f32,
    pub db_ram: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnterpriseManager {
    pub cores: f32,
    pub ram: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Site2 {
    pub site_name: String,
    pub site_id: String,
    pub compute: Compute,
    pub storage: Vec<Storage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Compute {
    pub proxy_cores: f32,
    pub proxy_ram: f32,
    pub repo_compute: Vec<RepoCompute>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoCompute {
    pub repo_id: String,
    pub repo_name: String,
    pub repo_inc_cores: f32,
    pub repo_inc_ram: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Storage {
    pub repo_name: String,
    pub site_id: String,
    pub repo_id: String,
    pub points: Points,
    pub performance_tier_cap: f64,
    pub workspace: f64,
    pub capacity_tier_cap: f64,
    pub archive_tier_cap: f64,
    pub performance_tier_copy_cap: f64,
    pub capacity_tier_copy_cap: f64,
    pub archive_tier_copy_cap: f64,
    pub total_performance_tier: f64,
    pub total_capacity_tier: f64,
    pub total_archive_tier: f64,
    pub pef_tier_transactions: PefTierTransactions,
    pub cap_tier_transactions: CapTierTransactions2,
    pub archive_transactions: ArchiveTransactions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Points {
    pub primary_backup: Vec<PrimaryBackup>,
    pub copy_backup: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryBackup {
    pub workload_name: String,
    pub workload_id: String,
    pub performance_tier_result: Vec<TierResult>,
    pub capacity_tier_result: Vec<TierResult>,
    pub archive_tier_result: Vec<TierResult>,
    pub perf_tier_point_count: f32,
    pub cap_tier_point_count: f32,
    pub arch_tier_point_count: f32,
    pub total_performance: f64,
    pub total_capacity: f64,
    pub total_archive: f64,
    pub workspace: f64,
    pub individual_incremental_size: f64,
    pub block_generation: f32,
    pub performance_tier_immutability_tax: f64,
    pub capacity_tier_immutability_tax: f32,
    pub repo_id: String,
    pub repo_name: String,
    pub cap_tier_repo_id: String,
    pub archive_tier_repo_id: String,
    pub backup_type: f32,
    pub copies_enabled: bool,
    pub perf_tier_transactions: PerfTierTransactions,
    pub cap_tier_transactions: CapTierTransactions,
    pub arch_tier_transactions: ArchTierTransactions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierResult {
    pub point_type: String,
    pub point: Point,
    pub backup_size: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub day: f32,
    pub is_full: bool,
    #[serde(rename = "isGFS")]
    pub is_gfs: bool,
    pub flags: Flags,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flags {
    pub daily: f32,
    pub weekly: f32,
    pub monthly: f32,
    pub yearly: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerfTierTransactions {
    pub offload_month: f32,
    pub offload_type: String,
    pub first_transactions: f32,
    pub second_transactions: f32,
    pub last_transactions: f32,
    pub first_immut_trans: f32,
    pub second_immut_trans: f32,
    pub last_immut_trans: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapTierTransactions {
    pub offload_month: f32,
    pub offload_type: String,
    pub first_transactions: f32,
    pub second_transactions: f32,
    pub last_transactions: f32,
    pub first_immut_trans: f32,
    pub second_immut_trans: f32,
    pub last_immut_trans: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchTierTransactions {
    pub offload_month: f32,
    pub offload_type: String,
    pub first_transactions: f32,
    pub second_transactions: f32,
    pub last_transactions: f32,
    pub first_immut_trans: f32,
    pub second_immut_trans: f32,
    pub last_immut_trans: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PefTierTransactions {
    pub offload_days: f32,
    pub first_transactions: f32,
    pub second_transactions: f32,
    pub last_transactions: f32,
    pub first_immut_transactions: f32,
    pub second_immut_transactions: f32,
    pub last_immut_transactions: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapTierTransactions2 {
    pub offload_days: f32,
    pub first_transactions: f32,
    pub second_transactions: f32,
    pub last_transactions: f32,
    pub first_immut_transactions: f32,
    pub second_immut_transactions: f32,
    pub last_immut_transactions: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveTransactions {
    pub offload_days: f32,
    pub first_transactions: f32,
    pub second_transactions: f32,
    pub last_transactions: f32,
    pub first_immut_transactions: f32,
    pub second_immut_transactions: f32,
    pub last_immut_transactions: f32,
}
