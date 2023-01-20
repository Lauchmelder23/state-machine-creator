use std::{collections::HashSet, thread::current};

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

    fn to_c_list(&mut self) -> Vec<HashSet<usize>> {
        self.add_incompatible_states();

        let mut c_list: Vec<HashSet<usize>> = Vec::new();

        for k in (0..self.width).rev() {
            for row in k+1..self.width+1 {
                match &self.entries[row * (row - 1) / 2 + k] {
                    Entry::Implications(pairs) => {
                        for pair in pairs {
                            let mut entry: HashSet<usize> = HashSet::new();
                            entry.insert(k);
                            entry.insert(row);

                            c_list.push(entry);
                        }
                    },

                    _ => {}
                }

            }

            if !c_list.is_empty() {
                break;
            }
        }

        for k in (0..self.width - 1).rev() {
            let mut valid_rows: HashSet<usize> = HashSet::new();

            for row in k+1..self.width+1 {
                match &self.entries[row * (row - 1) / 2 + k] {
                    Entry::Implications(_) => {
                        valid_rows.insert(row);
                    },

                    _ => {}
                }
            }

            if valid_rows.is_empty() {
                continue;
            }

            let mut to_remove: Vec<HashSet<usize>> = vec![];
            let mut new_entries: Vec<HashSet<usize>> = vec![];
            for entry in &c_list {
                let mut intersection: HashSet<&usize> = valid_rows.intersection(entry).collect();

                if intersection.len() > 1 {
                    let mut current_col = HashSet::new();
                    current_col.insert(k);

                    intersection.extend(&current_col);

                    new_entries.push(intersection.iter().map(|&&x| x).collect());
                    to_remove.push(entry.clone());
                }
            }

            c_list.retain(|x| !to_remove.contains(x));
            c_list.append(&mut new_entries);

            for row in k+1..self.width+1 {
                match &self.entries[row * (row - 1) / 2 + k] {
                    Entry::Implications(_) => {
                        let mut entry: HashSet<usize> = HashSet::new();
                        entry.insert(k);
                        entry.insert(row);

                        if !c_list.iter().any(|x| entry.is_subset(x)) {
                            c_list.push(entry);
                        }
                    },

                    _ => {}
                }
            }
        }

        for k in 0..self.width+1 {
            let mut entry: HashSet<usize> = HashSet::new();
            entry.insert(k);

            if !c_list.iter().any(|x| entry.is_subset(x)) {
                c_list.push(entry);
            }
        }

        dbg!(c_list)
    }

    pub fn get_reduced_states(&mut self) {
        let c_list = self.to_c_list();

        let implications: Vec<(HashSet<usize>, HashSet<usize>)> = vec![];
        for entry in &c_list {
            let values: Vec<usize> = entry.iter().map(|&x| x).collect();

            for first in &values {
                for second in &values {
                    if first == second {
                        continue;
                    }


                }
            }
        }
    }
}