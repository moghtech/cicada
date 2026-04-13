import ConfirmDelete from "@/components/confirm-delete";
import { DataTable, SortableHeader } from "mogh_ui";
import CreateOnboardingKey from "@/create/onboarding-key";
import { useInvalidate, useRead, useSetTitle, useWrite } from "@/lib/hooks";
import { Group, List, Text } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { RowSelectionState } from "@tanstack/react-table";
import { useMemo, useState } from "react";
import GroupMultiSelector from "@/components/group-multi-selector";
import ResourceLink from "@/components/resource-link";

export default function OnboardingKeysPage() {
  useSetTitle("Onboarding");
  
  const inv = useInvalidate();
  const { data } = useRead("ListOnboardingKeys", {});
  const byId = useMemo(
    () => data && Object.fromEntries(data.map((ok) => [ok.id, ok.name])),
    [data],
  );
  const [selected, setSelected] = useState<RowSelectionState>({});
  const selectedIds = useMemo(() => Object.keys(selected), [selected]);
  const { mutate: updateOnboardingKey } = useWrite("UpdateOnboardingKey", {
    onSuccess: () => inv(["ListOnboardingKeys"]),
  });
  const { mutateAsync: batchDelete } = useWrite("BatchDeleteOnboardingKeys", {
    onSuccess: (deleted) => {
      notifications.show({
        message: `Deleted ${deleted.length} onboarding key${deleted.length === 1 ? "" : "s"}.`,
      });
      inv(["ListOnboardingKeys"]);
      setSelected({});
    },
  });
  return (
    <>
      <Group>
        <CreateOnboardingKey />
        <ConfirmDelete
          name=""
          entityType={"Onboarding Key" + (selectedIds.length === 1 ? "" : "s")}
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
        tableKey="onboarding-keys-table-v1"
        data={data ?? []}
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
              <ResourceLink type="OnboardingKey" id={row.original.id} />
            ),
          },
          {
            header: "Groups",
            accessorKey: "groups",
            cell: ({ row }) => (
              <GroupMultiSelector
                value={row.original.groups}
                onChange={(groups) =>
                  updateOnboardingKey({ id: row.original.id, groups })
                }
                onClick={(e) => e.stopPropagation()}
              />
            ),
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Expires" />
            ),
            accessorKey: "expires",
            cell: ({ row }) =>
              row.original.expires
                ? new Date(row.original.expires).toLocaleString()
                : "Never expires",
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
