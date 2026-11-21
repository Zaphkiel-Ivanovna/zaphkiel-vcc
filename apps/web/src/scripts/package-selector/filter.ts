import { format } from '@/i18n/format';
import { filterInput, noMatch, packageList } from './elements';
import { messages } from './messages';

export function applyFilter(): void {
  const query = filterInput.value.trim();
  const needle = query.toLowerCase();
  const rows = [...packageList.querySelectorAll<HTMLElement>('.package')];
  const matches = rows.filter((row) => (row.dataset.search ?? '').includes(needle));
  for (const row of rows) {
    row.hidden = !matches.includes(row);
  }
  noMatch.hidden = matches.length > 0;
  noMatch.textContent = format(messages.noMatch, { query });
}
