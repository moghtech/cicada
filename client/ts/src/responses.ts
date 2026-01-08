import * as Types from "./types.js";

export type ReadResponses = {
  GetVersion: Types.GetVersionResponse;
  GetUser: Types.GetUserResponse;
  GetUsername: Types.GetUsernameResponse;

  // ==== FILESYSTEM ====
  ListFilesystems: Types.ListFilesystemsResponse;

  // ==== NODE ====
  ListNodes: Types.ListNodesResponse;
  GetNode: Types.GetNodeResponse;
  FindNode: Types.FindNodeResponse;
};

export type WriteResponses = {
  // ==== NODE ====
  CreateFilesystem: Types.CreateFilesystemResponse;
  UpdateFilesystem: Types.UpdateFilesystemResponse;
  DeleteFilesystem: Types.DeleteFilesystemResponse;

  // ==== NODE ====
  CreateNode: Types.CreateNodeResponse;
  UpdateNode: Types.UpdateNodeResponse;
  DeleteNode: Types.DeleteNodeResponse;
};
