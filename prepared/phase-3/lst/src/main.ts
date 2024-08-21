import {Api, Change, Connection} from './generated/lst.js';
import {after} from "node:test";
import {getEnvironment} from "wasi:cli/environment@0.2.0";
import {Api as ArchiveApi} from "demo:archive-stub/stub-archive";

interface EditorState {
    email: string,
    events: Change[]
}

class State {
    items: string[] = [];
    connected: Map<number, EditorState> = new Map();
    lastConnectionId: number = 0;

    archived: boolean = false;

    constructor() {
    }

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
        if (!state.archived && state.isConnected(c)) {
            state.items.push(value);
            state.addEvent({ tag: 'added', val: value });
        } else {
            console.log("Invalid connection or list is archived");
        }
    },
    delete(c: Connection, value: string): void {
        if (!state.archived && state.isConnected(c)) {
            state.items = state.items.filter(item => item !== value);
            state.addEvent({ tag: 'deleted', val: value });
        } else {
            console.log("Invalid connection or list is archived");
        }
    },
    insert(c: Connection, after: string, value: string): void {
        if (!state.archived && state.isConnected(c)) {
            const index = state.items.findIndex(item => item === after);
            if (index == -1) {
                api.add(c, value);
            } else {
                state.items.splice(index + 1, 0, value);
                state.addEvent({ tag: 'inserted', val: { after, value } });
            }
        } else {
            console.log("Invalid connection or list is archived");
        }
    },
    get(): string[] {
        return state.items;
    },
    connect(email: string): Connection {
        const id = state.lastConnectionId + 1;
        state.lastConnectionId += 1;
        state.connected.set(id, { email, events: [] });

        return { id: BigInt(id) };
    },
    disconnect(c: Connection): void {
        if (state.isConnected(c)) {
            state.connected.delete(Number(c.id));
        } else {
            console.log("Invalid connection");
        }
    },
    connectedEditors(): string[] {
        return Array.from(state.connected.values()).map(editor => editor.email);
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
    archive() {
        state.archived = true;

        const env: [string, string][] = getEnvironment()

        const name = env.find(([key, _]) => key === "GOLEM_WORKER_NAME")?.[1] ?? "unknown";
        const component_id = env.find(([key, _]) => key === "ARCHIVE_COMPONENT_ID")?.[1] ?? "unknown";

        const uri = { value: `urn:worker:${component_id}/archive` };
        const archiveApi = new ArchiveApi(uri);
        archiveApi.store(name, state.items);
    },
    isArchived(): boolean {
        return state.archived;
    }
}
