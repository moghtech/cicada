import DevicesPage from "./devices";
import OnboardingKeysPage from "./onboarding-keys";
import { ICONS } from "@/lib/icons";
import { TabbedPage } from "@/components/tabbed-page";

type Tab = "Devices" | "Onboarding";

const DevicesTabs = () => {
  return (
    <TabbedPage<Tab>
      storageKey="devices-tab-v1"
      tabs={[
        { tab: "Devices", icon: ICONS.Device, content: DevicesPage },
        {
          tab: "Onboarding",
          icon: ICONS.OnboardingKey,
          content: OnboardingKeysPage,
        },
      ]}
    />
  );
};

export default DevicesTabs;
