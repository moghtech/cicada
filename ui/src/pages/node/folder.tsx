import ConfirmDelete from "@/components/confirm-delete";
import { DataTable, SortableHeader } from "@/components/data-table";
import CreateNode from "@/create/node";
import { Page } from "@/layout/page";
import { useInvalidate, useRead, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { Text } from "@mantine/core";
import { Types } from "cicada_client";
import { useNavigate } from "react-router-dom";

const FolderPage = ({
  filesystem,
  node,
}: {
  filesystem: Types.FilesystemRecord | undefined;
  node: Types.NodeRecord | undefined;
}) => {
  const nav = useNavigate();
  const inv = useInvalidate();
  const { mutateAsync: deleteFs, isPending: deleteFsPending } = useWrite(
    "DeleteFilesystem",
    {
      onSuccess: () => {
        inv(["ListFilesystems"], ["ListNodes"]);
        nav("/");
      },
    }
  );
  const children =
    useRead("ListNodes", {
      filesystem: filesystem?.id,
      parent: node?.inode ?? 1,
    }).data ?? [];
  const { mutateAsync: deleteFolder, isPending: deleteFolderPending } =
    useWrite("DeleteNode", {
      onSuccess: () => {
        inv(["ListNodes"]);
        nav(`/filesystems/${node?.filesystem}/${node?.parent ?? 1}`);
      },
    });

  return (
    <Page
      fullTitle={
        <>
          <ICONS.Filesystem size={24} opacity={0.6} />
          <Text fz="h1" opacity={0.6}>
            Filesystem:
          </Text>
          <Text fz="h1">{filesystem?.name}</Text>
          <Text fz="h1" opacity={0.6}>
            |
          </Text>
          <ICONS.Folder size={24} />
          <Text fz="h1" opacity={0.6}>
            Folder:
          </Text>
          <Text fz="h1">{node?.name ?? "Root"}</Text>
        </>
      }
      actions={
        <>
          {Object.values(Types.NodeKind).map((kind) => (
            <CreateNode key={kind} kind={kind} parent={node?.inode ?? 1} />
          ))}
          {node === undefined && filesystem && (
            <ConfirmDelete
              entityType="Filesystem"
              name={filesystem.name}
              onConfirm={() => deleteFs({ id: filesystem.id })}
              loading={deleteFsPending}
              disabled={false}
            />
          )}
          {node && (
            <ConfirmDelete
              entityType="Folder"
              name={node.name}
              onConfirm={() => deleteFolder({ id: node.id })}
              loading={deleteFolderPending}
              disabled={false}
            />
          )}
        </>
      }
    >
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
    </Page>
  );
};

export default FolderPage;
