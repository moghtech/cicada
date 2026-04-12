import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { Button, Group, Stack, Text, TextInput } from "@mantine/core";
import { useNavigate } from "react-router-dom";
import { History } from "lucide-react";
import { useLocalStorage } from "@mantine/hooks";
import ConfirmSave from "@/components/confirm-save";
import ConfirmDelete from "@/components/confirm-delete";
import { Types } from "cicada_client";
import { notifications } from "@mantine/notifications";
import InitializeEncryptionKey from "@/components/initialize-encryption-key";
import {
  EntityHeader,
  EntityPage,
  languageFromPath,
  MonacoEditor,
} from "mogh_ui";
import { ReactNode, useEffect, useState } from "react";
import InterpolationModeSelector from "@/components/interpolation-mode-selector";
import EncryptionKeySelector from "@/components/encryption-key-selector";
import { ICONS } from "@/lib/icons";
import CheckpointingModeSelector from "@/components/checkpointing-mode-selector";
import Checkpoints from "@/components/checkpoints";

const FilePage = ({
  filesystem,
  node,
  nodeError,
  toggleInterpolation,
}: {
  filesystem: Types.FilesystemRecord | undefined;
  node: Types.NodeEntity | undefined;
  nodeError: { result?: unknown } | undefined;
  toggleInterpolation: ReactNode;
}) => {
  const inv = useInvalidate();
  const nav = useNavigate();

  const [perm, setPerm] = useState("");
  useEffect(
    () => setPerm(node?.perm ? `0o${node?.perm?.toString(8)}` : ""),
    [node?.perm],
  );

  const [{ data }, setEdit] = useLocalStorage<{ data: string | undefined }>({
    key: `node-${node?.id}-edit-v1`,
    defaultValue: { data: undefined },
  });
  const missingKey = useRead("ListEncryptionKeys", {}).data?.find(
    (key) => node?.missing_key && key.id === node?.encryption_key,
  );
  const { mutateAsync: updateNode } = useWrite("UpdateNode", {
    onSuccess: () => {
      inv(["ListNodes"], ["FindNode"]);
      notifications.show({ message: "Saved changes to node.", color: "green" });
    },
  });
  const {
    mutate: updateNodeEncryptionKey,
    isPending: updateEncryptionKeyPending,
  } = useWrite("UpdateNodeEncryptionKey", {
    onSuccess: () => {
      inv(["FindNode"]);
      notifications.show({
        message: "Saved changes to node encryption key.",
        color: "green",
      });
    },
  });
  const { mutateAsync: updateNodeData } = useWrite("UpdateNodeData", {
    onSuccess: () => {
      inv(["FindNode"], ["ListCheckpoints"]);
      notifications.show({ message: "Saved changes to file.", color: "green" });
      setEdit({ data: undefined });
    },
  });
  const { mutateAsync: deleteNode, isPending: deleteNodePending } = useWrite(
    "DeleteNode",
    {
      onSuccess: () => {
        notifications.show({ message: "File deleted." });
        nav(`/filesystems/${node?.filesystem}/${node?.parent}`);
      },
    },
  );

  return (
    <EntityPage>
      <EntityHeader
        name={node?.name}
        state="File"
        icon={ICONS.File}
        intent={node?.missing_key ? "Critical" : "Good"}
        onRename={async (name) =>
          node && (await updateNode({ id: node?.id, name }))
        }
        action={
          <ConfirmDelete
            entityType={node ? "Folder" : "Filesystem"}
            name={node ? node.name : (filesystem?.name ?? "")}
            onConfirm={async () => node && deleteNode({ id: node.id })}
            loading={deleteNodePending}
            disabled={false}
            iconOnly
          />
        }
      />
      <Group>
        <Button
          leftSection={<History size="1rem" />}
          disabled={!node || !data}
          onClick={() => setEdit({ data: undefined })}
        >
          Reset
        </Button>
        <ConfirmSave
          name={node?.name ?? ""}
          disabled={!node || !data}
          original={node?.data ?? ""}
          modified={data ?? ""}
          onConfirm={async () =>
            node && (await updateNodeData({ id: node.id, data: data ?? "" }))
          }
        />
        {node?.id && (
          <EncryptionKeySelector
            selected={node?.encryption_key}
            onSelect={(encryption_key) =>
              updateNodeEncryptionKey({ id: node.id, encryption_key })
            }
            targetProps={{
              w: { base: "100%", xs: 260 },
              loading: updateEncryptionKeyPending,
            }}
          />
        )}
        <TextInput
          placeholder="0o644"
          value={perm}
          onChange={(e) => setPerm(e.target.value)}
          onKeyDown={(e) => {
            if (node && perm && e.key === "Enter") {
              updateNode({ id: node.id, perm: Number(perm) });
            }
          }}
          disabled={!node}
        />
        {node && (
          <>
            <CheckpointingModeSelector
              value={node?.checkpointing}
              onChange={(checkpointing) =>
                updateNode({ id: node.id, checkpointing })
              }
              inherit={filesystem?.checkpointing}
            />
            <InterpolationModeSelector
              value={node?.interpolation}
              onChange={(interpolation) =>
                updateNode({ id: node.id, interpolation })
              }
              inherit={filesystem?.interpolation}
            />
          </>
        )}
        {toggleInterpolation}
      </Group>

      {nodeError ? (
        <Stack>
          <Text fz="h2">Failed to read data:</Text>
          <MonacoEditor
            value={JSON.stringify(
              nodeError.result ? nodeError.result : nodeError,
              undefined,
              2,
            )}
            language="json"
            readOnly
          />
        </Stack>
      ) : node?.missing_key ? (
        <>
          <Text fz="h2">
            Failed to read data: missing encryption key{" "}
            {missingKey && <b>{missingKey.name}</b>}
          </Text>
          {missingKey?.kind === Types.EncryptionKeyKind.Memory && (
            <Group>
              <InitializeEncryptionKey
                key_id={missingKey.id}
                onInit={() => inv(["FindNode"])}
              />
            </Group>
          )}
        </>
      ) : (
        node && (
          <>
            <MonacoEditor
              language={languageFromPath(node.name)}
              value={data ?? node.data ?? ""}
              onValueChange={(data) => setEdit({ data })}
            />
            <Checkpoints node={node.id} />
          </>
        )
      )}
    </EntityPage>
  );
};

export default FilePage;
