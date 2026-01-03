import { useInvalidate, useWrite } from "@/lib/hooks";
import { Button, Group, Modal, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { Types } from "cicada_client";

const CreateNode = ({
  kind,
  parent,
}: {
  kind: Types.NodeKind;
  parent: number;
}) => {
  const [opened, { open, close }] = useDisclosure(false);
  return (
    <>
      <Modal opened={opened} onClose={close} title={`Create ${kind}`}>
        <CreateNodeForm close={close} kind={kind} parent={parent} />
      </Modal>
      <Button onClick={open}>Create {kind}</Button>
    </>
  );
};

const CreateNodeForm = ({
  close,
  kind,
  parent,
}: {
  close: () => void;
  kind: Types.NodeKind;
  parent: number;
}) => {
  const inv = useInvalidate();
  const { mutate, isPending } = useWrite("CreateNode", {
    onSuccess: () => {
      inv(["ListNodes"]);
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
    <form onSubmit={form.onSubmit((form) => mutate({ ...form, kind, parent }))}>
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

export default CreateNode;
