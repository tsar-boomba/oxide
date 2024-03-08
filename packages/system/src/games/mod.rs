pub mod console;

pub use console::Console;
use fixed_map::Map;
use futures_util::{future::join_all, TryStreamExt};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tokio_stream::wrappers::ReadDirStream;

use std::{
    io,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Game {
    path: PathBuf,
    console: Console,
    core: String,
}

impl Game {
    /// Returns `None` if path isnt valid utf-8 or the extension isn't recognized
    fn new(path: impl AsRef<Path>) -> Option<Self> {
        let path = path.as_ref();
        let ext = path.extension()?.to_str()?;
        let console = Console::from_str(ext)?;

        if path.file_name()?.to_str()?.starts_with("._") {
            return None;
        }

        Some(Self {
            path: path.to_path_buf(),
            core: console.default_core().into(),
            console,
        })
    }

    /// Checks that `path` points to an actual file
    async fn exists(&self) -> bool {
        match tokio::fs::try_exists(&self.path).await {
            Ok(true) => {
                // File exists
                true
            }
            _ => {
                // File don't exist
                false
            }
        }
    }

    #[inline]
    pub fn as_path(&self) -> &Path {
        &self.path
    }

    #[inline]
    pub fn full_name(&self) -> &str {
        // Will fail if da file name isnt unicode so no one should do that
        self.path.file_stem().unwrap().to_str().unwrap()
    }

    #[inline]
    pub fn console(&self) -> &Console {
        &self.console
    }

    #[inline]
    pub fn core(&self) -> &str {
        &self.core
    }
}

pub type GameCache = Map<Console, Arc<[Game]>>;

type IntermediateGameCache = Map<Console, Vec<Game>>;

fn intermediate_to_final(mut intermediate: IntermediateGameCache) -> GameCache {
    let mut final_map: GameCache = GameCache::new();

    for console in Console::iter() {
        let games: Vec<Game> = intermediate.remove(console).unwrap_or_default();
        final_map.insert(console, (*games).into());
    }

    final_map
}

pub async fn refresh_game_cache() -> io::Result<GameCache> {
    tracing::debug!("Refreshing cache");
    // First, make sure the folder for each console exist
    let create_dirs = Console::iter()
        .map(|console| {
            tokio::spawn(async move {
                tokio::fs::create_dir_all(format!("/mnt/SDCARD/Games/{}", console.name())).await
            })
        })
        .collect::<Vec<_>>();

    // Wait for all dirs to be created and panic on errs
    for task in join_all(create_dirs).await {
        task.unwrap()?;
    }

    tracing::debug!("Reading games");
    // Read all the files in each game dir and turn them into `Game` structs
    let get_games = Console::iter()
        .map(|console| {
            tokio::spawn(async move {
                let mut dir = ReadDirStream::new(
                    tokio::fs::read_dir(format!("/mnt/SDCARD/Games/{}", console.name())).await?,
                );
                let mut games = Vec::<Game>::new();

                tracing::debug!("Reading console: {}", console.name());
                while let Some(file) = dir.try_next().await? {
                    let file_type = file.file_type().await?;

                    if file_type.is_file() {
                        // Attempt to create `Game`
                        let path = file.path();

                        if let Some(game) = Game::new(&path) {
                            games.push(game);
                        } else {
                            tracing::error!("Failed to create game from {}", path.display());
                        };
                    }
                }

                tracing::debug!("Done with console: {}", console.name());
                Ok::<Vec<Game>, io::Error>(games)
            })
        })
        .collect::<Vec<_>>();

    // Intermediate map to collect all games into correct arrays
    let mut intermediate: IntermediateGameCache = IntermediateGameCache::new();

    // Make sure all console at least have an empty Vec
    for console in Console::iter() {
        intermediate.insert(console, Vec::new());
    }

    tracing::debug!("Sorting games");
    // Go through each game we found and sort them into the hash map
    for game_dir in join_all(get_games)
        .await
        .into_iter()
        .map(|task| task.unwrap())
    {
        for game in game_dir? {
            intermediate.get_mut(*game.console()).unwrap().push(game);
        }
    }

    tracing::debug!("Writing games");
    // Serialize using intermediate because Arc<[Game]> isn't Serialize
    tokio::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(".game_cache.json")
        .await?
        .write_all(&serde_json::to_vec(&intermediate).unwrap())
        .await?;

    Ok(intermediate_to_final(intermediate))
}

/// Gets games from cache or creates cache, then check that all games exist
pub(crate) async fn init() -> io::Result<GameCache> {
    // Always just get all games on startup
    let games = refresh_game_cache().await?;

    // Make sure there is a save dir for each core
    let mut dir = tokio::fs::read_dir("/mnt/SDCARD/Cores").await?;

    while let Some(file) = dir.next_entry().await? {
        if file.file_type().await?.is_file() {
            let path = file.path();
            let core_name = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .trim_end_matches("_libretro.so");

            if let Err(err) =
                tokio::fs::create_dir_all(format!("/mnt/SDCARD/Saves/{core_name}/saves")).await
            {
                if err.kind() != io::ErrorKind::AlreadyExists {
                    return Err(err);
                }
            };
        }
    }

    Ok(games)
}
