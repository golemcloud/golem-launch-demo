import {Api} from './generated/lst.js';

class State {
    items: string[] = [];

    constructor() {
    }
}

let state = new State();

export const api: Api = {
    add(value: string): void {
        console.log(`Adding item ${value}`);
        state.items.push(value);
    },
    delete(value: string): void {
        console.log(`Deleting item ${value}`);
        state.items = state.items.filter(item => item !== value);
    },
    insert(after: string, value: string): void {
        const index = state.items.indexOf(after);
        if (index === -1) {
            this.add(value);
        } else {
            console.log(`Inserting item ${value} after ${after}`);
            state.items.splice(index + 1, 0, value);
        }
    },
    get(): string[] {
        return state.items;
    }

}
