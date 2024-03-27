import { html } from "lit";
import { customElement, property } from "lit/decorators.js";
import { WebComponent } from "./util/WebComponent";

@customElement("my-element")
export class MyElement extends WebComponent {
  @property()
  name = "World!";

  render() {
    return html`<button
      hx-swap="outerHTML"
      hx-get="/my-element"
      hx-target="#foo"
      class="p-2 text-center border-slate-400 border-2"
    >
      Hi, ${this.name}
    </button>`;
  }
}
