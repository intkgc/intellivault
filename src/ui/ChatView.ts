import { WorkspaceLeaf, ItemView, IconName, addIcon, setIcon, Plugin, MarkdownRenderer } from "obsidian";
import { compileFunction } from "vm";
import ChatPlugin from '../main';
import OpenAI from "openai"; 
import { APIPromise } from "openai/core";
import { Stream } from "openai/streaming";
import { ChatCompletionChunk } from "openai/resources";
import { OpenAIGenerator } from "src/openai";

const VIEW_TYPE_CHAT = "chat-view";

export class ChatView extends ItemView {
	private plugin: ChatPlugin;
	openaigenerator: OpenAIGenerator;

	constructor(leaf: WorkspaceLeaf, plugin: ChatPlugin) {
		super(leaf);
		this.plugin = plugin;
		this.openaigenerator = new OpenAIGenerator(this.plugin.settings.chatgpt_api_key);
		this.buildUI();
	}

	getViewType() {
		return VIEW_TYPE_CHAT;
	}

	getDisplayText() {
		return "Obsidian Assistant";
	}
    getIcon(): IconName {   
        return "bot-message-square";
    }

	async onOpen() {
		this.buildUI();
	}

	async onClose() {
		this.contentEl.empty();
	}


	buildUI() {
		const container = this.contentEl;
		container.empty();
        container.addClass("intvault-chat-container")

		const title = container.createEl("h2", { cls: 'intvault-title', text: "Obsidian AI Assistant" },);

		const messageArea = container.createDiv({cls: 'intvault-message-area'});
        const inputArea = container.createDiv({cls: 'intvault-input-area'});

		const updateMessages = async (sender: string, text: APIPromise<Stream<ChatCompletionChunk>> | string) => {
				const message = messageArea.createDiv({cls: 'intvault-message'});
				const iconDiv = message.createDiv({cls: 'intvault-message-icon'})
				if(sender === "user"){
					setIcon(iconDiv, "user-round")
				} else if (sender === "bot") {
					setIcon(iconDiv, "bot");
				}
				
				const textView = message.createDiv({cls: "intvault-message-content"});
				textView.addClass(sender === "user" ? "sender-user" : "sender-bot");

				if(sender === "bot"){
					let markdownText = "";
					for await (const chunk of text) {
						textView.innerHTML = "";
						markdownText += (chunk.choices[0]?.delta?.content || "");
						MarkdownRenderer.render(this.app, markdownText, textView, "", null);
					}
					this.openaigenerator.addToMessages({role: "assistant", content: markdownText})
				} else {
					MarkdownRenderer.render(this.app, text, textView, "", null);
				}
		};

		const addMessage = (sender: string, text: APIPromise<Stream<ChatCompletionChunk>> | string) => {
			updateMessages(sender, text);
		};
		const textAreaContainer = inputArea.createDiv({cls: "intvault-textarea-container"})

		const input = inputArea.createEl("textarea", {type: "text", placeholder: "Prompt", cls: "intvault-chat-input", attr: {rows: 1}});
		input.addEventListener('input', () => {
			input.style.height = 'auto';
			input.style.height = `${input.scrollHeight}px`;
			
			if (input.scrollHeight > input.clientHeight) {
				input.style.overflowY = "auto";
			} else {
				input.style.overflowY = 'hidden';
			}
		});
		const sendButton = inputArea.createEl("button", {cls: "intvault-send-button"});
        
       setIcon(sendButton, "send-horizontal");

        sendButton.onclick = () => {
            const userInput = input.value.trim();	
            if (userInput) {
                addMessage("user", userInput);
                input.value = "";
				input.style.height = "auto";
                this.openaigenerator.getMsg(userInput).then(response => {
					addMessage("bot", response);
				}).catch(error => {
					console.error(error);
				});;
            }
        };

		container.appendChild(messageArea);
        container.appendChild(inputArea);
		inputArea.appendChild(input);
		inputArea.appendChild(sendButton);
	}
}



