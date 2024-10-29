import { Plugin, WorkspaceLeaf, ItemView, PluginSettingTab } from "obsidian";
import { addCommands } from "./commands";
import { ChatView } from "./ui/ChatView"
import { SettingTab } from './ui/SettingTab';
import {OpenAIGenerator} from './openai'
const VIEW_TYPE_CHAT = "chat-view";


interface PluginSettings {
  chatgpt_api_key: string;
}

const DEFAULT_SETTINGS: Partial<PluginSettings> = {
	chatgpt_api_key: '',
};

export default class ChatPlugin extends Plugin {
	settings: PluginSettings;
	openaigenerator: OpenAIGenerator
	async onload() {
		await this.loadSettings();
		this.openaigenerator = new OpenAIGenerator(this.settings.chatgpt_api_key);
		this.registerView(VIEW_TYPE_CHAT, (leaf) => new ChatView(leaf, this));
		this.addSettingTab(new SettingTab(this.app, this));
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
	
	async loadSettings() {
		this.settings = Object.assign({}, DEFAULT_SETTINGS, await this.loadData());
	}

	async saveSettings() {
		await this.saveData(this.settings);
	}
}
