import ConfirmDelete from "@/components/confirm-delete";
import { DataTable, filterBySplit, SearchInput, SortableHeader } from "mogh_ui";
import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { Group, List, Text } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { RowSelectionState } from "@tanstack/react-table";
import { useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";
import GroupMultiSelector from "@/components/group-multi-selector";

export default function DevicesPage() {
  const inv = useInvalidate();
  const nav = useNavigate();
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
        onRowClick={(device) => nav("/devices/" + device.id)}
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
