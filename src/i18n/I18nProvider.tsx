import {
  createContext,
  useCallback,
  useEffect,
  useState,
  type ReactNode,
} from "react";
import {
  getLanguage,
  setLanguage,
  type Language,
  type LanguageState,
  type Locale,
} from "../api";
import { en } from "./en";
import { ja } from "./ja";
import type { Messages } from "./messages";

const MESSAGES: Record<Locale, Messages> = { ja, en };

type I18nContextValue = {
  messages: Messages;
  language: Language;
  locale: Locale;
  changeLanguage: (language: Language) => Promise<void>;
};

export const I18nContext = createContext<I18nContextValue | null>(null);

type I18nProviderProps = {
  children: ReactNode;
};

export function I18nProvider({ children }: I18nProviderProps) {
  const [state, setState] = useState<LanguageState | null>(null);

  useEffect(() => {
    let active = true;
    getLanguage().then((loaded) => {
      if (active) {
        setState(loaded);
      }
    });
    return () => {
      active = false;
    };
  }, []);

  useEffect(() => {
    if (state) {
      document.documentElement.lang = state.locale;
    }
  }, [state]);

  const changeLanguage = useCallback(async (language: Language) => {
    setState(await setLanguage(language));
  }, []);

  if (!state) {
    return null;
  }

  return (
    <I18nContext.Provider
      value={{
        messages: MESSAGES[state.locale],
        language: state.language,
        locale: state.locale,
        changeLanguage,
      }}
    >
      {children}
    </I18nContext.Provider>
  );
}
