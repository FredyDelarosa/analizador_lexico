use crate::lexer::diccionario::Diccionario;
use crate::lexer::expresiones;
//use crate::lexer::salida::exportar_tokens;

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
}

impl Token {
    pub fn tipo_str(&self) -> String {
        match &self.tipo {
            TipoToken::PalabraClave(t) => t.clone(),
            TipoToken::Identificador => "IDENTIFICADOR".to_string(),
            TipoToken::ErrorLexico => "ERROR_LEXICO".to_string(),
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

pub fn clasificar_palabra(palabra: &str, dic: &Diccionario) -> Token {
    if let Some(token) = dic.get(palabra) {
        return Token {
            tipo: TipoToken::PalabraClave(token.clone()),
            lexema: palabra.to_string(),
        };
    }

    if expresiones::es_identificador(palabra) {
        return Token {
            tipo: TipoToken::Identificador,
            lexema: palabra.to_string(),
        };
    }

    Token {
        tipo: TipoToken::ErrorLexico,
        lexema: palabra.to_string(),
    }
}
