import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { Button, Group, Modal, Text, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { Types } from "cicada_client";
import { Plus, SquareArrowUp } from "lucide-react";
import { FC } from "react";

const InitializeEncryptionKey = ({
  key_id,
  target: Target,
  onInit,
}: {
  key_id: string;
  target?: FC<{ onClick?: () => void }>;
  onInit?: () => void;
}) => {
  const [opened, { open, close }] = useDisclosure(false);
  const key = useRead("ListEncryptionKeys", {}).data?.find(
    (key) => key.id === key_id,
  );
  if (!key || key.kind !== Types.EncryptionKeyKind.Memory) {
    return;
  }
  return (
    <>
      {Target ? (
        <Target onClick={open} />
      ) : (
        <Button onClick={open} rightSection={<Plus size="1rem" />}>
          Initialize Encryption Key
        </Button>
      )}
      <Modal
        onClick={(e) => e.stopPropagation()}
        opened={opened}
        onClose={close}
        title={
          <Group gap="xs">
            <Text opacity={0.6}>Initialize Key:</Text>
            <Text fw="bold">{key.name}</Text>
          </Group>
        }
      >
        <InitializeEncryptionKeyForm
          key_id={key_id}
          close={close}
          onInit={onInit}
        />
      </Modal>
    </>
  );
};

const InitializeEncryptionKeyForm = ({
  key_id,
  close,
  onInit,
}: {
  key_id: string;
  close: () => void;
  onInit?: () => void;
}) => {
  const inv = useInvalidate();
  const { mutate: initialize, isPending } = useWrite(
    "InitializeEncryptionKey",
    {
      onSuccess: () => {
        inv(["GetEncryptionKey"], ["ListEncryptionKeys"]);
        notifications.show({ message: "Initialized encryption key" });
        close();
        onInit?.();
      },
    },
  );
  const form = useForm({
    mode: "uncontrolled",
    initialValues: {
      key: "",
    },
    validate: {
      key: (key) => (key.length ? null : "Key cannot be empty"),
    },
  });

  return (
    <form
      onSubmit={form.onSubmit((form) =>
        initialize({ id: key_id, key: form.key }),
      )}
      style={{ display: "flex", flexDirection: "column", gap: "1rem" }}
    >
      <TextInput
        {...form.getInputProps("key")}
        withAsterisk
        autoFocus
        label="Key"
        placeholder="Paste key"
        key={form.key("key")}
      />
      <Button
        leftSection={<SquareArrowUp size="1rem" />}
        type="submit"
        loading={isPending}
        disabled={!form.isValid()}
      >
        Initialize
      </Button>
    </form>
  );
};

export default InitializeEncryptionKey;
