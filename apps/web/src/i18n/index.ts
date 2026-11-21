import { en } from './en';
import { fr } from './fr';
import { toLocale } from './locales';
import type { Messages } from './types';

const MESSAGES = { en, fr };

export function messagesFor(locale: string | undefined): Messages {
  return MESSAGES[toLocale(locale)];
}
