use amethyst::utils::application_root_dir;

pub fn resources_dir() -> String {
    format!("{}/resources", application_root_dir())
}

pub fn resource<T: ToString>(path: T) -> String {
    format!("{}/{}", resources_dir(), path.to_string())
}
