# VADER-Sentiment-Analysis



VADER (Valence Aware Dictionary and sEntiment Reasoner) is a lexicon and rule-based sentiment analysis tool that is *specifically attuned to sentiments expressed in social media*. It is fully open-sourced under the [MIT License](http://choosealicense.com/). **This is a port of the original module**, which was written in Python. If you'd like to make a contribution, please checkout  [the original author's work here](https://github.com/cjhutto/vaderSentiment).

# Use Cases
	* examples of typical use cases for sentiment analysis, including proper handling of sentences with:

		- typical negations (e.g., "not good")
		- use of contractions as negations (e.g., "wasn't very good")
		- conventional use of punctuation to signal increased sentiment intensity (e.g., "Good!!!")
		- conventional use of word-shape to signal emphasis (e.g., using ALL CAPS for words/phrases)
		- using degree modifiers to alter sentiment intensity (e.g., intensity boosters such as "very" and intensity dampeners such as "kind of")
		- understanding many sentiment-laden slang words (e.g., 'sux')
		- understanding many sentiment-laden slang words as modifiers such as 'uber' or 'friggin' or 'kinda'
		- understanding many sentiment-laden emoticons such as :) and :D
		- translating utf-8 encoded emojis such as 💘 and 💋 and 😁
		- understanding sentiment-laden initialisms and acronyms (for example: 'lol')

	* more examples of tricky sentences that confuse other sentiment analysis tools
	* example for how VADER can work in conjunction with NLTK to do sentiment analysis on longer texts...i.e., decomposing paragraphs, articles/reports/publications, or novels into sentence-level analyses
	* examples of a concept for assessing the sentiment of images, video, or other tagged multimedia content
	* if you have access to the Internet, the demo has an example of how VADER can work with analyzing sentiment of texts in other languages (non-English text sentences).

# Usage

### Code
```rust
  extern crate vader_sentiment;

  fn main() {
      let analyzer = vader_sentiment::SentimentIntensityAnalyzer::new();
      println!("{:#?}", analyzer.polarity_scores("VADER is smart, handsome, and funny."));
      println!("{:#?}", analyzer.polarity_scores("VADER is VERY SMART, handsome, and FUNNY."));
  }
```

### Output
``` rust
{
    "compound": 0.8316320352807864,
    "pos": 0.7457627118644068,
    "neg": 0.0,
    "neu": 0.2542372881355932
}
{
    "compound": 0.9226571915792521,
    "pos": 0.7540988645515071,
    "neg": 0.0,
    "neu": 0.24590113544849293
}
```

# Citation Information

If you use either the dataset or any of the VADER sentiment analysis tools (VADER sentiment lexicon or Python code for rule-based sentiment analysis engine) in your research, please cite the above paper. For example:  

  **Hutto, C.J. & Gilbert, E.E. (2014). VADER: A Parsimonious Rule-based Model for Sentiment Analysis of Social Media Text. Eighth International Conference on Weblogs and Social Media (ICWSM-14). Ann Arbor, MI, June 2014.**

For questions, please contact:
C.J. Hutto
Georgia Institute of Technology, Atlanta, GA 30032  
cjhutto [at] gatech [dot] edu

# Demo
You can run a full demo including cases with sarcasm, negation, idioms, and punctuation with this code.

```rust
extern crate vader_sentiment;

fn main() {
    vader_sentiment::demo::run_demo();
}
```
