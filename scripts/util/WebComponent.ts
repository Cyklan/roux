import { process } from "htmx.org";
import {
  type CSSResultGroup,
  LitElement,
  unsafeCSS,
  PropertyValueMap,
} from "lit";
import styles from "../styles.css?inline";
/**
 * This file only exists to have a dry way of suppressing the shadow DOM.
 * We don't need the shadow dom, all cool interactions are done with HTMx.
 */
export class WebComponent extends LitElement {
  static styles?: CSSResultGroup | undefined = [unsafeCSS(styles)];

  protected updated(
    _changedProperties: PropertyValueMap<any> | Map<PropertyKey, unknown>
  ): void {
    // @ts-ignore
    process(this.shadowRoot?.children[0]);
  }
}
