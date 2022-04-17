pub fn correct(path: String) -> String {
    let mut new_path = path;

    if cfg!(target_os = "windows") {
        new_path = new_path.replace("/", "\\");

        if new_path.contains("\\*") {
            new_path = new_path.split("\\*").next().unwrap().to_string();
        }
    } else {
        new_path = new_path.replace("\\", "/");

        if new_path.contains("/*") {
            new_path = new_path.split("/*").next().unwrap().to_string();
        }
    }

    if new_path.starts_with("/") || new_path.starts_with("\\") {
        new_path = new_path.replacen("/", "", 1);
        new_path = new_path.replacen("\\", "", 1);
    }

    new_path
}

pub fn glob_dir(path: String) -> String {
    let mut new_path = correct(path);

    if cfg!(target_os = "windows") || new_path.starts_with(".\\") {
        new_path = new_path.replacen(".\\", "", 1);

    } else if new_path.starts_with("./") {
        new_path = new_path.replacen("./", "", 1);
    }

    new_path
}

pub fn join(pth1: String, pth2: String) -> String {
    let mut new_path = pth1;

    if cfg!(target_os = "windows") {
        new_path = new_path.replace("/", "\\");

        if !new_path.ends_with("\\") {
            new_path += "\\";
        }
    } else {
        new_path = new_path.replace("\\", "/");

        if !new_path.ends_with("/") {
            new_path += "/";
        }
    }

    format!("{}{}", new_path, correct(pth2))
}
