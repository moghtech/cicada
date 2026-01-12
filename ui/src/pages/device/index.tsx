import ConfirmDelete from "@/components/confirm-delete";
import { useRead, useWrite } from "@/lib/hooks";
import {
  ActionIcon,
  Center,
  Fieldset,
  Flex,
  Group,
  Loader,
  Switch,
  Text,
  TextInput,
} from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { Types } from "cicada_client";
import { Save, Server } from "lucide-react";
import { useState } from "react";
import { useNavigate, useParams } from "react-router-dom";

const DevicePage = () => {
  const { device: _device } = useParams() as {
    device: string;
  };
  const {
    data: device,
    isPending,
    refetch: refetchDevice,
  } = useRead("GetDevice", { id: _device });

  if (isPending) {
    return (
      <Center>
        <Loader />
      </Center>
    );
  }

  if (!device) {
    return (
      <Center>
        <Text size="lg">404: No device found</Text>
      </Center>
    );
  }

  return <DeviceInner device={device} refetchDevice={refetchDevice} />;
};

export default DevicePage;

const DeviceInner = ({
  device,
  refetchDevice,
}: {
  device: Types.DeviceRecord;
  refetchDevice: () => void;
}) => {
  const nav = useNavigate();
  const { mutate: updateDevice } = useWrite("UpdateDevice", {
    onSuccess: () => {
      refetchDevice();
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
        nav("/devices");
      },
    });
  const [publicKey, setPublicKey] = useState(device.public_key);

  return (
    <Flex direction="column" gap="lg">
      <Group>
        <Server size={20} />
        <Text fz="h2">{device.name}</Text>
        <Switch
          label="Enabled"
          checked={device.enabled}
          onChange={(e) =>
            updateDevice({ id: device.id, enabled: e.target.checked })
          }
        />
      </Group>

      <Group>
        <ConfirmDelete
          entityType="Device"
          name={device.name}
          onConfirm={() => deleteDevice({ id: device.id })}
          loading={deleteDevicePending}
          disabled={false}
        />
      </Group>

      <Fieldset legend={<Text size="lg">Config</Text>}>
        <Flex direction="column" gap="lg">
          <Group>
            <Text ff="monospace">Public Key:</Text>

            <TextInput
              w={520}
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
        </Flex>
      </Fieldset>
    </Flex>
  );
};
