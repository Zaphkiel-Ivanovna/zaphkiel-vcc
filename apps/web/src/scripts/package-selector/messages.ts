import { messagesFor } from '@/i18n';
import { localePath, toLocale } from '@/i18n/locales';

export const locale = toLocale(document.documentElement.lang);
export const messages = messagesFor(locale).client;
export const homePath = localePath(locale);
