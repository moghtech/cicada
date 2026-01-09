import { useRead } from "@/lib/hooks";
import {
  ActionIcon,
  Button,
  Center,
  Divider,
  Flex,
  Loader,
  ScrollArea,
  Text,
  Tree,
  TreeNodeData,
} from "@mantine/core";
import {
  ChevronRight,
  File,
  FolderOpen,
  HardDrive,
  Link2,
} from "lucide-react";
import { Link, useNavigate, useParams } from "react-router-dom";

export const Sidebar = ({ close }: { close: () => void }) => {
  const { filesystem: selected_filesystem, inode: _selected_inode } =
    useParams() as {
      filesystem?: string;
      inode?: string;
    };

  const n_selected_inode = _selected_inode
    ? Number(_selected_inode)
    : undefined;
  const selected_inode = n_selected_inode ? n_selected_inode : undefined;

  const _nav = useNavigate();
  const nav = (to: string) => {
    close();
    _nav(to);
  };

  return (
    <ScrollArea>
      <Flex direction="column" gap="lg" m={16}>
        <Filesystems filesystem={selected_filesystem} close={close} />
        <Divider />
        {selected_filesystem ? (
          <Flex direction="column" gap="sm">
            <Flex gap="sm" opacity={0.7} align="center">
              <FolderOpen size="1rem" />
              <Text>Files</Text>
            </Flex>

            <NodeTree
              filesystem={selected_filesystem}
              parent={1}
              selected={selected_inode}
              nav={nav}
            />
          </Flex>
        ) : (
          <Center>
            <Text>No Filesystem Selected</Text>
          </Center>
        )}
      </Flex>
    </ScrollArea>
  );
};

const Filesystems = ({
  filesystem,
  close,
}: {
  filesystem: string | undefined;
  close: () => void;
}) => {
  const { data: filesystems } = useRead("ListFilesystems", {});

  if (!filesystems) {
    return (
      <Center h="100%">
        <Loader size="lg" />
      </Center>
    );
  }

  return (
    <Flex direction="column" gap="sm">
      <Flex gap="sm" opacity={0.7} align="center">
        <HardDrive size="1rem" />
        <Text>Filesystems</Text>
      </Flex>
      {filesystems.map((fs) => (
        <Button
          key={fs.id}
          variant={fs.id === filesystem ? "default" : "subtle"}
          color="inherit"
          justify="start"
          component={Link}
          to={`/filesystems/${fs.id}`}
          onClick={close}
        >
          {fs.name}
        </Button>
      ))}
    </Flex>
  );
};

/* This value must not  */
const CHILD_VALUE_IDENTIFIER = "__CHILD__";

const NodeTree = ({
  filesystem,
  parent,
  selected,
  nav,
}: {
  filesystem: string;
  parent: number;
  selected: number | undefined;
  nav: (to: string) => void;
}) => {
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
                  selected={selected}
                  nav={nav}
                />
              ),
            },
          ]
        : undefined,
  }));

  return (
    <Tree
      w="100%"
      data={data}
      renderNode={({ node, expanded, hasChildren, elementProps }) => {
        if (node.value === CHILD_VALUE_IDENTIFIER) {
          return <Flex {...elementProps}>{node.label}</Flex>;
        }

        const inode = Number(node.value.split("/")[1]);

        return (
          <Button
            variant={selected && selected === inode ? "default" : "subtle"}
            c="inherit"
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
                    nav("/filesystems/" + node.value);
                  }}
                >
                  <Link2 size="1rem" />
                </ActionIcon>
              )
            }
            {...elementProps}
            onClick={(e) =>
              hasChildren
                ? elementProps.onClick(e)
                : nav("/filesystems/" + node.value)
            }
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
              <File
                size="1rem"
                style={{
                  marginRight: "0.5rem",
                }}
              />
            )}
            {node.label}
          </Button>
        );
      }}
    />
  );
};
