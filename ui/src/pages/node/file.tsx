import { useInvalidate, useWrite } from "@/lib/hooks";
import { Button, Center, Flex, Text } from "@mantine/core";
import { useNavigate } from "react-router-dom";
import { File, History } from "lucide-react";
import { language_from_path, MonacoEditor } from "@/components/monaco";
import { useLocalStorage } from "@mantine/hooks";
import ConfirmSave from "@/components/confirm-save";
import ConfirmDelete from "@/components/confirm-delete";
import { Types } from "cicada_client";

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
    <Flex direction="column" gap="md">
      <Flex align="center" gap="md">
        <Flex gap="sm" align="center">
          <File size={20} />
          <h2 style={{ opacity: 0.6 }}>File:</h2>
          <h2>{node.name}</h2>
        </Flex>
        <Flex gap="sm" align="center">
          <Button disabled={!data} onClick={() => setEdit({ data: undefined })}>
            <Flex align="center" gap="0.5rem">
              <History size="1rem" />
              Reset
            </Flex>
          </Button>
          <ConfirmSave
            name={node.name}
            disabled={!data}
            original={node.data ?? ""}
            modified={data ?? ""}
            onConfirm={() => updateNode({ id: node.id, data })}
          />
          <ConfirmDelete
            entityType="File"
            name={node.name}
            onConfirm={() => deleteNode({ id: node.id, move_children: 1 })}
            loading={deleteNodePending}
            disabled={false}
          />
        </Flex>
      </Flex>
      <MonacoEditor
        language={language_from_path(node.name)}
        value={data ?? node.data ?? ""}
        onValueChange={(data) => setEdit({ data })}
      />
    </Flex>
  );
};

export default FilePage;
