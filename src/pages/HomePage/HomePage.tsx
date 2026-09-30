import { requestPermission, type Status } from "../../api";
import { Button, Card, Slider, Switch } from "../../components";
import { useAutostart, useVolumes } from "../../hooks";

const VOLUME_MAX = 10;

type HomePageProps = {
  status: Status;
  onChange: () => Promise<void>;
};

export function HomePage({ status, onChange }: HomePageProps) {
  const { volumes, update } = useVolumes();
  const autostart = useAutostart();

  const handleRequestPermission = async () => {
    await requestPermission();
    await onChange();
  };

  return (
    <div className="page">
      {status.permissionGranted ? (
        <Card title="入力監視">
          <p className="text">許可されています。</p>
        </Card>
      ) : (
        <Card title="入力監視の許可が必要です">
          <p className="text">
            キー入力を検知するために、システム設定の「プライバシーとセキュリティ」から「入力監視」を許可してください。許可すると、数秒後に自動で有効になります。
          </p>
          <Button onClick={handleRequestPermission}>許可をリクエスト</Button>
        </Card>
      )}

      {volumes && (
        <Card title="音量">
          <Slider
            label="Enterキー"
            value={Math.round(volumes.enter * VOLUME_MAX)}
            min={0}
            max={VOLUME_MAX}
            onChange={(value) => update("enter", value / VOLUME_MAX)}
          />
          <Slider
            label="Spaceキー"
            value={Math.round(volumes.space * VOLUME_MAX)}
            min={0}
            max={VOLUME_MAX}
            onChange={(value) => update("space", value / VOLUME_MAX)}
          />
          <Slider
            label="その他のキー"
            value={Math.round(volumes.other * VOLUME_MAX)}
            min={0}
            max={VOLUME_MAX}
            onChange={(value) => update("other", value / VOLUME_MAX)}
          />
        </Card>
      )}

      {autostart.enabled !== null && (
        <Card title="スタートアップ設定">
          <Switch
            checked={autostart.enabled}
            label="ログイン時に起動"
            onChange={autostart.update}
          />
        </Card>
      )}
    </div>
  );
}
