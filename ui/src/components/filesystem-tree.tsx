import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import {
  ActionIcon,
  Box,
  Button,
  Center,
  Loader,
  Tree,
  TreeNodeData,
  TreeProps,
} from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { ChevronRight, Link2 } from "lucide-react";
import { DragEvent } from "react";
import { Link } from "react-router-dom";

/* This value must not be a node path */
const CHILD_VALUE_IDENTIFIER = "__CHILD__";

export interface FilesystemTreeProps {
  filesystem: string;
  selectedFilesystem: string | undefined;
  selectedInode: number | undefined;
}

export default function FilesystemTree({
  filesystem: _filesystem,
  selectedFilesystem,
  selectedInode: _selectedInode,
}: FilesystemTreeProps) {
  const filesystem = useRead("ListFilesystems", {}).data?.find(
    (fs) => fs.id === _filesystem,
  );

  if (!filesystem) {
    return (
      <Center>
        <Loader />
      </Center>
    );
  }

  const selectedInode =
    selectedFilesystem === filesystem.id ? _selectedInode : 0;

  const data: TreeNodeData = {
    value: filesystem.id,
    label: filesystem.name,
    children: [
      {
        value: CHILD_VALUE_IDENTIFIER,
        label: (
          <NodeTree
            filesystem={filesystem.id}
            parent={1}
            selectedInode={selectedInode}
          />
        ),
      },
    ],
  };

  return <Tree w="100%" data={[data]} renderNode={renderNode(selectedInode)} />;
}

function NodeTree({
  filesystem,
  parent,
  selectedInode,
}: {
  filesystem: string;
  parent: number;
  selectedInode: number | undefined;
}) {
  const nodes = useRead("ListNodes", { filesystem, parent }).data ?? [];

  const data: TreeNodeData[] = nodes.map((node) => ({
    value: `${node.filesystem}/${node.inode}/${node.id}`,
    label: node.name,
    children:
      node.kind === "Folder"
        ? [
            {
              value: CHILD_VALUE_IDENTIFIER,
              label: (
                <NodeTree
                  filesystem={node.filesystem}
                  parent={node.inode}
                  selectedInode={selectedInode}
                />
              ),
            },
          ]
        : undefined,
  }));

  return <Tree w="100%" data={data} renderNode={renderNode(selectedInode)} />;
}

const renderNode: (
  selectedInode: number | undefined,
) => TreeProps["renderNode"] =
  (selectedInode) =>
  ({ node, expanded, hasChildren, elementProps }) => {
    const inv = useInvalidate();
    const { mutateAsync: moveNode } = useWrite("MoveNode", {
      onSuccess: () => {
        notifications.show({ message: "Moved node.", color: "green" });
        inv(["ListNodes"]);
      },
    });

    if (node.value === CHILD_VALUE_IDENTIFIER) {
      return <Box {...elementProps}>{node.label}</Box>;
    }

    const parts = node.value.split("/");
    const linkPath = `${parts[0]}${parts[1] ? "/" + parts[1] : ""}`;
    const filesystem = parts[0];
    const inode = Number(parts[1]) || 1;
    const nodeId = parts[2] as string | undefined;

    return (
      <Button
        draggable={!!nodeId}
        onDragStart={(e: DragEvent<HTMLButtonElement>) => {
          if (!nodeId) return;
          e.dataTransfer.setData("application/node-id", nodeId);
          e.dataTransfer.effectAllowed = "move";
        }}
        onDragOver={(e: DragEvent<HTMLButtonElement>) => {
          if (!hasChildren) return;
          e.preventDefault();
          e.dataTransfer.dropEffect = "move";
          e.currentTarget.style.outline =
            "2px solid var(--mantine-color-blue-5)";
          e.currentTarget.style.borderRadius = "var(--mantine-radius-default)";
        }}
        onDragLeave={(e: DragEvent<HTMLButtonElement>) => {
          e.currentTarget.style.outline = "";
          e.currentTarget.style.borderRadius = "";
        }}
        onDrop={async (e: DragEvent<HTMLButtonElement>) => {
          e.preventDefault();
          e.currentTarget.style.outline = "";
          e.currentTarget.style.borderRadius = "";
          const draggedNodeId = e.dataTransfer.getData("application/node-id");
          if (!draggedNodeId || !inode) return;
          if (draggedNodeId === nodeId) return;
          await moveNode({ id: draggedNodeId, filesystem, parent: inode });
        }}
        variant={selectedInode === inode ? "default" : "subtle"}
        p="0rem 0.5rem"
        mb="0.25rem"
        justify="space-between"
        fullWidth
        rightSection={
          hasChildren && (
            <ActionIcon
              component="div"
              variant="subtle"
              onClick={(e) => {
                e.stopPropagation();
              }}
              renderRoot={(props) => (
                <Link to={"/filesystems/" + linkPath} {...props} />
              )}
            >
              <Link2 size="1rem" />
            </ActionIcon>
          )
        }
        {...elementProps}
        renderRoot={
          hasChildren
            ? undefined
            : (props) => <Link to={"/filesystems/" + linkPath} {...props} />
        }
        onClick={(e) => hasChildren && elementProps.onClick(e)}
      >
        {hasChildren ? (
          <ChevronRight
            size="1rem"
            style={{
              transitionProperty: "transform",
              transform: expanded ? "rotate(90deg)" : "rotate(0deg)",
              marginRight: "0.5rem",
            }}
          />
        ) : (
          <ICONS.File
            size="1rem"
            style={{
              marginRight: "0.5rem",
            }}
          />
        )}
        {node.label}
      </Button>
    );
  };
