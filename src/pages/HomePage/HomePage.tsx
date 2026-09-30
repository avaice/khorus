import { requestPermission, type Status } from "../../api";
import { Button, Card } from "../../components";

type HomePageProps = {
  status: Status;
  onChange: () => Promise<void>;
};

export function HomePage({ status, onChange }: HomePageProps) {
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
    </div>
  );
}
