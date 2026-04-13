import {
  DataTable,
  filterBySplit,
  Page,
  SearchInput,
  SortableHeader,
} from "mogh_ui";
import CreateFilesystem from "@/create/filesystem";
import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import ResourceLink from "@/components/resource-link";
import { useMemo, useState } from "react";
import ConfirmDelete from "@/components/confirm-delete";
import { notifications } from "@mantine/notifications";
import { RowSelectionState } from "@tanstack/react-table";
import { List, Text } from "@mantine/core";

const FilesystemsPage = () => {
  const inv = useInvalidate();

  const { data } = useRead("ListFilesystems", {});
  const byId = useMemo(
    () => data && Object.fromEntries(data.map((ok) => [ok.id, ok.name])),
    [data],
  );

  const [selected, setSelected] = useState<RowSelectionState>({});
  const selectedIds = useMemo(() => Object.keys(selected), [selected]);

  const { mutateAsync: batchDelete } = useWrite("BatchDeleteFilesystems", {
    onSuccess: (deleted) => {
      notifications.show({
        message: `Deleted ${deleted.length} filesystem${deleted.length === 1 ? "" : "s"}.`,
      });
      inv(["ListFilesystems"]);
      setSelected({});
    },
  });

  const [search, setSearch] = useState("");
  const filesystems = filterBySplit(
    data,
    search,
    (filesystem) => filesystem.name,
  );

  return (
    <Page
      title="Filesystems"
      icon={ICONS.Filesystem}
      actions={
        <>
          <CreateFilesystem />
          <ConfirmDelete
            name=""
            entityType={"Filesystem" + (selectedIds.length === 1 ? "" : "s")}
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
                  {selectedIds.map((id) => (
                    <List.Item key={id}>{byId?.[id]}</List.Item>
                  ))}
                </List>
              </>
            }
          />
          <SearchInput value={search} onSearch={setSearch} />
        </>
      }
    >
      <DataTable
        tableKey="filesystems-table-v1"
        data={filesystems}
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
            cell: ({ row }) => (
              <ResourceLink type="Filesystem" id={row.original.id} />
            ),
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Encryption Key" />
            ),
            accessorKey: "encryption_key",
            cell: ({ row }) =>
              row.original.encryption_key ? (
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
    </Page>
  );
};

export default FilesystemsPage;
