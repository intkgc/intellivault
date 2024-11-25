import OpenAI from "openai";
import { APIPromise } from "openai/core";
import { ChatCompletionChunk, ChatCompletionMessageParam } from "openai/resources";
import { Stream } from "openai/streaming";
import { zodResponseFormat } from "openai/helpers/zod";
import { z } from "zod";

const DecisionFormat = z.object({
    action: z.string(),
    notes: z.number()
});

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

    async makeDecision(userMsg: string, model: string){
        const response = await this.openai.beta.chat.completions.parse({
            model: model,
            messages: [
                { role: "system", content: "You are a decision-making assistant in a knowledge base. Based on the user's query, decide which of the following actions is most appropriate. Reply with the action's key only and also number from 0.0 to 1.0 where 1.0 it is getting all notes and 0.0 is zero (e.g., 'retrieve_notes_and_analyze 0.3', or 'standard_reply 0')."},
                { role: "user", content: userMsg}
            ],
            response_format: zodResponseFormat(DecisionFormat, "decision"),
        })
        console.log("Пользователь: " + userMsg)
        console.log(response.choices[0].message.parsed)
        return response.choices[0].message.content ?? "Error";
    }

    async addToMessages(msg: ChatCompletionMessageParam){
        this.messages.push(msg)
    }
}