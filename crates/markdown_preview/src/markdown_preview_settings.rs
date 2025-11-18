use settings::RegisterSetting;

#[derive(Debug, Clone, Copy, PartialEq, RegisterSetting)]
pub struct MarkdownPreviewSettings {
    pub auto_open_preview: bool,
}

impl settings::Settings for MarkdownPreviewSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let auto_open_preview = content
            .markdown_preview
            .as_ref()
            .and_then(|preview| preview.auto_open_preview)
            .unwrap_or(false);

        Self { auto_open_preview }
    }
}
