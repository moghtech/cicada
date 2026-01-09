import { DataTable, SortableHeader } from "@/components/data-table";
import CreateFilesystem from "@/create/filesystem";
import { useRead } from "@/lib/hooks";
import { Flex, Group, Text } from "@mantine/core";
import { HardDrive } from "lucide-react";
import { useNavigate } from "react-router-dom";

const FilesystemsPage = () => {
  const { data } = useRead("ListFilesystems", {});
  const nav = useNavigate();
  return (
    <Flex direction="column" gap="lg">
      <Group>
        <HardDrive size={20} />
        <Text fz="h2" opacity={0.6}>
          Filesystems
        </Text>
      </Group>
      <Flex>
        <CreateFilesystem />
      </Flex>
      <DataTable
        tableKey="filesystems-table-v1"
        data={data ?? []}
        onRowClick={(fs) => nav("/filesystems/" + fs.id)}
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

export default FilesystemsPage;
