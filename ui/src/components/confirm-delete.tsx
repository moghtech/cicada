import { Button, Flex, Modal } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { Trash2 } from "lucide-react";

const ConfirmDelete = ({
  entityType,
  name,
  onConfirm,
  loading,
  disabled,
}: {
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
        title={`Delete${formatted_et} ${name}`}
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
            fullWidth
          >
            Delete{formatted_et}
          </Button>
        </Flex>
      </Modal>
      <Button variant="filled" color="red" disabled={disabled} onClick={open}>
        <Flex align="center" gap="0.5rem">
          <Trash2 size="1rem" />
          Delete{formatted_et}
        </Flex>
      </Button>
    </>
  );
};

export default ConfirmDelete;
