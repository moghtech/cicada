import { useRead } from "@/lib/hooks";
import { MultiSelect, MultiSelectProps } from "@mantine/core";

export interface DeviceMultiSelectorProps extends Omit<
  MultiSelectProps,
  "data"
> {}

export default function DeviceMultiSelector(props: DeviceMultiSelectorProps) {
  const { data: devices } = useRead("ListDevices", {});
  return (
    <MultiSelect
      placeholder="Select devices"
      data={devices?.map((d) => ({ value: d.id, label: d.name })) ?? []}
      styles={{ inputField: { width: 120 } }}
      miw="max-content"
      searchable
      clearable
      {...props}
    />
  );
}
