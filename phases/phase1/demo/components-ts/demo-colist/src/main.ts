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

    add(item: string) {
        this.items.push(item);
    }

    insert(after: string, item: string) {
        const index = this.items.indexOf(after);
        if (index === -1) {
            this.add(item);
        } else {
            this.items.splice(index + 1, 0, item);
        }
    }

    delete(item: string) {
        this.items.filter(i => i !== item);
    }

    get() {
        return this.items;
    }
}
