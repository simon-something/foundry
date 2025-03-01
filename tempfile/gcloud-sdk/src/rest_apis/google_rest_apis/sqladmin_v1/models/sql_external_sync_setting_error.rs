/*
 * Cloud SQL Admin API
 *
 * API for Cloud SQL database instance management
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::google_rest_apis::sqladmin_v1::models;
use serde::{Deserialize, Serialize};

/// SqlExternalSyncSettingError : External primary instance migration setting error/warning.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlExternalSyncSettingError {
    /// Additional information about the error encountered.
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// Can be `sql#externalSyncSettingError` or `sql#externalSyncSettingWarning`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Identifies the specific error that occurred.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl SqlExternalSyncSettingError {
    /// External primary instance migration setting error/warning.
    pub fn new() -> SqlExternalSyncSettingError {
        SqlExternalSyncSettingError {
            detail: None,
            kind: None,
            r#type: None,
        }
    }
}
/// Identifies the specific error that occurred.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "SQL_EXTERNAL_SYNC_SETTING_ERROR_TYPE_UNSPECIFIED")]
    SqlExternalSyncSettingErrorTypeUnspecified,
    #[serde(rename = "CONNECTION_FAILURE")]
    ConnectionFailure,
    #[serde(rename = "BINLOG_NOT_ENABLED")]
    BinlogNotEnabled,
    #[serde(rename = "INCOMPATIBLE_DATABASE_VERSION")]
    IncompatibleDatabaseVersion,
    #[serde(rename = "REPLICA_ALREADY_SETUP")]
    ReplicaAlreadySetup,
    #[serde(rename = "INSUFFICIENT_PRIVILEGE")]
    InsufficientPrivilege,
    #[serde(rename = "UNSUPPORTED_MIGRATION_TYPE")]
    UnsupportedMigrationType,
    #[serde(rename = "NO_PGLOGICAL_INSTALLED")]
    NoPglogicalInstalled,
    #[serde(rename = "PGLOGICAL_NODE_ALREADY_EXISTS")]
    PglogicalNodeAlreadyExists,
    #[serde(rename = "INVALID_WAL_LEVEL")]
    InvalidWalLevel,
    #[serde(rename = "INVALID_SHARED_PRELOAD_LIBRARY")]
    InvalidSharedPreloadLibrary,
    #[serde(rename = "INSUFFICIENT_MAX_REPLICATION_SLOTS")]
    InsufficientMaxReplicationSlots,
    #[serde(rename = "INSUFFICIENT_MAX_WAL_SENDERS")]
    InsufficientMaxWalSenders,
    #[serde(rename = "INSUFFICIENT_MAX_WORKER_PROCESSES")]
    InsufficientMaxWorkerProcesses,
    #[serde(rename = "UNSUPPORTED_EXTENSIONS")]
    UnsupportedExtensions,
    #[serde(rename = "INVALID_RDS_LOGICAL_REPLICATION")]
    InvalidRdsLogicalReplication,
    #[serde(rename = "INVALID_LOGGING_SETUP")]
    InvalidLoggingSetup,
    #[serde(rename = "INVALID_DB_PARAM")]
    InvalidDbParam,
    #[serde(rename = "UNSUPPORTED_GTID_MODE")]
    UnsupportedGtidMode,
    #[serde(rename = "SQLSERVER_AGENT_NOT_RUNNING")]
    SqlserverAgentNotRunning,
    #[serde(rename = "UNSUPPORTED_TABLE_DEFINITION")]
    UnsupportedTableDefinition,
    #[serde(rename = "UNSUPPORTED_DEFINER")]
    UnsupportedDefiner,
    #[serde(rename = "SQLSERVER_SERVERNAME_MISMATCH")]
    SqlserverServernameMismatch,
    #[serde(rename = "PRIMARY_ALREADY_SETUP")]
    PrimaryAlreadySetup,
    #[serde(rename = "UNSUPPORTED_BINLOG_FORMAT")]
    UnsupportedBinlogFormat,
    #[serde(rename = "BINLOG_RETENTION_SETTING")]
    BinlogRetentionSetting,
    #[serde(rename = "UNSUPPORTED_STORAGE_ENGINE")]
    UnsupportedStorageEngine,
    #[serde(rename = "LIMITED_SUPPORT_TABLES")]
    LimitedSupportTables,
}

impl Default for Type {
    fn default() -> Type {
        Self::SqlExternalSyncSettingErrorTypeUnspecified
    }
}
