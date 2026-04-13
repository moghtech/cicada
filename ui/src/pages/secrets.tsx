import {
  DataTable,
  filterBySplit,
  Page,
  SearchInput,
  SharedTextUpdate,
  SortableHeader,
  useSharedTextUpdateData,
} from "mogh_ui";
import CreateSecret from "@/create/secret";
import EditSecretModal from "@/components/edit-secret-modal";
import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { Button, List, Text } from "@mantine/core";
import { useMemo, useState } from "react";
import { RowSelectionState } from "@tanstack/react-table";
import ConfirmDelete from "@/components/confirm-delete";
import { notifications } from "@mantine/notifications";
import ResourceLink from "@/components/resource-link";

export default function SecretsPage() {
  const inv = useInvalidate();

  const { data } = useRead("ListSecrets", {});
  const byId = useMemo(
    () => data && Object.fromEntries(data.map((ok) => [ok.id, ok.name])),
    [data],
  );

  const [selected, setSelected] = useState<RowSelectionState>({});
  const selectedIds = useMemo(() => Object.keys(selected), [selected]);

  const { mutate: updateSecret } = useWrite("UpdateSecret", {
    onSuccess: (secret) => {
      notifications.show({ message: "Updated secret.", color: "green" });
      inv(["ListSecrets"], ["GetSecret", { id: secret.id }]);
    },
  });

  const { mutateAsync: batchDelete } = useWrite("BatchDeleteSecrets", {
    onSuccess: (deleted) => {
      notifications.show({
        message: `Deleted ${deleted.length} filesystem secret${deleted.length === 1 ? "" : "s"}.`,
      });
      inv(["ListSecrets"]);
      setSelected({});
    },
  });

  const [search, setSearch] = useState("");
  const secrets = filterBySplit(data, search, (secret) => secret.name);
  const [updateMenuData, setUpdateMenuData] = useSharedTextUpdateData();
  
  return (
    <Page
      title="Secrets"
      icon={ICONS.Secret}
      actions={
        <>
          <CreateSecret />
          <ConfirmDelete
            name=""
            entityType="Secrets"
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
        tableKey="secrets-table-v1"
        data={secrets}
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
              <ResourceLink type="Secret" id={row.original.id} />
            ),
          },
          {
            header: "Data",
            accessorKey: "id",
            cell: ({ row }) => (
              <EditSecretModal id={row.original.id} name={row.original.name} />
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
            header: "Description",
            accessorKey: "description",
            cell: ({ row }) => {
              return (
                <Button
                  className="text-ellipsis"
                  onClick={() => {
                    setUpdateMenuData({
                      title: `${row.original.name} - Description`,
                      value: row.original.description ?? "",
                      placeholder: "Input description...",
                      onUpdate: (description) => {
                        if (row.original.description === description) {
                          return;
                        }
                        updateSecret({
                          id: row.original.id,
                          description,
                        });
                      },
                    });
                  }}
                  w={{ base: 200, lg: 300 }}
                  justify="start"
                >
                  {row.original.description || (
                    <Text c="dimmed">Add a description</Text>
                  )}
                </Button>
              );
            },
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

      <SharedTextUpdate data={updateMenuData} setData={setUpdateMenuData} />
    </Page>
  );
}
