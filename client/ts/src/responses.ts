import * as Types from "./types.js";

export type ReadResponses = {
  GetVersion: Types.GetVersionResponse;
  GetUser: Types.GetUserResponse;
  GetUsername: Types.GetUsernameResponse;

  // ==== DEVICE ====
  ListDevices: Types.ListDevicesResponse;
  GetDevice: Types.GetDeviceResponse;

  // ==== ONBOARDING KEY ====
  ListOnboardingKeys: Types.ListOnboardingKeysResponse;
  GetOnboardingKey: Types.GetOnboardingKeyResponse;

  // ==== FILESYSTEM ====
  ListFilesystems: Types.ListFilesystemsResponse;

  // ==== NODE ====
  ListNodes: Types.ListNodesResponse;
  GetNode: Types.GetNodeResponse;
  FindNode: Types.FindNodeResponse;

  // ==== ENCRYPTION KEY ====
  ListEncryptionKeys: Types.ListEncryptionKeysResponse;
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
  DeleteFilesystem: Types.DeleteFilesystemResponse;

  // ==== NODE ====
  CreateNode: Types.CreateNodeResponse;
  UpdateNode: Types.UpdateNodeResponse;
  UpdateNodeData: Types.UpdateNodeDataResponse;
  DeleteNode: Types.DeleteNodeResponse;
  BatchDeleteNodes: Types.BatchDeleteNodesResponse;

  // ==== ENCRYPTION KEY ====
  CreateEncryptionKey: Types.CreateEncryptionKeyResponse;
  UpdateEncryptionKey: Types.UpdateEncryptionKeyResponse;
};
