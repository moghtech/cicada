import InitializeEncryptionKey from "@/components/initialize-encryption-key";
import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { Group } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { Types } from "cicada_client";
import { ConfirmButton, EntityHeader, EntityPage, PageGuard } from "mogh_ui";
import { useParams } from "react-router-dom";

export default function EncryptionKeyPage() {
  const { encryptionKey: _encryptionKey } = useParams() as {
    encryptionKey: string;
  };

  const inv = useInvalidate();

  const { data: encryptionKey, isPending } = useRead("GetEncryptionKey", {
    id: _encryptionKey,
  });

  const { mutateAsync: updateEncryptionKey } = useWrite("UpdateEncryptionKey", {
    onSuccess: () => {
      inv(["ListEncryptionKeys"], ["GetEncryptionKey"]);
      notifications.show({
        message: "Saved changes to encryption key.",
        color: "green",
      });
    },
  });

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
        <EntityPage title={encryptionKey.name}>
          <EntityHeader
            name={encryptionKey.name}
            state="Encryption Key"
            status={encryptionKey.kind}
            icon={ICONS.EncryptionKey}
            intent={encryptionKey.initialized ? "Good" : "Critical"}
            onRename={async (name) =>
              await updateEncryptionKey({ id: encryptionKey.id, name })
            }
          />

          <Group>
            {encryptionKey.kind === Types.EncryptionKeyKind.Memory &&
              (encryptionKey.initialized ? (
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
              ))}
          </Group>
        </EntityPage>
      )}
    </PageGuard>
  );
}
