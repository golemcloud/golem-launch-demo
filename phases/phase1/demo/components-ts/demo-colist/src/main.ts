import {
    BaseAgent,
    agent
} from '@golemcloud/golem-ts-sdk';

@agent()
class ListAgent extends BaseAgent {
    private readonly name: string;
    private readonly items: string[] = [];

    constructor(name: string) {
        super()
        this.name = name;
    }

    add(item: string): number {
        this.items.push(item);
        return this.items.length;
    }

    insert(after: string, item: string): number {
        const index = this.items.indexOf(after);
        if (index === -1) {
            return this.add(item);
        } else {
            this.items.splice(index + 1, 0, item);
            return this.items.length;
        }
    }

    delete(item: string): number {
        this.items.filter(i => i !== item);
        return this.items.length;
    }

    get() {
        return this.items;
    }
}
