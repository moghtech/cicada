import { Button, Flex, Group, Modal } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { language_from_path, MonacoDiffEditor } from "@/components/monaco";
import { Save } from "lucide-react";

const ConfirmSave = ({
  name,
  original,
  modified,
  onConfirm,
  loading,
  disabled,
}: {
  name: string;
  original: string;
  modified: string;
  onConfirm: () => Promise<unknown>;
  loading?: boolean;
  disabled?: boolean;
}) => {
  const [opened, { open, close }] = useDisclosure(false);
  return (
    <>
      <Modal
        opened={opened}
        onClose={close}
        title={"Save changes to " + name}
        size="100%"
      >
        <MonacoDiffEditor
          original={original}
          modified={modified}
          language={language_from_path(name)}
        />
        <Group mt="md" w="100%">
          <Button
            onClick={() =>
              onConfirm()
                .then(close)
                .catch((err) => console.error(err))
            }
            loading={loading}
            fullWidth
          >
            Save
          </Button>
        </Group>
      </Modal>
      <Button disabled={disabled} onClick={open}>
        <Flex align="center" gap="0.5rem">
          <Save size="1rem" />
          Save
        </Flex>
      </Button>
    </>
  );
};

export default ConfirmSave;
