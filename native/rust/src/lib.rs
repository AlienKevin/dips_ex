use std::{
    ffi::CString,
    os::raw::{c_char, c_float},
};

#[link(name = "bert")]
extern "C" {
    fn init(path_model: *const c_char) -> bool;
    fn free_ctx();
    fn run(input: *const c_char) -> *mut c_float;
}

pub enum CutMode {
    Fine,
    Coarse,
    Dips,
    DipsStr,
}

#[rustler::nif]
pub fn init_model(path_model: &str) {
    let path_model = CString::new(path_model).unwrap();
    unsafe { init(path_model.as_ptr()) };
}

#[rustler::nif]
pub fn run_model(input: &str, mode: &str) -> Vec<String> {
    let mode = match mode {
        "fine" => CutMode::Fine,
        "coarse" => CutMode::Coarse,
        "dips" => CutMode::Dips,
        "dips_str" => CutMode::DipsStr,
        _ => CutMode::Fine,
    };

    let text = CString::new(input).unwrap();
    let logits = unsafe { run(text.as_ptr()) };
    let logits_len = (input.chars().count() + 2) * 4;
    let logits_slice = unsafe { std::slice::from_raw_parts(logits, logits_len) };

    const N_TAGS: usize = 4;
    let mut predictions = Vec::new();

    for chunk in logits_slice[N_TAGS..logits_slice.len() - N_TAGS].chunks(N_TAGS) {
        let max_index = chunk
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(index, _)| index)
            .unwrap();
        predictions.push(max_index);
    }

    let dips_result: String = input
        .chars()
        .zip(predictions.iter())
        .map(|(char, &pred)| format!("{}{}", ["-", "", "|", " "][pred], char))
        .collect::<String>()
        .trim_start()
        .to_string();

    match mode {
        CutMode::Fine => dips_result
            .split(|c| c == '-' || c == '|' || c == ' ')
            .map(String::from)
            .collect(),
        CutMode::Coarse => dips_result
            .replace("-", "")
            .replace("|", "")
            .split(' ')
            .map(String::from)
            .collect(),
        CutMode::Dips => predictions
            .iter()
            .map(|&pred| "DIPS".chars().nth(pred).unwrap().to_string())
            .collect(),
        CutMode::DipsStr => vec![dips_result],
    }
}

#[rustler::nif]
pub fn free_model() {
    unsafe { free_ctx() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let path = "models/electra-small-6-layers-q4_0.gguf";
        let model = BertModel::new(path);
        let result = model.run("阿張先生嗰時好nice㗎", CutMode::DipsStr);
        assert_eq!(result, vec!["阿-張|先生 嗰-時 好 nice 㗎"]);
    }
}

rustler::init!("Elixir.DipsEx");
