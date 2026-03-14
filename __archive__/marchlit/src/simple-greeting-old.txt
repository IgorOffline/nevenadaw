import {css, html, LitElement} from 'lit';
import {customElement, property} from 'lit/decorators.js';
import 'daisyui/daisyui.css';

@customElement('simple-greeting')
export class SimpleGreeting extends LitElement {
    static styles = css`
        :host {
            display: block;
            margin: 1rem;
            color: blue;
        }
    `;

    @property()
    name?: string = 'World';

    render() {
        return html`
            <p>Hello, ${this.name}!</p>
        `;
    }
}
