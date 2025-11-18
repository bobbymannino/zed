use gpui::{App, Focusable, actions};
use settings::Settings;
use workspace::Workspace;

pub mod markdown_elements;
mod markdown_minifier;
pub mod markdown_parser;
pub mod markdown_preview_view;
pub mod markdown_renderer;
mod markdown_preview_settings;

pub use markdown_preview_settings::MarkdownPreviewSettings;

actions!(
    markdown,
    [
        /// Scrolls up by one page in the markdown preview.
        MovePageUp,
        /// Scrolls down by one page in the markdown preview.
        MovePageDown,
        /// Opens a markdown preview for the current file.
        OpenPreview,
        /// Opens a markdown preview in a split pane.
        OpenPreviewToTheSide,
        /// Opens a following markdown preview that syncs with the editor.
        OpenFollowingPreview
    ]
);

pub fn init(cx: &mut App) {
    MarkdownPreviewSettings::register(cx);
    
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        markdown_preview_view::MarkdownPreviewView::register(workspace, window, cx);
        
        // Subscribe to workspace events for auto-opening markdown preview
        let workspace_weak = workspace.weak_handle();
        cx.subscribe_in(&workspace_weak.upgrade().unwrap(), window, |_, workspace, event, window, cx| {
            if let workspace::Event::ItemAdded { item } = event {
                // Check the open_markdown_preview setting
                let setting = MarkdownPreviewSettings::get_global(cx).open_markdown_preview;
                
                // Only act if the setting is not "code" (default)
                if setting == settings::OpenMarkdownPreview::Code {
                    return;
                }
                
                // Check if the added item is a markdown editor
                if let Some(editor) = item.act_as::<editor::Editor>(cx) {
                    if markdown_preview_view::MarkdownPreviewView::is_markdown_file(&editor, cx) {
                        workspace.update(cx, |workspace, cx| {
                            let view = markdown_preview_view::MarkdownPreviewView::create_markdown_view(
                                workspace, 
                                editor.clone(), 
                                window, 
                                cx
                            );
                            
                            match setting {
                                settings::OpenMarkdownPreview::Preview => {
                                    // Replace the editor with the preview in the same pane
                                    workspace.active_pane().update(cx, |pane, cx| {
                                        // Find the editor in the pane
                                        if let Some(editor_idx) = pane.items().position(|i| {
                                            i.item_id() == editor.entity_id()
                                        }) {
                                            // Close the editor
                                            pane.close_item_by_id(editor.entity_id(), workspace::SaveIntent::Skip, window, cx)
                                                .detach();
                                            // Add the preview at the same position
                                            pane.add_item(Box::new(view.clone()), true, true, Some(editor_idx), window, cx);
                                        }
                                    });
                                }
                                settings::OpenMarkdownPreview::PreviewToSide => {
                                    // Check if there's already a preview for this editor in the right pane
                                    let has_existing_preview = workspace
                                        .find_pane_in_direction(workspace::SplitDirection::Right, cx)
                                        .map(|pane| {
                                            pane.read(cx)
                                                .items_of_type::<markdown_preview_view::MarkdownPreviewView>()
                                                .any(|view| {
                                                    view.read(cx)
                                                        .active_editor
                                                        .as_ref()
                                                        .is_some_and(|state| state.editor == editor)
                                                })
                                        })
                                        .unwrap_or(false);
                                    
                                    if !has_existing_preview {
                                        let pane = workspace
                                            .find_pane_in_direction(workspace::SplitDirection::Right, cx)
                                            .unwrap_or_else(|| {
                                                workspace.split_pane(
                                                    workspace.active_pane().clone(),
                                                    workspace::SplitDirection::Right,
                                                    window,
                                                    cx,
                                                )
                                            });
                                        pane.update(cx, |pane, cx| {
                                            pane.add_item(Box::new(view.clone()), false, false, None, window, cx)
                                        });
                                        // Keep focus on the editor
                                        editor.focus_handle(cx).focus(window);
                                    }
                                }
                                settings::OpenMarkdownPreview::Code => {
                                    // Do nothing - just show the code
                                }
                            }
                        });
                    }
                }
            }
        }).detach();
    })
    .detach();
}
