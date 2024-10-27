import { Plugin } from "obsidian";

export function addCommands(plugin: Plugin) {
	plugin.addCommand({
		id: "open-chat",
		name: "Open Chat",
		callback: () => plugin.activateView(),
	});
}