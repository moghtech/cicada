import FilesystemTree from "@/components/filesystem-tree";
import { useRead } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { Button, Divider, Group, ScrollArea, Stack, Text } from "@mantine/core";
import { useNavigate, useParams } from "react-router-dom";

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
