import ConfirmDelete from "@/components/confirm-delete";
import DeviceMultiSelector from "@/components/device-multi-selector";
import FilesystemMultiSelector from "@/components/filesystem-multi-selector";
import GroupMultiSelector from "@/components/group-multi-selector";
import UserMultiSelector from "@/components/user-multi-selector";
import {
  DataTable,
  EnableSwitch,
  filterBySplit,
  SearchInput,
  SortableHeader,
} from "mogh_ui";
import { useInvalidate, useRead, useSetTitle, useWrite } from "@/lib/hooks";
import { Group, List, Text, TextInput } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { RowSelectionState } from "@tanstack/react-table";
import { useEffect, useMemo, useState } from "react";
import CreatePolicy from "@/create/policy";

export default function PoliciesPage() {
  useSetTitle("Policies");

  const inv = useInvalidate();
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
        color: "green",
      });
      inv(["ListPolicies"]);
      setSelected({});
    },
  });
  const { mutate: updatePolicy } = useWrite("UpdatePolicy", {
    onSuccess: () => inv(["ListPolicies"]),
  });
  const [search, setSearch] = useState("");
  const policies = filterBySplit(data, search, (policy) => policy.name);
  return (
    <>
      <Group>
        <CreatePolicy />
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
        <SearchInput value={search} onSearch={setSearch} />
      </Group>
      <DataTable
        tableKey="policies-table-v1"
        data={policies}
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
            cell: ({ row }) => {
              const [name, setName] = useState(row.original.name);
              useEffect(() => setName(row.original.name), [row.original.name]);
              return (
                <TextInput
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  onBlur={(e) =>
                    updatePolicy({ id: row.original.id, name: e.target.value })
                  }
                  onKeyDown={(e) => {
                    if (e.key === "Enter") {
                      e.currentTarget.blur();
                    }
                  }}
                  onClick={(e) => e.stopPropagation()}
                />
              );
            },
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
                  updatePolicy({ id: row.original.id, enabled })
                }
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
                  updatePolicy({ id: row.original.id, groups })
                }
                onClick={(e) => e.stopPropagation()}
              />
            ),
          },
          {
            header: "Users",
            accessorKey: "users",
            cell: ({ row }) => (
              <UserMultiSelector
                value={row.original.users}
                onChange={(users) =>
                  updatePolicy({ id: row.original.id, users })
                }
                onClick={(e) => e.stopPropagation()}
              />
            ),
          },
          {
            header: "Devices",
            accessorKey: "devices",
            cell: ({ row }) => (
              <DeviceMultiSelector
                value={row.original.devices}
                onChange={(devices) =>
                  updatePolicy({ id: row.original.id, devices })
                }
                onClick={(e) => e.stopPropagation()}
              />
            ),
          },
          {
            header: "Filesystems",
            accessorKey: "filesystems",
            cell: ({ row }) => (
              <FilesystemMultiSelector
                value={row.original.filesystems}
                onChange={(filesystems) =>
                  updatePolicy({ id: row.original.id, filesystems })
                }
                onClick={(e) => e.stopPropagation()}
              />
            ),
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Write Access" />
            ),
            accessorKey: "filesystem_write",
            cell: ({ row }) => (
              <EnableSwitch
                checked={row.original.filesystem_write}
                onCheckedChange={(filesystem_write) =>
                  updatePolicy({ id: row.original.id, filesystem_write })
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
    </>
  );
}
