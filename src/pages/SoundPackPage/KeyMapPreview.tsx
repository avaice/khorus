import { useState } from "react";
import { previewKey, type KeyMap } from "../../api";
import { Card, Keyboard, Text, type KeyboardKey } from "../../components";
import { usePressedKeys } from "../../hooks";
import { useMessages } from "../../i18n";

const SYSTEM_SOUND_PREFIX = "macos:";

const charKeys = (chars: string): KeyboardKey<string>[] =>
  [...chars].map((char) => ({ id: char, label: char.toUpperCase() }));

const LAYOUT: readonly (readonly KeyboardKey<string>[])[] = [
  [
    ...charKeys("`1234567890-="),
    { id: "backspace", label: "delete", size: "wide" },
  ],
  charKeys("qwertyuiop[]\\"),
  [...charKeys("asdfghjkl;'"), { id: "enter", label: "return", size: "wide" }],
  charKeys("zxcvbnm,./"),
  [{ id: "space", label: "space", size: "space" }],
];

const SPECIAL_KEYS = new Set(["backspace", "enter", "space"]);

const soundName = (spec: string) =>
  spec.startsWith(SYSTEM_SOUND_PREFIX)
    ? spec.slice(SYSTEM_SOUND_PREFIX.length)
    : (spec.split("/").pop() ?? spec);

type KeyMapPreviewProps = {
  keyMap: KeyMap;
};

export function KeyMapPreview({ keyMap }: KeyMapPreviewProps) {
  const text = useMessages().soundPack;
  const [hoveredId, setHoveredId] = useState<string | null>(null);
  const pressedKeys = usePressedKeys();

  const describe = (id: string) => {
    const assigned = keyMap.keys[id];
    if (assigned) {
      return soundName(assigned);
    }
    if (SPECIAL_KEYS.has(id) || keyMap.fallback.length === 0) {
      return null;
    }
    if (keyMap.fallback.length === 1) {
      return soundName(keyMap.fallback[0]);
    }
    return text.randomSound(keyMap.fallback.length);
  };

  const rows = LAYOUT.map((row) =>
    row.map((key) => ({ ...key, muted: describe(key.id) === null })),
  );

  const hoveredKey = LAYOUT.flat().find((key) => key.id === hoveredId);

  return (
    <Card title={text.keyMapTitle}>
      <Keyboard
        rows={rows}
        activeIds={pressedKeys}
        onPress={(id) => void previewKey(id)}
        onHover={setHoveredId}
      />
      <Text>
        {hoveredKey
          ? `${hoveredKey.label}: ${describe(hoveredKey.id) ?? text.noSound}`
          : text.keyMapHint}
      </Text>
    </Card>
  );
}
