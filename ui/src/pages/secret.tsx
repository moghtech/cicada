import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { Button, Center, Group, Loader, Text } from "@mantine/core";
import { useNavigate, useParams } from "react-router-dom";
import { History } from "lucide-react";
import { useLocalStorage } from "@mantine/hooks";
import ConfirmSave from "@/components/confirm-save";
import ConfirmDelete from "@/components/confirm-delete";
import { Types } from "cicada_client";
import { notifications } from "@mantine/notifications";
import InitializeEncryptionKey from "@/components/initialize-encryption-key";
import { languageFromPath, MonacoEditor, Page } from "mogh_ui";
import { ICONS } from "@/lib/icons";

export default function SecretPage() {
  const { secret: _secret } = useParams() as {
    secret: string;
  };
  const {
    data: secret,
    isPending,
    isRefetching,
  } = useRead("GetSecret", {
    id: _secret,
  });
  const inv = useInvalidate();
  const nav = useNavigate();
  const [{ data }, setEdit] = useLocalStorage<{ data: string | undefined }>({
    key: `secret-${secret?.id}-edit-v1`,
    defaultValue: { data: undefined },
  });
  const missing_key = useRead("ListEncryptionKeys", {}).data?.find(
    (key) => !secret?.data && key.id === secret?.encryption_key,
  );
  const { mutateAsync: updateSecretData } = useWrite("UpdateSecretData", {
    onSuccess: () => {
      notifications.show({
        message: "Saved changes to secret.",
        color: "green",
      });
      inv(["GetSecret", { id: _secret }]);
      setEdit({ data: undefined });
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

  if (isPending || isRefetching) {
    return (
      <Center>
        <Loader />
      </Center>
    );
  }

  if (!secret) {
    return (
      <Center>
        <Text size="lg">404: No file found</Text>
      </Center>
    );
  }

  return (
    <Page
      title={secret.name}
      icon={ICONS.Secret}
      actions={
        <>
          <Button
            leftSection={<History size="1rem" />}
            disabled={!data}
            onClick={() => setEdit({ data: undefined })}
          >
            Reset
          </Button>
          <ConfirmSave
            name={secret.name}
            disabled={!data}
            original={secret.data ?? ""}
            modified={data ?? ""}
            onConfirm={() =>
              updateSecretData({ id: secret.id, data: data ?? "" })
            }
          />
          <ConfirmDelete
            entityType="Secret"
            name={secret.name}
            onConfirm={() => deleteSecret({ id: secret.id })}
            loading={deleteSecretPending}
            disabled={false}
          />
        </>
      }
    >
      {missing_key ? (
        <>
          <Text fz="h2">
            Failed to read data: missing encryption key{" "}
            <b>{missing_key.name}</b>
          </Text>
          {missing_key?.kind === Types.EncryptionKeyKind.Memory && (
            <Group>
              <InitializeEncryptionKey
                key_id={missing_key.id}
                onInit={() => inv(["GetSecret", { id: _secret }])}
              />
            </Group>
          )}
        </>
      ) : (
        <MonacoEditor
          language={languageFromPath(secret.name)}
          value={data ?? secret.data ?? ""}
          onValueChange={(data) => setEdit({ data })}
        />
      )}
    </Page>
  );
}
