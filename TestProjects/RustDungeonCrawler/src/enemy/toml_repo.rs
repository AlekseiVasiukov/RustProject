use crate::enemy::{Enemy, EnemyBase, EnemyRepository};
use serde::Deserialize;
// use std::error::Error;
use std::{env, error::Error};

#[derive(Debug, Deserialize)]
struct EnemyFile {
    enemy_bases: Vec<EnemyToml>,
}

#[derive(Debug, Deserialize)]
struct EnemyToml {
    #[serde(rename = "name")]
    base: String,
    is_flying: bool,
}

pub struct TomlEnemyRepository {
    file_path: String,
}

impl TomlEnemyRepository {
    /// Creates a new instance of the TomlEnemyRepository.
    /// This version is expanded for clarity.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // 1. Get the full path to the currently running executable.
        //    env::current_exe() returns a Result, which we handle with `?`.
        //    If it fails, the function will return the error immediately.
        let exe_path = env::current_exe()?;

        // 2. Get the parent directory of the executable.
        //    This returns an Option because a path might not have a parent (e.g., "/").
        let exe_dir_option = exe_path.parent();

        // 3. Handle the Option to ensure we have a valid directory.
        //    We use a `match` statement to explicitly handle both Some and None cases.
        let assets_dir = match exe_dir_option {
            Some(dir) => {
                // If we have a directory, append the "assets" folder to its path.
                dir.join("assets")
            }
            None => {
                // If there is no parent directory, we cannot proceed.
                // We return a descriptive error. `.into()` converts the string to a Box<dyn Error>.
                return Err("Could not determine the executable's parent directory.".into());
            }
        };

        // 4. Construct the final path to the TOML file.
        let enemy_file_path = assets_dir.join("Enemy_Bases.toml");

        // 5. The struct field `file_path` is a String, but `enemy_file_path` is a PathBuf.
        //    We must convert it. This conversion can fail if the path contains non-UTF-8 characters.
        //    `to_str()` returns an Option<&str>.
        let file_path_str = match enemy_file_path.to_str() {
            Some(path_str) => {
                // If the conversion is successful, create an owned String from the &str.
                path_str.to_string()
            }
            None => {
                // If the path is not valid UTF-8, return an error.
                return Err("The constructed file path is not valid UTF-8.".into());
            }
        };

        // 6. If all steps were successful, create an instance of the struct and return it.
        Ok(Self {
            file_path: file_path_str,
        })
    }
}

impl EnemyRepository for TomlEnemyRepository {
    /// Loads enemy definitions from a TOML file.
    fn load_all_possible_enemies(&self) -> Result<Vec<Enemy>, Box<dyn Error>> {
        let text = std::fs::read_to_string(&self.file_path)?;
        let data: EnemyFile = toml::from_str(&text)?;

        let mut enemies = Vec::new();
        for (idx, entry) in data.enemy_bases.into_iter().enumerate() {
            let base = match EnemyBase::try_from_name(&entry.base) {
                Ok(base) => base,
                Err(err) => {
                    eprintln!("Enemy_Bases.toml: {err}");
                    continue;
                }
            };

            enemies.push(Enemy {
                id: (idx + 1) as u32,
                base,
            });
        }

        Ok(enemies)
    }
}