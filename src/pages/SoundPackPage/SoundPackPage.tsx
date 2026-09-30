import { ask, open } from "@tauri-apps/plugin-dialog";
import { Trash2 } from "lucide-react";
import { useState } from "react";
import { deletePack, importPack, selectPack } from "../../api";
import { Button, ChoiceList, IconButton } from "../../components";
import { usePacks } from "../../hooks";

export function SoundPackPage() {
  const { packs, refresh } = usePacks();
  const [error, setError] = useState<string | null>(null);

  const run = async (action: () => Promise<void>) => {
    setError(null);
    try {
      await action();
    } catch (reason) {
      setError(String(reason));
    }
    await refresh();
  };

  const handleSelect = (id: string) => run(() => selectPack(id));

  const handleImport = () =>
    run(async () => {
      const path = await open({
        multiple: false,
        filters: [{ name: "サウンドパック", extensions: ["zip"] }],
      });
      if (path === null) {
        return;
      }
      const id = await importPack(path);
      await selectPack(id);
    });

  const handleDelete = (id: string, title: string) =>
    run(async () => {
      const confirmed = await ask(`「${title}」を削除しますか？`, {
        title: "サウンドパックの削除",
        kind: "warning",
      });
      if (confirmed) {
        await deletePack(id);
      }
    });

  return (
    <div className="page">
      <div className="toolbar">
        <h2 className="toolbar-title">サウンドパック</h2>
        <Button onClick={handleImport}>読み込む…</Button>
      </div>

      {error && <p className="error-text">{error}</p>}

      <ChoiceList
        items={packs.map((pack) => ({
          id: pack.id,
          label: pack.title,
          description: pack.description,
          trailing: pack.builtin ? undefined : (
            <IconButton
              label="削除"
              icon={<Trash2 size={16} />}
              onClick={() => handleDelete(pack.id, pack.title)}
            />
          ),
        }))}
        selectedId={packs.find((pack) => pack.selected)?.id}
        onSelect={handleSelect}
      />
    </div>
  );
}
