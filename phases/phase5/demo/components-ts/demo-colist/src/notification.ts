import {agent, BaseAgent} from "@golemcloud/golem-ts-sdk";
import {ListAgent} from "./main";
import * as process from 'node:process';

@agent()
export class NotificationAgent extends BaseAgent {
    private readonly name: string;

    constructor(name: string) {
        super()
        this.name = name;
    }

    async run() {
        let finished = false;
        let list = ListAgent.get(this.name);
        let last = undefined;

        while (!finished) {
            console.debug(`Asking list ${this.name} for deadline`);
            const result = await list.getCurrentDeadline();
            if (result === undefined) {
                console.debug(`List ${this.name} is archived, closing notification agent`);
                finished = true;
            } else {
                const now = Date.now();
                if (now > result.deadline) {
                    if (result.deadline !== last) {
                        last = result.deadline;
                        console.debug(`List ${this.name} deadline reached, sending notifications`);
                        await this.sendNotifications(result.recipients);
                    } else {
                        // No news, check back periodically
                        await new Promise(resolve => setTimeout(resolve, 5000));
                    }
                } else {
                    await new Promise(resolve => setTimeout(resolve, result.deadline - now));
                }
            }
        }
    }

    private async sendNotifications(recipients: string[]) {
        console.info(`Sending e-mail to ${recipients.join(", ")}`);
        for (const recipient of recipients) {
            await this.sendNotification(recipient);
        }
    }

    private async sendNotification(recipient: string) {
        const apiKey = process.env.SENDGRID_API_KEY;
        const response = await fetch("https://api.sendgrid.com/v3/mail/send", {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${apiKey}`,
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                personalizations: [{to: [{email: recipient}]}],
                from: {email: "daniel.vigovszky@gmail.com"},
                subject: "CoList Notification",
                content: [{type: "text/plain", value: `List ${this.name} has not been modified for a while!`}]
            })
        });
        console.debug(`SendGrid status: ${response.status}`);
        if (response.status >= 400) {
            const body = await response.text();
            console.error("SendGrid failure:", body);
        }
    }
}