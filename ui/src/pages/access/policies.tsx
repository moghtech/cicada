import ConfirmDelete from "@/components/confirm-delete";
import { DataTable, SortableHeader } from "mogh_ui";
import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { Group, List, Text } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { RowSelectionState } from "@tanstack/react-table";
import { useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";

export default function PoliciesPage() {
  const inv = useInvalidate();
  const nav = useNavigate();
  const { data } = useRead("ListPolicies", {});
  const byId = useMemo(
    () => data && Object.fromEntries(data.map((ok) => [ok.id, ok.name])),
    [data],
  );
  const [selected, setSelected] = useState<RowSelectionState>({});
  const selectedIds = useMemo(() => Object.keys(selected), [selected]);
  const { mutateAsync: batchDelete } = useWrite("BatchDeletePolicies", {
    onSuccess: (deleted) => {
      notifications.show({
        message: `Deleted ${deleted.length} ${deleted.length === 1 ? "policy" : "policies"}.`,
      });
      inv(["ListPolicies"]);
      setSelected({});
    },
  });
  return (
    <>
      <Group>
        <ConfirmDelete
          name=""
          entityType={selectedIds.length === 1 ? "Policy" : "Policies"}
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
      </Group>
      <DataTable
        tableKey="policies-table-v1"
        data={data ?? []}
        onRowClick={(policy) => nav("/policies/" + policy.id)}
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
    </>
  );
}
