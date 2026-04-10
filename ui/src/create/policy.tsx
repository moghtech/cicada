import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import {
  Button,
  Group,
  MultiSelect,
  Popover,
  Stack,
  Stepper,
  TagsInput,
  Text,
  TextInput,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { ArrowLeft, ArrowRight, Plus } from "lucide-react";
import { useState } from "react";
import { EnableSwitch, useShiftKeyListener } from "mogh_ui";

export default function CreatePolicy() {
  const [opened, { open, close, toggle }] = useDisclosure(false);
  useShiftKeyListener("N", () => open());
  return (
    <Popover
      opened={opened}
      position="bottom-start"
      offset={21}
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
      notifications.show({ message: "Created policy." });
      inv(["ListPolicies"]);
      close();
    },
  });

  const { data: users } = useRead("ListUsers", {});
  const { data: devices } = useRead("ListDevices", {});
  const { data: groups } = useRead("ListGroups", {});
  const { data: filesystems } = useRead("ListFilesystems", {});

  const form = useForm({
    mode: "controlled",
    initialValues: {
      name: "",
      users: [] as string[],
      devices: [] as string[],
      groups: [] as string[],
      filesystems: [] as string[],
      filesystem_write: false,
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
          <TagsInput
            {...form.getInputProps("groups")}
            label="Groups"
            placeholder="Select or create groups"
            data={groups?.map((g) => g.name) ?? []}
            comboboxProps={{ withinPortal: false }}
            clearable
            key={form.key("groups")}
            mt="md"
            onKeyDown={(e) => e.stopPropagation()}
          />
          <MultiSelect
            {...form.getInputProps("users")}
            label="Users"
            placeholder="Select users"
            data={users?.map((u) => ({ value: u.id, label: u.username })) ?? []}
            comboboxProps={{ withinPortal: false }}
            searchable
            clearable
            key={form.key("users")}
            mt="md"
            onKeyDown={(e) => e.stopPropagation()}
          />
          <MultiSelect
            {...form.getInputProps("devices")}
            label="Devices"
            placeholder="Select devices"
            data={devices?.map((d) => ({ value: d.id, label: d.name })) ?? []}
            comboboxProps={{ withinPortal: false }}
            searchable
            clearable
            key={form.key("devices")}
            mt="md"
            onKeyDown={(e) => e.stopPropagation()}
          />
        </Stepper.Step>

        <Stepper.Step label="Step 2" description="Filesystems">
          <Text size="lg" c="dimmed">
            Select filesystems which clients can access.
          </Text>
          <MultiSelect
            {...form.getInputProps("filesystems")}
            label="Filesystems"
            placeholder="Select filesystems"
            data={
              filesystems?.map((f) => ({ value: f.id, label: f.name })) ?? []
            }
            comboboxProps={{ withinPortal: false }}
            searchable
            clearable
            key={form.key("filesystems")}
            mt="md"
            onKeyDown={(e) => e.stopPropagation()}
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

        <Stepper.Step label="Step 3" description="Policy name">
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
