import {
  DataTable,
  EnableSwitch,
  filterBySplit,
  SearchInput,
  SortableHeader,
} from "mogh_ui";
import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { useMemo, useState } from "react";
import { RowSelectionState } from "@tanstack/react-table";
import { notifications } from "@mantine/notifications";
import { Group, List, Stack, Text } from "@mantine/core";
import CreateApiKey from "@/create/api-key";
import ConfirmDelete from "./confirm-delete";

export interface ApiKeyTableProps {}

export default function ApiKeyTable({}: ApiKeyTableProps) {
  const inv = useInvalidate();

  const { data } = useRead("ListApiKeys", {});

  const byId = useMemo(
    () => data && Object.fromEntries(data.map((ok) => [ok.id, ok.name])),
    [data],
  );
  const [selected, setSelected] = useState<RowSelectionState>({});
  const selectedIds = useMemo(() => Object.keys(selected), [selected]);

  const { mutate: update } = useWrite("UpdateApiKey", {
    onSuccess: () => {
      inv(["ListApiKeys"]);
    },
  });

  const { mutateAsync: batchDelete } = useWrite("BatchDeleteApiKeys", {
    onSuccess: (deleted) => {
      notifications.show({
        message: `Deleted ${deleted.length} api key${deleted.length === 1 ? "" : "s"}.`,
        color: "green",
      });
      inv(["ListApiKeys"]);
      setSelected({});
    },
  });

  const [search, setSearch] = useState("");
  const apiKeys = filterBySplit(data, search, (k) => k.name);

  return (
    <Stack>
      <Group>
        <CreateApiKey />
        <ConfirmDelete
          name=""
          entityType={selectedIds.length === 1 ? "Api Key" : "Api Keys"}
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
      </Group>
      <DataTable
        tableKey="api-key-table-v1"
        data={apiKeys}
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
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Key" />
            ),
            accessorKey: "key",
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Enabled" />
            ),
            accessorKey: "enabled",
            cell: ({ row }) => (
              <EnableSwitch
                checked={row.original.enabled}
                onCheckedChange={(enabled) =>
                  update({ id: row.original.id, enabled })
                }
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
    </Stack>
  );
}
