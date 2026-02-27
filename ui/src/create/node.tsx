import { useInvalidate, useWrite } from "@/lib/hooks";
import { Button, Menu, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { Types } from "cicada_client";
import { FilePlus, FolderPlus } from "lucide-react";

interface CreateNodeProps {
  filesystem: string | undefined;
  kind: Types.NodeKind;
  parent: number;
}

const CreateNode = (props: CreateNodeProps) => {
  const [opened, { open, close }] = useDisclosure(false);
  return (
    <Menu opened={opened} onClose={close} position="bottom-start" width={400}>
      <Menu.Target>
        <Button
          onClick={open}
          leftSection={<CreateNodeIcon kind={props.kind} />}
        >
          Create {props.kind}
        </Button>
      </Menu.Target>
      <Menu.Dropdown p="1rem">
        <CreateNodeForm close={close} {...props} />
      </Menu.Dropdown>
    </Menu>
  );
};

const CreateNodeIcon = ({ kind }: { kind: Types.NodeKind }) => {
  return kind === "Folder" ? (
    <FolderPlus size="1rem" />
  ) : (
    <FilePlus size="1rem" />
  );
};

const CreateNodeForm = ({
  close,
  filesystem,
  kind,
  parent,
}: {
  close: () => void;
} & CreateNodeProps) => {
  const inv = useInvalidate();
  const { mutate, isPending } = useWrite("CreateNode", {
    onSuccess: () => {
      notifications.show({ message: `Created ${kind.toLowerCase()}.` });
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
    <form
      onSubmit={form.onSubmit((form) =>
        mutate({ ...form, filesystem, kind, parent }),
      )}
      style={{ display: "flex", flexDirection: "column", gap: "1rem" }}
      autoFocus
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
        leftSection={<CreateNodeIcon kind={kind} />}
        type="submit"
        loading={isPending}
        disabled={!form.isValid()}
      >
        Create {kind}
      </Button>
    </form>
  );
};

export default CreateNode;
