type ElementClass<Target extends Element> = new () => Target;

export function requireElement<Target extends Element>(
  root: ParentNode,
  selector: string,
  elementClass: ElementClass<Target>,
): Target {
  const element = root.querySelector(selector);
  if (!(element instanceof elementClass)) {
    throw new TypeError(`Missing element ${selector}`);
  }
  return element;
}

export function createLink(text: string, href: string | undefined): HTMLAnchorElement {
  const anchor = document.createElement('a');
  anchor.textContent = text;
  if (href !== undefined) {
    anchor.href = href;
    anchor.rel = 'noopener';
  }
  return anchor;
}
