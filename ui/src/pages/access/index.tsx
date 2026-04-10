import { TabbedPage } from "mogh_ui";
import { ICONS } from "@/lib/icons";
import DevicesPage from "./devices";
import OnboardingKeysPage from "./onboarding-keys";
import PoliciesPage from "./policies";

type Tab = "Users" | "Devices" | "Onboarding" | "Policies";

export default function AccessTabs() {
  return (
    <TabbedPage<Tab>
      storageKey="devices-tab-v1"
      tabs={[
        { tab: "Users", icon: ICONS.User, content: DevicesPage },
        { tab: "Devices", icon: ICONS.Device, content: DevicesPage },
        {
          tab: "Onboarding",
          icon: ICONS.OnboardingKey,
          content: OnboardingKeysPage,
        },
        {
          tab: "Policies",
          icon: ICONS.Policy,
          content: PoliciesPage,
        },
      ]}
      color="orange"
    />
  );
}
