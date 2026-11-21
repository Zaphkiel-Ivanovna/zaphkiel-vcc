import { requireElement } from './dom';

export const urlInput = requireElement(document, '#url', HTMLInputElement);
export const copyButton = requireElement(document, '#copy', HTMLButtonElement);
export const addLink = requireElement(document, '#add', HTMLAnchorElement);
export const summary = requireElement(document, '#summary', HTMLElement);
export const selectedList = requireElement(document, '#selected', HTMLElement);
export const toggleButton = requireElement(document, '#toggle', HTMLButtonElement);
export const retryButton = requireElement(document, '#retry', HTMLButtonElement);
export const stats = requireElement(document, '#stats', HTMLElement);
export const filterField = requireElement(document, '#filter-field', HTMLElement);
export const filterInput = requireElement(document, '#filter', HTMLInputElement);
export const noMatch = requireElement(document, '#no-match', HTMLElement);
export const packageList = requireElement(document, '#packages', HTMLElement);
export const packageTemplate = requireElement(document, '#package', HTMLTemplateElement);
