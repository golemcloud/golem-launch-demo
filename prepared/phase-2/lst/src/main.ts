import {Api, Change, Connection} from './generated/lst.js';

interface EditorState {
  email: string;
  events: Change[];
}

class State {
  items: string[] = [];
  connected: Map<number, EditorState> = new Map();
  lastConnectionId: number = 0;

  constructor() {
  }

  isConnected(connection: Connection): boolean {
    return this.connected.has(Number(connection.id));
  }

  addEvent(event: Change): void {
    for (const editor of this.connected.values()) {
      editor.events.push(event);
    }
  }
}

let state = new State();

export const api: Api = {
  add(connection: Connection, value: string): void {
    if (state.isConnected(connection)) {
      console.log(`Adding item ${value}`);
      state.items.push(value);
      state.addEvent({tag: 'added', val: value});
    } else {
      console.log('Invalid connection');
    }
  },
  delete(connection: Connection, value: string): void {
    if (state.isConnected(connection)) {
      console.log(`Deleting item ${value}`);
      state.items = state.items.filter(item => item !== value);
      state.addEvent({tag: 'deleted', val: value});
    } else {
      console.log('Invalid connection');
    }
  },
  insert(connection: Connection, after: string, value: string): void {
    if (state.isConnected(connection)) {
      const index = state.items.indexOf(after);
      if (index === -1) {
        this.add(connection, value);
      } else {
        console.log(`Inserting item ${value} after ${after}`);
        state.items.splice(index + 1, 0, value);
        state.addEvent({tag: 'inserted', val: {after, value}});
      }
    } else {
      console.log('Invalid connection');
    }
  },
  get(): string[] {
    return state.items;
  },
  connect(email: string): [Connection, string[]] {
    const id = state.lastConnectionId + 1;
    state.lastConnectionId = id;
    state.connected.set(id, { email, events: [] });

    console.log(`User ${email} connected with id ${id}`);
    return [{ id: BigInt(id) }, state.items];
  },
  connectedEditors(): string[] {
    return Array.from(state.connected.values()).map(editor => editor.email);
  },
  disconnect(c: Connection): void {
    if (state.connected.has(Number(c.id))) {
      state.connected.delete(Number(c.id));
      console.log(`Connection ${c.id} closed`);
    } else {
      console.log('Invalid connection');
    }
  },
  poll(c: Connection): Change[] {
    const editor = state.connected.get(Number(c.id));
    if (editor) {
      console.log(`Returning events for connection ${c.id}`);

      const events = editor.events;
      editor.events = [];
      return events;
    } else {
      console.log('Invalid connection');
      return [];
    }
  }

}
