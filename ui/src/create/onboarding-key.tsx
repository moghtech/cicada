import { CopyIconButton } from "@/components/copy-button";
import { useInvalidate, useWrite } from "@/lib/hooks";
import { Button, Flex, Group, Modal, Text, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { Plus } from "lucide-react";
import { useState } from "react";

const CreateOnboardingKey = () => {
  const [opened, { open, close }] = useDisclosure(false);
  return (
    <>
      <Modal opened={opened} onClose={close} title="Create Onboarding Key">
        <CreateOnboardingKeyForm close={close} />
      </Modal>
      <Button onClick={open} rightSection={<Plus size="1rem" />}>
        Create Onboarding Key
      </Button>
    </>
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
        !public_key.length || public_key.length === 64
          ? null
          : "Public key should be 64 characters",
    },
  });

  if (createdPrivateKey) {
    return (
      <Flex direction="column" gap="1rem">
        <Text>
          Save the onboarding private key. It cannot be retrieved again later.
        </Text>
        <Flex gap="md" align="center">
          <Text ff="monospace" opacity={0.6}>
            Private Key:
          </Text>
          <TextInput value={createdPrivateKey} disabled />
          <CopyIconButton content={createdPrivateKey} />
        </Flex>
        <Group justify="flex-end" mt="md">
          <Button onClick={() => close()}>Close</Button>
        </Group>
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
        withAsterisk
        autoFocus
        label="Public Key (Optional)"
        placeholder="Enter public key"
        key={form.key("public_key")}
      />
      <Group justify="flex-end" mt="md">
        <Button type="submit" loading={isPending}>
          Create
        </Button>
      </Group>
    </form>
  );
};

export default CreateOnboardingKey;
