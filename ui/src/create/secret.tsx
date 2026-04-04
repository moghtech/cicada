import { useInvalidate, useWrite } from "@/lib/hooks";
import { Button, Menu, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { Plus } from "lucide-react";

export default function CreateSecret() {
  const [opened, { open, close }] = useDisclosure(false);
  return (
    <Menu opened={opened} onClose={close} position="bottom-start" width={400}>
      <Menu.Target>
        <Button onClick={open} leftSection={<Plus size="1rem" />}>
          Create Secret
        </Button>
      </Menu.Target>
      <Menu.Dropdown p="1rem">
        <CreateSecretForm close={close} />
      </Menu.Dropdown>
    </Menu>
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
    mode: "uncontrolled",
    initialValues: {
      name: "",
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
        withAsterisk
        autoFocus
        label="Name"
        placeholder="Enter name"
        key={form.key("name")}
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
