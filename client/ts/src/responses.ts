import * as Types from "./types.js";

export type ReadResponses = {
  GetVersion: Types.GetVersionResponse;
  GetUser: Types.GetUserResponse;
  GetUsername: Types.GetUsernameResponse;

  // ==== DEVICE ====
  ListDevices: Types.ListDevicesResponse;

  // ==== FILESYSTEM ====
  ListFilesystems: Types.ListFilesystemsResponse;

  // ==== NODE ====
  ListNodes: Types.ListNodesResponse;
  GetNode: Types.GetNodeResponse;
  FindNode: Types.FindNodeResponse;
};

export type WriteResponses = {
  // ==== DEVICE ====
  CreateDevice: Types.CreateDeviceResponse;
  UpdateDevice: Types.UpdateDeviceResponse;
  DeleteDevice: Types.DeleteDeviceResponse;

  // ==== FILESYSTEM ====
  CreateFilesystem: Types.CreateFilesystemResponse;
  UpdateFilesystem: Types.UpdateFilesystemResponse;
  DeleteFilesystem: Types.DeleteFilesystemResponse;

  // ==== NODE ====
  CreateNode: Types.CreateNodeResponse;
  UpdateNode: Types.UpdateNodeResponse;
  DeleteNode: Types.DeleteNodeResponse;
};
