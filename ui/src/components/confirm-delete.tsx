import { Button, Flex, Modal, Text } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { Trash2 } from "lucide-react";

const ConfirmDelete = ({
  action = "Delete",
  entityType,
  name,
  onConfirm,
  loading,
  disabled,
}: {
  action?: string;
  entityType?: string;
  name: string;
  onConfirm: () => Promise<unknown>;
  loading?: boolean;
  disabled?: boolean;
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
      <Button
        variant="filled"
        color="red"
        disabled={disabled}
        onClick={open}
        leftSection={<Trash2 size="1rem" />}
      >
        {action}
        {formatted_et}
      </Button>
    </>
  );
};

export default ConfirmDelete;
