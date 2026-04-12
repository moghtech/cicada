import { ICONS } from "@/lib/icons";
import { Button, Group, Modal, Stack } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { languageFromPath, MonacoDiffEditor } from "mogh_ui";
import { ReactNode } from "react";

export default function ConfirmSave({
  name,
  original,
  modified,
  extra,
  onConfirm,
  loading,
  disabled,
  restore,
}: {
  name: string;
  original: string;
  modified: string;
  extra?: ReactNode;
  onConfirm: () => Promise<unknown>;
  loading?: boolean;
  disabled?: boolean;
  restore?: boolean;
}) {
  const [opened, { open, close }] = useDisclosure(false);
  const Icon = restore ? ICONS.Checkpoint : ICONS.Save;
  return (
    <>
      <Modal
        opened={opened}
        onClose={close}
        title={"Save changes to " + name}
        size="auto"
        trapFocus
      >
        <Stack>
          <MonacoDiffEditor
            original={original}
            modified={modified}
            language={languageFromPath(name)}
            style={{ width: 1400, maxWidth: "85vw" }}
            readOnly
          />
          {extra}
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
        </Stack>
      </Modal>
      <Button
        leftSection={<Icon size="1rem" />}
        disabled={disabled}
        onClick={open}
      >
        {restore ? "Restore" : "Save"}
      </Button>
    </>
  );
}
