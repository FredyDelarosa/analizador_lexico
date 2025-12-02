use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Estado(pub usize);

#[derive(Debug, Clone)]
pub struct ResultadoDFADiccionario {
    pub recorrido: Vec<(Estado, char)>,
    pub estado_final: Estado,
    pub valido: bool,
    pub error: Option<String>,
}

#[derive(Debug)]
pub struct DFADiccionario {
    pub trans: HashMap<(Estado, char), Estado>,
    pub estado_inicial: Estado,
    pub estados_aceptacion: HashSet<Estado>,
    pub siguiente_estado: usize,
}

impl DFADiccionario {
    pub fn nuevo() -> Self {
        Self {
            trans: HashMap::new(),
            estado_inicial: Estado(0),
            estados_aceptacion: HashSet::new(),
            siguiente_estado: 1,
        }
    }

    fn nuevo_estado(&mut self) -> Estado {
        let e = Estado(self.siguiente_estado);
        self.siguiente_estado += 1;
        e
    }

    pub fn agregar_palabra(&mut self, palabra: &str) {
        let mut actual = self.estado_inicial.clone();

        for c in palabra.chars() {
            let clave = (actual.clone(), c);

            let siguiente = if let Some(est) = self.trans.get(&clave) {
                est.clone()
            } else {
                let nuevo = self.nuevo_estado();
                self.trans.insert(clave, nuevo.clone());
                nuevo
            };

            actual = siguiente;
        }

        self.estados_aceptacion.insert(actual);
    }

    pub fn recorrer(&self, palabra: &str) -> ResultadoDFADiccionario {
        let mut actual = self.estado_inicial.clone();
        let mut recorrido = Vec::new();

        for c in palabra.chars() {
            recorrido.push((actual.clone(), c));

            let clave = (actual.clone(), c);

            if let Some(destino) = self.trans.get(&clave) {
                actual = destino.clone();
            } else {
                return ResultadoDFADiccionario {
                    recorrido,
                    estado_final: actual.clone(),
                    valido: false,
                    error: Some(format!(
                        "No existe transición para estado {:?} con '{}'",
                        actual, c
                    )),
                };
            }
        }

        let valido = self.estados_aceptacion.contains(&actual);

        ResultadoDFADiccionario {
            recorrido,
            estado_final: actual.clone(),
            valido,
            error: if valido {
                None
            } else {
                Some("Terminó en un estado no aceptador".into())
            },
        }
    }
}
