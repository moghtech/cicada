import { CopyButton, useManageAuth, useShiftKeyListener } from "mogh_ui";
import { useInvalidate } from "@/lib/hooks";
import {
  Button,
  Group,
  Popover,
  Select,
  Stack,
  Text,
  TextInput,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { CircleCheckBig, Plus } from "lucide-react";
import { useState } from "react";
import { MoghAuth } from "cicada_client";

export default function CreateApiKey() {
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
          Create Api Key
        </Button>
      </Popover.Target>
      <Popover.Dropdown p="1rem">
        <CreateApiKeyForm close={close} />
      </Popover.Dropdown>
    </Popover>
  );
}

type ExpiresOptions = "90 days" | "180 days" | "1 year" | "Never";
const ONE_DAY_MS = 1000 * 60 * 60 * 24;

function CreateApiKeyForm({ close }: { close: () => void }) {
  const inv = useInvalidate();
  const [created, setCreated] =
    useState<MoghAuth.Types.CreateApiKeyResponse | null>(null);
  const { mutate, isPending } = useManageAuth("CreateApiKey", {
    onSuccess: (response) => {
      notifications.show({
        message: "Created onboarding key.",
        color: "green",
      });
      inv(["ListApiKeys"]);
      setCreated(response);
    },
  });
  const now = Date.now();
  const expiresOptions: Record<ExpiresOptions, number> = {
    "90 days": now + ONE_DAY_MS * 90,
    "180 days": now + ONE_DAY_MS * 180,
    "1 year": now + ONE_DAY_MS * 365,
    Never: 0,
  };
  const form = useForm({
    mode: "uncontrolled",
    initialValues: {
      name: "",
      expires: expiresOptions["90 days"],
    },
    validate: {
      name: (name) => (name.length ? null : "Name cannot be empty"),
    },
  });

  if (created) {
    return (
      <Stack>
        <Text>Save the api key and secret. The secret cannot be retrieved again later.</Text>
        <Group gap="sm" wrap="nowrap">
          <Text w="100">Key</Text>
          <TextInput value={created.key} w="100%" contentEditable={false} />
          <CopyButton content={created.key} />
        </Group>
        <Group gap="sm" wrap="nowrap">
          <Text w="100">Secret</Text>
          <TextInput value={created.secret} w="100%" contentEditable={false} />
          <CopyButton content={created.secret} />
        </Group>
        <Button
          leftSection={<CircleCheckBig size="1rem" />}
          onClick={() => close()}
        >
          Done
        </Button>
      </Stack>
    );
  }

  return (
    <form
      onSubmit={form.onSubmit((form) => mutate(form))}
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
      <Button
        leftSection={<Plus size="1rem" />}
        type="submit"
        loading={isPending}
        disabled={!form.isValid()}
      >
        Create Api Key
      </Button>
    </form>
  );
}
