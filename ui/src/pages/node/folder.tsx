import ConfirmDelete from "@/components/confirm-delete";
import { DataTable, SortableHeader } from "@/components/data-table";
import CreateNode from "@/create/node";
import { Page } from "@/layout/page";
import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { Button, Flex, List, Text } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { RowSelectionState } from "@tanstack/react-table";
import { Types } from "cicada_client";
import { useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";

const FolderPage = ({
  filesystem,
  node,
}: {
  filesystem: Types.FilesystemRecord | undefined;
  node: Types.NodeRecord | undefined;
}) => {
  const nav = useNavigate();
  const inv = useInvalidate();
  const { mutateAsync: deleteFs, isPending: deleteFsPending } = useWrite(
    "DeleteFilesystem",
    {
      onSuccess: () => {
        inv(["ListFilesystems"], ["ListNodes"]);
        nav("/");
      },
    }
  );
  const children =
    useRead("ListNodes", {
      filesystem: filesystem?.id,
      parent: node?.inode ?? 1,
    }).data ?? [];
  const byId = useMemo(
    () =>
      children && Object.fromEntries(children.map((node) => [node.id, node])),
    [children]
  );
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
      });
      inv(["ListNodes"]);
      setSelected({});
    },
  });

  return (
    <Page
      title={node?.name ?? "Root"}
      icon={ICONS.Folder}
      customTitle={
        <>
          <ICONS.Folder size={22} opacity={0.6} />
          <Text fz="h2" opacity={0.6}>
            Folder:
          </Text>
          <Text fz="h2">{node?.name ?? "Root"}</Text>
        </>
      }
      customDescription={
        <>
          <ICONS.Filesystem size="1.1rem" opacity={0.6} />
          <Text opacity={0.6} size="lg">
            Filesystem:
          </Text>
          <Text size="lg">{filesystem?.name}</Text>
        </>
      }
      actions={
        <>
          {Object.values(Types.NodeKind).map((kind) => (
            <CreateNode key={kind} kind={kind} parent={node?.inode ?? 1} />
          ))}
          {!selectedIds.length && node === undefined && filesystem && (
            <ConfirmDelete
              entityType="Filesystem"
              name={filesystem.name}
              onConfirm={() => deleteFs({ id: filesystem.id })}
              loading={deleteFsPending}
              disabled={false}
            />
          )}
          {!selectedIds.length && node && (
            <ConfirmDelete
              entityType="Folder"
              name={node.name}
              onConfirm={() => deleteFolder({ id: node.id })}
              loading={deleteFolderPending}
              disabled={false}
            />
          )}
          {!!selectedIds.length && (
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
          )}
        </>
      }
    >
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
              <SortableHeader column={column} title="Id" />
            ),
            accessorKey: "id",
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Kind" />
            ),
            accessorKey: "kind",
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
    </Page>
  );
};

export default FolderPage;
