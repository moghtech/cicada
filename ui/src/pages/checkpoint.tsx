import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { Group, Text } from "@mantine/core";
import { useNavigate, useParams } from "react-router-dom";
import ConfirmDelete from "@/components/confirm-delete";
import { Types } from "cicada_client";
import { notifications } from "@mantine/notifications";
import InitializeEncryptionKey from "@/components/initialize-encryption-key";
import { languageFromPath, MonacoDiffEditor, Page, PageGuard } from "mogh_ui";
import { ICONS } from "@/lib/icons";
import EncryptionKeySelector from "@/components/encryption-key-selector";

export default function CheckpointPage() {
  const { checkpoint: _checkpoint } = useParams() as {
    checkpoint: string;
  };
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
  const inv = useInvalidate();
  const nav = useNavigate();
  const encryptionKeys = useRead("ListEncryptionKeys", {}).data;
  const missingKey = encryptionKeys?.find(
    (key) => !checkpoint?.data && key.id === checkpoint?.encryption_key,
  );
  const missingNodeKey = encryptionKeys?.find(
    (key) => !node?.data && key.id === node?.encryption_key,
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
        nav("/checkpoints");
      },
    });

  return (
    <PageGuard
      isPending={isPending || isRefetching || nodePending || nodeRefetching}
      error={
        !checkpoint
          ? "404: No file found"
          : !node
            ? "404: No node found associated with checkpoint"
            : undefined
      }
    >
      {checkpoint && (
        <Page
          title={checkpoint.name || "Checkpoint"}
          description={checkpoint.created_at}
          icon={ICONS.Checkpoint}
          actions={
            <>
              <ConfirmDelete
                entityType="Checkpoint"
                name={checkpoint.name}
                onConfirm={() => deleteCheckpoint({ id: checkpoint.id })}
                loading={deleteCheckpointPending}
                disabled={false}
              />
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
            </>
          }
        >
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
          ) : (
            <MonacoDiffEditor
              language={languageFromPath(checkpoint.name)}
              original={checkpoint.data ?? ""}
              modified={node?.data ?? ""}
            />
          )}
        </Page>
      )}
    </PageGuard>
  );
}
