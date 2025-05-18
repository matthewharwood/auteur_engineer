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
        this.interval = null;
        this.starting_year = 1985;
        this.ending_year = 2025;
        this.total_years = this.ending_year - this.starting_year;
    }
    connectedCallback() {
        this.textContent = this.starting_year;
        this.interval = setInterval(() => {
            if(Number(this.textContent) >= this.ending_year){
                this.interval.cancel();
            }
            this.textContent = (Number(this.textContent) + 1).toString();
        }, 10)
    }
}

customElements.define('counting-year', CountingYear);