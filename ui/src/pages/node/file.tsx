import { useInvalidate, useWrite } from "@/lib/hooks";
import { Button, Center, Text } from "@mantine/core";
import { useNavigate } from "react-router-dom";
import { History } from "lucide-react";
import { language_from_path, MonacoEditor } from "@/components/monaco";
import { useLocalStorage } from "@mantine/hooks";
import ConfirmSave from "@/components/confirm-save";
import ConfirmDelete from "@/components/confirm-delete";
import { Types } from "cicada_client";
import { notifications } from "@mantine/notifications";
import { Page } from "@/layout/page";
import { ICONS } from "@/lib/icons";

const FilePage = ({ node }: { node: Types.NodeRecord | undefined }) => {
  const inv = useInvalidate();
  const nav = useNavigate();
  const [{ data }, setEdit] = useLocalStorage<{ data: string | undefined }>({
    key: `node-${node?.id}-edit-v1`,
    defaultValue: { data: undefined },
  });
  const { mutateAsync: updateNode } = useWrite("UpdateNode", {
    onSuccess: () => {
      inv(["FindNode"]);
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
    }
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
      title={"File: " + node.name}
      icon={ICONS.File}
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
            onConfirm={() =>
              updateNode({ id: node.id, data }).then(() =>
                notifications.show({ message: "Saved changes to file." })
              )
            }
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
      <MonacoEditor
        language={language_from_path(node.name)}
        value={data ?? node.data ?? ""}
        onValueChange={(data) => setEdit({ data })}
      />
    </Page>
  );
};

export default FilePage;
