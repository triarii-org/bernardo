use std::path::PathBuf;

use log::debug;
use serde::{Deserialize, Serialize};

use crate::*;

/*
So a funny finding while adding NavCompTick channel is that it seems like I could immediately add
a parallel channel for "filesystem events" within the workspace (after all we don't care for other),
or even go so far as to merge the two, NavCompTicks being just a branch of multiple ticks we'd like
to receive.

In a way these two not obviously related structures seem to be intertwined.
 */

pub const WORKSPACE_FILE_NAME: &'static str = ".gladius_workspace.ron";

pub struct Scopes(Vec<ProjectScope>);

pub type ScopeLoadErrors = Vec<(PathBuf, ProjectLoadError)>;

pub struct Workspace {
    root_path: SPath,
    scopes: Vec<ProjectScope>,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableWorkspace {
    pub scopes: Vec<SerializableProjectScope>,
}

impl ToPrettyRonString for SerializableWorkspace {}

#[derive(Debug)]
pub enum WorkspaceLoadError {
    WorkspaceFileNotFound,
    ReadError(ReadError),
}

impl From<ReadError> for WorkspaceLoadError {
    fn from(re: ReadError) -> Self {
        WorkspaceLoadError::ReadError(re)
    }
}

impl Workspace {
    pub fn new(root_path: SPath, scopes: Vec<ProjectScope>) -> Workspace {
        Workspace { root_path, scopes }
    }

    pub fn try_load(root_path: SPath) -> Result<(Workspace, ScopeLoadErrors), WorkspaceLoadError> {
        let workspace_file = root_path
            .descendant_checked(WORKSPACE_FILE_NAME)
            .ok_or(WorkspaceLoadError::WorkspaceFileNotFound)?;
        debug!("loading workspace file from {:?}", workspace_file.absolute_path());
        let serialized_workspace = workspace_file.read_entire_file_to_item::<SerializableWorkspace>()?;
        Self::from(serialized_workspace, root_path)
    }

    pub fn save(&self) -> Result<usize, WriteOrSerError> {
        let file = self.root_path.descendant_unchecked(WORKSPACE_FILE_NAME).unwrap();
        let pill = self.serializable();
        file.overwrite_with_ron(&pill, false)
    }

    pub fn from(sw: SerializableWorkspace, root_path: SPath) -> Result<(Workspace, ScopeLoadErrors), WorkspaceLoadError> {
        let mut scopes: Vec<ProjectScope> = Vec::new();
        let mut scope_errors = ScopeLoadErrors::new();

        for sps in sw.scopes.into_iter() {
            match ProjectScope::from_serializable(sps, &root_path) {
                Ok(scope) => scopes.push(scope),
                Err(error_pair) => scope_errors.push((root_path.relative_path(), error_pair)),
            }
        }

        Ok((Workspace { root_path, scopes }, scope_errors))
    }

    pub fn serializable(&self) -> SerializableWorkspace {
        let serializable_scopes: Vec<_> = self.scopes.iter().map(|scope| scope.serializable()).collect();
        SerializableWorkspace {
            scopes: serializable_scopes,
        }
    }

    pub fn initialize_handlers(&mut self, providers: Providers) -> Result<Vec<HandlerLoadError>, ()> {
        let mut errors: Vec<HandlerLoadError> = Vec::default();
        let mut nav_comp_group = providers.navcomp_group().try_write().map_err(|_| ())?;

        for scope in self.scopes.iter_mut() {
            match providers
                .navcomp_loader()
                .load_handler(providers.config(), &scope, nav_comp_group.todo_sender().clone())
            {
                Ok(handler) => {
                    scope.handler = Some(handler);

                    let mut has_navcomp = false;
                    scope.handler.as_ref().map(|h| {
                        if let Some(navcomp) = h.navcomp() {
                            has_navcomp = true;
                            nav_comp_group.add_option(h.lang_id(), navcomp);
                        }
                    });

                    debug!(
                        "loaded handler for scope {:?}, has_navcomp: {}",
                        scope.path.absolute_path(),
                        has_navcomp
                    );
                }
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        debug!("created navcomp group with {} items", nav_comp_group.len());

        Ok(errors)
    }

    pub fn scopes(&self) -> &Vec<ProjectScope> {
        &self.scopes
    }
}
