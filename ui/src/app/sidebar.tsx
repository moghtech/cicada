import { useRead } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import {
  ActionIcon,
  Box,
  Button,
  Center,
  Divider,
  Group,
  Loader,
  ScrollArea,
  Stack,
  Text,
  Tree,
  TreeNodeData,
  TreeProps,
} from "@mantine/core";
import { ChevronRight, Link2 } from "lucide-react";
import { Link, useNavigate, useParams } from "react-router-dom";

export default function Sidebar({ close }: { close: () => void }) {
  const accessPage =
    location.pathname.startsWith("/access") ||
    location.pathname.startsWith("/users") ||
    location.pathname.startsWith("/devices") ||
    location.pathname.startsWith("/onboarding-keys") ||
    location.pathname.startsWith("/policies");
  const encryptionPage = location.pathname.startsWith("/encryption-keys");
  const secretPage = location.pathname.startsWith("/secrets");
  const filesystemPage = location.pathname.startsWith("/filesystems");
  const { filesystem: selectedFilesystem, inode: _selectedInode } =
    useParams() as {
      filesystem?: string;
      inode?: string;
    };

  const filesystems = useRead("ListFilesystems", {}).data;

  const nSelectedInode = _selectedInode ? Number(_selectedInode) : undefined;
  const selectedInode = nSelectedInode ? nSelectedInode : undefined;

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
            variant={filesystemPage ? "default" : "subtle"}
            onClick={() => nav("/filesystems")}
            leftSection={<ICONS.Filesystem size="1rem" />}
            justify="flex-start"
            fullWidth
          >
            Filesystems
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
            variant={encryptionPage ? "default" : "subtle"}
            onClick={() => nav("/encryption-keys")}
            leftSection={<ICONS.EncryptionKey size="1rem" />}
            justify="flex-start"
            fullWidth
          >
            Encryption
          </Button>
          <Button
            variant={accessPage ? "default" : "subtle"}
            onClick={() => nav("/access")}
            leftSection={<ICONS.Access size="1rem" />}
            justify="flex-start"
            fullWidth
          >
            Access
          </Button>

          <Divider
            label={
              <Group gap="sm" opacity={0.7} wrap="nowrap">
                <ICONS.Folder size="1rem" />
                <Text>Filesystems</Text>
              </Group>
            }
          />

          {filesystems?.map((filesystem) => (
            <FilesystemTree
              key={filesystem.id}
              filesystem={filesystem.id}
              selectedFilesystem={selectedFilesystem}
              selectedInode={selectedInode}
            />
          ))}
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

/* This value must not be a node path */
const CHILD_VALUE_IDENTIFIER = "__CHILD__";

function FilesystemTree({
  filesystem: _filesystem,
  selectedFilesystem,
  selectedInode: _selectedInode,
}: {
  filesystem: string;
  selectedFilesystem: string | undefined;
  selectedInode: number | undefined;
}) {
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
