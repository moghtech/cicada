import { ActionIcon, Button, Flex, Modal, Text } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { Trash2 } from "lucide-react";
import { ReactNode } from "react";

const ConfirmDelete = ({
  action = "Delete",
  entityType,
  name,
  info,
  onConfirm,
  loading,
  disabled,
  iconOnly,
}: {
  action?: string;
  entityType?: string;
  name: string;
  info?: ReactNode;
  onConfirm: () => Promise<unknown>;
  loading?: boolean;
  disabled?: boolean;
  iconOnly?: boolean;
}) => {
  const [opened, { open, close }] = useDisclosure(false);
  const formatted_et = entityType ? ` ${entityType}` : "";
  return (
    <>
      <Modal
        opened={opened}
        onClose={close}
        title={
          <Flex gap="0.3rem" align="center">
            {action}
            {formatted_et} <Text fw="bolder">{name}</Text>
          </Flex>
        }
      >
        <Flex direction="column" gap="lg">
          {info}
          <span>Note: This action may not be undoable.</span>
          <Button
            onClick={() =>
              onConfirm()
                .then(close)
                .catch((err) => console.error(err))
            }
            variant="filled"
            color="red"
            loading={loading}
            leftSection={<Trash2 size="1rem" />}
            fullWidth
          >
            {action}
            {formatted_et}
          </Button>
        </Flex>
      </Modal>
      {iconOnly ? (
        <ActionIcon disabled={disabled} onClick={open} size="xl">
          <Trash2 size="1.3rem" />
        </ActionIcon>
      ) : (
        <Button
          disabled={disabled}
          onClick={open}
          leftSection={<Trash2 size="1rem" />}
        >
          {action}
          {formatted_et}
        </Button>
      )}
    </>
  );
};

export default ConfirmDelete;
