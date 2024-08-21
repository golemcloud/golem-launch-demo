import { Api } from './generated/lst.js';

class State {
    items: string[] = [];

    constructor() {
    }
}

let state: State = new State();

export const api: Api = {
    add(value: string): void {
        state.items.push(value);
    },
    delete(value: string): void {
        state.items = state.items.filter(item => item !== value);
    },
    get(): string[] {
        return state.items;
    },
    insert(after: string, value: string): void {
        const index = state.items.findIndex(item => item === after);
        if (index == -1) {
            api.add(value);
        } else {
            state.items.splice(index + 1, 0, value);
        }
    }
}
