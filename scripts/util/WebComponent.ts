import { LitElement } from "lit";
/**
 * This file only exists to have a dry way of suppressing the shadow DOM.
 * We don't need the shadow dom, all cool interactions are done with HTMx.
 */
export class WebComponent extends LitElement {
  protected createRenderRoot(): HTMLElement | DocumentFragment {
    return this;
  }
}
