use std::borrow::Cow;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use super::release_notes::release_notes_close_button_rect;
use super::scrollbar::{release_notes_scrollbar_rect, render_scrollbar};
use super::widgets::{
    modal_stack_areas, panel_contrast_fg, render_action_button, render_modal_header,
    render_modal_shell,
};
use crate::app::input::NavigateAction;
use crate::app::AppState;

pub(super) type HelpEntry = (String, Cow<'static, str>, Option<NavigateAction>);
pub(super) type HelpGroup = (&'static str, Vec<HelpEntry>);

fn info_entry(key: impl Into<String>, label: &'static str) -> HelpEntry {
    (key.into(), Cow::Borrowed(label), None)
}

fn action_entry(key: impl Into<String>, label: &'static str, action: NavigateAction) -> HelpEntry {
    (key.into(), Cow::Borrowed(label), Some(action))
}

fn keybind_label(bindings: &crate::config::ActionKeybinds) -> String {
    bindings.label().unwrap_or_else(|| "unset".to_string())
}

fn indexed_label(bindings: &[crate::config::IndexedKeybind]) -> String {
    if bindings.is_empty() {
        "unset".to_string()
    } else if bindings.len() == 9 {
        let first = &bindings[0].label;
        if first.ends_with('1') {
            format!("{}1..9", first.trim_end_matches('1'))
        } else {
            bindings
                .iter()
                .map(|binding| binding.label.clone())
                .collect::<Vec<_>>()
                .join(" / ")
        }
    } else {
        bindings
            .iter()
            .map(|binding| binding.label.clone())
            .collect::<Vec<_>>()
            .join(" / ")
    }
}

pub(super) fn keybind_help_groups(app: &AppState) -> Vec<HelpGroup> {
    let kb = &app.keybinds;
    let mut groups = Vec::new();

    groups.push((
        "global",
        vec![
            info_entry(
                crate::config::format_key_combo((app.prefix_code, app.prefix_mods)),
                "prefix mode",
            ),
            action_entry(keybind_label(&kb.help), "keybinds", NavigateAction::Help),
            action_entry(
                keybind_label(&kb.settings),
                "settings",
                NavigateAction::Settings,
            ),
            action_entry(keybind_label(&kb.detach), "detach", NavigateAction::Detach),
            action_entry(
                keybind_label(&kb.reload_config),
                "reload config",
                NavigateAction::ReloadConfig,
            ),
            action_entry(
                keybind_label(&kb.open_notification_target),
                "open notification target",
                NavigateAction::OpenNotificationTarget,
            ),
        ],
    ));

    groups.push((
        "navigation",
        vec![
            info_entry("esc", "back"),
            info_entry(
                format!(
                    "{} / {}",
                    keybind_label(&kb.navigate.workspace_up),
                    keybind_label(&kb.navigate.workspace_down)
                ),
                "workspace list",
            ),
            info_entry(
                format!(
                    "{} / {} / {} / {} / left / right",
                    keybind_label(&kb.navigate.pane_left),
                    keybind_label(&kb.navigate.pane_down),
                    keybind_label(&kb.navigate.pane_up),
                    keybind_label(&kb.navigate.pane_right)
                ),
                "move focus",
            ),
            info_entry("tab / shift+tab", "cycle pane"),
            info_entry("enter", "open workspace"),
            info_entry("1..9", "switch workspace"),
        ],
    ));

    let workspace_tab = vec![
        action_entry(
            keybind_label(&kb.workspace_picker),
            "workspace navigation",
            NavigateAction::WorkspacePicker,
        ),
        action_entry(
            keybind_label(&kb.goto),
            "session navigator",
            NavigateAction::OpenNavigator,
        ),
        action_entry(
            keybind_label(&kb.new_workspace),
            "new workspace",
            NavigateAction::NewWorkspace,
        ),
        action_entry(
            keybind_label(&kb.new_worktree),
            "new worktree",
            NavigateAction::NewWorktree,
        ),
        action_entry(
            keybind_label(&kb.open_worktree),
            "open worktree",
            NavigateAction::OpenWorktree,
        ),
        action_entry(
            keybind_label(&kb.remove_worktree),
            "delete worktree checkout",
            NavigateAction::RemoveWorktree,
        ),
        action_entry(
            keybind_label(&kb.rename_workspace),
            "rename workspace",
            NavigateAction::RenameWorkspace,
        ),
        action_entry(
            keybind_label(&kb.close_workspace),
            "close workspace",
            NavigateAction::CloseWorkspace,
        ),
        action_entry(
            keybind_label(&kb.previous_workspace),
            "previous workspace",
            NavigateAction::PreviousWorkspace,
        ),
        action_entry(
            keybind_label(&kb.next_workspace),
            "next workspace",
            NavigateAction::NextWorkspace,
        ),
        info_entry(indexed_label(&kb.switch_workspace), "switch workspace 1-9"),
        action_entry(
            keybind_label(&kb.previous_agent),
            "previous agent",
            NavigateAction::PreviousAgent,
        ),
        action_entry(
            keybind_label(&kb.next_agent),
            "next agent",
            NavigateAction::NextAgent,
        ),
        info_entry(indexed_label(&kb.focus_agent), "focus agent 1-9"),
        action_entry(
            keybind_label(&kb.new_tab),
            "new tab",
            NavigateAction::NewTab,
        ),
        action_entry(
            keybind_label(&kb.rename_tab),
            "rename tab",
            NavigateAction::RenameTab,
        ),
        action_entry(
            keybind_label(&kb.previous_tab),
            "previous tab",
            NavigateAction::PreviousTab,
        ),
        action_entry(
            keybind_label(&kb.next_tab),
            "next tab",
            NavigateAction::NextTab,
        ),
        info_entry(indexed_label(&kb.switch_tab), "switch tab 1-9"),
        action_entry(
            keybind_label(&kb.close_tab),
            "close tab",
            NavigateAction::CloseTab,
        ),
    ];
    groups.push(("workspaces / tabs", workspace_tab));

    let panes = vec![
        action_entry(
            keybind_label(&kb.split_vertical),
            "split vertical",
            NavigateAction::SplitVertical,
        ),
        action_entry(
            keybind_label(&kb.split_horizontal),
            "split horizontal",
            NavigateAction::SplitHorizontal,
        ),
        action_entry(
            keybind_label(&kb.close_pane),
            "close pane",
            NavigateAction::ClosePane,
        ),
        action_entry(
            keybind_label(&kb.rename_pane),
            "rename pane",
            NavigateAction::RenamePane,
        ),
        action_entry(
            keybind_label(&kb.edit_scrollback),
            "edit scrollback",
            NavigateAction::EditScrollback,
        ),
        action_entry(
            keybind_label(&kb.copy_mode),
            "copy mode",
            NavigateAction::CopyMode,
        ),
        action_entry(keybind_label(&kb.zoom), "zoom pane", NavigateAction::Zoom),
        action_entry(
            keybind_label(&kb.resize_mode),
            "resize mode",
            NavigateAction::EnterResizeMode,
        ),
        action_entry(
            keybind_label(&kb.toggle_sidebar),
            "toggle sidebar",
            NavigateAction::ToggleSidebar,
        ),
        action_entry(
            keybind_label(&kb.focus_pane_left),
            "focus pane left",
            NavigateAction::FocusPaneLeft,
        ),
        action_entry(
            keybind_label(&kb.focus_pane_down),
            "focus pane down",
            NavigateAction::FocusPaneDown,
        ),
        action_entry(
            keybind_label(&kb.focus_pane_up),
            "focus pane up",
            NavigateAction::FocusPaneUp,
        ),
        action_entry(
            keybind_label(&kb.focus_pane_right),
            "focus pane right",
            NavigateAction::FocusPaneRight,
        ),
        action_entry(
            keybind_label(&kb.cycle_pane_next),
            "cycle pane next",
            NavigateAction::CyclePaneNext,
        ),
        action_entry(
            keybind_label(&kb.cycle_pane_previous),
            "cycle pane previous",
            NavigateAction::CyclePanePrevious,
        ),
        action_entry(
            keybind_label(&kb.last_pane),
            "last pane",
            NavigateAction::LastPane,
        ),
    ];
    groups.push(("panes", panes));

    if !kb.custom_commands.is_empty() {
        groups.push((
            "custom",
            kb.custom_commands
                .iter()
                .map(|binding| {
                    (
                        binding.label.clone(),
                        binding
                            .description
                            .clone()
                            .map(Cow::Owned)
                            .unwrap_or(Cow::Borrowed("custom command")),
                        None,
                    )
                })
                .collect(),
        ));
    }

    groups
}

/// A row in the rendered keybind help body.
///
/// `width` is the visual width used for max-width calculations. `action` is
/// `Some` when the row is selectable and represents a dispatchable navigate
/// action; `None` for headings, blank separators, and informational rows.
pub(crate) struct HelpLine {
    pub width: usize,
    pub line: Line<'static>,
    pub action: Option<NavigateAction>,
}

pub(crate) fn keybind_help_lines(app: &AppState) -> Vec<HelpLine> {
    let heading_style = Style::default()
        .fg(app.palette.accent)
        .add_modifier(Modifier::BOLD);
    let key_style = Style::default()
        .fg(app.palette.mauve)
        .add_modifier(Modifier::BOLD);
    let label_style = Style::default().fg(app.palette.text);

    let groups = keybind_help_groups(app);
    let key_width = groups
        .iter()
        .flat_map(|(_, entries)| entries.iter().map(|(key, _, _)| key.chars().count()))
        .max()
        .unwrap_or(8);

    let mut lines = Vec::new();

    for (group, entries) in groups {
        lines.push(HelpLine {
            width: group.len() + 1,
            line: Line::from(vec![Span::styled(format!(" {group}"), heading_style)]),
            action: None,
        });
        for (key, label, action) in entries {
            let padded_key = format!(" {:<width$} ", key, width = key_width);
            let width = padded_key.chars().count() + label.chars().count();
            lines.push(HelpLine {
                width,
                line: Line::from(vec![
                    Span::styled(padded_key, key_style),
                    Span::styled(label.into_owned(), label_style),
                ]),
                action,
            });
        }
        lines.push(HelpLine {
            width: 0,
            line: Line::raw(""),
            action: None,
        });
    }

    lines
}

/// Line indices of rows that can be selected and executed.
pub(crate) fn keybind_help_actionable_indices(app: &AppState) -> Vec<usize> {
    keybind_help_lines(app)
        .iter()
        .enumerate()
        .filter_map(|(idx, line)| line.action.map(|_| idx))
        .collect()
}

/// Look up the `NavigateAction` for the currently selected actionable row.
pub(crate) fn selected_keybind_help_action(app: &AppState) -> Option<NavigateAction> {
    let indices = keybind_help_actionable_indices(app);
    let line_idx = *indices.get(app.keybind_help.selected)?;
    keybind_help_lines(app)
        .get(line_idx)
        .and_then(|line| line.action)
}

pub(super) fn render_keybind_help_overlay(app: &AppState, frame: &mut Frame) {
    super::dim_background(frame, frame.area());

    let Some(inner) = render_modal_shell(frame, frame.area(), 76, 22, &app.palette) else {
        return;
    };
    if inner.height < 6 || inner.width < 20 {
        return;
    }

    let stack = modal_stack_areas(inner, 2, 1, 0, 1);
    let header_rows =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas::<2>(stack.header);

    render_modal_header(frame, header_rows[0], "keybinds", &app.palette);
    render_action_button(
        frame,
        release_notes_close_button_rect(header_rows[0]),
        Some("esc"),
        "close",
        Style::default()
            .fg(panel_contrast_fg(&app.palette))
            .bg(app.palette.accent)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(
        Paragraph::new(" available commands and configured shortcuts")
            .style(Style::default().fg(app.palette.overlay1)),
        header_rows[1],
    );

    let body_area = stack.content;
    let metrics = crate::pane::ScrollMetrics {
        offset_from_bottom: app
            .keybind_help_max_scroll()
            .saturating_sub(app.keybind_help.scroll) as usize,
        max_offset_from_bottom: app.keybind_help_max_scroll() as usize,
        viewport_rows: body_area.height.max(1) as usize,
    };
    let track = release_notes_scrollbar_rect(body_area, metrics);
    let text_area = track
        .map(|_| {
            Rect::new(
                body_area.x,
                body_area.y,
                body_area.width.saturating_sub(1),
                body_area.height,
            )
        })
        .unwrap_or(body_area);

    let all_lines = keybind_help_lines(app);
    let actionable: Vec<usize> = all_lines
        .iter()
        .enumerate()
        .filter_map(|(idx, line)| line.action.map(|_| idx))
        .collect();
    let selected_line_idx = actionable.get(app.keybind_help.selected).copied();
    let highlight_style = Style::default()
        .fg(panel_contrast_fg(&app.palette))
        .bg(app.palette.accent)
        .add_modifier(Modifier::BOLD);

    let rendered: Vec<Line<'static>> = all_lines
        .into_iter()
        .enumerate()
        .map(|(idx, help_line)| {
            if Some(idx) == selected_line_idx {
                let mut styled = help_line.line.clone();
                for span in &mut styled.spans {
                    span.style = highlight_style;
                }
                styled
            } else {
                help_line.line
            }
        })
        .collect();

    let body = Paragraph::new(rendered)
        .wrap(Wrap { trim: false })
        .scroll((app.keybind_help.scroll, 0));
    frame.render_widget(body, text_area);
    if let Some(track) = track {
        render_scrollbar(
            frame,
            metrics,
            track,
            app.palette.overlay0,
            app.palette.overlay1,
            "▐",
        );
    }

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" select ", Style::default().fg(app.palette.overlay0)),
            Span::styled("↑↓ / j k", Style::default().fg(app.palette.text)),
            Span::styled("  ·  ", Style::default().fg(app.palette.overlay0)),
            Span::styled("run", Style::default().fg(app.palette.overlay0)),
            Span::styled(" enter ", Style::default().fg(app.palette.text)),
            Span::styled("  ·  ", Style::default().fg(app.palette.overlay0)),
            Span::styled("close", Style::default().fg(app.palette.overlay0)),
            Span::styled(" esc ", Style::default().fg(app.palette.text)),
        ])),
        stack.footer.unwrap_or_default(),
    );
}
