import { useRead } from "@/lib/hooks";
import {
  ActionIcon,
  Button,
  ButtonProps,
  Combobox,
  ComboboxProps,
  Group,
  Stack,
  Text,
} from "@mantine/core";
import { ChevronsUpDown } from "lucide-react";
import { ReactNode } from "react";
import { ICONS } from "@/lib/icons";
import { hexColorByIntention, useSearchCombobox, filterBySplit } from "mogh_ui";

export interface CheckpointSelectorProps extends ComboboxProps {
  node: string;
  label?: ReactNode;
  defaultKey?: string;
  selected: string | undefined;
  onSelect?: (id: string) => void;
  autoSelectFirst?: boolean;
  disabled?: boolean;
  placeholder?: string;
  targetProps?: ButtonProps;
  clearable?: boolean;
  excludeId?: string;
}

export default function CheckpointSelector({
  node,
  label,
  defaultKey,
  selected,
  onSelect,
  autoSelectFirst,
  disabled,
  placeholder,
  position = "bottom-start",
  onOptionSubmit,
  targetProps,
  clearable = true,
  excludeId,
  ...comboboxProps
}: CheckpointSelectorProps) {
  const { data: _checkpoints, isPending } = useRead("ListCheckpoints", {
    node,
  });
  const checkpoints = excludeId
    ? _checkpoints?.filter((c) => c.id !== excludeId)
    : _checkpoints;

  const selectedCheckpoint = checkpoints?.find((s) => s.id === selected);
  const name = selectedCheckpoint?.name || selectedCheckpoint?.created_at;

  const intention = !selectedCheckpoint ? "None" : "Good";

  const { search, setSearch, combobox } = useSearchCombobox();

  const filtered = filterBySplit(checkpoints, search, (item) => item.name);

  const Selector = (
    <Combobox
      store={combobox}
      width="target"
      onOptionSubmit={(id, props) => {
        onSelect?.(id);
        onOptionSubmit?.(id, props);
        combobox.closeDropdown();
      }}
      position={position}
      {...comboboxProps}
    >
      <Combobox.Target>
        <Button
          justify="space-between"
          w="100%"
          rightSection={
            <Group gap="xs" ml="sm" wrap="nowrap">
              {clearable && (
                <ActionIcon
                  size="sm"
                  variant="filled"
                  color="red"
                  onClick={(e) => {
                    e.stopPropagation();
                    onSelect?.("");
                  }}
                  disabled={disabled || !selected}
                >
                  <ICONS.Clear size="0.8rem" />
                </ActionIcon>
              )}
              <ChevronsUpDown size="1rem" />
            </Group>
          }
          onClick={() => combobox.toggleDropdown()}
          disabled={disabled}
          loading={isPending}
          {...targetProps}
        >
          <Group gap="xs" wrap="nowrap">
            <ICONS.Checkpoint
              size="1rem"
              color={hexColorByIntention(intention)}
            />
            <Text className="text-ellipsis">
              {name || (placeholder ?? "Select checkpoint")}
            </Text>
          </Group>
        </Button>
      </Combobox.Target>

      <Combobox.Dropdown>
        <Combobox.Search
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          leftSection={<ICONS.Search size="1rem" style={{ marginRight: 6 }} />}
          placeholder="Search"
        />
        <Combobox.Options mah={224} style={{ overflowY: "auto" }}>
          {filtered.map((checkpoint) => (
            <Combobox.Option key={checkpoint.id} value={checkpoint.id}>
              <Group gap="xs">
                <ICONS.Checkpoint
                  size="1rem"
                  color={hexColorByIntention("Good")}
                />
                <Text>{checkpoint.name || checkpoint.created_at}</Text>
              </Group>
            </Combobox.Option>
          ))}
          {filtered.length === 0 && (
            <Combobox.Empty>No results.</Combobox.Empty>
          )}
        </Combobox.Options>
      </Combobox.Dropdown>
    </Combobox>
  );

  if (label) {
    return (
      <Stack gap="0.1rem">
        <Text size="sm" fw="600">
          {label}
        </Text>
        {Selector}
      </Stack>
    );
  } else {
    return Selector;
  }
}
