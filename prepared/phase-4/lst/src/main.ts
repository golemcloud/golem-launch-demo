import {Api, Change, Connection, EmailQuery} from './generated/lst.js';
import {after} from "node:test";
import {getEnvironment} from "wasi:cli/environment@0.2.0";
import {Api as ArchiveApi} from "demo:archive-stub/stub-archive";
import {Api as EmailApi} from "demo:email-stub/stub-email";

interface EditorState {
    email: string,
    events: Change[]
}

class State {
    items: string[] = [];
    connected: Map<number, EditorState> = new Map();
    lastConnectionId: number = 0;

    archived: boolean = false;

    emailDeadline: number = 0;
    emailRecipients: string[] = [];

    initialized: boolean = false;
    name: string = "unknown";
    emailComponentId: string = "unknown";

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

    updateEmailProperties(): void {
        const now = Date.now();
        const sendAt = now + 1000 * 60 * 60 * 12; // 12 hours
        const emailList = Array.from(state.connected.values()).map(editor => editor.email);

        this.emailDeadline = sendAt;
        this.emailRecipients = emailList;
    }

    ensureInitialized(): void {
        if (!this.initialized) {
            const env: [string, string][] = getEnvironment();

            this.name = env.find(([key, _]) => key === "GOLEM_WORKER_NAME")?.[1] ?? "unknown";
            this.emailComponentId = env.find(([key, _]) => key === "EMAIL_COMPONENT_ID")?.[1] ?? "unknown";

            this.updateEmailProperties();

            const uri = {value: `urn:worker:${this.emailComponentId}/${this.name}`};
            const emailApi = new EmailApi(uri);

            const selfComponentId = env.find(([key, _]) => key === "GOLEM_COMPONENT_ID")?.[1] ?? "unknown";
            const selfUri = `urn:worker:${selfComponentId}/${this.name}`;
            emailApi.sendEmail({value: selfUri});

            this.initialized = true;
        }
    }
}

let state: State = new State();

export const api: Api = {
    add(c: Connection, value: string): void {
        state.ensureInitialized();

        if (!state.archived && state.isConnected(c)) {
            state.updateEmailProperties();
            state.items.push(value);
            state.addEvent({ tag: 'added', val: value });
        } else {
            console.log("Invalid connection or list is archived");
        }
    },
    delete(c: Connection, value: string): void {
        state.ensureInitialized();

        if (!state.archived && state.isConnected(c)) {
            state.updateEmailProperties();
            state.items = state.items.filter(item => item !== value);
            state.addEvent({ tag: 'deleted', val: value });
        } else {
            console.log("Invalid connection or list is archived");
        }
    },
    insert(c: Connection, after: string, value: string): void {
        state.ensureInitialized();

        if (!state.archived && state.isConnected(c)) {
            const index = state.items.findIndex(item => item === after);
            if (index == -1) {
                api.add(c, value);
            } else {
                state.updateEmailProperties();
                state.items.splice(index + 1, 0, value);
                state.addEvent({ tag: 'inserted', val: { after, value } });
            }
        } else {
            console.log("Invalid connection or list is archived");
        }
    },
    get(): string[] {
        state.ensureInitialized();

        return state.items;
    },
    connect(email: string): Connection {
        state.ensureInitialized();

        const id = state.lastConnectionId + 1;
        state.lastConnectionId += 1;
        state.connected.set(id, { email, events: [] });
        state.updateEmailProperties();

        return { id: BigInt(id) };
    },
    disconnect(c: Connection): void {
        state.ensureInitialized();

        if (state.isConnected(c)) {
            state.connected.delete(Number(c.id));
            state.updateEmailProperties();
        } else {
            console.log("Invalid connection");
        }
    },
    connectedEditors(): string[] {
        state.ensureInitialized();

        return Array.from(state.connected.values()).map(editor => editor.email);
    },
    poll(c: Connection): Change[] {
        state.ensureInitialized();

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
        state.ensureInitialized();

        state.archived = true;

        const env: [string, string][] = getEnvironment()

        const name = env.find(([key, _]) => key === "GOLEM_WORKER_NAME")?.[1] ?? "unknown";
        const component_id = env.find(([key, _]) => key === "ARCHIVE_COMPONENT_ID")?.[1] ?? "unknown";

        const uri = { value: `urn:worker:${component_id}/archive` };
        const archiveApi = new ArchiveApi(uri);
        archiveApi.store(name, state.items);
    },
    isArchived(): boolean {
        state.ensureInitialized();

        return state.archived;
    }
}

export const emailQuery: EmailQuery = {
    deadline(): bigint | undefined {
        state.ensureInitialized();

        if (state.archived) {
            return undefined;
        } else {
            return BigInt(state.emailDeadline);
        }
    },
    recipients(): string[] {
        state.ensureInitialized();

        return state.emailRecipients;
    }
}