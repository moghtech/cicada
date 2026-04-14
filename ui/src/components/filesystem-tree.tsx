import { useRead } from "@/lib/hooks";
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
import { ChevronRight, Link2 } from "lucide-react";
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
    value: `${node.filesystem}/${node.inode}`,
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
    if (node.value === CHILD_VALUE_IDENTIFIER) {
      return <Box {...elementProps}>{node.label}</Box>;
    }

    const inode = Number(node.value.split("/")[1]) || undefined;

    return (
      <Button
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
                <Link to={"/filesystems/" + node.value} {...props} />
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
            : (props) => <Link to={"/filesystems/" + node.value} {...props} />
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
