import { Button } from "@mantine/core";
import { Check, CircleQuestionMark } from "lucide-react";
import { ComponentProps, ReactNode, useState } from "react";

export type ConfirmButtonProps = {
  children?: ReactNode;
  icon?: ReactNode;
} & ComponentProps<typeof Button<"button">>;

export function ConfirmButton({
  children,
  onClick,
  onBlur,
  leftSection,
  icon,
  miw,
  ...props
}: ConfirmButtonProps) {
  const [clickedOnce, setClickedOnce] = useState(false);

  return (
    <Button
      {...props}
      onClick={(e) => {
        e.stopPropagation();
        clickedOnce ? onClick?.(e) : setClickedOnce(true);
      }}
      onBlur={(e) => {
        setClickedOnce(false);
        onBlur?.(e);
      }}
      leftSection={
        clickedOnce ? (
          <Check size="1rem" />
        ) : (
          (leftSection ?? icon ?? <CircleQuestionMark size="1rem" />)
        )
      }
      miw={miw ?? 120}
    >
      {clickedOnce ? "Confirm" : children}
    </Button>
  );
}
