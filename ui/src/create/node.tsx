import { useInvalidate, useWrite } from "@/lib/hooks";
import { Button, Flex, Menu, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { Types } from "cicada_client";
import { FilePlus, FolderPlus } from "lucide-react";

const CreateNode = ({
  kind,
  parent,
}: {
  kind: Types.NodeKind;
  parent: number;
}) => {
  const [opened, { open, close }] = useDisclosure(false);
  return (
    <Menu opened={opened} onClose={close} position="bottom-start" width={400}>
      <Menu.Target>
        <Button onClick={open}>
          <CreateNodeTitle kind={kind} />
        </Button>
      </Menu.Target>
      <Menu.Dropdown p="1rem">
        <CreateNodeForm close={close} kind={kind} parent={parent} />
      </Menu.Dropdown>
    </Menu>
  );
  // return (
  //   <>
  //     <Modal
  //       opened={opened}
  //       onClose={close}
  //       title={<CreateNodeTitle kind={kind} />}
  //     >
  //       <CreateNodeForm close={close} kind={kind} parent={parent} />
  //     </Modal>
  //     <Button onClick={open}>
  //       <CreateNodeTitle kind={kind} />
  //     </Button>
  //   </>
  // );
};

const CreateNodeTitle = ({ kind }: { kind: Types.NodeKind }) => {
  return (
    <Flex align="center" gap="xs">
      {kind === "Folder" ? (
        <FolderPlus size="1rem" />
      ) : (
        <FilePlus size="1rem" />
      )}
      Create {kind}
    </Flex>
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
    <form
      onSubmit={form.onSubmit((form) => mutate({ ...form, kind, parent }))}
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
      <Button type="submit" loading={isPending}>
        <CreateNodeTitle kind={kind} />
      </Button>
    </form>
  );
};

export default CreateNode;
