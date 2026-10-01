import { InfoSheet, Link } from "../../components";
import { useAppInfo } from "../../hooks";
import { useMessages } from "../../i18n";

const REPOSITORY_URL = "https://github.com/avaice/khorus";

export function AboutPage() {
  const info = useAppInfo();
  const messages = useMessages();

  return (
    <InfoSheet
      title={info.name}
      items={[
        { label: messages.about.version, value: info.version },
        {
          label: "GitHub",
          value: <Link href={REPOSITORY_URL}>avaice/khorus</Link>,
        },
      ]}
    />
  );
}
