import {
    BaseAgent,
    agent,
    Result
} from '@golemcloud/golem-ts-sdk';

import {ArchiveAgent} from "./archive";

type ClientId = number;

type Client = {
    id: ClientId,
    email: string
}

type InsertParams = {
    after: string,
    value: string,
}

export type Change = {
    tag: "added",
    val: string,
} | {
    tag: "deleted",
    val: string,
} | {
    tag: "inserted",
    val: InsertParams
}

type EditorState = {
    client: Client,
    changes: Change[]
}

type ListError = {
    tag: "invalid-client-id",
    clientId: ClientId,
} | {
    tag: "archived"
};

@agent()
class ListAgent extends BaseAgent {
    private readonly name: string;
    private readonly items: string[] = [];

    private readonly clients = new Map<number, EditorState>();

    private lastClientId: number = 0;
    private archived: boolean = false;

    constructor(name: string) {
        super()
        this.name = name;
    }

    connect(email: string): { id: ClientId, items: string[] } {
        this.lastClientId++;
        const id = this.lastClientId;
        this.clients.set(id, {
            client: {
                id: id,
                email: email
            },
            changes: []
        });
        return {id, items: this.items};
    }

    disconnect(id: ClientId): boolean {
        return this.clients.delete(id);
    }

    add(id: ClientId, item: string): Result<number, ListError> {
        return this.ensureCanEdit(id, () => {
            this.addEvent({tag: "added", val: item});
            this.items.push(item);
            return Result.ok(this.items.length);
        });
    }

    insert(id: ClientId, after: string, item: string): Result<number, ListError> {
        return this.ensureCanEdit(id, () => {
            const index = this.items.indexOf(after);
            if (index === -1) {
                return this.add(id, item);
            } else {
                this.addEvent({tag: "inserted", val: {after, value: item}})
                this.items.splice(index + 1, 0, item);
                return Result.ok(this.items.length);
            }
        });
    }

    delete(id: ClientId, item: string): Result<number, ListError> {
        return this.ensureCanEdit(id, () => {
            this.addEvent({tag: "deleted", val: item});
            this.items.filter(i => i !== item);
            return Result.ok(this.items.length);
        });
    }

    get(): string[] {
        return this.items;
    }

    poll(id: ClientId): Result<Change[], ListError> {
        return this.ensureCanEdit(id, () => {
            const state = this.clients.get(id)!;
            const changes = state.changes;
            state.changes = [];

            return Result.ok(changes);
        });
    }

    async archive(): Promise<boolean> {
        if (this.archived) {
            return false;
        }

        const archive = ArchiveAgent.get();
        await archive.add({
            name: this.name,
            items: this.items
        })

        this.archived = true;
        return true;
    }

    private ensureCanEdit<R>(id: ClientId, inner: () => Result<R, ListError>): Result<R, ListError> {
        if (this.archived) {
            return Result.err({
                tag: "archived"
            });
        }
        if (!this.clients.has(id)) {
            return Result.err({
                tag: "invalid-client-id",
                clientId: id
            });
        } else {
            return inner();
        }
    }

    private addEvent(event: Change) {
        console.debug("Adding event", event);

        for (const [_, state] of this.clients) {
            state.changes.push(event);
        }
    }
}
