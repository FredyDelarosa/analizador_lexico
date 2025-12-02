use crate::lexer::diccionario::Diccionario;
use crate::lexer::dfa_diccionario::{DFADiccionario, ResultadoDFADiccionario};
use crate::lexer::expresiones::es_identificador;

#[derive(Debug, Clone)]
pub enum TipoToken {
    PalabraClave(String),
    Identificador,
    ErrorLexico,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tipo: TipoToken,
    pub lexema: String,
    pub dfa_diccionario: ResultadoDFADiccionario,
}

impl Token {
    pub fn tipo_str(&self) -> String {
        match &self.tipo {
            TipoToken::PalabraClave(t) => t.clone(),
            TipoToken::Identificador => "IDENTIFICADOR".into(),
            TipoToken::ErrorLexico => "ERROR_LEXICO".into(),
        }
    }

    pub fn es_clave(&self) -> bool {
        matches!(self.tipo, TipoToken::PalabraClave(_))
    }

    pub fn es_ident(&self) -> bool {
        matches!(self.tipo, TipoToken::Identificador)
    }

    pub fn es_error(&self) -> bool {
        matches!(self.tipo, TipoToken::ErrorLexico)
    }
}

pub fn clasificar_palabra(
    palabra: &str,
    dic: &Diccionario,
    dfa_dic: &DFADiccionario,
) -> Token {
    let recorrido_dic = dfa_dic.recorrer(palabra);

    if recorrido_dic.valido {
        let token = dic.get(palabra).unwrap_or(&"RESERVADA".into()).clone();
        return Token {
            tipo: TipoToken::PalabraClave(token),
            lexema: palabra.into(),
            dfa_diccionario: recorrido_dic,
        };
    }

    if es_identificador(palabra) {
        return Token {
            tipo: TipoToken::Identificador,
            lexema: palabra.into(),
            dfa_diccionario: recorrido_dic,
        };
    }

    Token {
        tipo: TipoToken::ErrorLexico,
        lexema: palabra.into(),
        dfa_diccionario: recorrido_dic,
    }
}
