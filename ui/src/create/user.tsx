import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import {
  Button,
  Group,
  PasswordInput,
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
import { EnableSwitch, useShiftKeyListener } from "mogh_ui";
import { useState } from "react";

export default function CreateUser() {
  const [opened, { open, close, toggle }] = useDisclosure(false);
  useShiftKeyListener("N", () => open());
  return (
    <Popover
      opened={opened}
      position="bottom-start"
      offset={21}
      width="400"
      onChange={toggle}
      trapFocus
    >
      <Popover.Target>
        <Button onClick={toggle} leftSection={<Plus size="1rem" />}>
          Create User
        </Button>
      </Popover.Target>
      <Popover.Dropdown p="lg">
        <CreateUserForm close={close} />
      </Popover.Dropdown>
    </Popover>
  );
}

function CreateUserForm({ close }: { close: () => void }) {
  const [step, setStep] = useState(0);
  const inv = useInvalidate();
  const { mutate, isPending } = useWrite("CreateUser", {
    onSuccess: () => {
      notifications.show({ message: "Created filesystem." });
      inv(["ListUsers"]);
      close();
    },
  });

  const { data: groups } = useRead("ListGroups", {});

  const form = useForm({
    mode: "controlled",
    initialValues: {
      username: "",
      password: "",
      passwordConfirm: "",
      groups: [],
      admin: false,
      super_admin: false,
      enabled: true,
    },
    validate: {
      username: (username) =>
        username.length ? null : "Username cannot be empty",
      password: (password) =>
        password.length ? null : "Password cannot be empty",
      passwordConfirm: (passwordConfirm, form) =>
        passwordConfirm !== form.password ? "Passwords do not match" : null,
    },
  });
  return (
    <Stack
      renderRoot={(props) => (
        <form
          onSubmit={(e) => {
            e.preventDefault();
            if (step < 1) {
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
        <Stepper.Step label="Step 1" description="Credentials">
          <Stack gap="xs">
            <Text size="lg" c="dimmed">
              Enter the user login credentials.
            </Text>
            <TextInput
              {...form.getInputProps("username")}
              autoFocus
              label="Username"
              placeholder="Enter username"
              key={form.key("username")}
            />
            <PasswordInput
              {...form.getInputProps("password")}
              label="Password"
              placeholder="Enter password"
              size="sm"
              key={form.key("password")}
            />
            <PasswordInput
              {...form.getInputProps("passwordConfirm")}
              label="Confirm"
              placeholder="Confirm password"
              size="sm"
              key={form.key("passwordConfirm")}
            />
          </Stack>
        </Stepper.Step>

        <Stepper.Step label="Step 2" description="Permissions">
          <Text size="lg" c="dimmed">
            Configure user permissions.
          </Text>
          <Stack gap="xs">
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
            <Group justify="space-between">
              <Text>Admin</Text>
              <EnableSwitch
                {...form.getInputProps("admin", { type: "checkbox" })}
                key={form.key("admin")}
                mt="md"
                onKeyDown={(e) => {
                  e.stopPropagation();
                  if (e.key === "Enter") {
                    e.preventDefault();
                    form.setFieldValue("admin", !form.values.admin);
                  }
                }}
              />
            </Group>
            <Group justify="space-between">
              <Text>Super Admin</Text>
              <EnableSwitch
                {...form.getInputProps("super_admin", { type: "checkbox" })}
                key={form.key("super_admin")}
                mt="md"
                onKeyDown={(e) => {
                  e.stopPropagation();
                  if (e.key === "Enter") {
                    e.preventDefault();
                    form.setFieldValue("super_admin", !form.values.super_admin);
                  }
                }}
              />
            </Group>
          </Stack>
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
            step < 1 ? <ArrowRight size="1rem" /> : <Plus size="1rem" />
          }
          type="submit"
          loading={isPending}
          disabled={!form.isValid()}
        >
          {step < 1 ? "Next" : "Create"}
        </Button>
      </Group>
    </Stack>
  );
}
