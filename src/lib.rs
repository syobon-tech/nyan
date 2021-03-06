use wasm_bindgen::prelude::*;
use regex::Regex;

#[wasm_bindgen]
pub fn translate(text: &str, bunmatsu: &str, nai_to_nyai: bool) -> String {
    let re1 = Regex::new(r"(.+?)(である|だ|です)(。|\(|\s|!|\?|！|？|…|$)").unwrap();
    let re2 = Regex::new(r"([えけせてねへめれ])ます(。|\(|\s|!|\?|！|？|…|$)").unwrap();
    let re3 = Regex::new(r"(\S+?[^、。\)\s])(。|\(|\s|!|\?|！|？|…|$)").unwrap();
    let nai = format!("ない{}", bunmatsu);
    let re4 = Regex::new(&nai).unwrap();
    let re5 = Regex::new(r"ない").unwrap();

    let mut new: String;
    new = text.to_string();
    new = re1.replace_all(&new, "$1$3").to_mut().to_string();
    new = re2.replace_all(&new, "$1る$2").to_mut().to_string();
    let replacement: &str = &format!("$1{}$2", bunmatsu);
    new = re3.replace_all(&new, replacement).to_mut().to_string();
    if nai_to_nyai {
        new = re4.replace_all(&new, "にゃい").to_mut().to_string();
        new = re5.replace_all(&new, "にゃい").to_mut().to_string();
    }
    return new;
}