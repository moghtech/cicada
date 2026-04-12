import { Types } from "cicada_client";
import ConfirmSave from "./confirm-save";
import { TextInput } from "@mantine/core";
import { useEffect, useState } from "react";
import { useInvalidate, useWrite } from "@/lib/hooks";
import { notifications } from "@mantine/notifications";

export interface ConfirmFileSaveProps {
  node: Types.NodeEntity | undefined;
  data: string | undefined;
  setEdit?: (data: { data: string | undefined }) => void;
  initName?: string;
  initDescription?: string;
  restore?: boolean;
  disabled?: boolean;
}

export default function ConfirmFileSave({
  node,
  data,
  setEdit,
  initName,
  initDescription,
  restore,
  disabled,
}: ConfirmFileSaveProps) {
  const inv = useInvalidate();
  const [checkpointName, setCheckpointName] = useState(initName);
  const [checkpointDescription, setCheckpointDescription] =
    useState(initDescription);
  useEffect(() => setCheckpointName(initName), [initName]);
  useEffect(() => setCheckpointDescription(initDescription), [initDescription]);
  const { mutateAsync: updateNodeData } = useWrite("UpdateNodeData", {
    onSuccess: () => {
      inv(["FindNode"], ["GetNode"], ["ListCheckpoints"]);
      notifications.show({ message: "Saved changes to file.", color: "green" });
      setEdit?.({ data: undefined });
      setCheckpointName("");
      setCheckpointDescription("");
    },
  });
  return (
    <ConfirmSave
      name={node?.name ?? ""}
      disabled={disabled || !node || !data}
      original={node?.data ?? ""}
      modified={data ?? ""}
      extra={
        <>
          <TextInput
            autoFocus
            label="Message"
            value={checkpointName}
            onChange={(e) => setCheckpointName(e.target.value)}
            placeholder="Add message (optional)"
          />
          <TextInput
            label="Description"
            value={checkpointDescription}
            onChange={(e) => setCheckpointDescription(e.target.value)}
            placeholder="Add description (optional)"
          />
        </>
      }
      onConfirm={async () =>
        node &&
        (await updateNodeData({
          id: node.id,
          data: data ?? "",
          checkpoint_name: checkpointName,
          checkpoint_description: checkpointDescription,
        }))
      }
      restore={restore}
    />
  );
}
