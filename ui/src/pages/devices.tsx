import ConfirmDelete from "@/components/confirm-delete";
import { DataTable, SortableHeader } from "@/components/data-table";
import { Page } from "@/layout/page";
import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { List, Text } from "@mantine/core";
import { RowSelectionState } from "@tanstack/react-table";
import { useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";

const DevicesPage = () => {
  const inv = useInvalidate();
  const nav = useNavigate();
  const { data } = useRead("ListDevices", {});
  const byId = useMemo(
    () => data && Object.fromEntries(data.map((ok) => [ok.id, ok.name])),
    [data]
  );
  const [selected, setSelected] = useState<RowSelectionState>({});
  const selectedIds = useMemo(() => Object.keys(selected), [selected]);
  const { mutateAsync: batchDelete } = useWrite("BatchDeleteDevices", {
    onSuccess: () => {
      inv(["ListDevices"]);
      setSelected({});
    },
  });
  return (
    <Page
      title="Devices"
      icon={ICONS.Device}
      actions={
        <ConfirmDelete
          name=""
          entityType="Devices"
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
      }
    >
      <DataTable
        tableKey="devices-table-v1"
        data={data ?? []}
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
    </Page>
  );
};

export default DevicesPage;
