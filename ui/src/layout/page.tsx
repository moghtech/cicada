import { Flex, Group, Text } from "@mantine/core";
import { CircleQuestionMark } from "lucide-react";
import { FC, ReactNode } from "react";

export const Page = ({
  title,
  icon,
  rightTitle,
  fullTitle,
  actions,
  children,
}: {
  title?: string;
  icon?: FC<{ size?: string | number }>;
  rightTitle?: ReactNode;
  /* Replace the default title / icon with a full custom ReactNode */
  fullTitle?: ReactNode;
  actions?: ReactNode;
  children?: ReactNode;
}) => {
  const Icon = icon ?? CircleQuestionMark;
  return (
    <Flex direction="column" gap="lg">
      <Group>
        {fullTitle ? (
          fullTitle
        ) : (
          <>
            <Icon size={24} />
            <Text fz="h1">
              {title}
            </Text>
          </>
        )}
        {rightTitle}
      </Group>

      {actions && <Group>{actions}</Group>}

      {children}
    </Flex>
  );
};
