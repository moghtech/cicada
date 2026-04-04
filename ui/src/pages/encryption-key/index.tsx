import InitializeEncryptionKey from "@/components/initialize-key";
import { useRead, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import {
  Button,
  Center,
  Fieldset,
  Flex,
  Group,
  Loader,
  Text,
} from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { Types } from "cicada_client";
import { Page } from "mogh_ui";
import { useParams } from "react-router-dom";

const EncryptionKeyPage = () => {
  const { encryptionKey: _encryptionKey } = useParams() as {
    encryptionKey: string;
  };
  const { data: encryptionKey, isPending } = useRead("GetEncryptionKey", {
    id: _encryptionKey,
  });
  const {} = useWrite("InitializeEncryptionKey", {
    onSuccess: () => {
      notifications.show({ message: "Initialized encryption key" });
    },
  });

  if (isPending) {
    return (
      <Center>
        <Loader />
      </Center>
    );
  }

  if (!encryptionKey) {
    return (
      <Center>
        <Text size="lg">404: No encryptionKey found</Text>
      </Center>
    );
  }

  return <EncryptionKeyInner encryptionKey={encryptionKey} />;
};

export default EncryptionKeyPage;

const EncryptionKeyInner = ({
  encryptionKey,
}: {
  encryptionKey: Types.EncryptionKeyEntity;
}) => {
  return (
    <Page
      title={encryptionKey.name}
      icon={ICONS.EncryptionKey}
      description={"Encryption Key - " + encryptionKey.kind}
    >
      <Fieldset legend={<Text size="lg">Config</Text>}>
        <Flex direction="column" gap="lg">
          <Group>
            {encryptionKey.initialized ? (
              <Button color="green.8">Ready for use</Button>
            ) : (
              <InitializeEncryptionKey key_id={encryptionKey.id} />
            )}
          </Group>
        </Flex>
      </Fieldset>
    </Page>
  );
};
