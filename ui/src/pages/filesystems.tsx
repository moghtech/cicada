import { DataTable, Page, SortableHeader } from "mogh_ui";
import CreateFilesystem from "@/create/filesystem";
import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { useNavigate } from "react-router-dom";
import ResourceLink from "@/components/resource-link";
import { useEffect, useState } from "react";
import { TextInput } from "@mantine/core";

const FilesystemsPage = () => {
  const inv = useInvalidate();
  const { data } = useRead("ListFilesystems", {});
  const nav = useNavigate();
  const { mutate: updateFilesystem } = useWrite("UpdateFilesystem", {
    onSuccess: () => inv(["ListFilesystems"]),
  });
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
            cell: ({ row }) => {
              const [name, setName] = useState(row.original.name);
              useEffect(() => setName(row.original.name), [row.original.name]);
              return (
                <TextInput
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  onBlur={(e) =>
                    updateFilesystem({
                      id: row.original.id,
                      name: e.target.value,
                    })
                  }
                  onKeyDown={(e) => {
                    if (e.key === "Enter") {
                      e.currentTarget.blur();
                    }
                  }}
                  onClick={(e) => e.stopPropagation()}
                />
              );
            },
          },
          {
            header: ({ column }) => (
              <SortableHeader column={column} title="Encryption Key" />
            ),
            accessorKey: "encryption_key",
            cell: ({ row }) =>
              row.original.encryption_key ? (
                <ResourceLink
                  type="EncryptionKey"
                  id={row.original.encryption_key}
                />
              ) : (
                ""
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
    </Page>
  );
};

export default FilesystemsPage;
