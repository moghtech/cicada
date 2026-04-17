import { useRead } from "@/lib/hooks";
import { TagsInput, TagsInputProps } from "@mantine/core";
import { useState } from "react";

export interface GroupMultiSelectorProps extends Omit<TagsInputProps, "data"> {}

export default function GroupMultiSelector(props: GroupMultiSelectorProps) {
  const { data } = useRead("ListGroups", {});
  const groups = data?.map((g) => g.name) ?? [];
  const [search, setSearch] = useState("");
  return (
    <TagsInput
      placeholder="Select or create groups"
      data={
        search ? [search, ...groups.filter((name) => name !== search)] : groups
      }
      styles={{ inputField: { width: 150 } }}
      miw="max-content"
      searchValue={search}
      onSearchChange={(search) => setSearch(search)}
      clearable
      {...props}
    />
  );
}
