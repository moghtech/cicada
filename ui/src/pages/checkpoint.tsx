import { useInvalidate, useRead, useSetTitle, useWrite } from "@/lib/hooks";
import { Group, Text } from "@mantine/core";
import { useNavigate, useParams } from "react-router-dom";
import ConfirmDelete from "@/components/confirm-delete";
import { Types } from "cicada_client";
import { notifications } from "@mantine/notifications";
import InitializeEncryptionKey from "@/components/initialize-encryption-key";
import {
  EntityHeader,
  EntityPage,
  languageFromPath,
  MonacoDiffEditor,
  MonacoEditor,
  PageGuard,
} from "mogh_ui";
import { ICONS } from "@/lib/icons";
import EncryptionKeySelector from "@/components/encryption-key-selector";
import ConfirmFileSave from "@/components/confirm-file-save";
import { useState } from "react";
import CheckpointSelector from "@/components/checkpoint-selector";
import Checkpoints from "@/components/checkpoints";
import ConfirmSecretSave from "@/components/confirm-secret-save";

export default function CheckpointPage() {
  const { checkpoint: _checkpoint } = useParams() as {
    checkpoint: string;
  };
  const inv = useInvalidate();
  const nav = useNavigate();

  const {
    data: checkpoint,
    isPending,
    isRefetching,
  } = useRead("GetCheckpoint", {
    id: _checkpoint,
  });

  const {
    data: target,
    isPending: targetPending,
    isRefetching: targetRefetching,
  } = useRead(
    `Get${checkpoint?.target.type ?? "Node"}`,
    {
      id: checkpoint?.target.id!,
    },
    { enabled: !!checkpoint?.target.id },
  );

  useSetTitle(checkpoint?.name + (target ? " | " + target?.name : ""));

  const { mutateAsync: updateCheckpoint } = useWrite("UpdateCheckpoint", {
    onSuccess: () => {
      inv(["ListCheckpoints"], ["GetCheckpoint"]);
      notifications.show({
        message: "Saved changes to checkpoint.",
        color: "green",
      });
    },
  });

  const encryptionKeys = useRead("ListEncryptionKeys", {}).data;
  const missingKey = encryptionKeys?.find(
    (key) => checkpoint?.data === null && key.id === checkpoint?.encryption_key,
  );
  const missingTargetKey = encryptionKeys?.find(
    (key) => target?.data === null && key.id === target?.encryption_key,
  );

  const {
    mutate: updateCheckpointEncryptionKey,
    isPending: updateEncryptionKeyPending,
  } = useWrite("UpdateCheckpointEncryptionKey", {
    onSuccess: () => {
      inv(["ListCheckpoints"], ["GetCheckpoint", { id: _checkpoint }]);
      notifications.show({
        message: "Saved changes to checkpoint encryption key.",
        color: "green",
      });
    },
  });

  const { mutateAsync: deleteCheckpoint, isPending: deleteCheckpointPending } =
    useWrite("DeleteCheckpoint", {
      onSuccess: () => {
        notifications.show({ message: "Checkpoint deleted." });
        inv(["ListCheckpoints"]);
        nav(-1);
      },
    });

  const [compare, setCompare] = useState<string | undefined>(undefined);

  const { data: compareCheckpoint } = useRead(
    "GetCheckpoint",
    {
      id: compare!,
    },
    { enabled: !!compare },
  );

  return (
    <PageGuard
      isPending={isPending || isRefetching || targetPending || targetRefetching}
      error={
        !checkpoint
          ? "404: No checkpoint found"
          : !target
            ? "404: No target found associated with checkpoint"
            : undefined
      }
    >
      {checkpoint && (
        <EntityPage
          backTo={
            target &&
            (checkpoint.target.type === "Node"
              ? `/filesystems/${(target as Types.NodeEntity)?.filesystem}/${(target as Types.NodeEntity)?.inode}`
              : `/secrets/${target.id}`)
          }
        >
          <EntityHeader
            name={checkpoint?.name || "Checkpoint"}
            state="Checkpoint"
            status={new Date(checkpoint.created_at).toLocaleString()}
            icon={ICONS.Checkpoint}
            intent={missingKey ? "Critical" : "Good"}
            onRename={async (name) =>
              await updateCheckpoint({ id: checkpoint.id, name })
            }
            action={
              <ConfirmDelete
                entityType="Checkpoint"
                name={checkpoint?.name ?? "Unknown"}
                onConfirm={async () => deleteCheckpoint({ id: checkpoint.id })}
                loading={deleteCheckpointPending}
                disabled={false}
                iconOnly
              />
            }
          />

          <Group>
            {checkpoint.data &&
              (checkpoint.target.type === "Node" ? (
                <ConfirmFileSave
                  node={target as Types.NodeEntity}
                  data={checkpoint.data}
                  initName="Restore checkpoint"
                  initDescription={`Restored contents at checkpoint "${checkpoint.name || checkpoint.id}"`}
                  disabled={target?.data === checkpoint.data}
                  restore
                />
              ) : (
                <ConfirmSecretSave
                  secret={target as Types.SecretEntity}
                  data={checkpoint.data}
                  initName="Restore checkpoint"
                  initDescription={`Restored contents at checkpoint "${checkpoint.name || checkpoint.id}"`}
                  disabled={target?.data === checkpoint.data}
                  restore
                />
              ))}
            <EncryptionKeySelector
              selected={checkpoint.encryption_key}
              onSelect={(encryption_key) =>
                updateCheckpointEncryptionKey({
                  id: checkpoint.id,
                  encryption_key,
                })
              }
              targetProps={{
                w: { base: "100%", xs: 260 },
                loading: updateEncryptionKeyPending,
              }}
            />
            <CheckpointSelector
              target={checkpoint.target}
              selected={compare}
              onSelect={setCompare}
              placeholder="Compare"
              targetProps={{
                w: { base: "100%", xs: 260 },
              }}
              excludeId={checkpoint.id}
            />
          </Group>

          {missingKey ? (
            <>
              <Text fz="h2">
                Failed to read data: missing encryption key{" "}
                <b>{missingKey.name}</b>
              </Text>
              {missingKey?.kind === Types.EncryptionKeyKind.Memory && (
                <Group>
                  <InitializeEncryptionKey
                    key_id={missingKey.id}
                    onInit={() => inv(["GetCheckpoint", { id: _checkpoint }])}
                  />
                </Group>
              )}
            </>
          ) : missingTargetKey ? (
            <>
              <Text fz="h2">
                Failed to read data: missing encryption key{" "}
                <b>{missingTargetKey.name}</b>
              </Text>
              {missingTargetKey?.kind === Types.EncryptionKeyKind.Memory && (
                <Group>
                  <InitializeEncryptionKey
                    key_id={missingTargetKey.id}
                    onInit={() => inv(["GetNode"])}
                  />
                </Group>
              )}
            </>
          ) : compareCheckpoint ? (
            <MonacoDiffEditor
              language={
                target?.name ? languageFromPath(target?.name) : undefined
              }
              original={checkpoint.data ?? ""}
              modified={compareCheckpoint?.data ?? ""}
              readOnly
            />
          ) : (
            <MonacoEditor
              language={
                target?.name ? languageFromPath(target?.name) : undefined
              }
              value={checkpoint.data ?? ""}
              readOnly
            />
          )}

          <Checkpoints target={checkpoint.target} />
        </EntityPage>
      )}
    </PageGuard>
  );
}
