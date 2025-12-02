use regex::Regex;

pub fn es_identificador(palabra: &str) -> bool {
    let re = Regex::new(r"^[a-z]+(_[a-z0-9]+)*$").unwrap();
    re.is_match(palabra)
}
