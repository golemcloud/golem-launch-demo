import {agent, BaseAgent} from "@golemcloud/golem-ts-sdk";
import {ListAgent} from "./main";
import * as llm from 'golem:llm/llm@1.0.0';

@agent()
export class AiAgent extends BaseAgent {
    private readonly name: string;

    constructor(name: string) {
        super();
        this.name = name;
    }

    async extendItems() {
        const list = ListAgent.get(this.name);
        const currentItems = await list.get();

        let response = llm.send([{
                tag: "message",
                val: {
                    role: "user",
                    content: [{
                        tag: "text",
                        val: `We have a list of strings: ${currentItems.join(", ")}. 
                        Add 10 more items to this list, try to make them related to the existing ones. Return them as a valid JSON array.
                        Do not return anything else. The response should be parsed as JSON, with a single field 'fruits' containing the array of strings. 
                        Do not include items that are already listed.
                        `
                    }]
                }
            }],
            {
                model: "gpt-3.5-turbo",
                providerOptions: [{
                    key: "responseFormat",
                    value: "json_object"
                }]
            }
        );

        const raw =
            response.content.filter(c => c.tag === "text").map(c => c.val).join();
        try {
            const json: { fruits: string[] } = JSON.parse(raw);

            const connection = await list.connect("ai@golem.cloud");
            for (const item of json.fruits) {
                console.info(`Adding item ${item}`);
                await list.add(connection.id, item);
            }
            await list.disconnect(connection.id);

        } catch (err) {
            console.warn(`Failed to parse LLM's result: ${raw}: ${err}`)
        }
    }
}