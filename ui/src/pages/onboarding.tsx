import { DataTable, SortableHeader } from "@/components/data-table";
import CreateOnboardingKey from "@/create/onboarding-key";
import { useRead } from "@/lib/hooks";
import { Flex, Group, Text } from "@mantine/core";
import { HardDrive } from "lucide-react";
import { useNavigate } from "react-router-dom";

const OnboardingKeysPage = () => {
  const { data } = useRead("ListOnboardingKeys", {});
  const nav = useNavigate();
  return (
    <Flex direction="column" gap="lg">
      <Group>
        <HardDrive size={20} />
        <Text fz="h2" opacity={0.6}>
          Onboarding Keys
        </Text>
      </Group>
      <Group>
        <CreateOnboardingKey />
      </Group>
      <DataTable
        tableKey="onboarding-keys-table-v1"
        data={data ?? []}
        onRowClick={(onboarding_key) =>
          nav("/onboarding-keys/" + onboarding_key.id)
        }
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
    </Flex>
  );
};

export default OnboardingKeysPage;
