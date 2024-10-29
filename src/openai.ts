import OpenAI from "openai";

export class OpenAIGenerator {
    private openai: OpenAI;

    constructor(apiKey: string) {
        this.openai = new OpenAI({ apiKey, dangerouslyAllowBrowser: true});
    }

    async getMsg(userMsg: string) {
        const completion = await this.openai.chat.completions.create({
            model: "gpt-4o-mini",
            messages: [
                { role: "system", content: "You are a helpful assistant. You write in Russian" },
                { role: "user", content: userMsg}
            ],
        });
        return completion.choices[0].message.content ?? "Error";
    }
}