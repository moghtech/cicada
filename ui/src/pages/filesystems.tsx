import { DataTable, SortableHeader } from "@/components/data-table";
import { useRead } from "@/lib/hooks";
import { Flex } from "@mantine/core";
import { useNavigate } from "react-router-dom";

const FilesystemsPage = () => {
  const { data } = useRead("ListFilesystems", {});
  const nav = useNavigate();
  return (
    <Flex direction="column">
      <h2>Filesystems</h2>
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
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Updated At" />
            ),
            accessorKey: "updated_at",
          },
        ]}
      />
    </Flex>
  );
};

export default FilesystemsPage;
