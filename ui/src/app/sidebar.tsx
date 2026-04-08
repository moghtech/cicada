import { useRead } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import {
  ActionIcon,
  Box,
  Button,
  Center,
  Divider,
  Group,
  ScrollArea,
  Stack,
  Text,
  Tree,
  TreeNodeData,
} from "@mantine/core";
import { ChevronRight, Link2, PointerOff } from "lucide-react";
import { useNavigate, useParams } from "react-router-dom";

export default function Sidebar({ close }: { close: () => void }) {
  const devicesPage =
    location.pathname.startsWith("/devices") ||
    location.pathname.startsWith("/onboarding-keys");
  const encryptionPage = location.pathname.startsWith("/encryption-keys");
  const secretPage = location.pathname.startsWith("/secrets");
  const filesystemPage = location.pathname.startsWith("/filesystems");
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
    <Stack justify="space-between" gap="md" h="96%" m="xl" mt="24" mr="md">
      {/* TOP AREA (scrolling) */}
      <ScrollArea>
        <Stack gap="0.15rem" mr="md">
          <Button
            variant={devicesPage ? "default" : "subtle"}
            onClick={() => nav("/devices")}
            leftSection={<ICONS.Device size="1rem" />}
            justify="flex-start"
            fullWidth
          >
            Devices
          </Button>
          <Button
            variant={encryptionPage ? "default" : "subtle"}
            onClick={() => nav("/encryption-keys")}
            leftSection={<ICONS.EncryptionKey size="1rem" />}
            justify="flex-start"
            fullWidth
          >
            Encryption
          </Button>
          <Button
            variant={secretPage ? "default" : "subtle"}
            onClick={() => nav("/secrets")}
            leftSection={<ICONS.Secret size="1rem" />}
            justify="flex-start"
            fullWidth
          >
            Secrets
          </Button>
          <Button
            variant={filesystemPage ? "default" : "subtle"}
            onClick={() => nav("/filesystems")}
            leftSection={<ICONS.Filesystem size="1rem" />}
            justify="flex-start"
            fullWidth
          >
            Filesystems
          </Button>

          <Divider
            label={
              <Group gap="sm" opacity={0.7} wrap="nowrap">
                <ICONS.Folder size="1rem" />
                <Text>Files</Text>
              </Group>
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
        </Stack>
      </ScrollArea>

      {/* BOTTOM AREA */}
      <Stack gap="lg">
        {/* <Button
          onClick={() => nav("/devices")}
          leftSection={<Server size="1rem" />}
          style={{ justifySelf: "flex-end" }}
          fullWidth
        >
          Devices
        </Button> */}
      </Stack>
    </Stack>
  );
}

/* This value must not  */
const CHILD_VALUE_IDENTIFIER = "__CHILD__";

function NodeTree({
  filesystem,
  parent,
  selected,
  nav,
}: {
  filesystem: string;
  parent: number;
  selected: number | undefined;
  nav: (to: string) => void;
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
          return <Box {...elementProps}>{node.label}</Box>;
        }

        const inode = Number(node.value.split("/")[1]);

        return (
          <Button
            variant={selected && selected === inode ? "default" : "subtle"}
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
}
