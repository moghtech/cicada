import { Badge, Group, Switch } from "@mantine/core";
import { ComponentProps } from "react";

export type EnableSwitchProps = {
  checked: boolean;
  onCheckedChange?: (checked: boolean) => void;
  redDisabled?: boolean;
} & ComponentProps<typeof Switch>;

export const EnableSwitch = ({
  checked,
  color,
  label,
  onChange,
  onCheckedChange,
  redDisabled,
  ...props
}: EnableSwitchProps) => {
  return (
    <Switch
      {...props}
      checked={checked}
      color={color}
      label={
        <Group>
          {label}
          <Badge
            color={checked ? color : redDisabled ? "red" : "gray"}
            style={{ cursor: "pointer" }}
          >
            {checked ? "Enabled" : "Disabled"}
          </Badge>
        </Group>
      }
      onChange={(e) => {
        onChange?.(e);
        onCheckedChange?.(e.target.checked);
      }}
    />
  );
};
