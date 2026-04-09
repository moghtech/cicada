import EncryptionKeySelector from "@/components/encryption-key-selector";
import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { Button, Popover, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { Types } from "cicada_client";
import { FilePlus, FolderPlus } from "lucide-react";
import { useShiftKeyListener } from "mogh_ui";

interface CreateNodeProps {
  filesystem: string | undefined;
  kind: Types.NodeKind;
  parent: number;
}

export default function CreateNode(props: CreateNodeProps) {
  const [opened, { open, close, toggle }] = useDisclosure(false);
  useShiftKeyListener("N", () => props.kind === Types.NodeKind.File && open());
  useShiftKeyListener(
    "F",
    () => props.kind === Types.NodeKind.Folder && open(),
  );
  return (
    <Popover
      opened={opened}
      position="bottom-start"
      width="400"
      onChange={toggle}
      trapFocus
    >
      <Popover.Target>
        <Button
          onClick={toggle}
          leftSection={<CreateNodeIcon kind={props.kind} />}
        >
          Create {props.kind}
        </Button>
      </Popover.Target>
      <Popover.Dropdown p="1rem">
        <CreateNodeForm close={close} {...props} />
      </Popover.Dropdown>
    </Popover>
  );
}

function CreateNodeIcon({ kind }: { kind: Types.NodeKind }) {
  return kind === "Folder" ? (
    <FolderPlus size="1rem" />
  ) : (
    <FilePlus size="1rem" />
  );
}

function CreateNodeForm({
  close,
  filesystem: _filesystem,
  kind,
  parent,
}: {
  close: () => void;
} & CreateNodeProps) {
  const inv = useInvalidate();
  const { mutate, isPending } = useWrite("CreateNode", {
    onSuccess: () => {
      notifications.show({ message: `Created ${kind.toLowerCase()}.` });
      inv(["ListNodes"]);
      close();
    },
  });
  const filesystem = useRead("ListFilesystems", {}).data?.find(
    (fs) => fs.id === _filesystem,
  );
  const form = useForm({
    mode: "uncontrolled",
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
      onSubmit={form.onSubmit((form) =>
        mutate({
          ...form,
          filesystem: _filesystem,
          kind,
          parent,
        }),
      )}
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
      {kind === Types.NodeKind.File && (
        <EncryptionKeySelector
          label="Encryption Key"
          defaultKey={filesystem?.encryption_key}
          selected={form.getValues().encryption_key}
          onSelect={(id) => form.setFieldValue("encryption_key", id)}
          withinPortal={false}
          autoSelectFirst
        />
      )}
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
}
