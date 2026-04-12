import { Select, SelectProps } from "@mantine/core";
import { Types } from "cicada_client";

const ALL = Object.values(Types.CheckpointingMode);
const EXCLUDE_INHERIT = ALL.filter(
  (mode) => mode !== Types.CheckpointingMode.Inherit,
);

export interface CheckpointingModeSelectorProps extends Omit<
  Omit<SelectProps, "data">,
  "onChange"
> {
  excludeInherit?: boolean;
  inherit?: Types.CheckpointingMode;
  onChange?: (interpolation: Types.CheckpointingMode) => void;
}

export default function CheckpointingModeSelector({
  excludeInherit,
  inherit,
  onChange,
  ...props
}: CheckpointingModeSelectorProps) {
  return (
    <Select
      data={
        excludeInherit
          ? EXCLUDE_INHERIT
          : ALL.map((mode) =>
              mode === Types.CheckpointingMode.Inherit
                ? {
                    label: inherit ? "Inherit: " + inherit : "Inherit",
                    value: Types.CheckpointingMode.Inherit,
                  }
                : mode,
            )
      }
      onChange={(value) =>
        value && onChange?.(value as Types.CheckpointingMode)
      }
      {...props}
    />
  );
}
