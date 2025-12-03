use crate::mecab::mecab_tokenize;
use crate::SRC_JSON;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Language {
    English,
    Japanese,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Category {
    MachineLearning,
    NaturalLanguageProcessing,
    Security,
    Organization,
    ComputerVision,
    Item,
    Topic,
    Task,
    Other,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Keyword {
    pub word: String,
    pub alias: String,
    pub score: isize,
    pub language: Language,
    pub category: Category,
}

impl Keyword {
    pub fn get_keyword_ptn(&self) -> String {
        // _keyword = self.word.lower().replace("-", r"(\-|\s)*").replace(" ", r"(\s|\-)*")
        // return re.compile(
        //     rf"""(?P<PREK>(^|\s|\(|'|")+)(?P<KEYWORD>{_keyword}(s|ing|al|d|ed|\-[^\s]+)*)(?P<POSTK>($|\s|\.|,|:|;|\)|'|")+)""",
        //     flags=re.IGNORECASE | re.MULTILINE | re.DOTALL,
        let kwd = self
            .word
            .to_lowercase()
            .replace("-", r"(\-|\s)*")
            .replace(" ", r"(\s|\-)*");
        let ptn = format!(
            r#"(^|\s|\(|'|"|\-)+(?i){}(s|ing|al|d|ed|\-[^\s]+)*($|\s|\)|\(|\.|,|:|;|\)|'|")+"#,
            kwd
        );
        return ptn;
    }
}

pub fn load_keywords() -> Vec<Keyword> {
    return serde_json::from_slice(SRC_JSON).expect("Unable to parse json");
}

pub fn load_keywords_from_rsc(rsc: &str) -> Vec<Keyword> {
    let src_json = std::fs::read_to_string(rsc).expect("Unable to read file");
    return serde_json::from_str(&src_json).expect("Unable to parse json");
}

pub fn extract_keywords(text: &str, keywords: Vec<Keyword>, lang: Language) -> Vec<Keyword> {
    let mut extracted_keywords: Vec<Keyword> = Vec::new();

    if lang == Language::English {
        keywords.iter().for_each(|keyword| {
            let re_str = keyword.get_keyword_ptn();
            let re = Regex::new(&re_str).unwrap();
            if re.is_match(text) {
                extracted_keywords.push(keyword.clone());
            }
        });
    } else if lang == Language::Japanese {
        let tokens = mecab_tokenize(text);
        let mecabed_text = tokens
            .iter()
            .map(|t| t.surface.clone())
            .collect::<Vec<String>>()
            .join(" ");
        keywords.iter().for_each(|keyword| {
            let re_str = keyword.get_keyword_ptn();
            let re = Regex::new(&re_str).unwrap();
            if text.contains(&keyword.word) {
                extracted_keywords.push(keyword.clone());
            } else if re.is_match(&mecabed_text) {
                extracted_keywords.push(keyword.clone());
            }
        });
    }

    return extracted_keywords;
}
