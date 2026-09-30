import type { Status } from "../../api";
import { Card } from "../../components";

type SoundPackPageProps = {
  status: Status;
};

export function SoundPackPage({ status }: SoundPackPageProps) {
  return (
    <div className="page">
      <Card title="使用中のサウンドパック">
        <p className="text pack-title">{status.pack.title}</p>
        <p className="text">{status.pack.description}</p>
      </Card>
    </div>
  );
}
