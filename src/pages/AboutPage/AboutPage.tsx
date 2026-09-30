import { InfoSheet, Link } from "../../components";
import { useAppInfo } from "../../hooks";

const REPOSITORY_URL = "https://github.com/avaice/khorus";

export function AboutPage() {
  const info = useAppInfo();

  if (!info) {
    return null;
  }

  return (
    <InfoSheet
      title={info.name}
      items={[
        { label: "バージョン", value: info.version },
        {
          label: "GitHub",
          value: <Link href={REPOSITORY_URL}>avaice/khorus</Link>,
        },
      ]}
    />
  );
}
