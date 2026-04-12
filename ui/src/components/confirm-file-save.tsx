import { Types } from "cicada_client";
import ConfirmSave from "./confirm-save";
import { useLocalStorage } from "@mantine/hooks";
import { TextInput } from "@mantine/core";
import { useState } from "react";
import { useInvalidate, useWrite } from "@/lib/hooks";
import { notifications } from "@mantine/notifications";

export interface ConfirmFileSaveProps {
  node: Types.NodeEntity | undefined;
}

export default function ConfirmFileSave({ node }: ConfirmFileSaveProps) {
  const inv = useInvalidate();
  const [{ data }, setEdit] = useLocalStorage<{ data: string | undefined }>({
    key: `node-${node?.id}-edit-v1`,
    defaultValue: { data: undefined },
  });
  const [checkpointName, setCheckpointName] = useState("");
  const [checkpointDescription, setCheckpointDescription] = useState("");
  const { mutateAsync: updateNodeData } = useWrite("UpdateNodeData", {
    onSuccess: () => {
      inv(["FindNode"], ["ListCheckpoints"]);
      notifications.show({ message: "Saved changes to file.", color: "green" });
      setEdit({ data: undefined });
      setCheckpointName("");
      setCheckpointDescription("");
    },
  });
  return (
    <ConfirmSave
      name={node?.name ?? ""}
      disabled={!node || !data}
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
    />
  );
}
