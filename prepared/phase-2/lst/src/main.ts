import { Api, Change, Connection } from "./generated/lst.js";

interface EditorState {
  email: string;
  events: Change[];
}

class State {
  items: string[] = [];
  connected: Map<number, EditorState> = new Map();
  lastConnectionId: number = 0;

  constructor() {}

  isConnected(c: Connection): boolean {
    return state.connected.has(Number(c.id));
  }

  addEvent(event: Change): void {
    for (const editor of state.connected.values()) {
      editor.events.push(event);
    }
  }
}

let state: State = new State();

export const api: Api = {
  add(c: Connection, value: string): void {
    if (state.isConnected(c)) {
      state.items.push(value);
      state.addEvent({ tag: "added", val: value });
    } else {
      console.log("Invalid connection");
    }
  },
  delete(c: Connection, value: string): void {
    if (state.isConnected(c)) {
      state.items = state.items.filter((item) => item !== value);
      state.addEvent({ tag: "deleted", val: value });
    } else {
      console.log("Invalid connection");
    }
  },
  insert(c: Connection, after: string, value: string): void {
    if (state.isConnected(c)) {
      const index = state.items.findIndex((item) => item === after);
      if (index == -1) {
        api.add(c, value);
      } else {
        state.items.splice(index + 1, 0, value);
        state.addEvent({ tag: "inserted", val: { after, value } });
      }
    } else {
      console.log("Invalid connection");
    }
  },
  get(): string[] {
    return state.items;
  },
  connect(email: string): [Connection, string[]] {
    const id = state.lastConnectionId + 1;
    state.lastConnectionId += 1;
    state.connected.set(id, { email, events: [] });

    return [{id: BigInt(id)}, state.items];
  },
  disconnect(c: Connection): void {
    if (state.isConnected(c)) {
      state.connected.delete(Number(c.id));
    } else {
      console.log("Invalid connection");
    }
  },
  connectedEditors(): string[] {
    return Array.from(state.connected.values()).map((editor) => editor.email);
  },
  poll(c: Connection): Change[] {
    const editor = state.connected.get(Number(c.id));
    if (editor) {
      const events = editor.events;
      editor.events = [];
      return events;
    } else {
      console.log("Invalid connection");
      return [];
    }
  },
};
