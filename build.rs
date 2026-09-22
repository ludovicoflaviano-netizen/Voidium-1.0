fn main() {
    tauri_runtime_verso_build::get_verso_as_external_bin().expect("failed to acquire Verso runtime");
    tauri_build::build();
}
