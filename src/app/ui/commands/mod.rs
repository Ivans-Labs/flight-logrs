use backend::DataProvider;
use std::fmt::Debug;

use multi_select_cmd::*;

use super::{App, HandleInputReturnType, MsgBoxResult, UIComponents};

use editor_cmd::*;
use entries_list_cmd::*;
use global_cmd::*;

mod editor_cmd;
mod entries_list_cmd;
mod global_cmd;
mod multi_select_cmd;

type CmdResult = anyhow::Result<HandleInputReturnType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UICommand {
    Quit,
    ShowHelp,
    CycleFocusedControlForward,
    CycleFocusedControlBack,
    SelectedNextEntry,
    SelectedPrevEntry,
    CreateEntry,
    EditCurrentEntry,
    DeleteCurrentEntry,
    StartEditEntryContent,
    FinishEditEntryContent,
    SaveEntryContent,
    DiscardChangesEntryContent,
    ReloadAll,
    ExportEntryContent,
    EditInExternalEditor,
    EnterMultiSelectMode,
    LeaveMultiSelectMode,
    MulSelToggleSelected,
    MulSelSelectAll,
    MulSelSelectNone,
    MulSelInverSelection,
    MulSelDeleteEntries,
    MulSelExportEntries,
    ShowFilter,
    ResetFilter,
    ShowFuzzyFind,
}

#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
}

impl CommandInfo {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
        }
    }
}

impl UICommand {
    pub fn get_info(&self) -> CommandInfo {
        match self {
            UICommand::Quit => CommandInfo::new("Exit", "Exit the program"),
            UICommand::ShowHelp => CommandInfo::new("Show help", "Show keybindings overview"),
            UICommand::CycleFocusedControlForward => {
                CommandInfo::new("Cycle focus forward", "Move focus to the next control")
            }
            UICommand::CycleFocusedControlBack => {
                CommandInfo::new("Cycle focus backward", "Move focus to the previous control")
            }
            UICommand::SelectedNextEntry => CommandInfo::new(
                "Select next Logbook",
                "Select next entry in the Logbooks list",
            ),
            UICommand::SelectedPrevEntry => CommandInfo::new(
                "Select previous Logbook",
                "Select previous entry in the Logbook list",
            ),
            UICommand::CreateEntry => CommandInfo::new(
                "Create new Logbook",
                "Opens dialog to add a new Logbook entry",
            ),
            UICommand::EditCurrentEntry => CommandInfo::new(
                "Edit current Logbook",
                "Open entry dialog to edit current Logbook entry if any",
            ),
            UICommand::DeleteCurrentEntry => {
                CommandInfo::new("Delete Logbook", "Delete current Logbook entry if any")
            }
            UICommand::StartEditEntryContent => CommandInfo::new(
                "Edit logbook content",
                "Start editing current Logbook entry content in editor",
            ),
            UICommand::FinishEditEntryContent => {
                CommandInfo::new("End edit mode", "Exit edit mode in editor")
            }
            UICommand::SaveEntryContent => {
                CommandInfo::new("Save", "Save changes on Logbook content")
            }
            UICommand::DiscardChangesEntryContent => {
                CommandInfo::new("Discard changes", "Discard changes on Logbook content")
            }
            UICommand::ReloadAll => CommandInfo::new("Reload all", "Reload all entries"),
            UICommand::ExportEntryContent => {
                CommandInfo::new("Export Logbook content", "Export current Logbook content")
            }
            UICommand::EditInExternalEditor => CommandInfo::new(
                "Edit in external editor",
                "Edit current Logbook content in external editor (The editor can be set in configurations file or via the environment variables VISUAL, EDITOR)",
            ),
            UICommand::EnterMultiSelectMode => CommandInfo::new(
                "Enter Logbooks multi selection mode",
                "Enter multi selection mode for Logbooks to work with multi Logbooks at once",
            ),
            UICommand::LeaveMultiSelectMode => CommandInfo::new(
                "Leave Logbooks multi selection mode",
                "Leave multi selection mode for Logbooks and return to normal mode",
            ),
            UICommand::MulSelToggleSelected => CommandInfo::new(
                "Toggle selected",
                "Toggle if the current Logbook is selected in multi selection mode",
            ),
            UICommand::MulSelSelectAll => CommandInfo::new(
                "Select all Logbooks",
                "Select all Logbooks in multi selection mode",
            ),
            UICommand::MulSelSelectNone => CommandInfo::new(
                "Clear selection",
                "Clear Logbooks selection in multi selection mode",
            ),
            UICommand::MulSelInverSelection => CommandInfo::new(
                "Invert selection",
                "Invert Logbooks selection in multi selection mode",
            ),
            UICommand::MulSelDeleteEntries => CommandInfo::new(
                "Delete selection",
                "Delete selected Logbooks in multi selection mode",
            ),
            UICommand::MulSelExportEntries => CommandInfo::new(
                "Export selection",
                "Export selected Logbooks to a transfer JSON file, which can be imported to other back-end files",
            ),
            UICommand::ShowFilter => CommandInfo::new(
                "Open filter",
                "Open filter popup for Logbooks",
            ),
            UICommand::ResetFilter => CommandInfo::new(
                "Reset filter",
                "Reset the applied filter on Logbooks",
            ),
            UICommand::ShowFuzzyFind => CommandInfo::new(
                "Fuzzy find",
                "Open fuzzy find popup for Logbooks",
            ),
        }
    }

    pub async fn execute<'a, D: DataProvider>(
        &self,
        ui_components: &mut UIComponents<'a>,
        app: &mut App<D>,
    ) -> CmdResult {
        match self {
            UICommand::Quit => exec_quit(ui_components),
            UICommand::ShowHelp => exec_show_help(ui_components),
            UICommand::CycleFocusedControlForward => exec_cycle_forward(ui_components),
            UICommand::CycleFocusedControlBack => exec_cycle_backward(ui_components),
            UICommand::SelectedNextEntry => exec_select_next_entry(ui_components, app),
            UICommand::SelectedPrevEntry => exec_select_prev_entry(ui_components, app),
            UICommand::CreateEntry => exec_create_entry(ui_components),
            UICommand::EditCurrentEntry => exec_edit_current_entry(ui_components, app),
            UICommand::DeleteCurrentEntry => exec_delete_current_entry(ui_components, app),
            UICommand::StartEditEntryContent => exec_start_edit_content(ui_components),
            UICommand::FinishEditEntryContent => exec_finish_editing(ui_components),
            UICommand::SaveEntryContent => exec_save_entry_content(ui_components, app).await,
            UICommand::DiscardChangesEntryContent => exec_discard_content(ui_components),
            UICommand::ReloadAll => exec_reload_all(ui_components, app).await,
            UICommand::ExportEntryContent => exec_export_entry_content(ui_components, app),
            UICommand::EditInExternalEditor => {
                exec_edit_in_external_editor(ui_components, app).await
            }
            UICommand::EnterMultiSelectMode => exec_enter_select_mode(ui_components),
            UICommand::LeaveMultiSelectMode => exec_leave_select_mode(ui_components, app),
            UICommand::MulSelToggleSelected => exec_toggle_selected(app),
            UICommand::MulSelSelectAll => exec_select_all(app),
            UICommand::MulSelSelectNone => exec_select_none(app),
            UICommand::MulSelInverSelection => exec_invert_selection(app),
            UICommand::MulSelDeleteEntries => exec_delete_selected_entries(ui_components, app),
            UICommand::MulSelExportEntries => exec_export_selected_entries(ui_components, app),
            UICommand::ShowFilter => exec_show_filter(ui_components, app),
            UICommand::ResetFilter => exec_reset_filter(app),
            UICommand::ShowFuzzyFind => exec_show_fuzzy_find(ui_components, app),
        }
    }

    pub async fn continue_executing<'a, D: DataProvider>(
        &self,
        ui_components: &mut UIComponents<'a>,
        app: &mut App<D>,
        msg_box_result: MsgBoxResult,
    ) -> CmdResult {
        let not_implemented = || unreachable!("continue exec isn't implemented for {:?}", self);
        match self {
            UICommand::Quit => continue_quit(ui_components, app, msg_box_result).await,
            UICommand::ShowHelp => not_implemented(),
            UICommand::CycleFocusedControlForward => not_implemented(),
            UICommand::CycleFocusedControlBack => not_implemented(),
            UICommand::SelectedNextEntry => {
                continue_select_next_entry(ui_components, app, msg_box_result).await
            }
            UICommand::SelectedPrevEntry => {
                continue_select_prev_entry(ui_components, app, msg_box_result).await
            }
            UICommand::CreateEntry => {
                continue_create_entry(ui_components, app, msg_box_result).await
            }
            UICommand::EditCurrentEntry => {
                continue_edit_current_entry(ui_components, app, msg_box_result).await
            }
            UICommand::DeleteCurrentEntry => {
                continue_delete_current_entry(app, msg_box_result).await
            }
            UICommand::StartEditEntryContent => not_implemented(),
            UICommand::FinishEditEntryContent => not_implemented(),
            UICommand::SaveEntryContent => not_implemented(),
            UICommand::DiscardChangesEntryContent => {
                continue_discard_content(ui_components, app, msg_box_result)
            }
            UICommand::ReloadAll => continue_reload_all(ui_components, app, msg_box_result).await,
            UICommand::ExportEntryContent => {
                continue_export_entry_content(ui_components, app, msg_box_result).await
            }
            UICommand::EditInExternalEditor => {
                continue_edit_in_external_editor(ui_components, app, msg_box_result).await
            }
            UICommand::EnterMultiSelectMode => {
                continue_enter_select_mode(ui_components, app, msg_box_result).await
            }
            UICommand::LeaveMultiSelectMode => not_implemented(),
            UICommand::MulSelToggleSelected => not_implemented(),
            UICommand::MulSelSelectAll => not_implemented(),
            UICommand::MulSelSelectNone => not_implemented(),
            UICommand::MulSelInverSelection => not_implemented(),
            UICommand::MulSelDeleteEntries => {
                continue_delete_selected_entries(app, msg_box_result).await
            }
            UICommand::MulSelExportEntries => not_implemented(),
            UICommand::ShowFilter => continue_show_filter(ui_components, app, msg_box_result).await,
            UICommand::ResetFilter => not_implemented(),
            UICommand::ShowFuzzyFind => {
                continue_fuzzy_find(ui_components, app, msg_box_result).await
            }
        }
    }
}
