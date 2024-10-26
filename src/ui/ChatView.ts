import { WorkspaceLeaf, ItemView, IconName } from "obsidian";

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
		return "Obsidian Assistent";
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

		const title = container.createEl("h2", { cls: 'intvault-title', text: "Obsidian AI Assistent" },);

		const messageArea = container.createDiv({cls: 'intvault-message-area'});
        const inputArea = container.createDiv({cls: 'intvault-input-area'});

		const updateMessages = () => {
			messageArea.empty();
			this.messages.forEach((msg) => {
				const message = messageArea.createDiv({cls: 'intvault-message'});
				message.addClass(msg.sender === "user" ? "sender-user" : "sender-bot");
				message.setText(`${msg.sender === "user" ? "Me" : "Obsidian"}: ${msg.text}`);
			});
		};

		const addMessage = (sender: string, text: string) => {
			this.messages.push({ sender, text });
			updateMessages();
		};

		const input = inputArea.createEl("input", { type: "text", placeholder: "Prompt", cls: "intvault-chat-input" });
		const sendButton = inputArea.createEl("button", {cls: "intvault-send-button"});
        
        const svg = sendButton.createSvg("svg", {
            attr: {
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                viewBox: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                "stroke-width": "2",
                "stroke-linecap": "round",
                "stroke-linejoin": "round"
            }
        });
        
        svg.createSvg("path", {
            attr: {
                d: "M3.714 3.048a.498.498 0 0 0-.683.627l2.843 7.627a2 2 0 0 1 0 1.396l-2.842 7.627a.498.498 0 0 0 .682.627l18-8.5a.5.5 0 0 0 0-.904z"
            }
        });
        
        svg.createSvg("path", {
            attr: {
                d: "M6 12h16"
            }
        });

        sendButton.onclick = () => {
            const userInput = input.value.trim();
            if (userInput) {
                addMessage("user", userInput);
                input.value = "";

                addMessage("bot", "Это фиксированный ответ от бота.");
            }
        };

		container.appendChild(messageArea);
        container.appendChild(inputArea);
		inputArea.appendChild(input);
		inputArea.appendChild(sendButton);
	}
}
