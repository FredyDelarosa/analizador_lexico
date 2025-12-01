use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub type Diccionario = HashMap<String, String>;

pub fn cargar_diccionario(path: &Path) -> Result<Diccionario, String> {
    let contenido = fs::read_to_string(path)
        .map_err(|e| format!("Error al leer diccionario: {}", e))?;

    let mut mapa = HashMap::new();

    for (i, linea) in contenido.lines().enumerate() {
        let linea = linea.trim();
        if linea.is_empty() {
            continue;
        }

        let partes: Vec<&str> = linea.split_whitespace().collect();
        if partes.len() != 2 {
            return Err(format!("Formato inválido en línea {}: {}", i + 1, linea));
        }

        mapa.insert(partes[1].to_string(), partes[0].to_string());
    }

    Ok(mapa)
}
