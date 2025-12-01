use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_IDENT: Regex = Regex::new(r"^[a-z]+(_[a-z0-9]+)*$").unwrap();
}

pub fn es_identificador(palabra: &str) -> bool {
    RE_IDENT.is_match(palabra)
}
