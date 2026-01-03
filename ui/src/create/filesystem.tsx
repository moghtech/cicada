import { useInvalidate, useWrite } from "@/lib/hooks";
import { Button, Group, Modal, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";

const CreateFilesystem = () => {
  const [opened, { open, close }] = useDisclosure(false);
  return (
    <>
      <Modal opened={opened} onClose={close} title="Create Filesystem">
        <CreateFilesystemForm close={close} />
      </Modal>
      <Button onClick={open}>Create</Button>
    </>
  );
};

const CreateFilesystemForm = ({ close }: { close: () => void; }) => {
  const inv = useInvalidate();
  const { mutate, isPending } = useWrite("CreateFilesystem", {
    onSuccess: () => {
      inv(["ListFilesystems"]);
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
    <form onSubmit={form.onSubmit((form) => mutate(form))}>
      <TextInput
        {...form.getInputProps("name")}
        withAsterisk
        autoFocus
        label="Name"
        placeholder="Enter name"
        key={form.key("name")}
      />
      <Group justify="flex-end" mt="md">
        <Button type="submit" loading={isPending}>
          Create
        </Button>
      </Group>
    </form>
  );
};

export default CreateFilesystem;
