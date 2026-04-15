import ConfirmDelete from "@/components/confirm-delete";
import { useInvalidate, useRead, useSetTitle, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { ActionIcon, Group, Text, TextInput } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { Save } from "lucide-react";
import { EnableSwitch, EntityHeader, EntityPage, PageGuard } from "mogh_ui";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";

export default function OnboardingKeyPage() {
  const { onboardingKey: _onboardingKey } = useParams() as {
    onboardingKey: string;
  };

  const inv = useInvalidate();
  const nav = useNavigate();

  const { data: onboardingKey, isPending } = useRead("GetOnboardingKey", {
    id: _onboardingKey,
  });

  useSetTitle(onboardingKey?.name + " | Onboarding");

  const { mutateAsync: updateOnboardingKey } = useWrite("UpdateOnboardingKey", {
    onSuccess: () => {
      inv(["ListOnboardingKeys"], ["GetOnboardingKey"]);
      notifications.show({
        message: "Saved changes to onboarding key.",
        color: "green",
      });
    },
  });

  const {
    mutateAsync: deleteOnboardingKey,
    isPending: deleteOnboardingKeyPending,
  } = useWrite("DeleteOnboardingKey", {
    onSuccess: () => {
      notifications.show({
        message: "Onboarding key deleted.",
        color: "green",
      });
      inv(["ListOnboardingKeys"]);
      nav("/secrets");
    },
  });

  const [publicKey, setPublicKey] = useState(onboardingKey?.public_key);
  useEffect(
    () => setPublicKey(onboardingKey?.public_key),
    [onboardingKey?.public_key],
  );

  return (
    <PageGuard
      isPending={isPending}
      error={!onboardingKey ? "404: No onboarding key found" : undefined}
    >
      {onboardingKey && (
        <EntityPage backTo="/access">
          <EntityHeader
            name={onboardingKey.name}
            state="Onboarding Key"
            status={new Date(onboardingKey.created_at).toLocaleString()}
            icon={ICONS.OnboardingKey}
            intent={onboardingKey.enabled ? "Good" : "Critical"}
            onRename={async (name) =>
              await updateOnboardingKey({ id: onboardingKey.id, name })
            }
            action={
              <ConfirmDelete
                entityType="Onboarding Key"
                name={onboardingKey?.name ?? "Unknown"}
                onConfirm={async () =>
                  deleteOnboardingKey({ id: onboardingKey.id })
                }
                loading={deleteOnboardingKeyPending}
                disabled={false}
                iconOnly
              />
            }
          />

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
          </Group>
        </EntityPage>
      )}
    </PageGuard>
  );
}
