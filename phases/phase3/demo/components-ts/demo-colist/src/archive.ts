import {agent, BaseAgent} from "@golemcloud/golem-ts-sdk";

export type ArchivedList = {
    name: string,
    items: string[]
}

@agent()
export class ArchiveAgent extends BaseAgent {
    private readonly archive: ArchivedList[] = [];

    add(list: ArchivedList) {
        this.archive.push(list);
    }

    getAll(): ArchivedList[] {
        return this.archive;
    }
}
