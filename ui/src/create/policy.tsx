import { useInvalidate, useWrite } from "@/lib/hooks";
import {
  Button,
  Group,
  Popover,
  Stack,
  Stepper,
  Text,
  TextInput,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { ArrowLeft, ArrowRight, Plus } from "lucide-react";
import { useState } from "react";
import { EnableSwitch, useShiftKeyListener } from "mogh_ui";
import GroupMultiSelector from "@/components/group-multi-selector";
import UserMultiSelector from "@/components/user-multi-selector";
import DeviceMultiSelector from "@/components/device-multi-selector";
import FilesystemMultiSelector from "@/components/filesystem-multi-selector";

export default function CreatePolicy() {
  const [opened, { open, close, toggle }] = useDisclosure(false);
  useShiftKeyListener("N", () => open());
  return (
    <Popover
      opened={opened}
      position="bottom-start"
      offset={16}
      onChange={toggle}
      trapFocus
      closeOnEscape={false}
    >
      <Popover.Target>
        <Button leftSection={<Plus size="1rem" />} onClick={toggle}>
          Create Policy
        </Button>
      </Popover.Target>
      <Popover.Dropdown p="lg">
        <CreatePolicyForm close={close} />
      </Popover.Dropdown>
    </Popover>
  );
}

function CreatePolicyForm({ close }: { close: () => void }) {
  const [step, setStep] = useState(0);
  const inv = useInvalidate();
  const { mutate, isPending } = useWrite("CreatePolicy", {
    onSuccess: () => {
      notifications.show({ message: "Created policy.", color: "green" });
      inv(["ListPolicies"]);
      close();
    },
  });

  const form = useForm({
    mode: "controlled",
    initialValues: {
      name: "",
      users: [] as string[],
      devices: [] as string[],
      groups: [] as string[],
      filesystems: [] as string[],
      filesystem_write: false,
      enabled: true,
    },
    validate: {
      name: (name) => (name.length ? null : "Name cannot be empty"),
    },
  });

  return (
    <Stack
      miw={{ xs: 500 }}
      renderRoot={(props) => (
        <form
          onSubmit={(e) => {
            e.preventDefault();
            if (step < 2) {
              setStep((s) => s + 1);
            } else {
              form.onSubmit((form) => mutate(form))(e);
            }
          }}
          {...props}
        />
      )}
    >
      <Stepper active={step} color="orange" size="sm">
        <Stepper.Step label="Step 1" description="Clients">
          <Text size="lg" c="dimmed">
            Select clients which gain access by this policy.
          </Text>
          <GroupMultiSelector
            {...form.getInputProps("groups")}
            label="Groups"
            comboboxProps={{ withinPortal: false }}
            key={form.key("groups")}
            mt="md"
          />
          <UserMultiSelector
            {...form.getInputProps("users")}
            label="Users"
            comboboxProps={{ withinPortal: false }}
            key={form.key("users")}
            mt="md"
          />
          <DeviceMultiSelector
            {...form.getInputProps("devices")}
            label="Devices"
            comboboxProps={{ withinPortal: false }}
            key={form.key("devices")}
            mt="md"
          />
        </Stepper.Step>

        <Stepper.Step label="Step 2" description="Filesystems">
          <Text size="lg" c="dimmed">
            Select filesystems which clients can access.
          </Text>
          <FilesystemMultiSelector
            {...form.getInputProps("filesystems")}
            label="Filesystems"
            placeholder="Select filesystems"
            comboboxProps={{ withinPortal: false }}
            key={form.key("filesystems")}
            mt="md"
          />
          <Group justify="end">
            <EnableSwitch
              {...form.getInputProps("filesystem_write", { type: "checkbox" })}
              label="Write access"
              key={form.key("filesystem_write")}
              mt="md"
              onKeyDown={(e) => {
                e.stopPropagation();
                if (e.key === "Enter") {
                  e.preventDefault();
                  form.setFieldValue(
                    "filesystem_write",
                    !form.values.filesystem_write,
                  );
                }
              }}
            />
          </Group>
        </Stepper.Step>

        <Stepper.Step label="Step 3" description="Name">
          <Text size="lg" c="dimmed">
            Name the policy.
          </Text>
          <TextInput
            {...form.getInputProps("name")}
            autoFocus
            label="Policy name"
            placeholder="Enter name"
            key={form.key("name")}
            mt="md"
          />
        </Stepper.Step>
      </Stepper>

      <Group justify="space-between" mt="md">
        <Button
          variant="outline"
          leftSection={<ArrowLeft size="1rem" />}
          onClick={() => setStep((s) => s - 1)}
          disabled={step === 0}
        >
          Back
        </Button>
        <Button
          leftSection={
            step < 2 ? <ArrowRight size="1rem" /> : <Plus size="1rem" />
          }
          type="submit"
          loading={isPending}
          disabled={step === 2 ? !form.isValid() : false}
        >
          {step < 2 ? "Next" : "Create"}
        </Button>
      </Group>
    </Stack>
  );
}
