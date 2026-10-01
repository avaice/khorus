import { requestPermission, type Locale, type Status } from "../../api";
import {
  Button,
  Card,
  ChoiceList,
  Page,
  Slider,
  Switch,
  Text,
  type ChoiceItem,
} from "../../components";
import { useAutostart, usePlayOnRepeat, useVolumes } from "../../hooks";
import { useI18n } from "../../i18n";

const VOLUME_MAX = 10;

const LANGUAGE_NAMES = {
  ja: "日本語",
  en: "English",
};

type HomePageProps = {
  status: Status;
  onChange: () => Promise<void>;
};

export function HomePage({ status, onChange }: HomePageProps) {
  const { volumes, update } = useVolumes();
  const autostart = useAutostart();
  const playOnRepeat = usePlayOnRepeat();
  const { messages, language, locale, changeLanguage } = useI18n();
  const text = messages.home;

  const languageItems: readonly ChoiceItem<Locale>[] = [
    { id: "en", label: LANGUAGE_NAMES.en },
    { id: "ja", label: LANGUAGE_NAMES.ja },
  ];

  const handleRequestPermission = async () => {
    await requestPermission();
    await onChange();
  };

  return (
    <Page>
      {status.permissionGranted ? (
        <Card title={text.permissionTitle}>
          <Text>{text.permissionGranted}</Text>
        </Card>
      ) : (
        <Card title={text.permissionRequiredTitle}>
          <Text>{text.permissionRequiredDescription}</Text>
          <Button onClick={handleRequestPermission}>
            {text.requestPermission}
          </Button>
        </Card>
      )}

      <Card title={text.volumeTitle}>
        <Slider
          label={text.enterKey}
          value={Math.round(volumes.enter * VOLUME_MAX)}
          min={0}
          max={VOLUME_MAX}
          onChange={(value) => update("enter", value / VOLUME_MAX)}
        />
        <Slider
          label={text.spaceKey}
          value={Math.round(volumes.space * VOLUME_MAX)}
          min={0}
          max={VOLUME_MAX}
          onChange={(value) => update("space", value / VOLUME_MAX)}
        />
        <Slider
          label={text.otherKeys}
          value={Math.round(volumes.other * VOLUME_MAX)}
          min={0}
          max={VOLUME_MAX}
          onChange={(value) => update("other", value / VOLUME_MAX)}
        />
      </Card>

      <Card title={text.keyRepeatTitle}>
        <Switch
          checked={playOnRepeat.enabled}
          label={text.playOnRepeat}
          onChange={playOnRepeat.update}
        />
      </Card>

      <Card title={text.startupTitle}>
        <Switch
          checked={autostart.enabled}
          label={text.launchAtLogin}
          onChange={autostart.update}
        />
      </Card>

      <Card title={text.languageTitle}>
        <Switch
          checked={language === "system"}
          label={text.systemLanguage}
          onChange={(useSystem) =>
            changeLanguage(useSystem ? "system" : locale)
          }
        />
        <ChoiceList
          items={languageItems}
          selectedId={locale}
          onSelect={changeLanguage}
          disabled={language === "system"}
        />
      </Card>
    </Page>
  );
}
