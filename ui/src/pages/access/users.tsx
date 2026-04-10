import { DataTable, SortableHeader } from "mogh_ui";
import { useRead } from "@/lib/hooks";
import { RowSelectionState } from "@tanstack/react-table";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

export default function UsersPage() {
  const nav = useNavigate();
  const { data } = useRead("ListUsers", {});
  const [selected, setSelected] = useState<RowSelectionState>({});
  return (
    <>
      {/* <Group>
        <ConfirmDelete
          name=""
          entityType={"User" + (selectedIds.length === 1 ? "" : "s")}
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
          onConfirm={async () => {
            if (selectedIds.length) {
              await batchDelete({ ids: selectedIds });
            }
          }}
          disabled={!selectedIds.length}
        />
      </Group> */}
      <DataTable
        tableKey="users-table-v1"
        data={data ?? []}
        onRowClick={(device) => nav("/users/" + device.id)}
        selectOptions={{
          selectKey: (row) => row.id,
          state: [selected, setSelected],
        }}
        columns={[
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Username" />
            ),
            accessorKey: "username",
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Id" />
            ),
            accessorKey: "id",
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
