import { House, Info, Music } from "lucide-react";
import { useState } from "react";
import { setEnabled } from "./api";
import {
  AppShell,
  HeaderBar,
  MenuList,
  Switch,
  type MenuItem,
} from "./components";
import { useStatus } from "./hooks";
import { AboutPage, HomePage, SoundPackPage } from "./pages";

type PageId = "home" | "sound-pack" | "about";

const MENU_ITEMS: readonly MenuItem<PageId>[] = [
  {
    id: "home",
    label: "ホーム",
    icon: <House size={16} />,
  },
  {
    id: "sound-pack",
    label: "サウンドパック",
    icon: <Music size={16} />,
  },
  {
    id: "about",
    label: "このアプリについて",
    icon: <Info size={16} />,
  },
];

export default function App() {
  const [pageId, setPageId] = useState<PageId>("home");
  const { status, refresh } = useStatus();

  const handleToggle = async (enabled: boolean) => {
    await setEnabled(enabled);
    await refresh();
  };

  return (
    <AppShell
      header={
        <HeaderBar
          title="Khorus"
          actions={
            status && (
              <Switch
                checked={status.enabled}
                label="音を鳴らす"
                onChange={handleToggle}
              />
            )
          }
        />
      }
      menu={
        <MenuList items={MENU_ITEMS} activeId={pageId} onSelect={setPageId} />
      }
    >
      {status && pageId === "home" && (
        <HomePage status={status} onChange={refresh} />
      )}
      {pageId === "sound-pack" && <SoundPackPage />}
      {pageId === "about" && <AboutPage />}
    </AppShell>
  );
}
