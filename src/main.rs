extern crate vader_sentiment;

use vader_sentiment::utils;

fn main() {
    // println!("B_INCR = {}", vader_sentiment::utils::B_INCR);
    // let words = vec!["HEyasd", "asdS(DASka)", "YupP"];
    // vader_sentiment::utils::negated(&words);
    // vader_sentiment::utils::negated(&words);
    // let words = vec!["FLAMBE", "Don't", "won't"];
    // vader_sentiment::utils::negated(&words);
    //
    // vader_sentiment::utils::normalize(123123.0);
    // vader_sentiment::utils::normalize(-1.0);
    // vader_sentiment::utils::normalize(-12.0);
    // vader_sentiment::utils::normalize(0.72);

    // let phrase: Vec<_> = "GEEZE LOUISE I'M BONKERS for cheese".split(' ').collect();
    // println!("{}", utils::allcaps_differential(&phrase));

    // println!("{}", utils::scalar_inc_dec("TOTALLY", 6.0, true));
    // let test_text = "hey babe's, hows,!! it **going**!? :))) :^) :D :D :'(";
    // println!("{:?}", utils::SentiText::from_text(test_text));

    // println!("{:#?}", *utils::EMOJI_LEXICON);
    // println!("{:#?}", *utils::LEXICON);
    let sia = utils::SentimentIntensityAnalyzer::new();
    let text = "This is not bad at all!";
    println!("{:#?}", sia.polarity_scores(&text));
}
