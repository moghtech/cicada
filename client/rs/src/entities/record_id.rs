use std::str::FromStr;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumDiscriminants, EnumString};
use surrealdb_types::{RecordIdKey, SurrealValue};
use typeshare::typeshare;

use crate::entities::{
  device::DeviceId, encryption_key::EncryptionKeyId,
  external_login::ExternalLoginId, filesystem::FilesystemId,
  master_key::MasterKeyId, node::NodeId,
  onboarding_key::OnboardingKeyId, user::UserId,
};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, EnumDiscriminants)]
#[strum_discriminants(name(CicadaTable))]
#[strum_discriminants(derive(
  Serialize,
  Deserialize,
  Display,
  EnumString,
  AsRefStr
))]
// This matches surrealdb RecordId json
#[serde(tag = "table", content = "key")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CicadaRecordId {
  User(UserId),
  ExternalLogin(ExternalLoginId),
  Device(DeviceId),
  OnboardingKey(OnboardingKeyId),
  Filesystem(FilesystemId),
  Node(NodeId),
  EncryptionKey(EncryptionKeyId),
  MasterKey(MasterKeyId),
}

impl CicadaRecordId {
  pub fn new(table: CicadaTable, id: impl Into<String>) -> Self {
    match table {
      CicadaTable::User => Self::User(UserId(id.into())),
      CicadaTable::ExternalLogin => {
        Self::ExternalLogin(ExternalLoginId(id.into()))
      }
      CicadaTable::Device => Self::Device(DeviceId(id.into())),
      CicadaTable::OnboardingKey => {
        Self::OnboardingKey(OnboardingKeyId(id.into()))
      }
      CicadaTable::Filesystem => {
        Self::Filesystem(FilesystemId(id.into()))
      }
      CicadaTable::Node => Self::Node(NodeId(id.into())),
      CicadaTable::EncryptionKey => {
        Self::EncryptionKey(EncryptionKeyId(id.into()))
      }
      CicadaTable::MasterKey => {
        Self::MasterKey(MasterKeyId(id.into()))
      }
    }
  }

  pub fn id(&self) -> &str {
    match self {
      Self::User(id) => &id.0,
      Self::ExternalLogin(id) => &id.0,
      Self::Device(id) => &id.0,
      Self::OnboardingKey(id) => &id.0,
      Self::Filesystem(id) => &id.0,
      Self::Node(id) => &id.0,
      Self::EncryptionKey(id) => &id.0,
      Self::MasterKey(id) => &id.0,
    }
  }

  pub fn as_record_id(&self) -> surrealdb_types::RecordId {
    match self {
      Self::User(id) => {
        surrealdb_types::RecordId::new("User", id.0.as_str())
      }
      Self::ExternalLogin(id) => {
        surrealdb_types::RecordId::new("ExternalLogin", id.0.as_str())
      }
      Self::Device(id) => {
        surrealdb_types::RecordId::new("Device", id.0.as_str())
      }
      Self::OnboardingKey(id) => {
        surrealdb_types::RecordId::new("OnboardingKey", id.0.as_str())
      }
      Self::Filesystem(id) => {
        surrealdb_types::RecordId::new("Filesystem", id.0.as_str())
      }
      Self::Node(id) => {
        surrealdb_types::RecordId::new("Node", id.0.as_str())
      }
      Self::EncryptionKey(id) => {
        surrealdb_types::RecordId::new("EncryptionKey", id.0.as_str())
      }
      Self::MasterKey(id) => {
        surrealdb_types::RecordId::new("MasterKey", id.0.as_str())
      }
    }
  }
}

impl SurrealValue for CicadaRecordId {
  fn kind_of() -> surrealdb_types::Kind {
    surrealdb_types::Kind::Record(vec![])
  }

  fn into_value(self) -> surrealdb_types::Value {
    match self {
      Self::User(id) => id.into_value(),
      Self::ExternalLogin(id) => id.into_value(),
      Self::Device(id) => id.into_value(),
      Self::OnboardingKey(id) => id.into_value(),
      Self::Filesystem(id) => id.into_value(),
      Self::Node(id) => id.into_value(),
      Self::EncryptionKey(id) => id.into_value(),
      Self::MasterKey(id) => id.into_value(),
    }
  }

  fn from_value(value: surrealdb_types::Value) -> anyhow::Result<Self>
  where
    Self: Sized,
  {
    let surrealdb_types::Value::RecordId(id) = value else {
      return Err(anyhow::anyhow!("Value is not RecordId"));
    };
    let table =
      CicadaTable::from_str(&id.table).with_context(|| {
        format!("Got unrecognized table: {}", id.table)
      })?;
    let RecordIdKey::String(id) = id.key else {
      return Err(anyhow::anyhow!("RecordIdKey is not String"));
    };
    Ok(Self::new(table, id))
  }
}
