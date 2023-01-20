use std::thread::current;

use crate::flowtable::FlowTableValue;

#[derive(Debug, Clone)]
enum Entry {
    Incompatible,
    Implications(Vec<(usize, usize)>)
}

impl Default for Entry {
    fn default() -> Self {
        Entry::Implications(vec![])
    }
}

#[derive(Debug, Clone)]
pub struct StateMatrix {
    entries: Vec<Entry>,
    width: usize
}

impl StateMatrix {
    pub fn new(num_states: usize) -> StateMatrix {
        let matrix_width = num_states - 1;

        StateMatrix {
            width: matrix_width,
            entries: vec![Entry::default(); num_states * (num_states - 1) / 2]
        }
    }

    pub fn add_pair(&mut self, first: usize, second: usize, pair: (usize, usize)) {
        let index = second * (second - 1) / 2 + first;
        
        let ordered_pair = if pair.0 > pair.1 {(pair.1, pair.0)} else {pair};

        match &mut self.entries[index] {
            Entry::Implications(pairs) => {
                if pairs.contains(&ordered_pair) {
                    return;
                }

                pairs.push(ordered_pair);
                return;
            },

            Entry::Incompatible => {}
        }

        self.entries[index] = Entry::Implications(vec![pair]);
    }

    pub fn set_incompatible(&mut self, first: usize, second: usize) {
        let index = second * (second - 1) / 2 + first;

        self.entries[index] = Entry::Incompatible;
    }

    fn propagate_incompatibility(&mut self, pair: (usize, usize)) {
        self.entries[pair.1 * (pair.1 - 1) / 2 + pair.0] = Entry::Incompatible; 

        let mut incompatible_pairs: Vec<(usize, usize)> = vec![];        

        let mut current_pair = (0, 1);
        for entry in &self.entries {
            match entry {
                Entry::Incompatible => {},
                Entry::Implications(pairs) => {
                    if pairs.iter().find(|&&x| x == pair).is_some() {
                        incompatible_pairs.push(current_pair);
                    }
                }
            }

            current_pair.0 += 1;
            if current_pair.0 == current_pair.1 {
                current_pair.1 += 1;
                current_pair.0 = 0;
            }
        }

        incompatible_pairs.iter().for_each(|&incompatible_pair| self.propagate_incompatibility(incompatible_pair));
    }

    fn add_incompatible_states(&mut self) {
        let mut incompatible_pairs: Vec<(usize, usize)> = vec![];

        let mut current_pair = (0, 1);
        for entry in &self.entries {
            match entry {
                Entry::Incompatible => incompatible_pairs.push(current_pair),
                _ => {}
            };

            current_pair.0 += 1;
            if current_pair.0 == current_pair.1 {
                current_pair.1 += 1;
                current_pair.0 = 0;
            }
        }

        incompatible_pairs.iter().for_each(|&pair| self.propagate_incompatibility(pair));
    }

    pub fn to_c_list(&mut self) {
        self.add_incompatible_states();
    }
}