import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { Button, Center, Group, Text } from "@mantine/core";
import { useNavigate } from "react-router-dom";
import { History } from "lucide-react";
import { language_from_path, MonacoEditor } from "@/components/monaco";
import { useLocalStorage } from "@mantine/hooks";
import ConfirmSave from "@/components/confirm-save";
import ConfirmDelete from "@/components/confirm-delete";
import { Types } from "cicada_client";
import { notifications } from "@mantine/notifications";
import { Page } from "@/layout/page";
import { NodePageDescription, NodePageTitle } from "./title";
import InitializeEncryptionKey from "@/components/initialize-key";

const FilePage = ({
  filesystem,
  node,
}: {
  filesystem: Types.FilesystemRecord | undefined;
  node: Types.NodeEntity | undefined;
}) => {
  const inv = useInvalidate();
  const nav = useNavigate();
  const [{ data }, setEdit] = useLocalStorage<{ data: string | undefined }>({
    key: `node-${node?.id}-edit-v1`,
    defaultValue: { data: undefined },
  });
  const missing_key = useRead("ListEncryptionKeys", {}).data?.find(
    (key) => key.id === node?.missing_key,
  );
  const { mutateAsync: updateNodeData } = useWrite("UpdateNodeData", {
    onSuccess: () => {
      inv(["FindNode"]);
      notifications.show({ message: "Saved changes to file." });
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

  if (!node) {
    return (
      <Center>
        <Text size="lg">404: No file found</Text>
      </Center>
    );
  }

  return (
    <Page
      customTitle={<NodePageTitle node={node} />}
      customDescription={<NodePageDescription filesystem={filesystem} />}
      actions={
        <>
          <Button
            leftSection={<History size="1rem" />}
            disabled={!data}
            onClick={() => setEdit({ data: undefined })}
          >
            Reset
          </Button>
          <ConfirmSave
            name={node.name}
            disabled={!data}
            original={node.data ?? ""}
            modified={data ?? ""}
            onConfirm={() => updateNodeData({ id: node.id, data: data ?? "" })}
          />
          <ConfirmDelete
            entityType="File"
            name={node.name}
            onConfirm={() => deleteNode({ id: node.id, move_children: 1 })}
            loading={deleteNodePending}
            disabled={false}
          />
        </>
      }
    >
      {node.missing_key ? (
        <>
          <Text fz="h2">Failed to read data: missing encryption key</Text>
          {missing_key?.kind === Types.EncryptionKeyKind.Memory && (
            <Group>
              <InitializeEncryptionKey
                key_id={missing_key.id}
                onInit={() => inv(["FindNode"])}
              />
            </Group>
          )}
        </>
      ) : (
        <MonacoEditor
          language={language_from_path(node.name)}
          value={data ?? node.data ?? ""}
          onValueChange={(data) => setEdit({ data })}
        />
      )}
    </Page>
  );
};

export default FilePage;
