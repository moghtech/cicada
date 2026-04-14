import { DataTable, SortableHeader } from "mogh_ui";
import { useRead } from "@/lib/hooks";
import CheckpointLink from "./checkpoint-link";
import { Types } from "cicada_client";

export interface CheckpointsProps {
  target: Types.CheckpointTarget | undefined;
}

export default function Checkpoints({ target }: CheckpointsProps) {
  const { data: checkpoints } = useRead(
    "ListCheckpoints",
    { target: target! },
    { enabled: !!target },
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
            <CheckpointLink id={row.original.id} target={row.original.target} />
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
