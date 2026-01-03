import { useRead } from "@/lib/hooks";
import { Center, Flex, Group, Loader, Tree, TreeNodeData } from "@mantine/core";
import { ChevronRight, File } from "lucide-react";
import { useNavigate } from "react-router-dom";

export const Sidebar = () => {
  const { data: filesystems } = useRead("ListFilesystems", {});

  if (!filesystems) {
    return (
      <Center h="100%">
        <Loader size="lg" />
      </Center>
    );
  }

  const data: TreeNodeData[] = filesystems.map((fs) => ({
    value: fs.id,
    label: fs.name,
    children: [
      { value: "children", label: <NodeTree filesystem={fs.id} parent={1} /> },
    ],
  }));

  return (
    <Flex m={16}>
      <Tree
        data={data}
        renderNode={({ node, expanded, hasChildren, elementProps }) => (
          <Group gap={5} {...elementProps}>
            {hasChildren && (
              <ChevronRight
                size={18}
                style={{
                  transitionProperty: "transform",
                  transform: expanded ? "rotate(90deg)" : "rotate(0deg)",
                }}
              />
            )}

            <span>{node.label}</span>
          </Group>
        )}
      />
    </Flex>
  );
};

/* This value must not  */
const CHILD_VALUE_IDENTIFIER = "__CHILD__";

const NodeTree = ({
  filesystem,
  parent,
}: {
  filesystem: string;
  parent: number;
}) => {
  const nav = useNavigate();
  const nodes = useRead("ListNodes", { filesystem, parent }).data ?? [];

  const data: TreeNodeData[] = nodes.map((node) => ({
    value: node.id,
    label: node.name,
    children:
      node.kind === "Folder"
        ? [
            {
              value: CHILD_VALUE_IDENTIFIER,
              label: (
                <NodeTree filesystem={node.filesystem} parent={node.ino} />
              ),
            },
          ]
        : undefined,
  }));

  return (
    <Tree
      data={data}
      renderNode={({ node, expanded, hasChildren, elementProps }) => (
        <Group
          gap={5}
          {...elementProps}
          onClick={(e) =>
            node.value === CHILD_VALUE_IDENTIFIER
              ? undefined
              : hasChildren
                ? elementProps.onClick(e)
                : nav("/files/" + node.value)
          }
        >
          {node.value !== CHILD_VALUE_IDENTIFIER &&
            (hasChildren ? (
              <ChevronRight
                size={18}
                style={{
                  transitionProperty: "transform",
                  transform: expanded ? "rotate(90deg)" : "rotate(0deg)",
                }}
              />
            ) : (
              <File size={18} />
            ))}

          <span>{node.label}</span>
        </Group>
      )}
    />
  );
};
