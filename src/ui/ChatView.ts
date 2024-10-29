import { WorkspaceLeaf, ItemView, IconName, addIcon, setIcon } from "obsidian";
import { compileFunction } from "vm";

const VIEW_TYPE_CHAT = "chat-view";

export class ChatView extends ItemView {
	messages: { sender: string; text: string }[] = [];

	constructor(leaf: WorkspaceLeaf) {
		super(leaf);
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

		const updateMessages = () => {
			messageArea.empty();
			this.messages.forEach((msg) => {
				const message = messageArea.createDiv({cls: 'intvault-message'});
				message.addClass(msg.sender === "user" ? "sender-user" : "sender-bot");
				
				if(msg.sender === "user"){
					setIcon(message, "user-round")
				} else if (msg.sender === "bot") {
					setIcon(message, "bot");
				}
				const text = message.createDiv({cls: "intvault-message-content"});
				text.setText(`${msg.text}`);
			});
		};

		const addMessage = (sender: string, text: string) => {
			this.messages.push({ sender, text });
			updateMessages();
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
                addMessage("bot", "Это фиксированный ответ от бота.");
            }
        };

		container.appendChild(messageArea);
        container.appendChild(inputArea);
		inputArea.appendChild(input);
		inputArea.appendChild(sendButton);
	}
}
