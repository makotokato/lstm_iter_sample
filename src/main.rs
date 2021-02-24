use icu_segmenter_lstm::lstm::Lstm;

#[macro_use]
extern crate lazy_static;

const THAI_MODEL: &[u8; 373466] =
    include_bytes!("../data/Thai_codepoints_exclusive_model4_heavy/weights.json");

lazy_static! {
    static ref THAI_LSTM: Lstm = {
        let lstm_data = serde_json::from_slice(THAI_MODEL).expect("JSON syntax error");
        Lstm::try_new(lstm_data).unwrap()
    };
}

struct LstmSegmenterIterator {
    bies_str: String,
    pos: usize,
}

impl Iterator for LstmSegmenterIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.bies_str.chars().nth(self.pos)?;
            self.pos += 1;
            if ch == 'e' {
                return Some(self.pos);
            }
        }
    }
}

impl LstmSegmenterIterator {
    pub fn new(lstm: &Lstm, input: &str) -> Self {
        let lstm_output = lstm.word_segmenter(input);
        Self {
            bies_str: lstm_output,
            pos: 0,
        }
    }
}

fn main() {
    const TEST_STR: &str = "ภาษาไทยภาษาไทย";
    let mut iter2 = LstmSegmenterIterator::new(&*THAI_LSTM, TEST_STR);

    println!("text string = {}", TEST_STR);
    println!("iter = {}", iter2.next().unwrap());
    println!("iter = {}", iter2.next().unwrap());
    println!("iter = {}", iter2.next().unwrap());
    println!("iter = {}", iter2.next().unwrap());
}
