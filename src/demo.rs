pub fn run_demo() {
    let sentences = vec!["VADER is smart, handsome, and funny.",  // positive sentence example
                         "VADER is smart, handsome, and funny!",
                         // punctuation emphasis handled correctly (sentiment intensity adjusted)
                         "VADER is very smart, handsome, and funny.",
                         // booster words handled correctly (sentiment intensity adjusted)
                         "VADER is VERY SMART, handsome, and FUNNY.",  // emphasis for ALLCAPS handled
                         "VADER is VERY SMART, handsome, and FUNNY!!!",
                         // combination of signals - VADER appropriately adjusts intensity
                         "VADER is VERY SMART, uber handsome, and FRIGGIN FUNNY!!!",
                         // booster words & punctuation make this close to ceiling for score
                         "VADER is not smart, handsome, nor funny.",  // negation sentence example
                         "The book was good.",  // positive sentence
                         "At least it isn't a horrible book.",  // negated negative sentence with contraction
                         "The book was only kind of good.",
                         // qualified positive sentence is handled correctly (intensity adjusted)
                         "The plot was good, but the characters are uncompelling and the dialog is not great.",
                         // mixed negation sentence
                         "Today SUX!",  // negative slang with capitalization emphasis
                         "Today only kinda sux! But I'll get by, lol",
                         // mixed sentiment example with slang and constrastive conjunction "but"
                         "Make sure you :) or :D today!",  // emoticons handled
                         "Catch utf-8 emoji such as 💘 and 💋 and 😁",  // emojis handled
                         "Not bad at all"]; // Capitalized negation
    let analyzer = ::SentimentIntensityAnalyzer::new();
    println!("----------------------------------------------------");
    println!(" - Analyze typical example cases, including handling of:");
    println!("  -- negations");
    println!("  -- punctuation emphasis & punctuation flooding");
    println!("  -- word-shape as emphasis (capitalization difference)");
    println!("  -- degree modifiers (intensifiers such as 'very' and dampeners such as 'kind of')");
    println!("  -- slang words as modifiers such as 'uber' or 'friggin' or 'kinda'");
    println!("  -- contrastive conjunction 'but' indicating a shift in sentiment; sentiment of later text is dominant");
    println!("  -- use of contractions as negations");
    println!("  -- sentiment laden emoticons such as :) and :D");
    println!("  -- utf-8 encoded emojis such as 💘 and 💋 and 😁");
    println!("  -- sentiment laden slang words (e.g., 'sux')");
    println!("  -- sentiment laden initialisms and acronyms (for example: 'lol') \n");
    for sentence in sentences{
        let scores = analyzer.polarity_scores(sentence);
        println!("{:-<65} {:#?}", sentence, scores);
    }
    println!("----------------------------------------------------");
    println!(" - About the scoring: ");
    println!("  -- The 'compound' score is computed by summing the valence scores of each word in the lexicon, adjusted
      according to the rules, and then normalized to be between -1 (most extreme negative) and +1 (most extreme positive).
      This is the most useful metric if you want a single unidimensional measure of sentiment for a given sentence.
      Calling it a 'normalized, weighted composite score' is accurate.");
    println!("  -- The 'pos', 'neu', and 'neg' scores are ratios for proportions of text that fall in each category (so these
      should all add up to be 1... or close to it with float operation).  These are the most useful metrics if
      you want multidimensional measures of sentiment for a given sentence.");
    println!("----------------------------------------------------");

    let tricky_sentences = vec!["Sentiment analysis has never been good.",
                                "Sentiment analysis has never been this good!",
                                "Most automated sentiment analysis tools are shit.",
                                "With VADER, sentiment analysis is the shit!",
                                "Other sentiment analysis tools can be quite bad.",
                                "On the other hand, VADER is quite bad ass",
                                "VADER is such a badass!",  // slang with punctuation emphasis
                                "Without a doubt, excellent idea.",
                                "Roger Dodger is one of the most compelling variations on this theme.",
                                "Roger Dodger is at least compelling as a variation on the theme.",
                                "Roger Dodger is one of the least compelling variations on this theme.",
                                "Not such a badass after all.", // Capitalized negation with slang
                                "Without a doubt, an excellent idea."]; // "without {any} doubt" as negation
    println!("----------------------------------------------------");
    println!(" - Analyze examples of tricky sentences that cause trouble to other sentiment analysis tools.");
    println!("  -- special case idioms - e.g., 'never good' vs 'never this good', or 'bad' vs 'bad ass'.");
    println!("  -- special uses of 'least' as negation versus comparison \n");
    for sentence in tricky_sentences {
        let scores = analyzer.polarity_scores(sentence);
        println!("{:-<65} {:#?}", sentence, scores);
    }
    println!("----------------------------------------------------");
}
