import { customElement, property } from "lit/decorators.js";
import { WebComponent } from "./util/WebComponent";
import { html } from "lit";

@customElement("htmx-counter")
export class HTMXCounter extends WebComponent {
  @property()
  count = 0;

  render() {
    // treat the returned html in a react fashion, only returning one "root" element with multiple childrens
    return html`<button
      hx-swap="outerHTML"
      hx-get="/counter/${this.count}"
      hx-target="#counter"
      class="text-main bg-button hover:bg-button-hover font-bold py-2 px-4 rounded"
    >
      Clicked ${this.count} times
    </button>`;
  }
}
