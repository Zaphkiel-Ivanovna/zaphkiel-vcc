import type { Locale } from './locales';
import type { PluralForms } from './types';

type Values = Readonly<Record<string, string | number>>;

export function format(template: string, values: Values): string {
  let text = template;
  for (const [key, value] of Object.entries(values)) {
    text = text.replaceAll(`{${key}}`, String(value));
  }
  return text;
}

export function plural(
  locale: Locale,
  forms: PluralForms,
  values: Values & { count: number },
): string {
  const isOne = new Intl.PluralRules(locale).select(values.count) === 'one';
  return format(isOne ? forms.one : forms.other, values);
}
