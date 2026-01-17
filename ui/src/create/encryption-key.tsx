import { CopyIconButton } from "@/components/copy-button";
import { useInvalidate, useWrite } from "@/lib/hooks";
import { Button, Flex, Menu, Select, Text, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { Types } from "cicada_client";
import { CircleCheckBig, Plus } from "lucide-react";
import { useState } from "react";

const CreateEncryptionKey = () => {
  const [opened, { open, close }] = useDisclosure(false);
  return (
    <Menu opened={opened} onClose={close} position="bottom-start" width={400}>
      <Menu.Target>
        <Button onClick={open} rightSection={<Plus size="1rem" />}>
          Create Encryption Key
        </Button>
      </Menu.Target>
      <Menu.Dropdown p="1rem">
        <CreateEncryptionKeyForm close={close} />
      </Menu.Dropdown>
    </Menu>
  );
};

const CreateEncryptionKeyForm = ({ close }: { close: () => void }) => {
  const inv = useInvalidate();
  const [created, setCreated] = useState<Types.EncryptionKeyRecord | null>(
    null,
  );
  const { mutate, isPending } = useWrite("CreateEncryptionKey", {
    onSuccess: (key) => {
      notifications.show({ message: "Created onboarding key." });
      inv(["ListEncryptionKeys"]);
      if (key && key.kind === Types.EncryptionKeyKind.Memory) {
        setCreated(key);
      } else {
        close();
      }
    },
  });
  const form = useForm({
    mode: "uncontrolled",
    initialValues: {
      name: "",
      kind: Types.EncryptionKeyKind.Memory,
      key: "",
    },
    validate: {
      name: (name) => (name.length ? null : "Name cannot be empty"),
      kind: (kind) =>
        Object.values(Types.EncryptionKeyKind).includes(kind)
          ? null
          : "Invalid encryption key kind",
      key: (_) => null,
    },
  });

  if (created?.key) {
    return (
      <Flex direction="column" gap="1rem">
        <Text>
          Save the encryption key. It cannot be retrieved again later.
        </Text>
        <Flex gap="md" align="center" w="100%">
          <TextInput value={created.key} w="100%" disabled />
          <CopyIconButton content={created.key} />
        </Flex>
        <Button
          leftSection={<CircleCheckBig size="1rem" />}
          onClick={() => close()}
        >
          Done
        </Button>
      </Flex>
    );
  }

  return (
    <form
      onSubmit={form.onSubmit((form) => mutate(form))}
      style={{ display: "flex", flexDirection: "column", gap: "1rem" }}
    >
      <TextInput
        {...form.getInputProps("name")}
        withAsterisk
        autoFocus
        label="Name"
        placeholder="Enter name"
        key={form.key("name")}
      />
      <Select
        {...form.getInputProps("kind")}
        label="Kind"
        placeholder="Choose key kind"
        key={form.key("kind")}
        data={Object.values(Types.EncryptionKeyKind)}
        allowDeselect={false}
        comboboxProps={{ withinPortal: false }}
      />
      <TextInput
        {...form.getInputProps("key")}
        label="Pre-existing Key (Optional)"
        placeholder="Enter encryption key"
        key={form.key("key")}
      />
      <Button
        leftSection={<Plus size="1rem" />}
        type="submit"
        loading={isPending}
        disabled={!form.isValid()}
      >
        Create Encryption Key
      </Button>
    </form>
  );
};

export default CreateEncryptionKey;
