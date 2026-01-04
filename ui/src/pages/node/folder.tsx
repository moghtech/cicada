import { DataTable, SortableHeader } from "@/components/data-table";
import CreateNode from "@/create/node";
import { useRead } from "@/lib/hooks";
import { Flex, Group } from "@mantine/core";
import { Types } from "cicada_client";
import { FolderOpen, HardDrive } from "lucide-react";
import { useNavigate } from "react-router-dom";

const FolderPage = ({
  filesystem: _filesystem,
  node,
}: {
  filesystem: string;
  node: Types.NodeRecord | undefined;
}) => {
  const filesystem = useRead("ListFilesystems", {}).data?.find(
    (fs) => fs.id === _filesystem
  );
  const children =
    useRead("ListNodes", {
      filesystem: filesystem?.id,
      parent: node?.inode ?? 1,
    }).data ?? [];
  const nav = useNavigate();
  return (
    <Flex direction="column" gap="lg">
      <Flex gap="sm" align="center">
        <HardDrive size={20} />
        <h2 style={{ opacity: 0.6 }}>Filesystem:</h2>
        <h2>{filesystem?.name}</h2>
        <h2 style={{ opacity: 0.6 }}>|</h2>
        <FolderOpen size={20} />
        <h2 style={{ opacity: 0.6 }}>Folder:</h2>
        <h2>{node?.name ?? "Root"}</h2>
      </Flex>
      <Group>
        {Object.values(Types.NodeKind).map((kind) => (
          <CreateNode key={kind} kind={kind} parent={node?.inode ?? 1} />
        ))}
      </Group>
      <DataTable
        tableKey="filesystem-table-v1"
        data={children}
        onRowClick={(node) =>
          nav(`/filesystems/${filesystem?.id}/${node.inode}`)
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
              <SortableHeader column={column} title="Kind" />
            ),
            accessorKey: "kind",
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

export default FolderPage;
