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
import { useStatus, useSystemAccentColor } from "./hooks";
import { useMessages } from "./i18n";
import { AboutPage, HomePage, SoundPackPage } from "./pages";

type PageId = "home" | "sound-pack" | "about";

export default function App() {
  const messages = useMessages();
  const [pageId, setPageId] = useState<PageId>("home");
  const { status, refresh } = useStatus();
  useSystemAccentColor();

  const menuItems: readonly MenuItem<PageId>[] = [
    {
      id: "home",
      label: messages.menu.home,
      icon: <House size={16} />,
    },
    {
      id: "sound-pack",
      label: messages.menu.soundPack,
      icon: <Music size={16} />,
    },
    {
      id: "about",
      label: messages.menu.about,
      icon: <Info size={16} />,
    },
  ];

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
                label={messages.soundEnabled}
                onChange={handleToggle}
              />
            )
          }
        />
      }
      menu={
        <MenuList items={menuItems} activeId={pageId} onSelect={setPageId} />
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
