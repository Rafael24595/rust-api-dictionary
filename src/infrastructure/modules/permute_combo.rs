pub struct PermuteCombo {
    combo: String
}

impl PermuteCombo {

    pub fn new(combo: String) -> PermuteCombo {
        return PermuteCombo {
            combo
        }
    }

    pub fn permute(&self) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut used = vec![false; self.combo.len()];
        self.build_permutations(&mut result, &mut current, &mut used);
        return result; 
    }

    fn build_permutations(&self, result: &mut Vec<String>, current: &mut String, used: &mut Vec<bool>) {
        result.push(current.to_string());

        if current.len() == self.combo.len() {
            return;
        }
    
        for (i, ch) in self.combo.chars().enumerate() {
            if used[i] {
                continue;
            }
    
            used[i] = true;
            current.push(ch);
    
            self.build_permutations(result, current, used);
    
            used[i] = false;
            current.pop();
        }
    }

}