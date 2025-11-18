use settings::{OpenMarkdownPreview, RegisterSetting};

#[derive(Debug, Clone, Copy, PartialEq, RegisterSetting)]
pub struct MarkdownPreviewSettings {
    pub open_markdown_preview: OpenMarkdownPreview,
}

impl settings::Settings for MarkdownPreviewSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let open_markdown_preview = content
            .markdown_preview
            .as_ref()
            .and_then(|preview| preview.open_markdown_preview)
            .unwrap_or_default();

        Self {
            open_markdown_preview,
        }
    }
}
