use std::path::PathBuf;

use rfd::FileDialog;

use crate::lexer::{
    diccionario::cargar_diccionario,
    clasificador::{clasificar_palabra, Token},
    salida::exportar_tokens,
};
use crate::utils::archivos::leer_texto_a_palabras;

pub struct ControladorGUI {
    pub diccionario: Option<crate::lexer::diccionario::Diccionario>,
    pub ruta_diccionario: Option<PathBuf>,
    pub ruta_texto: Option<PathBuf>,
    pub resultados: Vec<Token>,
    pub texto_mostrar: String,
    pub stats: String,
}

impl ControladorGUI {
    pub fn new() -> Self {
        Self {
            diccionario: None,
            ruta_diccionario: None,
            ruta_texto: None,
            resultados: Vec::new(),
            texto_mostrar: "Cargue los archivos para comenzar.".to_string(),
            stats: "Estadísticas: ---".to_string(),
        }
    }

    pub fn cargar_diccionario(&mut self) {
        if let Some(path) = FileDialog::new().add_filter("txt", &["txt"]).pick_file() {
            match cargar_diccionario(&path) {
                Ok(dic) => {
                    self.diccionario = Some(dic);
                    self.ruta_diccionario = Some(path);
                }
                Err(e) => self.texto_mostrar = e,
            }
        }
    }

    pub fn cargar_texto(&mut self) {
        if let Some(path) = FileDialog::new().add_filter("txt", &["txt"]).pick_file() {
            self.ruta_texto = Some(path);
        }
    }

    pub fn analizar(&mut self) {
        self.texto_mostrar.clear();
        self.resultados.clear();

        if self.diccionario.is_none() || self.ruta_texto.is_none() {
            self.texto_mostrar = "Debe cargar diccionario y texto antes de analizar.".to_string();
            return;
        }

        let dic = self.diccionario.as_ref().unwrap();
        let ruta = self.ruta_texto.as_ref().unwrap();

        let palabras = match leer_texto_a_palabras(ruta) {
            Ok(p) => p,
            Err(e) => {
                self.texto_mostrar = e;
                return;
            }
        };

        for p in palabras {
            self.resultados.push(clasificar_palabra(&p, dic));
        }

        // mostrar
        let mut out = "Token\tLexema\n".to_string();
        for t in &self.resultados {
            out += &format!("{}\t{}\n", t.tipo_str(), t.lexema);
        }
        self.texto_mostrar = out;

        // estadísticas
        let total = self.resultados.len();
        let claves = self.resultados.iter().filter(|t| t.es_clave()).count();
        let ids = self.resultados.iter().filter(|t| t.es_ident()).count();
        let errs = self.resultados.iter().filter(|t| t.es_error()).count();

        self.stats = format!(
            "Palabras analizadas: {}\nPalabras clave: {}\nIdentificadores: {}\nErrores léxicos: {}",
            total, claves, ids, errs
        );

        // exportar archivo
        let _ = exportar_tokens("tokens_salida.txt", &self.resultados);
    }
}
