import { CopyButton, useShiftKeyListener } from "mogh_ui";
import { useInvalidate, useWrite } from "@/lib/hooks";
import { Button, Flex, Popover, Select, Text, TextInput } from "@mantine/core";
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

type ExpiresOptions = "1 day" | "7 days" | "30 days" | "Never";
const ONE_DAY_MS = 1000 * 60 * 60 * 24;

function CreateOnboardingKeyForm({ close }: { close: () => void }) {
  const inv = useInvalidate();
  const [createdPrivateKey, setCreatedPrivateKey] = useState<string | null>(
    null,
  );
  const { mutate, isPending } = useWrite("CreateOnboardingKey", {
    onSuccess: ({ private_key }) => {
      notifications.show({
        message: "Created onboarding key.",
        color: "green",
      });
      inv(["ListOnboardingKeys"]);
      if (private_key) {
        setCreatedPrivateKey(private_key);
      } else {
        close();
      }
    },
  });
  const now = Date.now();
  const expiresOptions: Record<ExpiresOptions, number> = {
    "1 day": now + ONE_DAY_MS,
    "7 days": now + ONE_DAY_MS * 7,
    "30 days": now + ONE_DAY_MS * 90,
    Never: 0,
  };
  const form = useForm({
    mode: "uncontrolled",
    initialValues: {
      name: "",
      expires: expiresOptions["1 day"],
      private_key: "",
    },
    validate: {
      name: (name) => (name.length ? null : "Name cannot be empty"),
      private_key: (private_key: string) =>
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
      onSubmit={form.onSubmit((form) =>
        mutate({
          ...form,
          enabled: true,
        }),
      )}
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
      <Select
        {...form.getInputProps("expires")}
        label="Expires"
        data={Object.entries(expiresOptions).map(([label, value]) => ({
          label,
          value,
        }))}
        comboboxProps={{ withinPortal: false }}
        key={form.key("expires")}
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
