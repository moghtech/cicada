import { ICONS } from "@/lib/icons";
import {
  Badge,
  Center,
  HoverCard,
  Select,
  SelectProps,
  Text,
} from "@mantine/core";
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
      leftSection={
        <HoverCard position="bottom-start" offset={12}>
          <HoverCard.Target>
            <Badge px="0.15rem" py="0.1rem">
              <Center>
                <ICONS.Checkpoint size="0.9rem" />
              </Center>
            </Badge>
          </HoverCard.Target>
          <HoverCard.Dropdown>
            <Text>Checkpointing mode</Text>
          </HoverCard.Dropdown>
        </HoverCard>
      }
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
