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
				if(msg.sender === "user"){
					const userIcon = message.createSvg("svg", {
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
					userIcon.createSvg("circle", {
						attr: {
							cx: "12",
							cy:"8",
							r:"5"
						}
					})
					userIcon.createSvg("path", {
						attr: {
							d: "M20 21a8 8 0 0 0-16 0"
						}
					});
				} else if (msg.sender === "bot") {
					const botIcon = message.createSvg("svg", {
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
					botIcon.createSvg("path", {
						attr: {
							d: "M12 8V4H8"
						}
					})
					botIcon.createSvg("rect", {
						attr: {
							width:"16",
							height:"12",
							x:"4",
							y:"8",
							rx:"2"
						}
					});
					botIcon.createSvg("path", {
						attr: {
							d: "M2 14h2"
						}
					})
					botIcon.createSvg("path", {
						attr: {
							d: "M20 14h2"
						}
					})
					botIcon.createSvg("path", {
						attr: {
							d: "M15 13v2"
						}
					})
					botIcon.createSvg("path", {
						attr: {
							d: "M9 13v2"
						}
					})
					
				}
				const text = message.createDiv({cls: "intvault-message-content"});
				text.setText(`${msg.text}`);
			});
		};

		const addMessage = (sender: string, text: string) => {
			this.messages.push({ sender, text });
			updateMessages();
		};

		const input = inputArea.createEl("textarea", {type: "text", placeholder: "Prompt", cls: "intvault-chat-input" });
		input.addEventListener('input', () => {
			input.style.height = 'auto';
			input.style.height = `${input.scrollHeight}px`;
			if (input.scrollHeight > 240) {
				input.style.height = '240px'; // 240 - это максимум для 10 строк
				input.style.overflowY = 'auto'; // Включаем прокрутку, если текст превышает максимум
			} else {
				input.style.overflowY = 'hidden'; // Убираем прокрутку, если текст в пределах максимума
			}
		});
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
