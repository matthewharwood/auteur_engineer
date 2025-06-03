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

// jsaction dispatcher
document.addEventListener('click', e => {
    const act = e.target.getAttribute('jsaction');
    if (!act) return;
    const [evt, method] = act.split(':');
    if (evt !== 'click') return;
    const host = e.target.closest('[is="art-counter"]');
    if (host && typeof host[method] === 'function') host[method](e);
});

class ArtCounter extends HTMLFormElement {
    static observedAttributes = ['count'];

    connectedCallback() {
        this.updateText(this.getAttribute('count'));
    }

    inc(e) {
        e.preventDefault();
        const next = (+this.getAttribute('count') + 1).toString();
        this.setAttribute('count', next);
        const fd = new FormData(this);
        fd.set('action', 'inc');
        fetch(this.action, { method: this.method, body: fd });
    }

    dec(e) {
        e.preventDefault();
        const next = (+this.getAttribute('count') - 1).toString();
        this.setAttribute('count', next);
        const fd = new FormData(this);
        fd.set('action', 'dec');
        fetch(this.action, { method: this.method, body: fd });
    }

    attributeChangedCallback(name, _, val) {
        if (name === 'count') this.updateText(val);
    }
    updateText(val) {
        this.querySelector('button').textContent = `Increment (${val})`;
    }
}
customElements.define('art-counter', ArtCounter, { extends: 'form' });

function setupSockets() {
    document.querySelectorAll('script[data-for]').forEach(s => {
        const id = s.dataset.for;
        const socket = new WebSocket(`ws://${location.host}/ws/counter/${id}`);
        socket.addEventListener('message', e => {
            const msg = JSON.parse(e.data);
            const el = document.querySelector(`form[data-uid="${msg.id}"]`);
            if (el) el.setAttribute('count', msg.count);
        });
    });
}

setupSockets();
