use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Quotes {
    quotes: Vec<&'static str>,
}

impl Quotes {
    pub fn new() -> Self {
        Self {
            quotes: vec![
                "Anyone can cook… but only the bold can serve under pressure.",
                "The recipe for success? One part courage, two parts chaos.",
                "Even a rat can rise, if the kitchen gets hot enough.",
                "Every dish is a story. Just hope today’s isn’t a tragedy.",
                "Mistakes are just spicy surprises in the recipe of life.",
                "Don’t chase perfection — sauté it gently.",
                "Behind every great dish is a rodent with a dream.",
                "Add love. Then panic. Then garnish with flair.",
                "If you're not crying in the kitchen, are you even cooking?",
                "Order up! Destiny, drama, and extra cheese.",
                "Burnt toast? That’s just experience with a crunch.",
                "Even the fanciest soufflé falls sometimes.",
                "Keep calm and let the sous-chef handle it... oh wait, it’s you.",
                "The kitchen is chaos — but so is growth.",
                "Ctrl+C doesn't clean the kitchen, Chef.",
                "I'm just a little guy, in a big codebase, with a dream and a whisk.",
                "Rats don’t sweat. We just simmer.",
                "Not every dish needs salt. Just like not every project needs more crates. Maybe.",
                "We don't panic in this kitchen... we pivot, flambé, and roll with it.",
                "You either debug with elegance or sprint with flavor. Choose wisely.",
                "Welcome to the kitchen — where the fire is real and the bugs are sentient.",
                "Be the dish you wish to see on the menu.",
                "Behind every ‘cargo run’ is a rat praying to Gusteau.",
                "Compile errors? Just spice in disguise.",
                "Bon appé-code!",
            ],
        }
    }

    pub fn get_random_quote(&self) -> &'static str {
        let mut rng = thread_rng();
        self.quotes
            .choose(&mut rng)
            .copied()
            .unwrap_or("Surprise me!")
    }
}
