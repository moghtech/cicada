import * as Types from "./types.js";

export type ReadResponses = {
  GetVersion: Types.GetVersionResponse;

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

  // ==== NODE ====
  CreateNode: Types.CreateNodeResponse;
  UpdateNode: Types.UpdateNodeResponse;
};
