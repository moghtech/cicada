import { Types } from "cicada_client";
import ConfirmSave from "./confirm-save";
import { TextInput } from "@mantine/core";
import { useEffect, useState } from "react";
import { useInvalidate, useWrite } from "@/lib/hooks";
import { notifications } from "@mantine/notifications";

export interface ConfirmSecretSaveProps {
  secret: Types.SecretEntity | undefined;
  data: string | undefined;
  setEdit?: (data: { data: string | undefined }) => void;
  initName?: string;
  initDescription?: string;
  restore?: boolean;
  disabled?: boolean;
}

export default function ConfirmSecretSave({
  secret,
  data,
  setEdit,
  initName,
  initDescription,
  restore,
  disabled,
}: ConfirmSecretSaveProps) {
  const inv = useInvalidate();
  const [checkpointName, setCheckpointName] = useState(initName);
  const [checkpointDescription, setCheckpointDescription] =
    useState(initDescription);
  useEffect(() => setCheckpointName(initName), [initName]);
  useEffect(() => setCheckpointDescription(initDescription), [initDescription]);
  const { mutateAsync: updateSecretData } = useWrite("UpdateSecretData", {
    onSuccess: () => {
      inv(["FindSecret"], ["GetSecret"], ["ListCheckpoints"]);
      notifications.show({
        message: "Saved changes to secret.",
        color: "green",
      });
      setEdit?.({ data: undefined });
      setCheckpointName("");
      setCheckpointDescription("");
    },
  });
  return (
    <ConfirmSave
      name={secret?.name ?? ""}
      disabled={disabled || !secret || !data}
      original={secret?.data ?? ""}
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
        secret &&
        (await updateSecretData({
          id: secret.id,
          data: data ?? "",
          checkpoint_name: checkpointName,
          checkpoint_description: checkpointDescription,
        }))
      }
      restore={restore}
    />
  );
}
