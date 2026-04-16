import FilesystemTree from "@/components/filesystem-tree";
import { useRead } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { Button, Divider, Group, ScrollArea, Stack, Text } from "@mantine/core";
import { ReactNode } from "react";
import { Link, useParams } from "react-router-dom";

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

  return (
    <Stack justify="space-between" gap="md" h="96%" m="xl" mt="24" mr="md">
      {/* TOP AREA (scrolling) */}
      <ScrollArea>
        <Stack gap="0.15rem" mr="md">
          <SidebarLink
            selected={filesystemPage}
            to="/filesystems"
            icon={ICONS.Filesystem}
            close={close}
          >
            Filesystems
          </SidebarLink>
          <SidebarLink
            selected={secretPage}
            to="/secrets"
            icon={ICONS.Secret}
            close={close}
          >
            Secrets
          </SidebarLink>
          <SidebarLink
            selected={encryptionPage}
            to="/encryption-keys"
            icon={ICONS.EncryptionKey}
            close={close}
          >
            Encryption
          </SidebarLink>
          <SidebarLink
            selected={accessPage}
            to="/access"
            icon={ICONS.Access}
            close={close}
          >
            Access
          </SidebarLink>

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

function SidebarLink({
  to,
  children,
  selected,
  icon: Icon,
  close,
}: {
  to: string;
  children: ReactNode;
  selected: boolean;
  icon: (props: { size: string }) => ReactNode;
  close: () => void;
}) {
  return (
    <Button
      component={Link}
      to={to}
      onClick={close}
      variant={selected ? "default" : "subtle"}
      leftSection={<Icon size="1rem" />}
      className="hover-underline"
      justify="flex-start"
      fullWidth
    >
      {children}
    </Button>
  );
}
