import {
  DataTable,
  EnableSwitch,
  filterBySplit,
  SearchInput,
  SortableHeader,
  useLoginOptions,
} from "mogh_ui";
import { useInvalidate, useRead, useUser, useWrite } from "@/lib/hooks";
import { RowSelectionState } from "@tanstack/react-table";
import { useMemo, useState } from "react";
import GroupMultiSelector from "@/components/group-multi-selector";
import { Group, List, Text } from "@mantine/core";
import ConfirmDelete from "@/components/confirm-delete";
import { notifications } from "@mantine/notifications";
import CreateUser from "@/create/user";

export default function UsersPage() {
  const client = useUser().data;
  const loginOptions = useLoginOptions().data;
  const inv = useInvalidate();
  const { data } = useRead("ListUsers", {});
  const byId = useMemo(
    () =>
      data && Object.fromEntries(data.map((user) => [user.id, user.username])),
    [data],
  );
  const [selected, setSelected] = useState<RowSelectionState>({});
  const selectedIds = useMemo(() => Object.keys(selected), [selected]);
  const { mutateAsync: batchDelete } = useWrite("BatchDeleteUsers", {
    onSuccess: (deleted) => {
      notifications.show({
        message: `Deleted ${deleted.length} user${deleted.length === 1 ? "" : "s"}.`,
      });
      inv(["ListUsers"]);
      setSelected({});
    },
  });
  const { mutate: updateUser } = useWrite("UpdateUser", {
    onSuccess: () => inv(["ListUsers"]),
  });
  const [search, setSearch] = useState("");
  const users = filterBySplit(data, search, (user) => user.username);
  return (
    <>
      <Group>
        {loginOptions?.local && <CreateUser />}
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
        <SearchInput value={search} onSearch={setSearch} />
      </Group>
      <DataTable
        tableKey="users-table-v1"
        data={users}
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
              <SortableHeader column={column} title="Enabled" />
            ),
            accessorKey: "enabled",
            cell: ({ row }) => (
              <EnableSwitch
                checked={row.original.enabled}
                onCheckedChange={(enabled) =>
                  updateUser({ id: row.original.id, enabled })
                }
              />
            ),
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Admin" />
            ),
            accessorKey: "admin",
            cell: ({ row }) => (
              <EnableSwitch
                checked={row.original.admin}
                onCheckedChange={(admin) =>
                  updateUser({ id: row.original.id, admin })
                }
              />
            ),
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Super Admin" />
            ),
            accessorKey: "super_admin",
            cell: ({ row }) => (
              <EnableSwitch
                checked={row.original.super_admin}
                onCheckedChange={(super_admin) =>
                  updateUser({ id: row.original.id, super_admin })
                }
                disabled={!client?.super_admin}
              />
            ),
          },
          {
            header: "Groups",
            accessorKey: "groups",
            cell: ({ row }) => (
              <GroupMultiSelector
                value={row.original.groups}
                onChange={(groups) =>
                  updateUser({ id: row.original.id, groups })
                }
                onClick={(e) => e.stopPropagation()}
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
    </>
  );
}
