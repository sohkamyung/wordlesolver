/// Structure to hold list of possible Wordle words.
pub struct WordleWords {
    // original list of words
    original_list: Vec<String>,
    // latest list of words after filtering out invalid words based on provided patterns
    current_list: Vec<String>,
}

impl WordleWords {
    /// Returns a new WordleWords structure
    ///
    /// Expects an initial_list of possible Wordle words.
    pub fn new(initial_list: Vec<String>) -> WordleWords {
        WordleWords {
            // current_list is a clone of the initial list at the start
            current_list: initial_list.clone(),
            original_list: initial_list,
        }
    }

    /// Resets the list of possible words.
    ///
    /// This clears out all the filters that have been applied.
    pub fn reset_list(&mut self) {
        self.current_list = self.original_list.clone();
    }

    /// Returns the current list of possible words after filters have
    /// been applied.
    pub fn get_word_list(&self) -> &Vec<String> {
        &self.current_list
    }

    /// Removes words that contains the letters
    ///
    /// # Example
    /// ```
    /// use WordleWords;
    ///
    /// let v = vec![
    ///     String::from("aaaaa"),
    ///     String::from("bbbbb"),
    ///     String::from("ccccc"),
    /// ];
    /// let mut list = WordleWords::new(v);
    /// list.remove_letters("a");
    /// let result = list.get_word_list();
    /// assert_eq!(result.len(), 2);
    /// assert_eq!("bbbbb", result[0]);
    /// assert_eq!("ccccc", result[1]);
    /// ```
    pub fn remove_letters(&mut self, letters : &str) {
        let mut new_list: Vec<String> = Vec::new();
        // add the word to the new list if there is no match to any
        // letter in the remove list
        for word in &self.current_list {
            let mut matched = false;
            for letter in letters.chars() {
                if word.contains(letter) {
                    matched = true;
                    break;
                }
            }
            if !matched {
                new_list.push(word.clone());
            }
        }
        // replace the current list with the new one
        self.current_list = new_list;
    }

    /// Include words that contain the letters in the correct location(s)
    ///
    /// Letters should be five chars.
    ///
    /// Unknown letters should be replaced by a '.'
    ///
    /// # Example
    /// ```
    /// use WordleWords;
    ///
    /// let v = vec![
    ///     String::from("aaaaa"),
    ///     String::from("bbbbb"),
    ///     String::from("ccccc"),
    /// ];
    /// let mut list = WordleWords::new(v);
    /// list.correct_letters("a....");
    /// let result = list.get_word_list();
    /// assert_eq!(result.len(), 1);
    /// assert_eq!("aaaaa", result[0]);
    /// ```
    pub fn correct_letters(&mut self, letters: &str) {
        let i = letters.len();
        if i != 5 {
            println!("length of correct letters is not 5");
            return;
        }

        let mut new_list: Vec<String> = Vec::new();
        // add the word to the new list if all the letters in the
        // correct list (excluding '.') match the word
        for word in &self.current_list {
            let mut matched = true;
            for j in 0 .. 5 {
                match &letters[j .. (j+1)] {
                    "." => (),
                    x => {
                        if x != &word[j .. (j+1)] {
                            matched = false;
                        }
                    }
                }
            }
            if matched {
                new_list.push(word.clone());
            }
        }
        // replace the current list with the new one
        self.current_list = new_list;
    }

    /// Include words that contain the letters but in the incorrect
    /// location(s)
    ///
    /// Letters should be five chars.
    ///
    /// Unknown letters should be replaced by a '.'
    ///
    /// ```
    /// use WordleWords;
    ///
    /// let v = vec![
    ///     String::from("aaaaa"),
    ///     String::from("bbabb"),
    ///     String::from("ccccc"),
    /// ];
    /// let mut list = WordleWords::new(v);
    /// list.incorrect_letters(".a...");
    /// let result = list.get_word_list();
    /// assert_eq!(result.len(), 1);
    /// assert_eq!("bbabb", result[0]);
    /// ```
    pub fn incorrect_letters(&mut self, letters: &str) {
        let i = letters.len();
        // println!("{} {}", letters, i);
        if i != 5 {
            println!("length of incorrect letters is not 5");
            return;
        }
        let mut new_list: Vec<String> = Vec::new();
        // add the word to the new list if there is no match to the
        // letters in the incorrect location list (excluding '.')
        for word in &self.current_list {
            let mut expected_matches = 5; // decremented when a '.' is encountered
            let mut match_found = 0; // incremented when a match is found but not at the expected location
            for j in 0 .. 5 {
                match &letters[j .. (j+1)] {
                    "." => expected_matches -= 1,
                    x => {
                        if word.contains(x) && x != &word[j .. (j+1)] {
                            match_found += 1;
                        }
                    }
                }
            }
            if expected_matches == match_found {
                new_list.push(word.clone());
            }
        }
        // replace the current list with the new one
        self.current_list = new_list;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_test() {
        let v = vec![
            String::from("aaaaa"),
            String::from("bbbbb"),
            String::from("ccccc")
        ];
        let list = WordleWords::new(v);
        let result = list.get_word_list();
        assert_eq!("aaaaa", result[0]);
        assert_eq!("bbbbb", result[1]);
        assert_eq!("ccccc", result[2]);
    }

    #[test]
    fn remove_test_1() {
        let v = vec![
            String::from("aaaaa"),
            String::from("bbbbb"),
            String::from("ccccc")
        ];
        let mut list = WordleWords::new(v);
        list.remove_letters("b");
        let result = list.get_word_list();
        assert_eq!(result.len(), 2);
        assert_eq!("aaaaa", result[0]);
        assert_eq!("ccccc", result[1]);
    }

    #[test]
    fn remove_test_2() {
        let v = vec![
            String::from("aaaaa"),
            String::from("bbbbb"),
            String::from("ccccc"),
            String::from("cabcc"),
        ];
        let mut list = WordleWords::new(v);
        list.remove_letters("ba");
        let result = list.get_word_list();
        assert_eq!(result.len(), 1);
        assert_eq!("ccccc", result[0]);
    }

    #[test]
    fn correct_test_1() {
        let v = vec![
            String::from("aaaaa"),
            String::from("bbbbb"),
            String::from("ccccc"),
            String::from("acaca"),
            String::from("abcde"),
        ];
        let mut list = WordleWords::new(v);
        list.correct_letters("a.a.a");
        let result = list.get_word_list();
        assert_eq!(result.len(), 2);
        assert_eq!("aaaaa", result[0]);
        assert_eq!("acaca", result[1]);
    }

    #[test]
    fn correct_test_2() {
        let v = vec![
            String::from("aaaaa"),
            String::from("bbbbb"),
            String::from("ccccc"),
            String::from("acaca"),
            String::from("abcde"),
        ];
        let mut list = WordleWords::new(v);
        list.correct_letters("a....");
        let result = list.get_word_list();
        assert_eq!(result.len(), 3);
        assert_eq!("aaaaa", result[0]);
        assert_eq!("acaca", result[1]);
        assert_eq!("aaaaa", result[0]);
    }

    #[test]
    fn incorrect_test_1() {
        let v = vec![
            String::from("aaaaa"),
            String::from("bbbbb"),
            String::from("ccccc"),
            String::from("acaca"),
            String::from("bacde"),
        ];
        let mut list = WordleWords::new(v);
        list.incorrect_letters("a....");
        let result = list.get_word_list();
        assert_eq!(result.len(), 1);
        assert_eq!("bacde", result[0]);
    }

    #[test]
    fn incorrect_test_2() {
        let v = vec![
            String::from("aaaaa"),
            String::from("bbbbb"),
            String::from("ccccc"),
            String::from("acaca"),
            String::from("abcde"),
        ];
        let mut list = WordleWords::new(v);
        list.incorrect_letters("..b..");
        let result = list.get_word_list();
        assert_eq!(result.len(), 1);
        assert_eq!("abcde", result[0]);
    }

    #[test]
    fn incorrect_test_3() {
        let v = vec![
            String::from("aaaaa"),
            String::from("ddddd"),
            String::from("ccccc"),
            String::from("acaca"),
            String::from("abcde"),
        ];
        let mut list = WordleWords::new(v);
        list.incorrect_letters(".d.d.");
        let result = list.get_word_list();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn remove_correct_test_1() {
        let v = vec![
            String::from("aaaaa"),
            String::from("bbbbb"),
            String::from("ccccc"),
            String::from("acaca"),
            String::from("abcde"),
        ];
        let mut list = WordleWords::new(v);
        list.remove_letters("a");
        list.correct_letters("b....");
        let result = list.get_word_list();
        assert_eq!(result.len(), 1);
        assert_eq!("bbbbb", result[0]);
    }

    #[test]
    fn remove_correct_incorrect_test_1() {
        let v = vec![
            String::from("aaaaa"),
            String::from("bbbbb"),
            String::from("ccccc"),
            String::from("bbbbc"),
            String::from("bbcdb"),
        ];
        let mut list = WordleWords::new(v);
        list.remove_letters("a");
        list.correct_letters("bb...");
        list.incorrect_letters("....b");
        let result = list.get_word_list();
        assert_eq!(result.len(), 1);
        assert_eq!("bbbbc", result[0]);
    }

    #[test]
    fn reset_list_test_1() {
        let v = vec![
            String::from("aaaaa"),
            String::from("bbbbb"),
            String::from("ccccc"),
        ];
        let mut list = WordleWords::new(v);
        list.remove_letters("a");
        {
            let result = list.get_word_list();
            assert_eq!(result.len(), 2);
        }
        list.reset_list();
        let result = list.get_word_list();
        assert_eq!(result.len(), 3);
        assert_eq!("aaaaa", result[0]);
        assert_eq!("bbbbb", result[1]);
        assert_eq!("ccccc", result[2]);
    }
}
