import OpenAI from "openai";
import { APIPromise } from "openai/core";
import { ChatCompletionChunk, ChatCompletionMessageParam } from "openai/resources";
import { Stream } from "openai/streaming";

export class OpenAIGenerator {
    private openai: OpenAI;
    private messages: Array<ChatCompletionMessageParam> = [
        { role: "system", content: "You are a helpful assistant. You write in Russian" }
    ];
    constructor(apiKey: string) {
        this.openai = new OpenAI({ apiKey, dangerouslyAllowBrowser: true});
    }

    async getMsg(userMsg: string, model: string) {
        this.addToMessages({role: "user", content: userMsg});
        const response = await this.openai.chat.completions.create({
            model: model,
            messages: this.messages,
            stream: true
        });
        return response;
    }
    async addToMessages(msg: ChatCompletionMessageParam){
        this.messages.push(msg)
    }
}