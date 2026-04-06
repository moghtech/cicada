import InitializeEncryptionKey from "@/components/initialize-key";
import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { Fieldset, Flex, Group, Text } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { ConfirmButton, Page, PageGuard } from "mogh_ui";
import { useParams } from "react-router-dom";

export default function EncryptionKeyPage() {
  const { encryptionKey: _encryptionKey } = useParams() as {
    encryptionKey: string;
  };
  const { data: encryptionKey, isPending } = useRead("GetEncryptionKey", {
    id: _encryptionKey,
  });
  const inv = useInvalidate();
  const { mutate: uninit, isPending: uninitPending } = useWrite(
    "UninitializeEncryptionKey",
    {
      onSuccess: () => {
        inv(["GetEncryptionKey"], ["ListEncryptionKeys"]);
        notifications.show({
          message: "Uninitialized encryption key",
          color: "green",
        });
      },
    },
  );

  return (
    <PageGuard
      isPending={isPending}
      error={!encryptionKey ? "Encryption Key could not be found." : undefined}
    >
      {encryptionKey && (
        <Page
          title={encryptionKey.name}
          icon={ICONS.EncryptionKey}
          description={"Encryption Key - " + encryptionKey.kind}
        >
          <Fieldset legend={<Text size="lg">Config</Text>}>
            <Flex direction="column" gap="lg">
              <Group>
                {encryptionKey.initialized ? (
                  <ConfirmButton
                    icon={<ICONS.Clear size="1rem" />}
                    onClick={() => uninit({ id: encryptionKey.id })}
                    loading={uninitPending}
                    confirmProps={{ variant: "filled", color: "red" }}
                  >
                    Uninitialize
                  </ConfirmButton>
                ) : (
                  <InitializeEncryptionKey key_id={encryptionKey.id} />
                )}
              </Group>
            </Flex>
          </Fieldset>
        </Page>
      )}
    </PageGuard>
  );
}
