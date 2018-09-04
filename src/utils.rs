use std::cmp::min;
use std::collections::HashMap;
use regex::Regex;

const B_INCR: f64 =  0.293;
const B_DECR: f64 = -0.293;

const C_INCR:   f64 =  0.733;
const N_SCALAR: f64 = -0.740;

//Empirically derived sentiment increases for text with question or exclamation marks
const QMARK_INCR: f64 = 0.180;
const EMARK_INCR: f64 = 0.292;

//Maximum amount of question or question marks before their contribution to sentiment is
//disregarded
const MAX_EMARK: i32 = 4;
const MAX_QMARK: i32 = 3;

const MAX_QMARK_INCR: f64 = 0.96;

const NORMALIZATION_ALPHA: f64 = 15.0;

const PUNC_LIST: [&'static str; 17] =
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

const RAW_LEXICON: &'static str = include_str!("resources/vader_lexicon.txt");
const RAW_EMOJI_LEXICON: &'static str = include_str!("resources/emoji_utf8_lexicon.txt");

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

    static ref PUNCTUATION: &'static str = "[!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~]";

    pub static ref LEXICON: HashMap<&'static str, f64> = parse_raw_lex(RAW_LEXICON);
    pub static ref EMOJI_LEXICON: HashMap<&'static str, &'static str> = parse_raw_emoji_lex(RAW_EMOJI_LEXICON);
}

fn vec_to_lowercase<'a>(input_vec: &Vec<&str>) -> Vec<String> {
    input_vec.iter().map(|s| s.to_lowercase()).collect()
}

// TODO: least and include_nt features
pub fn negated(input_words: &Vec<&str>) -> bool {
    let input_words = vec_to_lowercase(input_words);
    for neg_word in NEGATE.iter() {
        if input_words.iter().any(|s| s == neg_word) {
            return true;
        }
    }
    false
}

fn is_negated(input_word: &str) -> bool {
    let input_word = input_word.to_lowercase();
    for neg_word in NEGATE.iter() {
        if input_word == *neg_word {
            return true;
        }
    }
    false
}

pub fn normalize(score: f64) -> f64 {
    let norm_score = score / (score * score + NORMALIZATION_ALPHA).sqrt();
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

#[derive(Debug)]
pub struct SentiText<'a> {
    pub text: &'a str,
    pub words_and_emoticons: Vec<&'a str>,
    pub is_cap_diff: bool,
}

impl<'a> SentiText<'a> {
    pub fn from_text(_text: &'a str) -> SentiText {
        let _words_and_emot = SentiText::words_and_emoticons(_text);
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
                                            .map(|s| SentiText::strip_punc_if_word(s))
                                            .collect();
        words_and_emot
    }

    /**
    Removes punctuation from words, ie "hello!!!" -> "hello" and ",don't??" -> "don't"
    Keeps most emoticons, ie ":^)" -> ":^)"
    **/
    fn strip_punc_if_word(word_or_emot: &str) -> &str {
        let stripped = word_or_emot.trim_matches(|c| PUNCTUATION.contains(c));
        if stripped.len() <= 1 {
            return word_or_emot;
        }
        stripped
    }
}

pub fn parse_raw_lex(raw_lexicon: &str) -> HashMap<&str, f64> {
    let lines = raw_lexicon.split("\n");
    let mut lex_dict = HashMap::new();
    for line in lines {
        let mut split_line = line.split('\t');
        let word = split_line.next().unwrap();
        let val  = split_line.next().unwrap();
        lex_dict.insert(word, val.parse().unwrap());
    }
    lex_dict
}

pub fn parse_raw_emoji_lex(raw_emoji_lexicon: &str) -> HashMap<&str, &str> {
    let lines = raw_emoji_lexicon.split("\n");
    let mut emoji_dict = HashMap::new();
    for line in lines {
        let mut split_line = line.split('\t');
        let word = split_line.next().unwrap();
        let desc  = split_line.next().unwrap();
        emoji_dict.insert(word, desc);
    }
    emoji_dict
}

fn sift_sentiment_scores(scores: Vec<f64>) -> (f64, f64, i32) {
    let (mut pos_sum, mut neg_sum, mut neu_count) = (0f64, 0f64, 0);
    for score in scores {
        if score > 0f64 {
            pos_sum += score + 1.0;
        } else if score < 0f64 {
            neg_sum += score - 1.0;
        } else {
            neu_count += 1;
        }
    }
    (pos_sum, neg_sum, neu_count)
}

pub fn punctuation_emphasis(text: &str) -> f64 {
   let emark_count: i32 = text.as_bytes().iter().filter(|b| **b == b'!').count() as i32;
   let qmark_count: i32 = text.as_bytes().iter().filter(|b| **b == b'?').count() as i32;

   let emark_emph = min(emark_count, MAX_EMARK) as f64 * EMARK_INCR;
   let mut qmark_emph = (qmark_count as f64) * QMARK_INCR;
   if qmark_count > MAX_QMARK {
       qmark_emph = MAX_QMARK_INCR;
   }
   qmark_emph + emark_emph
}

fn negation_check(valence: f64, words: &Vec<&str>, start_i: usize, i: usize) -> f64 {
   let mut valence = valence;
   let words = vec_to_lowercase(words);
   if start_i == 0 {
       if is_negated(&words[i - start_i - 1]) {
           valence *= N_SCALAR;
       }
   } else if start_i == 1 {
       if words[i - 2] == "never" &&
         (words[i - 1] == "so" ||
          words[i - 1] == "this") {
           valence *= 1.25
       } else if words[i - 2] == "without" && words[i - 1] == "doubt" {
           valence *= 1.0
       } else if is_negated(&words[i - start_i - 1]) {
           valence *= N_SCALAR;
       }
   } else if start_i == 2 {
       if words[i - 3] == "never" &&
          words[i - 2] == "so" || words[i - 2] == "this" ||
          words[i - 1] == "so" || words[i - 1] == "this" {
           valence *= 1.25
       } else if words[i - 3] == "without" &&
                 words[i - 2] == "doubt" ||
                 words[i - 1] == "doubt" {
           valence *= 1.0;
       } else if is_negated(&words[i - start_i - 1]) {
           valence *= N_SCALAR;
       }
   }
   valence
}

pub struct SentimentIntensityAnalyzer<'a> {
    lexicon: &'a HashMap<&'a str, f64>,
    emoji_lexicon: &'a HashMap<&'a str, &'a str>,
}

impl<'a> SentimentIntensityAnalyzer<'static> {
    pub fn new() -> SentimentIntensityAnalyzer<'static>{
        SentimentIntensityAnalyzer {
            lexicon: &LEXICON,
            emoji_lexicon: &EMOJI_LEXICON,
        }
    }

     fn score_valence(&self, sentiments: Vec<f64>, text: &str) -> HashMap<&str, f64> {
         let (mut neg, mut neu, mut pos, mut compound) = (0f64, 0f64, 0f64, 0f64);
         if sentiments.len() > 0 {
            let mut total_sentiment: f64 = sentiments.iter().sum();
            let punct_emph_amplifier = punctuation_emphasis(text);
            if total_sentiment > 0f64 {
                total_sentiment += punct_emph_amplifier;
            } else {
                total_sentiment -= punct_emph_amplifier;
            }
            compound = normalize(total_sentiment);

            let (mut pos_sum, mut neg_sum, neu_count) = sift_sentiment_scores(sentiments);

            if pos_sum > neg_sum.abs() {
                pos_sum += punct_emph_amplifier;
            } else if pos_sum < neg_sum.abs() {
                neg_sum -= punct_emph_amplifier;
            }

            let total = pos_sum + neg_sum.abs() + (neu_count as f64);
            pos = (pos_sum / total).abs();
            neg = (neg_sum / total).abs();
            neu = (neu_count as f64 / total).abs();
         }
         let sentiment_dict = hashmap!["neg" => neg,
                                       "neu" => neu,
                                       "pos" => pos,
                                       "compound" => compound];
         sentiment_dict
     }

    pub fn polarity_scores(&self, text: &str) -> HashMap<&str, f64>{
        let text = self.append_emoji_descriptions(text);
        let sentitext = SentiText::from_text(&text);
        let mut sentiments = Vec::new();
        let words = &sentitext.words_and_emoticons;

        for (i, word) in words.iter().enumerate() {
            if BOOSTER_DICT.contains_key(word.to_lowercase().as_str()) {
                sentiments.push(0f64);
            } else if i < words.len() - 1 && word.to_lowercase() == "kind"
                                  && words[i + 1].to_lowercase() == "of" {
                sentiments.push(0f64);
            }
            sentiments.push(self.sentiment_valence(&sentitext, word, i));
        }
        self.score_valence(sentiments, &text)
    }

    fn append_emoji_descriptions(&self, text: &str) -> String {
        let mut result = String::new();
        for token in text.split_whitespace() {
            if self.emoji_lexicon.contains_key(token) {
                result.push_str(self.emoji_lexicon.get(token).unwrap());
            } else {
                result.push_str(token);
            }
            result.push(' ');
        }
        result
    }

    fn sentiment_valence(&self, sentitext: &SentiText, word: &str, i: usize) -> f64 {
        let mut valence = 0f64;
        let is_cap_diff = &sentitext.is_cap_diff;
        let words = &sentitext.words_and_emoticons;
        let word_lower = word.to_lowercase();
        if self.lexicon.contains_key(word_lower.as_str()) {
            valence = *self.lexicon.get(word_lower.as_str()).unwrap();

            if is_uppercase(word) && *is_cap_diff {
                if valence > 0f64 {
                    valence += C_INCR;
                } else {
                    valence -= C_INCR;
                }
            }

            for start_i in 0..3 {
                if i <= start_i {
                    break;
                }
                let curr_word = words[i - start_i - 1].to_lowercase();
                if !self.lexicon.contains_key(curr_word.as_str()) {
                    let mut s = scalar_inc_dec(words[i - start_i - 1], valence, *is_cap_diff);
                    if start_i == 1 {
                        s *= 0.95;
                    } else if start_i == 2 {
                        s *= 0.9
                    }
                    valence += s;
                    valence = negation_check(valence, words, start_i, i);
                    if start_i == 2 {
                        //TODO: special_idioms
                    }
                }
            }
            //TODO: Least check
        }
        valence
    }
}
