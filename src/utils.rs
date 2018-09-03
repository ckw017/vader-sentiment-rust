use std::collections::HashMap;
use regex::Regex;

pub const B_INCR: f64 =  0.293;
pub const B_DECR: f64 = -0.293;

pub const C_INCR:   f64 =  0.733;
pub const N_SCALAR: f64 = -0.740;

pub const NORMALIZATION_ALPHA: f64 = 15.0;

pub const PUNC_LIST: [&'static str; 17] =
    [".", "!", "?", ",", ";", ":", "-", "'", "\"","!!",
    "!!!", "??", "???", "?!?", "!?!", "?!?!", "!?!?"];

const NEGATE: [&'static str; 59] =
    ["aint", "arent", "cannot", "cant", "couldnt", "darent", "didnt", "doesnt",
    "ain't", "aren't", "can't", "couldn't", "daren't", "didn't", "doesn't",
    "dont", "hadnt", "hasnt", "havent", "isnt", "mightnt", "mustnt", "neither",
    "don't", "hadn't", "hasn't", "haven't", "isn't", "mightn't", "mustn't",
    "neednt", "needn't", "never", "none", "nope", "nor", "not", "nothing", "nowhere",
    "oughtnt", "shant", "shouldnt", "uhuh", "wasnt", "werent",
    "oughtn't", "shan't", "shouldn't", "uh-uh", "wasn't", "weren't",
    "without", "wont", "wouldnt", "won't", "wouldn't", "rarely", "seldom", "despite"];

lazy_static! {
    static ref BOOSTER_DICT: HashMap<&'static str, f64> =  hashmap![
         "absolutely" => B_INCR, "amazingly" => B_INCR, "awfully" => B_INCR, "completely" => B_INCR, "considerably" => B_INCR,
         "decidedly" => B_INCR, "deeply" => B_INCR, "effing" => B_INCR, "enormously" => B_INCR,
         "entirely" => B_INCR, "especially" => B_INCR, "exceptionally" => B_INCR, "extremely" => B_INCR,
         "fabulously" => B_INCR, "flipping" => B_INCR, "flippin" => B_INCR,
         "fricking" => B_INCR, "frickin" => B_INCR, "frigging" => B_INCR, "friggin" => B_INCR, "fully" => B_INCR, "fucking" => B_INCR,
         "greatly" => B_INCR, "hella" => B_INCR, "highly" => B_INCR, "hugely" => B_INCR, "incredibly" => B_INCR,
         "intensely" => B_INCR, "majorly" => B_INCR, "more" => B_INCR, "most" => B_INCR, "particularly" => B_INCR,
         "purely" => B_INCR, "quite" => B_INCR, "really" => B_INCR, "remarkably" => B_INCR,
         "so" => B_INCR, "substantially" => B_INCR,
         "thoroughly" => B_INCR, "totally" => B_INCR, "tremendously" => B_INCR,
         "uber" => B_INCR, "unbelievably" => B_INCR, "unusually" => B_INCR, "utterly" => B_INCR,
         "very" => B_INCR,
         "almost" => B_DECR, "barely" => B_DECR, "hardly" => B_DECR, "just enough" => B_DECR,
         "kind of" => B_DECR, "kinda" => B_DECR, "kindof" => B_DECR, "kind-of" => B_DECR,
         "less" => B_DECR, "little" => B_DECR, "marginally" => B_DECR, "occasionally" => B_DECR, "partly" => B_DECR,
         "scarcely" => B_DECR, "slightly" => B_DECR, "somewhat" => B_DECR,
         "sort of" => B_DECR, "sorta" => B_DECR, "sortof" => B_DECR, "sort-of" => B_DECR];

    //check for sentiment laden idioms that do not contain lexicon words (future work, not yet implemented)
    static ref SENTIMENT_LADEN_IDIOMS: HashMap<&'static str, f64> = hashmap![
         "cut the mustard" => 2.0, "hand to mouth" => -2.0,
         "back handed" => -2.0, "blow smoke" => -2.0, "blowing smoke" => -2.0,
         "upper hand" => 1.0, "break a leg" => 2.0,
         "cooking with gas" => 2.0, "in the black" => 2.0, "in the red" => -2.0,
         "on the ball" => 2.0, "under the weather" => -2.0];

    //check for special case idioms containing lexicon words
    static ref SPECIAL_CASE_IDIOMS: HashMap<&'static str, f64> = hashmap![
         "the shit" => 3.0, "the bomb" => 3.0, "bad ass" => 1.5, "yeah right" => -2.0,
         "kiss of death" => -1.5];

    static ref UPPERCASE_RE: Regex = Regex::new(r"^[A-Z\W]+$").unwrap();

    static ref PUNCTUATION_STRING: &'static str = "[!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~]";
}

fn vec_to_lowercase(input_vec: &Vec<&str>) -> Vec<String> {
    input_vec.iter().map(|&s| s.to_lowercase()).collect()
}

// TODO: least and include_nt features
pub fn negated(input_words: &Vec<&str>) -> bool {
    let input_words = vec_to_lowercase(input_words);
    for neg_word in NEGATE.iter() {
        if input_words.iter().any(|s| s == neg_word) {
            println!("negated(): Found a match: {}", neg_word);
            return true;
        }
    }
    println!("negated(): Found no matches.");
    false
}

pub fn normalize(score: f64) -> f64 {
    let norm_score = score / (score * score + NORMALIZATION_ALPHA).sqrt();
    println!("Norm Score: {}", norm_score);
    if norm_score < -1.0 {
        return -1.0;
    } else if norm_score > 1.0 {
        return 1.0;
    }
    norm_score
}

pub fn is_uppercase(word: &str) -> bool {
    UPPERCASE_RE.is_match(word)
}

pub fn allcaps_differential(input_words: &Vec<&str>) -> bool {
    let (mut has_upper, mut has_lower) = (false, false);
    for word in input_words.iter() {
        if is_uppercase(word) {
            has_upper = true;
        } else {
            has_lower = true;
        }
        if has_lower && has_upper {
            return true;
        }
    }
    false
}

pub fn scalar_inc_dec(word: &str, valence: f64, is_cap_diff: bool) -> f64 {
    let mut scalar = 0.0;
    let word_lower: &str = &word.to_lowercase();
    if BOOSTER_DICT.contains_key(word_lower) {
        scalar = *BOOSTER_DICT.get(word_lower).unwrap();
        if valence < 0.0 {
            scalar *= -1.0;
        }
        if is_uppercase(word) && is_cap_diff {
            if valence > 0.0 {
                scalar += C_INCR;
            } else {
                scalar -= C_INCR;
            }
        }
    }
    scalar
}

struct SentiText<'a> {
    text: &'a str,
    words_and_emoticons: Vec<&'a str>,
    is_cap_diff: bool,
}

impl<'a> SentiText<'a> {
    fn from_text(_text: &'a str) -> SentiText {
        let _words_and_emot: Vec<&str> = _text.split_whitespace().collect();
        let _is_cap_diff = allcaps_differential(&_words_and_emot);
        SentiText {
            text: _text,
            words_and_emoticons: _words_and_emot,
            is_cap_diff: _is_cap_diff,
         }
    }

    fn words_and_emoticons(text: &'a str) -> Vec<&str> {
        let words_and_emot: Vec<&str> = text.split_whitespace()
                                            .filter(|s| s.len() > 1)
                                            .map(|s| s.trim_matches(|c| PUNCTUATION_STRING.contains(c)))
                                            .collect();
        words_and_emot
    }
}
