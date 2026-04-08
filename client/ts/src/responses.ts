import * as Types from "./types.js";

export type ReadResponses = {
  GetVersion: Types.GetVersionResponse;
  GetUsername: Types.GetUsernameResponse;

  // ==== DEVICE ====
  ListDevices: Types.ListDevicesResponse;
  GetDevice: Types.GetDeviceResponse;

  // ==== ONBOARDING KEY ====
  ListOnboardingKeys: Types.ListOnboardingKeysResponse;
  GetOnboardingKey: Types.GetOnboardingKeyResponse;

  // ==== FILESYSTEM ====
  ListFilesystems: Types.ListFilesystemsResponse;
  GetFilesystem: Types.GetFilesystemResponse;

  // ==== NODE ====
  ListNodes: Types.ListNodesResponse;
  GetNode: Types.GetNodeResponse;
  FindNode: Types.FindNodeResponse;

  // ==== SECRET ====
  ListSecrets: Types.ListSecretsResponse;
  GetSecret: Types.GetSecretResponse;
  FindSecret: Types.FindSecretResponse;

  // ==== ENCRYPTION KEY ====
  ListEncryptionKeys: Types.ListEncryptionKeysResponse;
  GetEncryptionKey: Types.GetEncryptionKeyResponse;
};

export type WriteResponses = {
  // ==== DEVICE ====
  CreateDevice: Types.CreateDeviceResponse;
  UpdateDevice: Types.UpdateDeviceResponse;
  DeleteDevice: Types.DeleteDeviceResponse;
  BatchDeleteDevices: Types.BatchDeleteDevicesResponse;

  // ==== ONBOARDING KEY ====
  CreateOnboardingKey: Types.CreateOnboardingKeyResponse;
  UpdateOnboardingKey: Types.UpdateOnboardingKeyResponse;
  DeleteOnboardingKey: Types.DeleteOnboardingKeyResponse;
  BatchDeleteOnboardingKeys: Types.BatchDeleteOnboardingKeysResponse;

  // ==== FILESYSTEM ====
  CreateFilesystem: Types.CreateFilesystemResponse;
  UpdateFilesystem: Types.UpdateFilesystemResponse;
  UpdateFilesystemEncryptionKey: Types.UpdateFilesystemEncryptionKeyResponse;
  DeleteFilesystem: Types.DeleteFilesystemResponse;

  // ==== NODE ====
  CreateNode: Types.CreateNodeResponse;
  UpdateNode: Types.UpdateNodeResponse;
  UpdateNodeData: Types.UpdateNodeDataResponse;
  UpdateNodeEncryptionKey: Types.UpdateNodeEncryptionKeyResponse;
  RotateNodeEnvelopeKey: Types.RotateNodeEnvelopeKeyResponse;
  DeleteNode: Types.DeleteNodeResponse;
  BatchDeleteNodes: Types.BatchDeleteNodesResponse;

  // ==== SECRET ====
  CreateSecret: Types.CreateSecretResponse;
  UpdateSecret: Types.UpdateSecretResponse;
  UpdateSecretData: Types.UpdateSecretDataResponse;
  UpdateSecretEncryptionKey: Types.UpdateSecretEncryptionKeyResponse;
  RotateSecretEnvelopeKey: Types.RotateSecretEnvelopeKeyResponse;
  DeleteSecret: Types.DeleteSecretResponse;
  BatchDeleteSecrets: Types.BatchDeleteSecretsResponse;

  // ==== ENCRYPTION KEY ====
  CreateEncryptionKey: Types.CreateEncryptionKeyResponse;
  UpdateEncryptionKey: Types.UpdateEncryptionKeyResponse;
  InitializeEncryptionKey: Types.InitializeEncryptionKeyResponse;
  UninitializeEncryptionKey: Types.UninitializeEncryptionKeyResponse;
};
