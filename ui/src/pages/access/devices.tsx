import ConfirmDelete from "@/components/confirm-delete";
import {
  DataTable,
  EnableSwitch,
  filterBySplit,
  SearchInput,
  SortableHeader,
} from "mogh_ui";
import { useInvalidate, useRead, useSetTitle, useWrite } from "@/lib/hooks";
import { Group, List, Text } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { RowSelectionState } from "@tanstack/react-table";
import { useMemo, useState } from "react";
import GroupMultiSelector from "@/components/group-multi-selector";
import ResourceLink from "@/components/resource-link";

export default function DevicesPage() {
  useSetTitle("Devices");
  
  const inv = useInvalidate();
  const { data } = useRead("ListDevices", {});
  const byId = useMemo(
    () =>
      data &&
      Object.fromEntries(data.map((device) => [device.id, device.name])),
    [data],
  );
  const [selected, setSelected] = useState<RowSelectionState>({});
  const selectedIds = useMemo(() => Object.keys(selected), [selected]);
  const { mutateAsync: batchDelete } = useWrite("BatchDeleteDevices", {
    onSuccess: (deleted) => {
      notifications.show({
        message: `Deleted ${deleted.length} device${deleted.length === 1 ? "" : "s"}.`,
      });
      inv(["ListDevices"]);
      setSelected({});
    },
  });
  const { mutate: updateDevice } = useWrite("UpdateDevice", {
    onSuccess: () => inv(["ListDevices"]),
  });
  const [search, setSearch] = useState("");
  const devices = filterBySplit(data, search, (device) => device.name);
  return (
    <>
      <Group>
        <ConfirmDelete
          name=""
          entityType={"Device" + (selectedIds.length === 1 ? "" : "s")}
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
        tableKey="devices-table-v1"
        data={devices}
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
              <ResourceLink type="Device" id={row.original.id} />
            ),
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
                  updateDevice({ id: row.original.id, enabled })
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
                  updateDevice({ id: row.original.id, groups })
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
