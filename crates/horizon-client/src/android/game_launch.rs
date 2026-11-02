//! Frontend-independent game launch parameters. No launcher widgets or config reads.
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct LaunchRequest {
    pub resources: PathBuf,
    pub game_home: PathBuf,
    pub session_file: Option<PathBuf>,
    pub offline_name: String,
    pub server: String,
    pub control_socket: Option<PathBuf>,
    /// Optional Sylphra skin selected for this game session.
    pub skin_file: Option<PathBuf>,
    pub skin_slim: bool,
    pub extra_arguments: Vec<String>,
    pub open_gl: bool,
    pub gpu_batching: bool,
}
impl LaunchRequest {
    pub fn arguments(&self) -> Vec<String> {
        let mut args = vec![
            "--resources".into(),
            self.resources.to_string_lossy().into_owned(),
            "--game-dir".into(),
            self.game_home.to_string_lossy().into_owned(),
        ];
        if let Some(socket) = &self.control_socket {
            args.extend([
                "--control-socket".into(),
                socket.to_string_lossy().into_owned(),
            ]);
        }
        if let Some(session) = &self.session_file {
            args.extend([
                "--session-file".into(),
                session.to_string_lossy().into_owned(),
            ]);
        } else if !self.offline_name.is_empty() {
            args.extend(["--name".into(), self.offline_name.clone()]);
        }
        if !self.server.is_empty() {
            args.extend(["--server".into(), self.server.clone()]);
        }
        if let Some(skin) = &self.skin_file {
            args.extend(["--sylphra-skin".into(), skin.to_string_lossy().into_owned()]);
            args.extend([
                "--sylphra-skin-model".into(),
                if self.skin_slim { "slim" } else { "classic" }.into(),
            ]);
        }
        args.extend(self.extra_arguments.iter().cloned());
        args
    }
}
/// The native host owns the window lifecycle; a future frontend supplies this request.
#[cfg(target_os = "android")]
pub(crate) fn start(
    request: LaunchRequest,
) -> horizon_client::Result<Box<dyn winit::application::ApplicationHandler>> {
    if !request.game_home.is_absolute() || !request.resources.is_absolute() {
        return Err("Game and resource directories must be absolute".into());
    }
    let previous = std::env::current_dir()?;
    std::env::set_current_dir(&request.game_home)?;
    horizon_client::android_runtime::configure_graphics(request.open_gl);
    horizon_client::android_runtime::configure_gpu(request.gpu_batching);
    crate::game::android_reset_bundled_mods();
    let result = crate::game::android_game(request.arguments());
    if result.is_err() {
        let _ = std::env::set_current_dir(previous);
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    fn request() -> LaunchRequest {
        LaunchRequest {
            resources: "resources with spaces".into(),
            game_home: "profile".into(),
            session_file: None,
            offline_name: "Offline".into(),
            server: "example.test".into(),
            control_socket: None,
            skin_file: None,
            skin_slim: false,
            extra_arguments: vec![],
            open_gl: false,
            gpu_batching: false,
        }
    }
    #[test]
    fn session_identity_wins_over_offline_name() {
        let mut r = request();
        r.session_file = Some("private/session.json".into());
        let args = r.arguments();
        assert!(!args.iter().any(|a| a == "--name"));
        assert!(
            args.windows(2)
                .any(|a| a == ["--session-file", "private/session.json"])
        );
    }
    #[test]
    fn selected_skin_is_passed_as_separate_arguments() {
        let mut r = request();
        r.skin_file = Some("private skins/custom.png".into());
        r.skin_slim = true;
        let args = r.arguments();
        assert!(args.windows(2).any(|a| a == ["--sylphra-skin", "private skins/custom.png"]));
        assert!(args.windows(2).any(|a| a == ["--sylphra-skin-model", "slim"]));
    }
    #[test]
    fn no_selected_skin_keeps_launch_arguments_unchanged() {
        let args = request().arguments();
        assert!(!args.iter().any(|a| a == "--sylphra-skin" || a == "--sylphra-skin-model"));
    }
    #[test]
    fn paths_are_arguments_not_shell_text_and_snapshot_is_independent() {
        let mut r = request();
        let snapshot = r.arguments();
        r.offline_name = "Changed".into();
        assert_eq!(snapshot[1], "resources with spaces");
        assert!(snapshot.windows(2).any(|a| a == ["--name", "Offline"]));
        assert!(snapshot.windows(2).any(|a| a == ["--game-dir", "profile"]));
        assert!(
            snapshot
                .windows(2)
                .any(|a| a == ["--server", "example.test"])
        );
    }
}
