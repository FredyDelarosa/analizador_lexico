use std::fs;
use std::path::Path;

pub fn leer_archivo_string(path: &Path) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("No se pudo leer archivo {:?}: {}", path, e))
}

pub fn leer_texto_a_palabras(path: &Path) -> Result<Vec<String>, String> {
    let contenido = leer_archivo_string(path)?;
    let palabras = contenido
        .split_whitespace()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    Ok(palabras)
}
