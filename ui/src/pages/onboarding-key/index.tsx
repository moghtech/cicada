import ConfirmDelete from "@/components/confirm-delete";
import { useRead, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import {
  ActionIcon,
  Center,
  Fieldset,
  Flex,
  Group,
  Loader,
  Text,
  TextInput,
} from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { Types } from "cicada_client";
import { Save } from "lucide-react";
import { EnableSwitch, Page } from "mogh_ui";
import { useState } from "react";
import { useNavigate, useParams } from "react-router-dom";

const OnboardingKeyPage = () => {
  const { onboardingKey: _onboardingKey } = useParams() as {
    onboardingKey: string;
  };
  const {
    data: onboardingKey,
    isPending,
    refetch: refetchOnboardingKey,
  } = useRead("GetOnboardingKey", { id: _onboardingKey });

  if (isPending) {
    return (
      <Center>
        <Loader />
      </Center>
    );
  }

  if (!onboardingKey) {
    return (
      <Center>
        <Text size="lg">404: No onboardingKey found</Text>
      </Center>
    );
  }

  return (
    <OnboardingKeyInner
      onboardingKey={onboardingKey}
      refetchOnboardingKey={refetchOnboardingKey}
    />
  );
};

export default OnboardingKeyPage;

const OnboardingKeyInner = ({
  onboardingKey,
  refetchOnboardingKey,
}: {
  onboardingKey: Types.OnboardingKeyRecord;
  refetchOnboardingKey: () => void;
}) => {
  const nav = useNavigate();
  const { mutate: updateOnboardingKey } = useWrite("UpdateOnboardingKey", {
    onSuccess: () => {
      refetchOnboardingKey();
      notifications.show({
        message: "OnboardingKey updated.",
      });
    },
  });
  const {
    mutateAsync: deleteOnboardingKey,
    isPending: deleteOnboardingKeyPending,
  } = useWrite("DeleteOnboardingKey", {
    onSuccess: () => {
      notifications.show({
        message: "OnboardingKey deleted.",
      });
      nav("/devices");
    },
  });
  const [publicKey, setPublicKey] = useState(onboardingKey.public_key);

  return (
    <Page
      title={onboardingKey.name}
      icon={ICONS.OnboardingKey}
      description="Onboarding Key"
      actions={
        <>
          <ConfirmDelete
            entityType="Onboarding Key"
            name={onboardingKey.name}
            onConfirm={() => deleteOnboardingKey({ id: onboardingKey.id })}
            loading={deleteOnboardingKeyPending}
            disabled={false}
          />
          <EnableSwitch
            ml="md"
            label="Can onboard devices"
            color="green.8"
            checked={onboardingKey.enabled}
            onCheckedChange={(enabled) =>
              updateOnboardingKey({ id: onboardingKey.id, enabled })
            }
            redDisabled
          />
        </>
      }
    >
      <Fieldset legend={<Text size="lg">Config</Text>}>
        <Flex direction="column" gap="lg">
          <Group>
            <Text ff="monospace">Public Key:</Text>

            <TextInput
              w={550}
              maw="90vw"
              value={publicKey}
              onChange={(e) => setPublicKey(e.target.value)}
            />

            <ActionIcon
              onClick={() =>
                publicKey &&
                updateOnboardingKey({
                  id: onboardingKey.id,
                  public_key: publicKey,
                })
              }
              disabled={!publicKey || publicKey === onboardingKey.public_key}
            >
              <Save size="1rem" />
            </ActionIcon>
          </Group>
        </Flex>
      </Fieldset>
    </Page>
  );
};
