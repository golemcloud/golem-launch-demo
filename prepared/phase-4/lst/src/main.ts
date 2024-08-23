import {Api, Change, Connection, EmailQuery} from './generated/lst.js';
import {getEnvironment} from "wasi:cli/environment@0.2.0";
import {Api as ArchiveApi} from "demo:archive-stub/stub-archive";
import {Api as EmailApi} from "demo:email-stub/stub-email";

interface EditorState {
    email: string;
    events: Change[];
}

class State {
    items: string[] = [];
    connected: Map<number, EditorState> = new Map();
    lastConnectionId: number = 0;

    archived: boolean = false;

    emailDeadline: number = 0;
    emailRecipients: string[] = [];

    initialized: boolean = false;

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

            const name = env.find(([key, _]) => key === "GOLEM_WORKER_NAME")?.[1] ?? "unknown";
            const emailComponentId = env.find(([key, _]) => key === "EMAIL_COMPONENT_ID")?.[1] ?? "unknown";

            this.updateEmailProperties();

            const uri = {value: `urn:worker:${emailComponentId}/${name}`};
            const emailApi = new EmailApi(uri);

            const selfComponentId = env.find(([key, _]) => key === "GOLEM_COMPONENT_ID")?.[1] ?? "unknown";
            const selfUri = `urn:worker:${selfComponentId}/${name}`;
            emailApi.sendEmail({value: selfUri});

            this.initialized = true;
        }
    }
}

let state = new State();

export const api: Api = {
    add(connection: Connection, value: string): void {
        state.ensureInitialized();

        if (!state.archived && state.isConnected(connection)) {
            console.log(`Adding item ${value}`);
            state.items.push(value);
            state.addEvent({tag: 'added', val: value});
        } else {
            console.log('Invalid connection or already archived');
        }
    },
    delete(connection: Connection, value: string): void {
        state.ensureInitialized();

        if (!state.archived && state.isConnected(connection)) {
            console.log(`Deleting item ${value}`);
            state.items = state.items.filter(item => item !== value);
            state.addEvent({tag: 'deleted', val: value});
        } else {
            console.log('Invalid connection or already archived');
        }
    },
    insert(connection: Connection, after: string, value: string): void {
        state.ensureInitialized();

        if (!state.archived && state.isConnected(connection)) {
            const index = state.items.indexOf(after);
            if (index === -1) {
                this.add(connection, value);
            } else {
                console.log(`Inserting item ${value} after ${after}`);
                state.items.splice(index + 1, 0, value);
                state.addEvent({tag: 'inserted', val: {after, value}});
            }
        } else {
            console.log('Invalid connection or already archived');
        }
    },
    get(): string[] {
        state.ensureInitialized();

        return state.items;
    },
    connect(email: string): [Connection, string[]] {
        state.ensureInitialized();

        const id = state.lastConnectionId + 1;
        state.lastConnectionId = id;
        state.connected.set(id, { email, events: [] });

        console.log(`User ${email} connected with id ${id}`);
        return [{ id: BigInt(id) }, state.items];
    },
    connectedEditors(): string[] {
        state.ensureInitialized();

        return Array.from(state.connected.values()).map(editor => editor.email);
    },
    disconnect(c: Connection): void {
        state.ensureInitialized();

        if (state.connected.has(Number(c.id))) {
            state.connected.delete(Number(c.id));
            console.log(`Connection ${c.id} closed`);
        } else {
            console.log('Invalid connection');
        }
    },
    poll(c: Connection): Change[] {
        state.ensureInitialized();

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
    },
    archive(): void {
        state.ensureInitialized();

        state.archived = true;

        const env: [string, string][] = getEnvironment()

        const name = env.find(([key, _]) => key === "GOLEM_WORKER_NAME")?.[1] ?? "unknown";
        const component_id = env.find(([key, _]) => key === "ARCHIVE_COMPONENT_ID")?.[1] ?? "unknown";

        const uri = {value: `urn:worker:${component_id}/archive`};
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
            console.log(`Returning no deadline because list is already archived`);

            return undefined;
        } else {
            console.log(`Returning deadline ${state.emailDeadline}`);

            return BigInt(state.emailDeadline);
        }
    },
    recipients(): string[] {
        state.ensureInitialized();

        console.log(`Returning recipients ${state.emailRecipients}`);

        return state.emailRecipients;
    }
}