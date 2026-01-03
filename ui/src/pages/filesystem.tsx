import { DataTable, SortableHeader } from "@/components/data-table";
import { useRead } from "@/lib/hooks";
import { Flex } from "@mantine/core";
import { useNavigate, useParams } from "react-router-dom";

const FilesystemPage = () => {
  const { filesystem, parent: _parent } = useParams() as {
    filesystem: string;
    parent?: string;
  };
  const parent = _parent && Number(_parent) ? Number(_parent) : 1;
  const fs = useRead("ListFilesystems", {}).data?.find(
    (fs) => fs.id === filesystem
  );
  const { data: node } = useRead(
    "FindNode",
    { filesystem, ino: parent },
    { enabled: parent > 1 }
  );
  const { data } = useRead("ListNodes", {
    filesystem,
    parent: parent && Number(parent) ? Number(parent) : 1,
  });
  const nav = useNavigate();
  return (
    <Flex direction="column">
      <Flex gap="sm">
        <h2 style={{ color: "gray" }}>Filesystem:</h2>
        <h2>{fs?.name ?? "Unknown"}</h2>
        <h2 style={{ color: "gray" }}>|</h2>
        <h2 style={{ color: "gray" }}>Node:</h2>
        <h2>{parent === 1 ? "Root" : node?.name ?? "Unknown"}</h2>
      </Flex>
      <DataTable
        tableKey="filesystem-table-v1"
        data={data ?? []}
        onRowClick={(node) => {
          if (node.kind === "Folder") {
            nav(`/filesystems/${filesystem}/${node.ino}`);
          }
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
              <SortableHeader column={column} title="Kind" />
            ),
            accessorKey: "kind",
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

export default FilesystemPage;
