import { DataTable, SortableHeader } from "mogh_ui";
import { useRead } from "@/lib/hooks";
import CheckpointLink from "./checkpoint-link";

export interface CheckpointsProps {
  node: string | undefined;
}

export default function Checkpoints({ node }: CheckpointsProps) {
  const { data: checkpoints } = useRead(
    "ListCheckpoints",
    { node: node! },
    { enabled: !!node },
  );

  return (
    <DataTable
      tableKey="checkpoints-table-v1"
      data={checkpoints ?? []}
      columns={[
        {
          header: ({ column }) => (
            <SortableHeader column={column} title="Name" />
          ),
          accessorKey: "name",
          cell: ({ row }) => (
            <CheckpointLink id={row.original.id} nodeId={row.original.node} />
          ),
        },
        // {
        //   header: "Description",
        //   accessorKey: "description",
        // },
        {
          header: ({ column }) => (
            <SortableHeader column={column} title="Created At" />
          ),
          accessorKey: "created_at",
          cell: ({ row }) => new Date(row.original.created_at).toLocaleString(),
        },
        {
          header: ({ column }) => (
            <SortableHeader column={column} title="Updated At" />
          ),
          accessorKey: "updated_at",
          cell: ({ row }) => new Date(row.original.updated_at).toLocaleString(),
        },
      ]}
    />
  );
}
