//! # Evo
//! Evo is a graphical environment variable editor for Linux (library).

use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

/// Fetch current environment variables
pub fn fetch_vars() -> Result<HashMap<String, String>, io::Error> {
    let mut vars = HashMap::new();
    let evo_path = get_evo_path()?;
    if evo_path.exists() {
        let f = File::open(evo_path)?;
        let reader = BufReader::new(f);
        let lines = reader.lines().map(|l| l.unwrap());
        for line in lines {
            let assignment: Vec<&str> = line.split(' ').last().unwrap().split('=').collect();
            let key = assignment[0];
            let value = assignment[1];
            vars.insert(key.to_string(), value.to_string());
        }
    } else {
        for (key, value) in std::env::vars_os() {
            vars.insert(
                key.to_str().unwrap().to_string(),
                value.to_str().unwrap().to_string(),
            );
        }
    }
    Ok(vars)
}

/// Make backup for evo file
pub fn make_backup(vars: HashMap<String, String>) -> Result<(), io::Error> {
    let home_path = match home::home_dir() {
        Some(v) => v,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Impossible to get your home dir!",
            ))
        }
    };
    let evo_back_path = home_path.join(".local/share/evo/.evo.bk");
    if evo_back_path.exists() {
        fs::remove_file(evo_back_path.clone())?;
    }
    let mut evo_file = File::create(evo_back_path)?;
    for (key, value) in vars {
        evo_file.write_all(format!("export {}={}\n", key, value).as_bytes())?;
    }
    Ok(())
}

/// Restore the backup
pub fn restore_backup() -> Result<(), io::Error> {
    let home_path = match home::home_dir() {
        Some(v) => v,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Impossible to get your home dir!",
            ))
        }
    };
    let evo_back_path = home_path.join(".local/share/evo/.evo.bk");
    let evo_path = get_evo_path()?;
    fs::copy(evo_back_path, evo_path)?;
    Ok(())
}

/// Set a new(s) variable(s)
pub fn set_var(key: &str, value: String) -> Result<HashMap<String, String>, io::Error> {
    let evo_path = get_evo_path()?;
    if !evo_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "The evo file does not exist for the user!",
        ));
    }
    let mut vars = fetch_vars()?;
    vars.insert(key.to_string(), value.clone());
    let mut evo_file = OpenOptions::new().write(true).append(true).open(evo_path)?;
    writeln!(evo_file, "export {}={}", key, value)?;
    Ok(vars)
}

/// Edit an existing variable
pub fn edit_var(key: &str, value: String) -> Result<HashMap<String, String>, io::Error> {
    let evo_path = get_evo_path()?;
    if !evo_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "The evo file does not exist for the user!",
        ));
    }
    let mut vars = fetch_vars()?;
    if !vars.contains_key(key) {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Key not found!"));
    }
    let evo_data = fs::read_to_string(evo_path.clone())?;
    fs::write(
        evo_path,
        evo_data
            .replace(
                format!("{}={}", key, vars.get(key).unwrap()).as_str(),
                format!("{}={}", key, value).as_str(),
            )
            .as_bytes(),
    )?;
    match vars.get_mut(key) {
        Some(v) => *v = value,
        None => return Err(io::Error::new(io::ErrorKind::NotFound, "Key not found")),
    }
    Ok(vars)
}

/// Unset an existing variable
pub fn unset_var(key: &str) -> Result<HashMap<String, String>, io::Error> {
    let evo_path = get_evo_path()?;
    if !evo_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "The evo file does not exist for the user!",
        ));
    }
    let mut vars = fetch_vars()?;
    if !vars.contains_key(key) {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Key not found!"));
    }
    let evo_data = OpenOptions::new()
        .read(true)
        .write(true)
        .open(evo_path.clone())?;
    let lines = BufReader::new(evo_data)
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l != &format!("export {}={}", key, vars.get(key).unwrap()))
        .collect::<Vec<String>>()
        .join("\n");
    fs::write(evo_path, lines)?;
    match vars.get_mut(key) {
        Some(v) => *v = String::new(),
        None => return Err(io::Error::new(io::ErrorKind::NotFound, "Key not found")),
    }
    Ok(vars)
}

/// Create an evo file in the user directory
pub fn create_evo(vars: HashMap<String, String>) -> Result<(), io::Error> {
    let evo_path = get_evo_path()?;
    let mut evo_file = File::create(evo_path)?;
    for (key, value) in vars {
        evo_file.write_all(format!("export {}={}\n", key, value).as_bytes())?;
    }
    Ok(())
}

/// Get evo file path
pub fn get_evo_path() -> Result<PathBuf, io::Error> {
    let home_path = match home::home_dir() {
        Some(v) => v,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Impossible to get your home dir!",
            ))
        }
    };
    Ok(home_path.join(".evo"))
}

mod tests {
    #[test]
    fn test_fetch_vars() {
        let vars = crate::fetch_vars().unwrap();
        assert_eq!(vars["SHLVL"], "0");
    }
    #[test]
    fn test_create_evo() {
        crate::create_evo(crate::fetch_vars().unwrap()).unwrap();
    }
    #[test]
    fn test_set_var() {
        crate::set_var("SHLVL", String::from("0")).unwrap();
    }
    #[test]
    fn test_edit_var() {
        crate::edit_var("SHLVL", String::from("2")).unwrap();
    }
    #[test]
    fn test_unset_var() {
        crate::unset_var("SHLVL").unwrap();
    }
}
