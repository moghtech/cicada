import CreateFilesystem from "@/create/filesystem";
import { useRead } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
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
import { ChevronRight, Link2, PointerOff } from "lucide-react";
import { Link, useNavigate, useParams } from "react-router-dom";

export const Sidebar = ({ close }: { close: () => void }) => {
  const devicesPage =
    location.pathname.startsWith("/devices") ||
    location.pathname.startsWith("/onboarding-keys");
  const encryptionPage = location.pathname.startsWith("/encryption-keys");
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
    <Flex direction="column" justify="space-between" gap="lg" h="100%" m={16}>
      {/* TOP AREA (scrolling) */}
      <ScrollArea>
        <Flex direction="column" gap="0.5rem">
          <Button
            variant={devicesPage ? "filled" : "subtle"}
            c="inherit"
            onClick={() => nav("/devices")}
            leftSection={<ICONS.Device size="1rem" />}
            justify="flex-start"
            fullWidth
          >
            Devices
          </Button>
          <Button
            variant={encryptionPage ? "filled" : "subtle"}
            c="inherit"
            onClick={() => nav("/encryption-keys")}
            leftSection={<ICONS.EncryptionKey size="1rem" />}
            justify="flex-start"
            fullWidth
          >
            Encryption
          </Button>

          <Filesystems filesystem={selected_filesystem} close={close} />
          <Divider
            label={
              <Flex gap="sm" opacity={0.7} align="center">
                <ICONS.Folder size="1rem" />
                <Text>Files</Text>
              </Flex>
            }
          />
          {selected_filesystem ? (
            <NodeTree
              filesystem={selected_filesystem}
              parent={1}
              selected={selected_inode}
              nav={nav}
            />
          ) : (
            <Center opacity={0.6} mt="xs">
              <PointerOff size="1rem" />
              <Text ml="xs">No Filesystem Selected</Text>
            </Center>
          )}
        </Flex>
      </ScrollArea>

      {/* BOTTOM AREA */}
      <Flex direction="column" gap="lg">
        {/* <Button
          onClick={() => nav("/devices")}
          leftSection={<Server size="1rem" />}
          style={{ justifySelf: "flex-end" }}
          fullWidth
        >
          Devices
        </Button> */}
      </Flex>
    </Flex>
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
    <>
      <Divider
        label={
          <Flex gap="sm" opacity={0.7} align="center">
            <ICONS.Filesystem size="1rem" />
            <Text>Filesystems</Text>
          </Flex>
        }
      />
      {filesystems.map((fs) => (
        <Button
          key={fs.id}
          variant={fs.id === filesystem ? "filled" : "subtle"}
          c="inherit"
          justify="start"
          component={Link}
          to={`/filesystems/${fs.id}`}
          onClick={close}
          leftSection={<ICONS.Filesystem size="1rem" />}
        >
          {fs.name}
        </Button>
      ))}
      {!filesystems.length && <CreateFilesystem />}
    </>
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
      }}
    />
  );
};
