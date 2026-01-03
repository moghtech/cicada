import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { Button, Center, Flex, Loader, Text } from "@mantine/core";
import { useNavigate, useParams } from "react-router-dom";
import { File, History } from "lucide-react";
import { language_from_path, MonacoEditor } from "@/components/monaco";
import { useLocalStorage } from "@mantine/hooks";
import ConfirmSave from "@/components/confirm-save";
import { useEffect } from "react";

const FilePage = () => {
  const { file } = useParams() as {
    file: string;
  };
  const inv = useInvalidate();
  const [{ data }, setEdit] = useLocalStorage<{ data: string | undefined }>({
    key: `node-${file}-edit-v1`,
    defaultValue: { data: undefined },
  });
  const { data: node, isPending } = useRead("GetNode", { id: file });
  const { mutateAsync } = useWrite("UpdateNode", {
    onSuccess: () => {
      inv(["GetNode", { id: file }]);
      setEdit({ data: undefined });
    },
  });
  const nav = useNavigate();

  // Auto redirect folder nodes to appropriate page.
  useEffect(() => {
    if (!node) return;
    if (node.kind === "Folder") {
      nav(`/filesystems/${node.filesystem}/${node.ino}`, { replace: true });
    }
  }, [node?.kind]);

  if (!isPending && !node) {
    return (
      <Center>
        <Text size="lg">404: No file found matching '{file}'</Text>
      </Center>
    );
  }

  if (!node) {
    return (
      <Center h="70vh">
        <Loader size="lg" />
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
            onConfirm={() => mutateAsync({ id: node.id, data })}
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
