use std::{env, fs::read_to_string, io};

pub fn get_input() -> Vec<String> {

    let input_path: String = format!(
        r"{}\src\input.txt",
        env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
    );
    return read_file_line_by_line(&input_path).unwrap();
}

pub fn read_file_line_by_line(filepath: &str) -> Result<Vec<String>, io::Error> {
    Ok(read_to_string(filepath)?
        .lines()
        .map(|a| a.to_string())
        .collect())
}