import { useRead } from "@/lib/hooks";
import { MultiSelect, MultiSelectProps } from "@mantine/core";

export interface UserMultiSelectorProps
  extends Omit<MultiSelectProps, "data"> {}

export default function UserMultiSelector(props: UserMultiSelectorProps) {
  const { data: users } = useRead("ListUsers", {});
  return (
    <MultiSelect
      placeholder="Select users"
      data={users?.map((u) => ({ value: u.id, label: u.username })) ?? []}
      searchable
      clearable
      {...props}
    />
  );
}
