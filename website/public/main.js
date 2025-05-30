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
class BlockAdder extends HTMLElement {
    connectedCallback() {
        this.schemas = JSON.parse(this.dataset.schemas || '[]');
        this.postId = this.dataset.postId;
        this.select = document.createElement('select');
        this.form = document.createElement('form');
        this.preview = document.createElement('div');
        this.preview.className = 'preview';
        this.append(this.select, this.form, this.preview);
        this.schemas.forEach(s => {
            const opt = document.createElement('option');
            opt.value = s.block_type;
            opt.textContent = s.block_type;
            this.select.appendChild(opt);
        });
        this.select.addEventListener('change', () => this.renderFields());
        this.form.addEventListener('submit', e => this.submitBlock(e));
        this.renderFields();
    }

    renderFields() {
        const type = this.select.value;
        this.form.innerHTML = '';
        const schema = this.schemas.find(s => s.block_type === type);
        if (!schema) return;
        schema.fields.forEach(f => {
            const label = document.createElement('label');
            label.textContent = f.label;
            const input = document.createElement(f.form_type === 'InputArea' ? 'textarea' : 'input');
            input.name = f.name;
            this.form.append(label, input);
        });
        const btn = document.createElement('button');
        btn.type = 'submit';
        btn.textContent = 'Add Block';
        this.form.append(btn);
    }

    async submitBlock(e) {
        e.preventDefault();
        const type = this.select.value;
        const data = Object.fromEntries(new FormData(this.form).entries());
        const payload = { block_type: type, block_data: data };
        const res = await fetch(`/api/posts/${this.postId}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });
        if (res.ok) {
            this.dispatchEvent(new CustomEvent('blockAdded', { bubbles: true }));
            setTimeout(() => location.reload(), 1500);
        } else {
            alert('Failed to add block');
        }
    }
}

customElements.define('block-adder', BlockAdder);
