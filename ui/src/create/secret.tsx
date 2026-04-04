import EncryptionKeySelector from "@/components/encryption-key-selector";
import { useInvalidate, useWrite } from "@/lib/hooks";
import { Button, Popover, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { Plus } from "lucide-react";
import { useShiftKeyListener } from "mogh_ui";

export default function CreateSecret() {
  const [opened, { open, close, toggle }] = useDisclosure(false);
  useShiftKeyListener("N", () => open());
  return (
    <Popover
      opened={opened}
      position="bottom-start"
      offset={21}
      width="400"
      onChange={toggle}
      trapFocus
    >
      <Popover.Target>
        <Button leftSection={<Plus size="1rem" />} onClick={toggle}>
          Create Secret
        </Button>
      </Popover.Target>
      <Popover.Dropdown p="lg">
        <CreateSecretForm close={close} />
      </Popover.Dropdown>
    </Popover>
  );
}

function CreateSecretForm({ close }: { close: () => void }) {
  const inv = useInvalidate();
  const { mutate, isPending } = useWrite("CreateSecret", {
    onSuccess: () => {
      notifications.show({ message: "Created secret." });
      inv(["ListSecrets"]);
      close();
    },
  });
  const form = useForm({
    mode: "controlled",
    initialValues: {
      name: "",
      encryption_key: undefined as string | undefined,
    },
    validate: {
      name: (name) => (name.length ? null : "Name cannot be empty"),
    },
  });
  return (
    <form
      onSubmit={form.onSubmit((form) => mutate(form))}
      style={{ display: "flex", flexDirection: "column", gap: "1rem" }}
    >
      <TextInput
        {...form.getInputProps("name")}
        autoFocus
        label="Name"
        placeholder="Enter name"
        key={form.key("name")}
      />
      <EncryptionKeySelector
        label="Encryption Key"
        selected={form.getValues().encryption_key}
        onSelect={(id) => form.setFieldValue("encryption_key", id)}
        withinPortal={false}
      />
      <Button
        leftSection={<Plus size="1rem" />}
        type="submit"
        loading={isPending}
        disabled={!form.isValid()}
      >
        Create Secret
      </Button>
    </form>
  );
}
