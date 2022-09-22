use std::path::PathBuf;
use std::rc::Rc;

use crossbeam_channel::select;
use log::{debug, error};

use crate::config::config::ConfigRef;
use crate::experiments::clipboard::ClipboardRef;
use crate::fs::fsf_ref::FsfRef;
use crate::io::input::Input;
use crate::io::input_event::InputEvent;
use crate::io::output::FinalOutput;
use crate::Theme;
use crate::tsw::language_set::LanguageSet;
use crate::tsw::tree_sitter_wrapper::TreeSitterWrapper;
use crate::w7e::inspector::{inspect_workspace, InspectError};
use crate::w7e::workspace::{LoadError, ScopeLoadErrors, Workspace};
use crate::w7e::workspace::WORKSPACE_FILE_NAME;
use crate::widget::any_msg::AnyMsg;
use crate::widget::widget::Widget;
use crate::widgets::main_view::main_view::MainView;

pub fn run_gladius<
    I: Input,
    O: FinalOutput>(
    fsf: FsfRef,
    config: ConfigRef,
    clipboard: ClipboardRef,
    input: I,
    mut output: O,
    files: Vec<PathBuf>,
    theme: &Theme,
) {
    let tree_sitter = Rc::new(TreeSitterWrapper::new(LanguageSet::full()));

    // Loading / Building workspace file
    let workspace_dir = fsf.root();
    let (workspace_op, _scope_errors): (Option<Workspace>, ScopeLoadErrors) = match Workspace::try_load(workspace_dir.clone()) {
        Ok(res) => (Some(res.0), res.1),
        Err(e) => {
            match e {
                LoadError::WorkspaceFileNotFound => {
                    (None, ScopeLoadErrors::default())
                }
                LoadError::ReadError(e) => {
                    error!("failed reading workspace file at {}, because:\n{}\nterminating. To continue, rename/remove {} in that folder.",
                        fsf.root(), e, WORKSPACE_FILE_NAME);

                    return;
                }
            }
        }
    };

    // TODO add option to NOT *save* workspace after creation?
    let mut workspace: Workspace = match workspace_op {
        Some(w) => w,
        None => {
            // Attempting to create a reasonable workspace
            match inspect_workspace(&workspace_dir) {
                Err(e) => {
                    match &e {
                        InspectError::NotAFolder => {
                            error!("failed inspecting workspace at {:?}, because it doesn't seem to be a folder.",
                            workspace_dir.absolute_path());
                            // This should never happen, so I terminate program.
                            return;
                        }
                        _ => {
                            error!("failed inspecting workspace at {:?}, because:\n{}", workspace_dir.absolute_path(), e);
                            // I decided inspection should not have non-fatal errors, just worst case scenario being "no scopes".
                            return;
                        }
                    }
                }
                Ok(scopes) => {
                    debug!("creating new workspace at {:?} with {} scopes", workspace_dir.absolute_path(), scopes.len());
                    let workspace = Workspace::new(workspace_dir, scopes);
                    match workspace.save() {
                        Ok(_) => {
                            debug!("saved successfully");
                        }
                        Err(e) => {
                            error!("failed writing workspace file: {:?}", e);
                        }
                    }
                    workspace
                }
            }
        }
    };

    // At this point it is guaranteed that we have a Workspace present, though it might be not saved!

    // Initializing handlers
    let (nav_comp_group_ref, scope_errors) = workspace.initialize_handlers(&config);
    if !scope_errors.is_empty() {
        debug!("{} handlers failed to load, details : {:?}", scope_errors.len(), scope_errors);
    }

    let mut main_view = MainView::new(
        config.clone(),
        tree_sitter,
        fsf.clone(),
        clipboard,
        nav_comp_group_ref.clone(),
    );
    for f in files.iter() {
        if !fsf.descendant_checked(f).map(|ff| main_view.open_file(ff)).unwrap_or(false) {
            error!("failed opening file {:?}", f);
        }
    }

    // returns (consumed, message_to_parent)
    fn recursive_treat_views(
        view: &mut dyn Widget,
        ie: InputEvent,
    ) -> (bool, Option<Box<dyn AnyMsg>>) {
        let my_desc = format!("{:?}", &view).clone();
        let my_id = view.id();

        let focused_child_op = view.get_focused_mut();
        let child_desc = format!("{:?}", &focused_child_op);

        debug!(target: "recursive_treat_views", "{:?}: event {:?}, active_child: {:?}", my_desc, ie, child_desc);

        // first, dig as deep as possible.
        let (child_have_consumed, message_from_child_op) = match focused_child_op {
            Some(focused_child) => {
                debug_assert!(focused_child.id() != my_id,
                              "widget {:?} pointed to itself as it's own child, causing stack overflow", view
                );


                recursive_treat_views(focused_child, ie)
            }
            None => (false, None)
        };
        debug!(target: "recursive_treat_views", "{:?}: event {:?}, active_child: {:?}, child_consumed: {}, message_from_child: {:?}",
            my_desc, ie, child_desc, child_have_consumed, &message_from_child_op);

        if child_have_consumed {
            return match message_from_child_op {
                None => (true, None),
                Some(message_from_child) => {
                    let msg_from_child_text = format!("{:?}", &message_from_child);
                    let my_message_to_parent = view.update(message_from_child);
                    debug!(target: "recursive_treat_views", "{:?}: message_from_child: {:?} sent to me, responding {:?} to parent",
                        my_desc, msg_from_child_text, &my_message_to_parent);
                    (true, my_message_to_parent)
                }
            };
        };

        // Either child did not consume (unwinding), or we're on the bottom of path.
        // We're here to consume the Input.
        match view.on_input(ie) {
            None => {
                debug!(target: "recursive_treat_views", "{:?}: did not consume {:?} either.", my_desc, ie);
                // we did not see this input as useful, unfolding the recursion:
                // no consume, no message.
                (false, None)
            }
            Some(internal_message) => {
                debug!(target: "recursive_treat_views", "{:?}: consumed {:?} and am pushing {:?} to myself", my_desc, ie, internal_message);
                let message_to_parent = view.update(internal_message);
                debug!(target: "recursive_treat_views", "{:?}: send {:?} to parent", my_desc, message_to_parent);
                // (message_to_parent.is_some(), message_to_parent)
                (true, message_to_parent)
            }
        }
    }

    // Genesis
    'main:
    loop {
        match output.clear() {
            Ok(_) => {}
            Err(e) => {
                error!("failed to clear output: {}", e);
                break;
            }
        }
        main_view.update_and_layout(output.size_constraint());
        main_view.render(&theme, true, &mut output);
        match output.end_frame() {
            Ok(_) => {}
            Err(e) => {
                error!("failed to end frame: {}", e);
                break;
            }
        }

        select! {
            recv(input.source()) -> msg => {
                // debug!("processing input: {:?}", msg);
                match msg {
                    Ok(mut ie) => {
                        // debug!("msg ie {:?}", ie);
                        match ie {
                            InputEvent::KeyInput(key) if key.as_focus_update().is_some() && key.modifiers.alt => {
                                ie = InputEvent::FocusUpdate(key.as_focus_update().unwrap());
                            },
                            InputEvent::KeyInput(key) if key == config.keyboard_config.global.everything_bar => {
                                ie = InputEvent::EverythingBarTrigger;
                            }
                            // TODO move to message, to handle signals in the same way?
                            InputEvent::KeyInput(key) if key == config.keyboard_config.global.close => {
                                break 'main;
                            }
                            _ => {}
                        }

                        recursive_treat_views(&mut main_view, ie);
                    },
                    Err(e) => {
                        error!("failed receiving input: {}", e);
                    }
                };
            }

            recv(nav_comp_group_ref.recvr()) -> tick => {
                match tick {
                    _ => {
                        // warn!("unhandled tick : {:?}", tick)
                    }
                }
            }
        }
    }
}