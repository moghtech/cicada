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
    data: node,
    isPending: nodePending,
    isRefetching: nodeRefetching,
  } = useRead(
    "GetNode",
    {
      id: checkpoint?.node!,
    },
    { enabled: !!checkpoint?.node },
  );

  useSetTitle(checkpoint?.name + (node ? " | " + node?.name : ""));

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
  const missingNodeKey = encryptionKeys?.find(
    (key) => node?.data === null && key.id === node?.encryption_key,
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
      isPending={isPending || isRefetching || nodePending || nodeRefetching}
      error={
        !checkpoint
          ? "404: No checkpoint found"
          : !node
            ? "404: No node found associated with checkpoint"
            : undefined
      }
    >
      {checkpoint && (
        <EntityPage
          backTo={node && `/filesystems/${node?.filesystem}/${node?.inode}`}
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
            {checkpoint.data && (
              <ConfirmFileSave
                node={node}
                data={checkpoint.data}
                initName="Restore checkpoint"
                initDescription={`Restored contents at checkpoint "${checkpoint.name || checkpoint.id}"`}
                disabled={node?.data === checkpoint.data}
                restore
              />
            )}
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
              node={checkpoint.node}
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
          ) : missingNodeKey ? (
            <>
              <Text fz="h2">
                Failed to read data: missing encryption key{" "}
                <b>{missingNodeKey.name}</b>
              </Text>
              {missingNodeKey?.kind === Types.EncryptionKeyKind.Memory && (
                <Group>
                  <InitializeEncryptionKey
                    key_id={missingNodeKey.id}
                    onInit={() => inv(["GetNode"])}
                  />
                </Group>
              )}
            </>
          ) : compareCheckpoint ? (
            <MonacoDiffEditor
              language={node?.name ? languageFromPath(node?.name) : undefined}
              original={checkpoint.data ?? ""}
              modified={compareCheckpoint?.data ?? ""}
              readOnly
            />
          ) : (
            <MonacoEditor
              language={node?.name ? languageFromPath(node?.name) : undefined}
              value={checkpoint.data ?? ""}
              readOnly
            />
          )}

          <Checkpoints node={checkpoint.node} />
        </EntityPage>
      )}
    </PageGuard>
  );
}
