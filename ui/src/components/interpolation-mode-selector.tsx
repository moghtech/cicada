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

const LABELS = {
  Inherit: "Inherit",
  Brackets: "Brackets: [[SECRET]]",
  CurlyBrackets: "Curly Brackets: {{SECRET}}",
  EnvVar: "Env Var: ${SECRET}",
  Disabled: "Disabled",
};
const INHERIT_LABELS = {
  Inherit: "Inherit",
  Brackets: "[[SECRET]]",
  CurlyBrackets: "{{SECRET}}",
  EnvVar: "${SECRET}",
  Disabled: "Disabled",
};

const ALL = Object.values(Types.InterpolationMode).map((mode) => ({
  label: LABELS[mode],
  value: mode,
}));
const EXCLUDE_INHERIT = ALL.filter(
  (mode) => mode.value !== Types.InterpolationMode.Inherit,
);

export interface InterpolationModeSelectorProps extends Omit<
  Omit<SelectProps, "data">,
  "onChange"
> {
  excludeInherit?: boolean;
  inherit?: Types.InterpolationMode;
  onChange?: (interpolation: Types.InterpolationMode) => void;
}

export default function InterpolationModeSelector({
  excludeInherit,
  inherit,
  onChange,
  ...props
}: InterpolationModeSelectorProps) {
  return (
    <Select
      leftSection={
        <HoverCard position="bottom-start" offset={12}>
          <HoverCard.Target>
            <Badge px="0.15rem" py="0.1rem">
              <Center>
                <ICONS.Interpolation size="1rem" />
              </Center>
            </Badge>
          </HoverCard.Target>
          <HoverCard.Dropdown>
            <Text>Interpolation mode</Text>
          </HoverCard.Dropdown>
        </HoverCard>
      }
      data={
        excludeInherit
          ? EXCLUDE_INHERIT
          : ALL.map((mode) =>
              mode.value === Types.InterpolationMode.Inherit
                ? {
                    label: inherit
                      ? "Inherit: " + INHERIT_LABELS[inherit]
                      : "Inherit",
                    value: Types.InterpolationMode.Inherit,
                  }
                : mode,
            )
      }
      onChange={(value) =>
        value && onChange?.(value as Types.InterpolationMode)
      }
      {...props}
    />
  );
}
