import { requestPermission, setEnabled } from "../../api";
import { Button, Card, Switch } from "../../components";
import { useStatus } from "../../hooks";

export function HomePage() {
  const { status, refresh } = useStatus();

  if (!status) {
    return null;
  }

  const handleToggle = async (enabled: boolean) => {
    await setEnabled(enabled);
    await refresh();
  };

  const handleRequestPermission = async () => {
    await requestPermission();
    await refresh();
  };

  return (
    <main className="page">
      <h1 className="page-title">Khorus</h1>

      <Card title="効果音">
        <Switch
          checked={status.enabled}
          label="キーを押すと音を鳴らす"
          onChange={handleToggle}
        />
      </Card>

      {!status.permissionGranted && (
        <Card title="入力監視の許可が必要です">
          <p className="text">
            キー入力を検知するために、システム設定の「プライバシーとセキュリティ」から「入力監視」を許可してください。許可すると、数秒後に自動で有効になります。
          </p>
          <Button onClick={handleRequestPermission}>許可をリクエスト</Button>
        </Card>
      )}

      <Card title="使用中のサウンドパック">
        <p className="text pack-title">{status.pack.title}</p>
        <p className="text">{status.pack.description}</p>
      </Card>
    </main>
  );
}
