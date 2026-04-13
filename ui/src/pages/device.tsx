import ConfirmDelete from "@/components/confirm-delete";
import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import {
  ActionIcon,
  Fieldset,
  Group,
  Stack,
  Text,
  TextInput,
} from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { Save } from "lucide-react";
import { EnableSwitch, EntityHeader, EntityPage, PageGuard } from "mogh_ui";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";

export default function DevicePage() {
  const { device: _device } = useParams() as {
    device: string;
  };
  const inv = useInvalidate();
  const nav = useNavigate();

  const { data: device, isPending } = useRead("GetDevice", { id: _device });

  const { mutateAsync: updateDevice } = useWrite("UpdateDevice", {
    onSuccess: () => {
      inv(["ListDevices"], ["GetDevice"]);
      notifications.show({
        message: "Device updated.",
      });
    },
  });

  const { mutateAsync: deleteDevice, isPending: deleteDevicePending } =
    useWrite("DeleteDevice", {
      onSuccess: () => {
        notifications.show({
          message: "Device deleted.",
        });
        inv(["ListDevices"]);
        nav("/devices");
      },
    });

  const [publicKey, setPublicKey] = useState(device?.public_key);
  useEffect(() => setPublicKey(device?.public_key), [device?.public_key]);

  return (
    <PageGuard
      isPending={isPending}
      error={!device ? "404: No device found" : undefined}
    >
      {device && (
        <EntityPage>
          <EntityHeader
            name={device?.name}
            state="Device"
            status={device.created_at}
            icon={ICONS.Device}
            intent={device.enabled ? "Good" : "Critical"}
            onRename={(name) => updateDevice({ id: device.id, name })}
            action={
              <ConfirmDelete
                entityType="Device"
                name={device?.name ?? "Unknown"}
                onConfirm={async () => deleteDevice({ id: device.id })}
                loading={deleteDevicePending}
                disabled={false}
                iconOnly
              />
            }
          />

          <Fieldset legend={<Text size="lg">Config</Text>}>
            <Stack>
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
                    updateDevice({ id: device.id, public_key: publicKey })
                  }
                  disabled={!publicKey || publicKey === device.public_key}
                >
                  <Save size="1rem" />
                </ActionIcon>
              </Group>

              <Group>
                <EnableSwitch
                  ml="md"
                  label="File Access"
                  color="green.8"
                  checked={device.enabled}
                  onCheckedChange={(enabled) =>
                    updateDevice({ id: device.id, enabled })
                  }
                  redDisabled
                />
              </Group>
            </Stack>
          </Fieldset>
        </EntityPage>
      )}
    </PageGuard>
  );
}
