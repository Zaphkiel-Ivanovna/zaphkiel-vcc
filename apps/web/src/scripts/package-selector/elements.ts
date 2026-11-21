import { requireElement } from './dom';

export const urlInput = requireElement(document, '#url', HTMLInputElement);
export const copyButton = requireElement(document, '#copy', HTMLButtonElement);
export const addLink = requireElement(document, '#add', HTMLAnchorElement);
export const summary = requireElement(document, '#summary', HTMLElement);
export const toggleButton = requireElement(document, '#toggle', HTMLButtonElement);
export const packageList = requireElement(document, '#packages', HTMLElement);
export const packageTemplate = requireElement(document, '#package', HTMLTemplateElement);
