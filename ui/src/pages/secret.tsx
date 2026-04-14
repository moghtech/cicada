import { useInvalidate, useRead, useSetTitle, useWrite } from "@/lib/hooks";
import { Button, Group, Text } from "@mantine/core";
import { useNavigate, useParams } from "react-router-dom";
import { History } from "lucide-react";
import { useLocalStorage } from "@mantine/hooks";
import ConfirmDelete from "@/components/confirm-delete";
import { Types } from "cicada_client";
import { notifications } from "@mantine/notifications";
import InitializeEncryptionKey from "@/components/initialize-encryption-key";
import {
  EntityHeader,
  EntityPage,
  languageFromPath,
  MonacoEditor,
  PageGuard,
} from "mogh_ui";
import { ICONS } from "@/lib/icons";
import EncryptionKeySelector from "@/components/encryption-key-selector";
import Checkpoints from "@/components/checkpoints";
import ConfirmSecretSave from "@/components/confirm-secret-save";

export default function SecretPage() {
  const { secret: _secret } = useParams() as {
    secret: string;
  };

  const inv = useInvalidate();
  const nav = useNavigate();

  const {
    data: secret,
    isPending,
    isRefetching,
  } = useRead("GetSecret", {
    id: _secret,
  });

  useSetTitle(secret?.name + " | Secret");

  const [{ data }, setEdit] = useLocalStorage<{ data: string | undefined }>({
    key: `secret-${secret?.id}-edit-v1`,
    defaultValue: { data: undefined },
  });

  const missingKey = useRead("ListEncryptionKeys", {}).data?.find(
    (key) => !secret?.data && key.id === secret?.encryption_key,
  );

  const { mutateAsync: updateSecret } = useWrite("UpdateSecret", {
    onSuccess: () => {
      inv(["ListSecrets"], ["GetSecret"]);
      notifications.show({
        message: "Saved changes to secret.",
        color: "green",
      });
    },
  });

  const {
    mutate: updateSecretEncryptionKey,
    isPending: updateEncryptionKeyPending,
  } = useWrite("UpdateSecretEncryptionKey", {
    onSuccess: () => {
      inv(["ListSecrets"], ["GetSecret", { id: _secret }]);
      notifications.show({
        message: "Saved changes to secret encryption key.",
        color: "green",
      });
    },
  });
  const { mutateAsync: deleteSecret, isPending: deleteSecretPending } =
    useWrite("DeleteSecret", {
      onSuccess: () => {
        notifications.show({ message: "Secret deleted." });
        inv(["ListSecrets"]);
        nav("/secrets");
      },
    });

  return (
    <PageGuard
      isPending={isPending || isRefetching}
      error={!secret ? "404: No file found" : undefined}
    >
      {secret && (
        <EntityPage backTo="/secrets">
          <EntityHeader
            name={secret?.name}
            state="Secret"
            icon={ICONS.Secret}
            intent={missingKey ? "Critical" : "Good"}
            onRename={async (name) =>
              await updateSecret({ id: secret.id, name })
            }
            action={
              <ConfirmDelete
                entityType="Secret"
                name={secret?.name ?? "Unknown"}
                onConfirm={async () => deleteSecret({ id: secret.id })}
                loading={deleteSecretPending}
                disabled={false}
                iconOnly
              />
            }
          />

          <Group>
            <Button
              leftSection={<History size="1rem" />}
              disabled={!data}
              onClick={() => setEdit({ data: undefined })}
            >
              Reset
            </Button>
            <ConfirmSecretSave secret={secret} data={data} setEdit={setEdit} />
            <EncryptionKeySelector
              selected={secret.encryption_key}
              onSelect={(encryption_key) =>
                updateSecretEncryptionKey({ id: secret.id, encryption_key })
              }
              targetProps={{
                w: { base: "100%", xs: 260 },
                loading: updateEncryptionKeyPending,
              }}
            />
          </Group>

          {missingKey ? (
            <>
              <Text fz="h2">
                Failed to read data: missing encryption key{" "}
                <b>{missingKey.name}</b>
              </Text>
              {missingKey?.kind === Types.EncryptionKeyKind.Memory && (
                <Group>
                  <InitializeEncryptionKey
                    key_id={missingKey.id}
                    onInit={() => inv(["GetSecret", { id: _secret }])}
                  />
                </Group>
              )}
            </>
          ) : (
            <>
              <MonacoEditor
                language={languageFromPath(secret.name)}
                value={data ?? secret.data ?? ""}
                onValueChange={(data) => setEdit({ data })}
              />
              <Checkpoints target={{ type: "Secret", id: secret.id }} />
            </>
          )}
        </EntityPage>
      )}
    </PageGuard>
  );
}
