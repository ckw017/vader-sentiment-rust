#[test]
fn test_lexicon() {
    assert_eq!(*::LEXICON.get("feudally").unwrap(), -0.6);
    assert_eq!(*::LEXICON.get("irrationalism").unwrap(), -1.5);
    assert_eq!(*::LEXICON.get("sentimentalize").unwrap(), 0.8);
    assert_eq!(*::LEXICON.get("wisewomen").unwrap(), 1.3);
}

#[test]
fn test_emoji_lexicon() {
    assert_eq!(*::EMOJI_LEXICON.get("👽").unwrap(), "alien");
    assert_eq!(*::EMOJI_LEXICON.get("👨🏿‍🎓").unwrap(), "man student: dark skin tone");
    assert_eq!(*::EMOJI_LEXICON.get("🖖🏻").unwrap(), "vulcan salute: light skin tone");
}

#[test]
fn test_parsed_text() {
    let messy_text = "WOAH!!! ,Who? DO u Think you're?? :) :D :^(";
    let parsed_messy = ::ParsedText::from_text(messy_text);
    assert_eq!(parsed_messy.tokens, vec!["WOAH", "Who", "DO", "Think", "you\'re", ":)", ":D", ":^("]);
    assert_eq!(parsed_messy.has_mixed_caps, true);
    assert_eq!(parsed_messy.punc_amplifier, 1.416);

    assert_eq!(::ParsedText::has_mixed_caps(&::ParsedText::tokenize("yeah!!! I'm aLLERGIC to ShouTING.")), false);
    assert_eq!(::ParsedText::has_mixed_caps(&::ParsedText::tokenize("OH MAN I LOVE SHOUTING!")), false);
    assert_eq!(::ParsedText::has_mixed_caps(&::ParsedText::tokenize("I guess I CAN'T MAKE UP MY MIND")), true);
    assert_eq!(::ParsedText::has_mixed_caps(&::ParsedText::tokenize("Hmm, yeah ME NEITHER")), true);
}

#[test]
fn but_check_test() {
    let tokens     = vec!["yeah", "waffles", "are", "great", "but", "have", "you", "ever", "tried", "spam"];
    let mut sents  = vec![ 0.5,    0.1,       0.0,   0.2,     0.6,   0.25,    0.5,   0.5,    0.5,     0.5];
    ::but_check(&tokens, &mut sents);
    assert_eq!(sents, vec![0.25,   0.05,      0.0,   0.1,     0.6,   0.375,  0.75,   0.75,  0.75,   0.75]);
}

#[test]
fn demo_test() {
    ::demo::run_demo();
}

#[test]
fn embedded_emoji_test() {
    let analyzer = ::SentimentIntensityAnalyzer::new();
    let single_emoji = "😀";
    let embedded_emoji = "heyyyy 😀 what're you up to???";
    let multiple_emoji = "woah there 😀😀😀 :) :)";
    assert_eq!(analyzer.append_emoji_descriptions(single_emoji), "grinning face");
    assert_eq!(analyzer.append_emoji_descriptions(embedded_emoji), "heyyyy grinning face what're you up to???");
    assert_eq!(analyzer.append_emoji_descriptions(multiple_emoji), "woah there grinning face grinning face grinning face :) :)");
}
