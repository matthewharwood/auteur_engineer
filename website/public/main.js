const sys = window.matchMedia('(prefers-color-scheme: dark)'),
    key = 'user-theme-preference',
    apply = t => {
        document.documentElement.setAttribute(
            'data-theme',
            t==='system'
                ? (sys.matches ? 'dark' : 'light')
                : t
        );
    },
    save = t => localStorage.setItem(key, t),
    load = () => localStorage.getItem(key) || 'system';

class IconSelect extends HTMLElement {
    constructor() {
        super();
        const tpl = document.getElementById('icon-select-template');
        this.appendChild(tpl.content.cloneNode(true));

        this.btn      = this.querySelector('.trigger');
        this.dropdown = this.querySelector('.dropdown');
        this.options  = Array.from(this.querySelectorAll('li'));
        this.sunIcon  = this.querySelector('.sun');
        this.moonIcon = this.querySelector('.moon');

        this.current = load();
        apply(this.current);
        this._updateIcons();
    }

    connectedCallback() {
        this.btn.addEventListener('click', e => this._toggle());
        this.options.forEach(opt =>
            opt.addEventListener('click', e => this._select(e))
        );
        sys.addEventListener('change', () => {
            if (this.current === 'system') {
                apply('system');
                this._updateIcons();
            }
        });
        document.addEventListener('click', e => {
            if (!this.contains(e.target)) this._hide();
        });
    }

    _toggle() {
        const expanded = this.btn.getAttribute('aria-expanded') === 'true';
        this.btn.setAttribute('aria-expanded', String(!expanded));
        this.dropdown.setAttribute('aria-hidden', String(expanded));
    }

    _hide() {
        this.btn.setAttribute('aria-expanded', 'false');
        this.dropdown.setAttribute('aria-hidden', 'true');
    }

    _select(e) {
        this.current = e.currentTarget.dataset.value;
        save(this.current);
        apply(this.current);
        this._updateIcons();
        this._hide();
    }

    _updateIcons() {
        const isDark = this.current === 'dark' || (this.current==='system' && sys.matches);
        this.sunIcon .classList.toggle('show', !isDark);
        this.moonIcon.classList.toggle('show',  isDark);
    }
}

customElements.define('icon-select', IconSelect);

class CountingYear extends HTMLElement {
    constructor() {
        super();
        this.interval = null;
        this.starting_year = 1985;
        this.ending_year = 2025;
        this.total_years = this.ending_year - this.starting_year;
    }
    connectedCallback() {
        this.textContent = this.starting_year;
        this.interval = setInterval(() => {
            if(Number(this.textContent) >= this.ending_year){
                clearInterval(this.interval);
            }
            this.textContent = (Number(this.textContent) + 1).toString();
        }, 10)
    }
}

customElements.define('counting-year', CountingYear);

import { signal, effect, computed } from 'https://esm.sh/@preact/signals-core';

const table = new Map();
export function useCounter(id, initial = 0) {
    if (!table.has(id)) table.set(id, signal(Number(initial)));
    return table.get(id);
}

class IncButton extends HTMLButtonElement {
    connectedCallback() {
        this.addEventListener('click', () => {
            const id = this.getAttribute('counter-id');
            const initial = this.getAttribute('counter-initial') || 0;
            if (!id) return;
            const sig = useCounter(id, initial);
            sig.value = Number(sig.value) + 1;
            this.setAttribute('counter-value', sig.value);
        });
    }
}
customElements.define('inc-button', IncButton, { extends: 'button' });

class DecButton extends HTMLButtonElement {
    connectedCallback() {
        this.addEventListener('click', () => {
            const id = this.getAttribute('counter-id');
            const initial = this.getAttribute('counter-initial') || 0;
            if (!id) return;
            const sig = useCounter(id, initial);
            sig.value = Number(sig.value) - 1;
            this.setAttribute('counter-value', sig.value);
        });
    }
}
customElements.define('dec-button', DecButton, { extends: 'button' });

class ArtCounterValue extends HTMLParagraphElement {
    connectedCallback() {
        const id = this.getAttribute('counter-id');
        const initial = this.getAttribute('counter-initial') || 0;
        if (!id) return;
        const sig = useCounter(id, initial);
        this.dispose = effect(() => {
            this.textContent = String(sig.value);
        });
    }
    disconnectedCallback() {
        this.dispose?.();
    }
}
customElements.define('art-counter-value', ArtCounterValue, { extends: 'p' });

class JsAction extends HTMLFormElement {
    connectedCallback() {
        const id = this.getAttribute('counter-id');
        const initial = this.getAttribute('counter-initial') || 0;
        if (!id) return;
        const sig = useCounter(id, initial);
        const doubleValue = computed(() => sig.value * 2);
        this.dispose = effect(() => {
            this.setAttribute('counter-value', sig.value);
            this.setAttribute('double-value', doubleValue.value);
        });
    }
    disconnectedCallback() {
        this.dispose?.();
    }
}
customElements.define('js-action', JsAction, { extends: 'form' });

class CounterWs extends HTMLElement {
    connectedCallback() {
        const id = this.getAttribute('counter-id');
        if (!id) return;
        const sig = useCounter(id);
        const socket = new WebSocket(`ws://${location.host}/ws/counter/${id}`);
        socket.addEventListener('message', e => {
            const msg = JSON.parse(e.data);
            if (msg.id === id) {
                sig.value = Number(msg.count);
            }
        });
    }
}
customElements.define('counter-ws', CounterWs);
