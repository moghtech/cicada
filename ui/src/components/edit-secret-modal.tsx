import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import {
  Button,
  Center,
  Loader,
  Modal,
  Stack,
  Text,
  Textarea,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { useState } from "react";
import { ICONS } from "@/lib/icons";

export interface EditSecretModalProps {
  id: string;
  name: string;
}

export default function EditSecretModal({ id, name }: EditSecretModalProps) {
  const [opened, { open, close }] = useDisclosure(false);
  const inv = useInvalidate();

  const { data: secret, isPending } = useRead(
    "GetSecret",
    { id },
    { enabled: opened },
  );

  const [edited, setEdited] = useState<string | undefined>(undefined);

  const { mutate: updateSecretData, isPending: saving } = useWrite(
    "UpdateSecretData",
    {
      onSuccess: () => {
        notifications.show({ message: "Saved secret data.", color: "green" });
        inv(["GetSecret", { id }]);
        inv(["ListSecrets"]);
        setEdited(undefined);
        close();
      },
    },
  );

  return (
    <>
      <Modal
        opened={opened}
        onClose={() => {
          setEdited(undefined);
          close();
        }}
        title={<Text fz="h2">{name} - Data</Text>}
        size="xl"
      >
        {isPending || saving ? (
          <Center py="xl">
            <Loader />
          </Center>
        ) : (
          <Stack>
            <Textarea
              value={edited ?? secret?.data ?? ""}
              onChange={(e) => setEdited(e.target.value)}
              placeholder="Input data..."
              resize="vertical"
              styles={{ input: { minHeight: 200, fontFamily: "monospace" } }}
            />
            <Button
              leftSection={<ICONS.Save size="1rem" />}
              onClick={() =>
                updateSecretData({
                  id,
                  data: edited ?? secret?.data ?? "",
                })
              }
              loading={saving}
              disabled={edited === undefined}
              ml="auto"
            >
              Save
            </Button>
          </Stack>
        )}
      </Modal>

      <Button
        leftSection={<ICONS.Edit size="1rem" />}
        onClick={(e) => {
          e.stopPropagation();
          setEdited(undefined);
          open();
        }}
        w={{ base: 200, lg: 300 }}
        justify="start"
      >
        Edit
      </Button>
    </>
  );
}
