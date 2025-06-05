import {  signal } from '/signal.js';

const table = new Map();

export function useStore(id, initial) {
    if (!table.has(id)) {
        table.set(id, signal(initial));
    }
    return table.get(id);
}