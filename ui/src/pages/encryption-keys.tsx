import {
  DataTable,
  filterBySplit,
  Page,
  SearchInput,
  SortableHeader,
} from "mogh_ui";
import InitializeEncryptionKey from "@/components/initialize-encryption-key";
import CreateEncryptionKey from "@/create/encryption-key";
import { useInvalidate, useRead, useSetTitle, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { Badge, List, Text } from "@mantine/core";
import ResourceLink from "@/components/resource-link";
import { useMemo, useState } from "react";
import { RowSelectionState } from "@tanstack/react-table";
import { notifications } from "@mantine/notifications";
import ConfirmDelete from "@/components/confirm-delete";

const EncryptionKeysPage = () => {
  useSetTitle("Encryption");

  const inv = useInvalidate();

  const { data } = useRead("ListEncryptionKeys", {});
  const byId = useMemo(
    () => data && Object.fromEntries(data.map((ok) => [ok.id, ok.name])),
    [data],
  );

  const [selected, setSelected] = useState<RowSelectionState>({});
  const selectedIds = useMemo(() => Object.keys(selected), [selected]);

  const { mutateAsync: batchDelete } = useWrite("BatchDeleteEncryptionKeys", {
    onSuccess: (deleted) => {
      notifications.show({
        message: `Deleted ${deleted.length} encryption key${deleted.length === 1 ? "" : "s"}.`,
      });
      inv(["ListEncryptionKeys"]);
      setSelected({});
    },
  });

  const [search, setSearch] = useState("");
  const encryptionKeys = filterBySplit(
    data,
    search,
    (encryptionKey) => encryptionKey.name,
  );

  return (
    <Page
      title="Encryption Keys"
      icon={ICONS.EncryptionKey}
      actions={
        <>
          <CreateEncryptionKey />
          <ConfirmDelete
            name=""
            entityType={
              "Encryption Key" + (selectedIds.length === 1 ? "" : "s")
            }
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
        tableKey="encryption-keys-table-v1"
        data={encryptionKeys}
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
              <ResourceLink type="EncryptionKey" id={row.original.id} />
            ),
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Kind" />
            ),
            accessorKey: "kind",
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Status" />
            ),
            accessorKey: "initialized",
            cell: ({ row }) =>
              row.original.initialized ? (
                <Badge color="green.8">Ready</Badge>
              ) : (
                <InitializeEncryptionKey
                  key_id={row.original.id}
                  target={({ onClick }) => (
                    <Badge
                      color="red"
                      onClick={(e) => {
                        e.stopPropagation();
                        onClick?.();
                      }}
                      style={{ cursor: "pointer" }}
                    >
                      Uninitialized
                    </Badge>
                  )}
                />
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

export default EncryptionKeysPage;
