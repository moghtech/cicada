import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { Center, Group, Loader } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { Types } from "cicada_client";
import { hexColorByIntention } from "mogh_ui";
import { Link } from "react-router-dom";

export interface NodeLinkProps {
  node: Types.NodeListItem | undefined;
}

export default function NodeLink({ node }: NodeLinkProps) {
  const encryptionKeys = useRead("ListEncryptionKeys", {}).data;

  const inv = useInvalidate();
  const { mutateAsync: moveNode } = useWrite("MoveNode", {
    onSuccess: () => {
      notifications.show({ message: "Moved node.", color: "green" });
      inv(["ListNodes"]);
    },
  });

  if (!node) {
    return (
      <Center>
        <Loader />
      </Center>
    );
  }

  const Icon = ICONS[node.kind];
  const intention =
    node.kind === Types.NodeKind.File
      ? !encryptionKeys?.find((e) => e.id === node?.encryption_key)?.initialized
        ? "Critical"
        : "Good"
      : "None";

  return (
    <Group
      draggable={true}
      onDragStart={(e) => {
        e.dataTransfer.setData("application/node-id", node.id);
        e.dataTransfer.effectAllowed = "move";
      }}
      onDragOver={(e) => {
        if (node.kind === Types.NodeKind.File) return;
        const draggedNodeId = e.dataTransfer.getData("application/node-id");
        if (!draggedNodeId || node.id === draggedNodeId) return;
        e.preventDefault();
        e.dataTransfer.dropEffect = "move";
        e.currentTarget.style.outline = "2px solid var(--mantine-color-blue-5)";
      }}
      onDragLeave={(e) => {
        if (node.kind === Types.NodeKind.File) return;
        e.currentTarget.style.outline = "";
      }}
      onDrop={async (e) => {
        if (node.kind === Types.NodeKind.File) return;
        const draggedNodeId = e.dataTransfer.getData("application/node-id");
        if (!draggedNodeId || node.id === draggedNodeId) return;
        e.preventDefault();
        e.currentTarget.style.outline = "";
        e.currentTarget.style.borderRadius = "";
        await moveNode({
          id: draggedNodeId,
          filesystem: node.filesystem,
          parent: node.inode,
        });
      }}
      renderRoot={(props) => (
        <Link to={`/filesystems/${node.filesystem}/${node.inode}`} {...props} />
      )}
      gap="xs"
      wrap="nowrap"
      w="fit-content"
      className="hover-underline"
      onClick={(e) => e.stopPropagation()}
      px="12"
      py="6"
      bdrs="sm"
    >
      <Icon size="1rem" color={hexColorByIntention(intention)} />
      {node?.name ?? "Unknown"}
    </Group>
  );
}
