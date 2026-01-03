import { DataTable, SortableHeader } from "@/components/data-table";
import CreateNode from "@/create/node";
import { useRead } from "@/lib/hooks";
import { Flex, Group } from "@mantine/core";
import { Types } from "cicada_client";
import { FolderOpen, HardDrive } from "lucide-react";
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
      <Flex align="center" gap="md">
        <Flex gap="sm" align="center">
          <HardDrive size={20} />
          <h2 style={{ opacity: 0.6 }}>Filesystem:</h2>
          <h2>{fs?.name}</h2>
          <h2 style={{ opacity: 0.6 }}>|</h2>
          <FolderOpen size={20} />
          <h2 style={{ opacity: 0.6 }}>Folder:</h2>
          <h2>{parent === 1 ? "Root" : node?.name}</h2>
        </Flex>
        <Group>
          {Object.values(Types.NodeKind).map((kind) => (
            <CreateNode key={kind} kind={kind} />
          ))}
        </Group>
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

export default FilesystemPage;
