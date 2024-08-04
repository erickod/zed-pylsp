/* SPDX-License-Identifier: Apache-2.0 */
use std::str::FromStr;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

struct PyLSP {}

impl PyLSP {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("pylsp") {
            return Ok(path);
        }
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );
        return Err(String::from_str("pylsp must be installed manually").unwrap());
    }
}

impl zed::Extension for PyLSP {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(PyLSP);
