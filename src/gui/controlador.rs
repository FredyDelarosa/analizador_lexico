use std::path::PathBuf;

use rfd::FileDialog;

use crate::lexer::{
    diccionario::cargar_diccionario,
    clasificador::{clasificar_palabra, Token},
    dfa_diccionario::DFADiccionario,
    salida::exportar_tokens,
};
use crate::utils::archivos::leer_texto_a_palabras;

pub struct ControladorGUI {
    pub diccionario: Option<crate::lexer::diccionario::Diccionario>,
    pub dfa_diccionario: Option<DFADiccionario>,
    pub ruta_diccionario: Option<PathBuf>,
    pub ruta_texto: Option<PathBuf>,
    pub resultados: Vec<Token>,
    pub texto_mostrar: String,
    pub stats: String,
    pub recorrido_diccionario: String,
}

impl ControladorGUI {
    pub fn new() -> Self {
        Self {
            diccionario: None,
            dfa_diccionario: None,
            ruta_diccionario: None,
            ruta_texto: None,
            resultados: Vec::new(),
            texto_mostrar: String::new(),
            stats: "---".to_string(),
            recorrido_diccionario: "---".to_string(),
        }
    }

    pub fn cargar_diccionario(&mut self) {
        if let Some(path) = FileDialog::new().add_filter("txt", &["txt"]).pick_file() {
            match cargar_diccionario(&path) {
                Ok(dic) => {
                    let mut dfa = DFADiccionario::nuevo();
                    for clave in dic.keys() {
                        dfa.agregar_palabra(clave);
                    }

                    self.diccionario = Some(dic);
                    self.dfa_diccionario = Some(dfa);
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
        self.recorrido_diccionario.clear();

        if self.diccionario.is_none() || self.dfa_diccionario.is_none() || self.ruta_texto.is_none()
        {
            self.texto_mostrar = "Debe cargar diccionario y texto antes de analizar.".into();
            return;
        }

        let dic = self.diccionario.as_ref().unwrap();
        let dfa_dic = self.dfa_diccionario.as_ref().unwrap();
        let ruta = self.ruta_texto.as_ref().unwrap();

        let palabras = match leer_texto_a_palabras(ruta) {
            Ok(p) => p,
            Err(e) => {
                self.texto_mostrar = e;
                return;
            }
        };

        for p in &palabras {
            self.resultados.push(clasificar_palabra(p, dic, dfa_dic));
        }

        // Resultados
        let mut out = "Token\tLexema\n".to_string();
        for t in &self.resultados {
            out += &format!("{}\t{}\n", t.tipo_str(), t.lexema);
        }
        self.texto_mostrar = out;

        // Estadísticas
        let total = self.resultados.len();
        let claves = self.resultados.iter().filter(|t| t.es_clave()).count();
        let ids = self.resultados.iter().filter(|t| t.es_ident()).count();
        let errs = self.resultados.iter().filter(|t| t.es_error()).count();

        self.stats = format!(
            "Analizadas: {}\nClaves: {}\nIdentificadores: {}\nErrores: {}",
            total, claves, ids, errs
        );

        // Recorrido del DFA del diccionario
        let mut rec = String::from("=== RECORRIDO DFA DEL DICCIONARIO ===\n");

        for token in &self.resultados {
            rec += &format!("\nPalabra: {}\n", token.lexema);

            for (est, c) in &token.dfa_diccionario.recorrido {
                rec += &format!("  {:?} -- '{}' -->\n", est, c);
            }

            if let Some(err) = &token.dfa_diccionario.error {
                rec += &format!("ERROR: {}\n", err);
            }

            rec += "------------------------------------\n";
        }

        self.recorrido_diccionario = rec;

        let _ = exportar_tokens("tokens_salida.txt", &self.resultados);
    }
}
