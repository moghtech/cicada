import { DataTable, SortableHeader } from "@/components/data-table";
import CreateFilesystem from "@/create/filesystem";
import { Page } from "@/layout/page";
import { useRead } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { useNavigate } from "react-router-dom";

const FilesystemsPage = () => {
  const { data } = useRead("ListFilesystems", {});
  const nav = useNavigate();
  return (
    <Page
      title="Filesystems"
      icon={ICONS.Filesystem}
      actions={<CreateFilesystem />}
    >
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
    </Page>
  );
};

export default FilesystemsPage;
