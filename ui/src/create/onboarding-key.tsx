import { CopyButton } from "mogh_ui";
import { useInvalidate, useWrite } from "@/lib/hooks";
import { Button, Flex, Menu, Text, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { CircleCheckBig, Plus } from "lucide-react";
import { useState } from "react";

const CreateOnboardingKey = () => {
  const [opened, { open, close }] = useDisclosure(false);
  return (
    <Menu opened={opened} onClose={close} position="bottom-start" width={400}>
      <Menu.Target>
        <Button onClick={open} rightSection={<Plus size="1rem" />}>
          Create Onboarding Key
        </Button>
      </Menu.Target>
      <Menu.Dropdown p="1rem">
        <CreateOnboardingKeyForm close={close} />
      </Menu.Dropdown>
    </Menu>
  );
};

const CreateOnboardingKeyForm = ({ close }: { close: () => void }) => {
  const inv = useInvalidate();
  const [createdPrivateKey, setCreatedPrivateKey] = useState<string | null>(
    null
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
      public_key: "",
    },
    validate: {
      name: (name) => (name.length ? null : "Name cannot be empty"),
      public_key: (public_key) =>
        !public_key.length || public_key.length === 60
          ? null
          : "Public key should be 60 characters",
    },
  });

  if (createdPrivateKey) {
    return (
      <Flex direction="column" gap="1rem">
        <Text>
          Save the onboarding private key. It cannot be retrieved again later.
        </Text>
        <Flex gap="md" align="center" w="100%">
          <TextInput value={createdPrivateKey} w="100%" disabled />
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
    >
      <TextInput
        {...form.getInputProps("name")}
        withAsterisk
        autoFocus
        label="Name"
        placeholder="Enter name"
        key={form.key("name")}
      />
      <TextInput
        {...form.getInputProps("public_key")}
        label="Pre-existing Public Key (Optional)"
        placeholder="Enter public key"
        key={form.key("public_key")}
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
};

export default CreateOnboardingKey;
