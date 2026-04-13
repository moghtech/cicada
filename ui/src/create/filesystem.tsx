import EncryptionKeySelector from "@/components/encryption-key-selector";
import { useInvalidate, useWrite } from "@/lib/hooks";
import { Button, Popover, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { Plus } from "lucide-react";
import { useShiftKeyListener } from "mogh_ui";

export default function CreateFilesystem() {
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
        <Button onClick={toggle} leftSection={<Plus size="1rem" />}>
          Create Filesystem
        </Button>
      </Popover.Target>
      <Popover.Dropdown p="lg">
        <CreateFilesystemForm close={close} />
      </Popover.Dropdown>
    </Popover>
  );
}

function CreateFilesystemForm({ close }: { close: () => void }) {
  const inv = useInvalidate();
  const { mutate, isPending } = useWrite("CreateFilesystem", {
    onSuccess: () => {
      notifications.show({ message: "Created filesystem.", color: "green" });
      inv(["ListFilesystems"]);
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
      autoFocus
    >
      <TextInput
        {...form.getInputProps("name")}
        autoFocus
        label="Name"
        placeholder="Enter name"
        key={form.key("name")}
      />
      <EncryptionKeySelector
        label="Default Encryption Key"
        selected={form.getValues().encryption_key}
        onSelect={(id) => form.setFieldValue("encryption_key", id)}
        withinPortal={false}
        autoSelectFirst
      />
      <Button
        leftSection={<Plus size="1rem" />}
        type="submit"
        loading={isPending}
        disabled={!form.isValid()}
      >
        Create Filesystem
      </Button>
    </form>
  );
}
