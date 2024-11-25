import ChatPlugin from '../main';
import { App, PluginSettingTab, Setting } from 'obsidian';

export class SettingTab extends PluginSettingTab {
  plugin: ChatPlugin;

  constructor(app: App, plugin: ChatPlugin) {
    super(app, plugin);
    this.plugin = plugin;
  }

  display(): void {
    let { containerEl } = this;

    containerEl.empty();

    new Setting(containerEl)
      .setName('ChatGPT API Token')
      .setDesc('You can get it on https://platform.openai.com/')
      .addText((text) =>
        text
          .setPlaceholder('You API token')
          .setValue(this.plugin.settings.chatgpt_api_key)
          .onChange(async (value) => {
            this.plugin.settings.chatgpt_api_key = value;
            await this.plugin.saveSettings();
          })
      );
    new Setting(containerEl)
      .setName('Chat-GPT Model')
      .setDesc('You can find all models here https://platform.openai.com/docs/models')
      .addText((text) =>
        text
          .setPlaceholder('gpt-4o-mini for example')
          .setValue(this.plugin.settings.model)
          .onChange(async (value) => {
            this.plugin.settings.model = value;
            await this.plugin.saveSettings();
          })
      );
  }
}