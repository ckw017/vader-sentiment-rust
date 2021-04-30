/**
 * If you use the VADER sentiment analysis tools, please cite:
 * Hutto, C.J. & Gilbert, E.E. (2014). VADER: A Parsimonious Rule-based Model for
 * Sentiment Analysis of Social Media Text. Eighth International Conference on
 * Weblogs and Social Media (ICWSM-14). Ann Arbor, MI, June 2014.
 **/


#[macro_use] extern crate maplit;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate unicase;

use std::cmp::min;
use std::collections::{HashMap, HashSet};
use regex::Regex;
use unicase::UniCase;

#[cfg(test)]
mod tests;

//empirically derived constants for scaling/amplifying sentiments
const B_INCR: f64 =  0.293;
const B_DECR: f64 = -0.293;

const C_INCR:   f64 =  0.733;
const NEGATION_SCALAR: f64 = -0.740;

//sentiment increases for text with question or exclamation marks
const QMARK_INCR: f64 = 0.180;
const EMARK_INCR: f64 = 0.292;

//Maximum amount of question or question marks before their contribution to sentiment is
//disregarded
const MAX_EMARK: i32 = 4;
const MAX_QMARK: i32 = 3;
const MAX_QMARK_INCR: f64 = 0.96;

const NORMALIZATION_ALPHA: f64 = 15.0;

static RAW_LEXICON: &'static str = include_str!("resources/vader_lexicon.txt");
static RAW_EMOJI_LEXICON: &'static str = include_str!("resources/emoji_utf8_lexicon.txt");

lazy_static! {

    static ref NEGATION_TOKENS: HashSet<UniCase<&'static str>> = convert_args!(hashset!(
        "aint", "arent", "cannot", "cant", "couldnt", "darent", "didnt", "doesnt",
        "ain't", "aren't", "can't", "couldn't", "daren't", "didn't", "doesn't",
        "dont", "hadnt", "hasnt", "havent", "isnt", "mightnt", "mustnt", "neither",
        "don't", "hadn't", "hasn't", "haven't", "isn't", "mightn't", "mustn't",
        "neednt", "needn't", "never", "none", "nope", "nor", "not", "nothing", "nowhere",
        "oughtnt", "shant", "shouldnt", "uhuh", "wasnt", "werent",
        "oughtn't", "shan't", "shouldn't", "uh-uh", "wasn't", "weren't",
        "without", "wont", "wouldnt", "won't", "wouldn't", "rarely", "seldom", "despite"));

    static ref BOOSTER_DICT: HashMap<UniCase<&'static str>, f64> =  convert_args!(hashmap!(
         "absolutely"=> B_INCR, "amazingly"=> B_INCR, "awfully"=> B_INCR,
          "completely"=> B_INCR, "considerable"=> B_INCR, "considerably"=> B_INCR,
          "decidedly"=> B_INCR, "deeply"=> B_INCR, "effing"=> B_INCR, "enormous"=> B_INCR, "enormously"=> B_INCR,
          "entirely"=> B_INCR, "especially"=> B_INCR, "exceptional"=> B_INCR, "exceptionally"=> B_INCR,
          "extreme"=> B_INCR, "extremely"=> B_INCR,
          "fabulously"=> B_INCR, "flipping"=> B_INCR, "flippin"=> B_INCR, "frackin"=> B_INCR, "fracking"=> B_INCR,
          "fricking"=> B_INCR, "frickin"=> B_INCR, "frigging"=> B_INCR, "friggin"=> B_INCR, "fully"=> B_INCR,
          "fuckin"=> B_INCR, "fucking"=> B_INCR, "fuggin"=> B_INCR, "fugging"=> B_INCR,
          "greatly"=> B_INCR, "hella"=> B_INCR, "highly"=> B_INCR, "hugely"=> B_INCR,
          "incredible"=> B_INCR, "incredibly"=> B_INCR, "intensely"=> B_INCR,
          "major"=> B_INCR, "majorly"=> B_INCR, "more"=> B_INCR, "most"=> B_INCR, "particularly"=> B_INCR,
          "purely"=> B_INCR, "quite"=> B_INCR, "really"=> B_INCR, "remarkably"=> B_INCR,
          "so"=> B_INCR, "substantially"=> B_INCR,
          "thoroughly"=> B_INCR, "total"=> B_INCR, "totally"=> B_INCR, "tremendous"=> B_INCR, "tremendously"=> B_INCR,
          "uber"=> B_INCR, "unbelievably"=> B_INCR, "unusually"=> B_INCR, "utter"=> B_INCR, "utterly"=> B_INCR,
          "very"=> B_INCR,
          "almost"=> B_DECR, "barely"=> B_DECR, "hardly"=> B_DECR, "just enough"=> B_DECR,
          "kind of"=> B_DECR, "kinda"=> B_DECR, "kindof"=> B_DECR, "kind-of"=> B_DECR,
          "less"=> B_DECR, "little"=> B_DECR, "marginal"=> B_DECR, "marginally"=> B_DECR,
          "occasional"=> B_DECR, "occasionally"=> B_DECR, "partly"=> B_DECR,
          "scarce"=> B_DECR, "scarcely"=> B_DECR, "slight"=> B_DECR, "slightly"=> B_DECR, "somewhat"=> B_DECR,
          "sort of"=> B_DECR, "sorta"=> B_DECR, "sortof"=> B_DECR, "sort-of"=> B_DECR
));

    /**
     * These dicts were used in some WIP or planned features in the original
     * I may implement them later if I can understand how they're intended to work
     **/

    // // check for sentiment laden idioms that do not contain lexicon words (future work, not yet implemented)
    // static ref SENTIMENT_LADEN_IDIOMS: HashMap<&'static str, f64> = hashmap![
    //      "cut the mustard" => 2.0, "hand to mouth" => -2.0,
    //      "back handed" => -2.0, "blow smoke" => -2.0, "blowing smoke" => -2.0,
    //      "upper hand" => 1.0, "break a leg" => 2.0,
    //      "cooking with gas" => 2.0, "in the black" => 2.0, "in the red" => -2.0,
    //      "on the ball" => 2.0, "under the weather" => -2.0];


    // check for special case idioms containing lexicon words
    static ref SPECIAL_CASE_IDIOMS: HashMap<UniCase<&'static str>, f64> = convert_args!(hashmap!(
         "the shit" => 3.0, "the bomb" => 3.0, "bad ass" => 1.5, "badass" => 1.5, "yeah right" => -2.0,
         "kiss of death" => -1.5, "to die for" => 3.0));

    static ref ALL_CAPS_RE: Regex = Regex::new(r"^[A-Z\W]+$").unwrap();

    static ref PUNCTUATION: &'static str = "[!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~]";

    pub static ref LEXICON: HashMap<UniCase<&'static str>, f64> = parse_raw_lexicon(RAW_LEXICON);
    pub static ref EMOJI_LEXICON: HashMap<&'static str, &'static str> = parse_raw_emoji_lexicon(RAW_EMOJI_LEXICON);

    static ref STATIC_BUT: UniCase<&'static str> = UniCase::new("but");
    static ref STATIC_THIS: UniCase<&'static str> = UniCase::new("this");
    static ref STATIC_AT: UniCase<&'static str> = UniCase::new("at");
    static ref STATIC_LEAST: UniCase<&'static str> = UniCase::new("least");
    static ref STATIC_VERY: UniCase<&'static str> = UniCase::new("very");
    static ref STATIC_WITHOUT: UniCase<&'static str> = UniCase::new("without");
    static ref STATIC_DOUBT: UniCase<&'static str> = UniCase::new("doubt");
    static ref STATIC_SO: UniCase<&'static str> = UniCase::new("so");
    static ref STATIC_NEVER: UniCase<&'static str> = UniCase::new("never");
    static ref STATIC_KIND: UniCase<&'static str> = UniCase::new("kind");
    static ref STATIC_OF: UniCase<&'static str> = UniCase::new("of");


}


/**
 * Takes the raw text of the lexicon files and creates HashMaps
 **/
pub fn parse_raw_lexicon(raw_lexicon: &str) -> HashMap<UniCase<&str>, f64> {
    let lines = raw_lexicon.trim_end_matches("\n").split("\n");
    let mut lex_dict = HashMap::new();
    for line in lines {
        if line.is_empty() {
          continue;
        }
        let mut split_line = line.split('\t');
        let word = split_line.next().unwrap();
        let val  = split_line.next().unwrap();
        lex_dict.insert(UniCase::new(word), val.parse().unwrap());
    }
    lex_dict
}

pub fn parse_raw_emoji_lexicon(raw_emoji_lexicon: &str) -> HashMap<&str, &str> {
    let lines = raw_emoji_lexicon.trim_end_matches("\n").split("\n");
    let mut emoji_dict = HashMap::new();
    for line in lines {
        if line.is_empty() {
          continue;
        }
        let mut split_line = line.split('\t');
        let word = split_line.next().unwrap();
        let desc  = split_line.next().unwrap();
        emoji_dict.insert(word, desc);
    }
    emoji_dict
}

/**
 *  Stores tokens and useful info about text
 **/
struct ParsedText<'a> {
    tokens: Vec<UniCase<&'a str>>,
    has_mixed_caps: bool,
    punc_amplifier: f64,
}

impl<'a> ParsedText<'a> {
    //Tokenizes and extracts useful properties of input text
    fn from_text(text: &'a str) -> ParsedText {
        let _tokens = ParsedText::tokenize(text);
        let _has_mixed_caps = ParsedText::has_mixed_caps(&_tokens);
        let _punc_amplifier = ParsedText::get_punctuation_emphasis(text);
        ParsedText {
            tokens: _tokens,
            has_mixed_caps: _has_mixed_caps,
            punc_amplifier: _punc_amplifier,
         }
    }

    fn tokenize(text: &str) -> Vec<UniCase<&str>> {
        let tokens = text.split_whitespace()
                                    .filter(|s| s.len() > 1)
                                    .map(|s| ParsedText::strip_punc_if_word(s))
                                    .map(UniCase::new)
                                    .collect();
        tokens
    }

    // Removes punctuation from words, ie "hello!!!" -> "hello" and ",don't??" -> "don't"
    // Keeps most emoticons, ie ":^)" -> ":^)"\
    fn strip_punc_if_word(token: &str) -> &str {
        let stripped = token.trim_matches(|c| PUNCTUATION.contains(c));
        if stripped.len() <= 1 {
            return token;
        }
        stripped
    }

    // Determines if message has a mix of both all caps and non all caps words
    fn has_mixed_caps<S: AsRef<str>>(tokens: &[S]) -> bool {
        let (mut has_caps, mut has_non_caps) = (false, false);
        for token in tokens.iter() {
            if is_all_caps(token.as_ref()) {
                has_caps = true;
            } else {
                has_non_caps = true;
            }
            if has_non_caps && has_caps {
                return true;
            }
        }
        false
    }

    //uses empirical values to determine how the use of '?' and '!' contribute to sentiment
    fn get_punctuation_emphasis(text: &str) -> f64 {
       let emark_count: i32 = text.as_bytes().iter().filter(|b| **b == b'!').count() as i32;
       let qmark_count: i32 = text.as_bytes().iter().filter(|b| **b == b'?').count() as i32;

       let emark_emph = min(emark_count, MAX_EMARK) as f64 * EMARK_INCR;
       let mut qmark_emph = (qmark_count as f64) * QMARK_INCR;
       if qmark_count > MAX_QMARK {
           qmark_emph = MAX_QMARK_INCR;
       }
       qmark_emph + emark_emph
    }
}

//Checks if all letters in token are capitalized
fn is_all_caps<S: AsRef<str>>(token: S) -> bool {
    let token_ref = token.as_ref();
    ALL_CAPS_RE.is_match(token_ref) && token_ref.len() > 1
}

//Checks if token is in the list of NEGATION_SCALAR
fn is_negated(token: &UniCase<&str>) -> bool {
    if NEGATION_TOKENS.contains(token) {
        return true;
    }
    token.contains("n't")
}

//Normalizes score between -1.0 and 1.0. Alpha value is expected upper limit for a score
fn normalize_score(score: f64) -> f64 {
    let norm_score = score / (score * score + NORMALIZATION_ALPHA).sqrt();
    if norm_score < -1.0 {
        return -1.0;
    } else if norm_score > 1.0 {
        return 1.0;
    }
    norm_score
}

//Checks how previous tokens affect the valence of the current token
fn scalar_inc_dec(token: &UniCase<&str>, valence: f64, has_mixed_caps: bool) -> f64 {
    let mut scalar = 0.0;
    if BOOSTER_DICT.contains_key(token) {
        scalar = *BOOSTER_DICT.get(token).unwrap();
        if valence < 0.0 {
            scalar *= -1.0;
        }
        if is_all_caps(token) && has_mixed_caps {
            if valence > 0.0 {
                scalar += C_INCR;
            } else {
                scalar -= C_INCR;
            }
        }
    }
    scalar
}

fn sum_sentiment_scores(scores: Vec<f64>) -> (f64, f64, u32) {
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

pub struct SentimentIntensityAnalyzer<'a> {
    lexicon: &'a HashMap<UniCase<&'a str>, f64>,
    emoji_lexicon: &'a HashMap<&'a str, &'a str>,
}

impl<'a> SentimentIntensityAnalyzer<'a> {
    pub fn new() -> SentimentIntensityAnalyzer<'static>{
        SentimentIntensityAnalyzer {
            lexicon: &LEXICON,
            emoji_lexicon: &EMOJI_LEXICON,
        }
    }

    pub fn from_lexicon<'b>(_lexicon: &'b HashMap<UniCase<&str>, f64>) ->
                                        SentimentIntensityAnalyzer<'b> {
        SentimentIntensityAnalyzer {
            lexicon: _lexicon,
            emoji_lexicon: &EMOJI_LEXICON,
        }
    }

    fn get_total_sentiment(&self, sentiments: Vec<f64>, punct_emph_amplifier: f64) -> HashMap<&str, f64> {
        let (mut neg, mut neu, mut pos, mut compound) = (0f64, 0f64, 0f64, 0f64);
        if sentiments.len() > 0 {
            let mut total_sentiment: f64 = sentiments.iter().sum();
            if total_sentiment > 0f64 {
                total_sentiment += punct_emph_amplifier;
            } else {
                total_sentiment -= punct_emph_amplifier;
            }
            compound = normalize_score(total_sentiment);

            let (mut pos_sum, mut neg_sum, neu_count) = sum_sentiment_scores(sentiments);

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
        let parsedtext = ParsedText::from_text(&text);
        let mut sentiments = Vec::new();
        let tokens = &parsedtext.tokens;

        for (i, word) in tokens.iter().enumerate() {
            if BOOSTER_DICT.contains_key(word) {
                sentiments.push(0f64);
            } else if i < tokens.len() - 1 && word == &*STATIC_KIND
                                  && tokens[i + 1] == *STATIC_OF {
                sentiments.push(0f64);
            } else {
                sentiments.push(self.sentiment_valence(&parsedtext, word, i));
            }
        }
        but_check(tokens, &mut sentiments);
        self.get_total_sentiment(sentiments, parsedtext.punc_amplifier)
    }

    //Removes emoji and appends their description to the end the input text
    fn append_emoji_descriptions(&self, text: &str) -> String {
        let mut result = String::new();
        let mut prev_space = true;
        for chr in text.chars() {
            let chr_string = chr.to_string();
            if let Some(chr_replacement) = self.emoji_lexicon.get(chr_string.as_str()) {
                if !prev_space {
                    result.push(' ');
                }
                result.push_str(chr_replacement);
                prev_space = false;
            } else {
                prev_space = chr == ' ';
                result.push(chr);
            }
        }
        result
    }

    fn sentiment_valence(&self, parsed: &ParsedText, word: &UniCase<&str>, i: usize) -> f64 {
        let mut valence = 0f64;
        let tokens = &parsed.tokens;
        if let Some(word_valence) = self.lexicon.get(word) {
            valence = *word_valence;
            if is_all_caps(word) && parsed.has_mixed_caps {
                if valence > 0f64 {
                    valence += C_INCR;
                } else {
                    valence -= C_INCR
                }
            }
            for start_i in 0..3 {
                if i > start_i && !self.lexicon.contains_key(
                                &tokens[i - start_i - 1]) {
                    let mut s = scalar_inc_dec(&tokens[i - start_i - 1], valence, parsed.has_mixed_caps);
                    if start_i == 1 {
                        s *= 0.95;
                    } else if start_i == 2 {
                        s *= 0.9
                    }
                    valence += s;
                    valence = negation_check(valence, tokens, start_i, i);
                    if start_i == 2 {
                        valence = special_idioms_check(valence, tokens, i);
                    }
                }
            }
            valence = least_check(valence, tokens, i);
        }
        valence
    }
}

/**
 * Check for specific patterns or tokens, and modify sentiment as needed
 **/
fn negation_check(valence: f64, tokens: &[UniCase<&str>], start_i: usize, i: usize) -> f64 {
   let mut valence = valence;
   if start_i == 0 {
       if is_negated(&tokens[i - start_i - 1]) {
           valence *= NEGATION_SCALAR;
       }
   } else if start_i == 1 {
       if tokens[i - 2] == *STATIC_NEVER &&
         (tokens[i - 1] == *STATIC_SO ||
          tokens[i - 1] == *STATIC_THIS) {
           valence *= 1.25
       } else if tokens[i - 2] == *STATIC_WITHOUT && tokens[i - 1] == *STATIC_DOUBT {
           valence *= 1.0
       } else if is_negated(&tokens[i - start_i - 1]) {
           valence *= NEGATION_SCALAR;
       }
   } else if start_i == 2 {
       if tokens[i - 3] == *STATIC_NEVER &&
          tokens[i - 2] == *STATIC_SO || tokens[i - 2] == *STATIC_THIS||
          tokens[i - 1] == *STATIC_SO || tokens[i - 1] == *STATIC_THIS {
           valence *= 1.25
       } else if tokens[i - 3] == *STATIC_WITHOUT &&
                 tokens[i - 2] == *STATIC_DOUBT ||
                 tokens[i - 1] == *STATIC_DOUBT {
           valence *= 1.0;
       } else if is_negated(&tokens[i - start_i - 1]) {
           valence *= NEGATION_SCALAR;
       }
   }
   valence
}

// If "but" is in the tokens, scales down the sentiment of words before "but" and
// adds more emphasis to the words after
fn but_check(tokens: &[UniCase<&str>], sentiments: &mut Vec<f64>) {
    match tokens.iter().position(|&s| s == *STATIC_BUT) {
        Some(but_index) => {
            for i in 0..sentiments.len() {
                if i < but_index {
                    sentiments[i] *= 0.5;
                } else if i > but_index {
                    sentiments[i] *= 1.5;
                }
            }
        },
        None => return,
    }
}

fn least_check(_valence: f64, tokens: &[UniCase<&str>], i: usize) -> f64 {
    let mut valence = _valence;
    if i > 1 && tokens[i - 1] == *STATIC_LEAST
             && tokens[i - 2] == *STATIC_AT
             && tokens[i - 2] == *STATIC_VERY {
        valence *= NEGATION_SCALAR;
    } else if i > 0 && tokens[i - 1] == *STATIC_LEAST {
        valence *= NEGATION_SCALAR;
    }
    valence
}

// //This was present in the original python implementation, but unused
// fn idioms_check(valence: f64, text: &str) -> f64 {
//     let mut total_valence = 0f64;
//     let mut count = 0;
//     for (idiom, val) in SENTIMENT_LADEN_IDIOMS.iter() {
//         if text.contains(idiom) {
//             total_valence += val;
//             count += 1;
//         }
//     }
//     if count > 0 {
//         return total_valence / count as f64;
//     }
//     0f64
// }

fn special_idioms_check(_valence: f64, tokens: &[UniCase<&str>], i: usize) -> f64 {
    assert_eq!(i > 2, true);
    let mut valence = _valence;
    let mut end_i = i + 1;

    //if i isn't the last index
    if tokens.len() - 1 > i {
        //make the end of the window 2 words ahead, or until the end of the tokens
        end_i = min(i + 3, tokens.len());
    }

    // TODO: We can do this faster by comparing splits?
    let target_window = tokens[(i - 3)..end_i].iter().map(|u| u.as_ref()).collect::<Vec<&str>>().join(" ").to_lowercase();

    for (key, val) in SPECIAL_CASE_IDIOMS.iter() {
        if target_window.contains(key.as_ref()) {
            valence = *val;
            break;
        }
    }
    let prev_three = tokens[(i - 3)..i].iter().map(|u| u.as_ref()).collect::<Vec<&str>>().join(" ").to_lowercase();
    for (key, val) in BOOSTER_DICT.iter() {
        if prev_three.contains(key.as_ref()) {
            valence += *val;
        }
    }
    valence
}

pub mod demo;
