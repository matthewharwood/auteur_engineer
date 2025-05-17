const sel=document.getElementById('theme-select'),
    root=document.documentElement,
    sys=window.matchMedia('(prefers-color-scheme: dark)'),
    key='user-theme-preference',
    apply=(t)=>{
        root.setAttribute('data-theme',t==='system' ? (sys.matches?'dark':'light') : t);
        sel.value=t;
    },
    save=(t)=>localStorage.setItem(key,t),
    load=()=>localStorage.getItem(key);

apply(load()||'system');

sel.addEventListener('change',e => {
    apply(e.target.value);save(e.target.value)
});
sys.addEventListener('change', () => sel.value==='system' && apply('system'));



class CountingYear extends HTMLElement {
    constructor() {
        super();
        this.year = 1985;
    }
    connectedCallback() {
        const initialText = this.textContent.trim();

    }
}

customElements.define('counting-year', CountingYear);