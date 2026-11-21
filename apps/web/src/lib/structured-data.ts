import { AUTHOR_URL, SITE_AUTHOR, SITE_TITLE } from '@/consts';
import { LOCALES } from '@/i18n/locales';

interface PageDescription {
  description: string;
  locale: string;
  site: string;
  title: string;
  url: string;
}

export function pageStructuredData(page: PageDescription): string {
  const authorId = `${page.site}#author`;
  const websiteId = `${page.site}#website`;
  return JSON.stringify({
    '@context': 'https://schema.org',
    '@graph': [
      {
        '@id': websiteId,
        '@type': 'WebSite',
        inLanguage: LOCALES,
        name: SITE_TITLE,
        publisher: { '@id': authorId },
        url: page.site,
      },
      { '@id': authorId, '@type': 'Person', name: SITE_AUTHOR, url: AUTHOR_URL },
      {
        '@id': `${page.url}#page`,
        '@type': 'CollectionPage',
        author: { '@id': authorId },
        description: page.description,
        inLanguage: page.locale,
        isPartOf: { '@id': websiteId },
        name: page.title,
        url: page.url,
      },
    ],
  });
}
