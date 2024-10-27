import { Plugin, WorkspaceLeaf, ItemView } from "obsidian";
import { addCommands } from "./commands";
import { ChatView } from "./ui/ChatView"
const VIEW_TYPE_CHAT = "chat-view";

export default class ChatPlugin extends Plugin {
	async onload() {
		this.registerView(VIEW_TYPE_CHAT, (leaf) => new ChatView(leaf));

		addCommands(this);
	}

	async onunload() {
		this.app.workspace.detachLeavesOfType(VIEW_TYPE_CHAT);
	}

	async activateView() {
		this.app.workspace.detachLeavesOfType(VIEW_TYPE_CHAT);
		const leaf = this.app.workspace.getRightLeaf(false);
		await leaf.setViewState({ type: VIEW_TYPE_CHAT });
		this.app.workspace.revealLeaf(leaf);
	}
}
