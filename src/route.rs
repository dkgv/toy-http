use regex::Regex;
use std::cmp::{Ord, PartialOrd};
use std::collections::BTreeMap;

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Clone)]
pub enum HttpMethod {
    Get,
    Post,
}

#[derive(Eq, PartialEq, Hash, Clone, Ord, PartialOrd, Copy)]
pub enum ParamType {
    Str,
    Int,
    Float,
}

impl ParamType {
    fn regex(self) -> String {
        let regex = match self {
            ParamType::Str => r".+",
            ParamType::Int => r"[0-9]+",
            ParamType::Float => r"[-+]?[0-9]*\.?[0-9]+",
        };
        String::from(regex)
    }
}

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Clone)]
pub struct Route {
    method: HttpMethod,
    params: BTreeMap<String, ParamType>,
    pub regex: String,
}

impl Route {
    pub fn new(endpoint: &String, method: HttpMethod) -> Route {
        let param_matcher = Regex::new(r"<([A-Za-z]+):([A-Za-z]+)>").unwrap();
        let mut params: BTreeMap<String, ParamType> = BTreeMap::new();
        let mut endpoint_regex = endpoint.clone();

        for group in param_matcher.captures_iter(endpoint) {
            let param_name = &group[1];
            let param_type = match group[2].to_lowercase().as_str() {
                "int" => ParamType::Int,
                "str" => ParamType::Str,
                "float" => ParamType::Float,
                _ => ParamType::Str,
            };
            // Replace param with equivalent regex
            endpoint_regex = endpoint_regex.replace(&group[0], &param_type.regex());
            params.insert(param_name.to_string(), param_type);
        }

        Route {
            method: method,
            params: params,
            regex: endpoint_regex,
        }
    }
}
