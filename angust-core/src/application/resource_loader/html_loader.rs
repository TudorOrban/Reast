use std::{fs, path::PathBuf};

use super::path_navigator;

pub fn load_index_html(index_html_relative_path: String) -> Option<String> {
    let project_root_path = path_navigator::identify_project_root_path();
    let path = 
        project_root_path + "/" +
        index_html_relative_path.as_str();

    fs::read_to_string(PathBuf::from(path))
        .map_or(None, |content| Some(content))
}

pub fn load_html(html_directory_relative_path: String, html_file_relative_path: String) -> Option<String> {
    let path = 
        path_navigator::get_html_directory_path(html_directory_relative_path) + "/" +
        html_file_relative_path.as_str();

    fs::read_to_string(PathBuf::from(path))
        .map_or(None, |content| Some(content))
}