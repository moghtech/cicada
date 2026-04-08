import { CopyButton, useShiftKeyListener } from "mogh_ui";
import { useInvalidate, useWrite } from "@/lib/hooks";
import { Button, Flex, Popover, Text, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { CircleCheckBig, Plus } from "lucide-react";
import { useState } from "react";

export default function CreateOnboardingKey() {
  const [opened, { open, close, toggle }] = useDisclosure(false);
  useShiftKeyListener("N", () => open());
  return (
    <Popover
      opened={opened}
      position="bottom-start"
      offset={16}
      width="400"
      onChange={toggle}
      trapFocus
    >
      <Popover.Target>
        <Button onClick={toggle} leftSection={<Plus size="1rem" />}>
          Create Onboarding Key
        </Button>
      </Popover.Target>
      <Popover.Dropdown p="1rem">
        <CreateOnboardingKeyForm close={close} />
      </Popover.Dropdown>
    </Popover>
  );
}

function CreateOnboardingKeyForm({ close }: { close: () => void }) {
  const inv = useInvalidate();
  const [createdPrivateKey, setCreatedPrivateKey] = useState<string | null>(
    null,
  );
  const { mutate, isPending } = useWrite("CreateOnboardingKey", {
    onSuccess: ({ private_key }) => {
      notifications.show({ message: "Created onboarding key." });
      inv(["ListOnboardingKeys"]);
      if (private_key) {
        setCreatedPrivateKey(private_key);
      } else {
        close();
      }
    },
  });
  const form = useForm({
    mode: "uncontrolled",
    initialValues: {
      name: "",
      private_key: "",
    },
    validate: {
      name: (name) => (name.length ? null : "Name cannot be empty"),
      private_key: (private_key) =>
        !private_key.length || private_key.length === 32
          ? null
          : "Private key should be 32 characters",
    },
  });

  if (createdPrivateKey) {
    return (
      <Flex direction="column" gap="1rem">
        <Text>
          Save the onboarding key. It cannot be retrieved again later.
        </Text>
        <Flex gap="md" align="center" w="100%">
          <TextInput
            value={createdPrivateKey}
            w="100%"
            contentEditable={false}
          />
          <CopyButton content={createdPrivateKey} />
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
      onSubmit={form.onSubmit((form) => mutate({ ...form, enabled: true }))}
      style={{ display: "flex", flexDirection: "column", gap: "1rem" }}
      onClick={(e) => e.stopPropagation()}
    >
      <TextInput
        {...form.getInputProps("name")}
        autoFocus
        label="Name"
        placeholder="Enter name"
        key={form.key("name")}
      />
      <TextInput
        {...form.getInputProps("private_key")}
        label="Pre-existing Key (Optional)"
        placeholder="Enter key"
        key={form.key("private_key")}
      />
      <Button
        leftSection={<Plus size="1rem" />}
        type="submit"
        loading={isPending}
        disabled={!form.isValid()}
      >
        Create Onboarding Key
      </Button>
    </form>
  );
}
