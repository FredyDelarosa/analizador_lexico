use std::fs::File;
use std::io::{Write, Result};

use super::clasificador::Token;

pub fn exportar_tokens(ruta: &str, tokens: &[Token]) -> Result<()> {
    let mut file = File::create(ruta)?;
    writeln!(file, "Token\tLexema")?;

    for t in tokens {
        writeln!(file, "{}\t{}", t.tipo_str(), t.lexema)?;
    }

    Ok(())
}
