import ConfirmDelete from "@/components/confirm-delete";
import { DataTable, EntityHeader, EntityPage, SortableHeader } from "mogh_ui";
import CreateNode from "@/create/node";
import { useInvalidate, useRead, useSetTitle, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { Button, Flex, Group, List, Text } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { RowSelectionState } from "@tanstack/react-table";
import { Types } from "cicada_client";
import { useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";
import InterpolationModeSelector from "@/components/interpolation-mode-selector";
import EncryptionKeySelector from "@/components/encryption-key-selector";
import ResourceLink from "@/components/resource-link";
import CheckpointingModeSelector from "@/components/checkpointing-mode-selector";

const FolderPage = ({
  filesystem,
  node,
}: {
  filesystem: Types.FilesystemRecord | undefined;
  node: Types.NodeEntity | undefined;
}) => {
  useSetTitle((node ? node?.name + " | " : "") + filesystem?.name);

  const nav = useNavigate();
  const inv = useInvalidate();
  const { mutateAsync: deleteFilesystem, isPending: deleteFilesystemPending } =
    useWrite("DeleteFilesystem", {
      onSuccess: () => {
        inv(["ListFilesystems"], ["ListNodes"]);
        notifications.show({ message: "Deleted filesystem.", color: "green" });
        nav("/");
      },
    });
  const { mutate: updateFilesystem } = useWrite("UpdateFilesystem", {
    onSuccess: () => {
      inv(["ListFilesystems"]);
      notifications.show({ message: "Updated filesystem.", color: "green" });
    },
  });

  const children =
    useRead(
      "ListNodes",
      {
        filesystem: filesystem?.id!,
        parent: node?.inode ?? 1,
      },
      { enabled: !!filesystem?.id },
    ).data ?? [];
  const byId = useMemo(
    () =>
      children && Object.fromEntries(children.map((node) => [node.id, node])),
    [children],
  );
  const { mutateAsync: updateNode } = useWrite("UpdateNode", {
    onSuccess: () => inv(["ListNodes"], ["FindNode"]),
  });
  const { mutateAsync: deleteFolder, isPending: deleteFolderPending } =
    useWrite("DeleteNode", {
      onSuccess: () => {
        inv(["ListNodes"]);
        nav(`/filesystems/${node?.filesystem}/${node?.parent ?? 1}`);
      },
    });

  const [selected, setSelected] = useState<RowSelectionState>({});
  const selectedIds = useMemo(() => Object.keys(selected), [selected]);

  const { mutateAsync: batchDelete } = useWrite("BatchDeleteNodes", {
    onSuccess: (deleted) => {
      notifications.show({
        message: `Deleted ${deleted.length} filesystem node${deleted.length === 1 ? "" : "s"}.`,
        color: "green",
      });
      inv(["ListNodes"]);
      setSelected({});
    },
  });

  return (
    <EntityPage
      backTo={
        node
          ? `/filesystems/${node.filesystem}${node.parent === 1 ? "" : "/" + node.parent}`
          : "/filesystems"
      }
    >
      <EntityHeader
        name={node?.name ?? filesystem?.name}
        state={node ? "Folder" : "Filesystem"}
        icon={node ? ICONS.Folder : ICONS.Filesystem}
        intent="Good"
        onRename={async (name) =>
          node
            ? await updateNode({ id: node?.id, name })
            : filesystem &&
              (await updateFilesystem({ id: filesystem?.id, name }))
        }
        action={
          <ConfirmDelete
            entityType={node ? "Folder" : "Filesystem"}
            name={node ? node.name : (filesystem?.name ?? "")}
            onConfirm={async () =>
              node
                ? deleteFolder({ id: node.id })
                : filesystem && deleteFilesystem({ id: filesystem.id })
            }
            loading={node ? deleteFolderPending : deleteFilesystemPending}
            disabled={false}
            iconOnly
          />
        }
      />
      <Group>
        {filesystem &&
          Object.values(Types.NodeKind).map((kind) => (
            <CreateNode
              key={kind}
              filesystem={filesystem.id}
              kind={kind}
              parent={node?.inode ?? 1}
            />
          ))}
        <ConfirmDelete
          name=""
          entityType="Files"
          onConfirm={async () => {
            if (selectedIds.length) {
              await batchDelete({ ids: selectedIds });
            }
          }}
          disabled={!selectedIds.length}
          info={
            <>
              <Text fw="bold" fz="lg">
                To Delete:
              </Text>
              <List>
                {selectedIds.map((id) => {
                  const Icon = byId?.[id] && ICONS[byId?.[id].kind];
                  return (
                    <List.Item key={id}>
                      <Flex align="center" gap="0.4rem">
                        <Icon size="1rem" />
                        <Text fw="bold">{byId?.[id]?.name}</Text>
                        <Text opacity={0.6}>({byId?.[id].kind})</Text>
                      </Flex>
                    </List.Item>
                  );
                })}
              </List>
            </>
          }
        />
        {node === undefined && filesystem && (
          <>
            <EncryptionKeySelector
              selected={filesystem.encryption_key}
              onSelect={(encryption_key) =>
                updateFilesystem({
                  id: filesystem.id,
                  encryption_key,
                })
              }
              targetProps={{
                w: { base: "100%", xs: 260 },
              }}
            />
            <CheckpointingModeSelector
              value={filesystem.checkpointing}
              onChange={(checkpointing) =>
                updateFilesystem({ id: filesystem.id, checkpointing })
              }
              excludeInherit
            />
            <InterpolationModeSelector
              value={filesystem.interpolation}
              onChange={(interpolation) =>
                updateFilesystem({ id: filesystem.id, interpolation })
              }
              excludeInherit
            />
          </>
        )}
      </Group>

      <DataTable
        tableKey="filesystem-table-v1"
        data={children}
        onRowClick={(node) =>
          nav(`/filesystems/${filesystem?.id}/${node.inode}`)
        }
        selectOptions={{
          selectKey: (row) => row.id,
          state: [selected, setSelected],
        }}
        columns={[
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Name" />
            ),
            accessorKey: "name",
            cell: ({ row: { original: node } }) => {
              const Icon = ICONS[node.kind];
              return (
                <Button
                  variant="transparent"
                  c="inherit"
                  leftSection={<Icon size="1rem" />}
                >
                  {node.name}
                </Button>
              );
            },
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Kind" />
            ),
            accessorKey: "kind",
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Encryption Key" />
            ),
            accessorKey: "encryption_key",
            cell: ({ row }) =>
              row.original.kind === Types.NodeKind.Folder ? (
                ""
              ) : row.original.encryption_key ? (
                <ResourceLink
                  type="EncryptionKey"
                  id={row.original.encryption_key}
                />
              ) : (
                ""
              ),
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Created At" />
            ),
            accessorKey: "created_at",
            cell: ({ row }) =>
              new Date(row.original.created_at).toLocaleString(),
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Updated At" />
            ),
            accessorKey: "updated_at",
            cell: ({ row }) =>
              new Date(row.original.updated_at).toLocaleString(),
          },
        ]}
      />
    </EntityPage>
  );
};

export default FolderPage;
