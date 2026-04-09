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
import { ReactNode, useEffect } from "react";
import { ICONS } from "@/lib/icons";
import { hexColorByIntention, useSearchCombobox, filterBySplit } from "mogh_ui";

export interface EncryptionKeySelectorProps extends ComboboxProps {
  label?: ReactNode;
  defaultKey?: string;
  selected: string | undefined;
  onSelect?: (id: string) => void;
  autoSelectFirst?: boolean;
  disabled?: boolean;
  placeholder?: string;
  targetProps?: ButtonProps;
  clearable?: boolean;
}

export default function EncryptionKeySelector({
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
  clearable,
  ...comboboxProps
}: EncryptionKeySelectorProps) {
  const { data: encryptionKeys, isPending } = useRead("ListEncryptionKeys", {});

  const firstEncryptionKey = defaultKey ?? encryptionKeys?.[0]?.id;
  useEffect(() => {
    autoSelectFirst &&
      !clearable &&
      firstEncryptionKey &&
      !selected &&
      onSelect?.(firstEncryptionKey);
  }, [firstEncryptionKey]);

  const selectedEncryptionKey = encryptionKeys?.find((s) => s.id === selected);
  const name = selectedEncryptionKey?.name;

  const intention = !selectedEncryptionKey
    ? "None"
    : selectedEncryptionKey.initialized
      ? "Good"
      : "Critical";

  const { search, setSearch, combobox } = useSearchCombobox();

  const filtered = filterBySplit(
    encryptionKeys,
    search,
    (item) => item.name,
  ).sort((a, b) => {
    if (a.name > b.name) {
      return 1;
    } else if (a.name < b.name) {
      return -1;
    } else {
      return 0;
    }
  });

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
            <ICONS.EncryptionKey
              size="1rem"
              color={hexColorByIntention(intention)}
            />
            <Text className="text-ellipsis">
              {name || (placeholder ?? "Select encryption key")}
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
          {filtered.map((encryptionKey) => (
            <Combobox.Option key={encryptionKey.id} value={encryptionKey.id}>
              <Group gap="xs">
                <ICONS.EncryptionKey
                  size="1rem"
                  color={hexColorByIntention(
                    encryptionKey.initialized ? "Good" : "Critical",
                  )}
                />
                <Text>{encryptionKey.name}</Text>
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
