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
        
        match &mut self.entries[index] {
            Entry::Implications(pairs) => {
                pairs.push(pair);
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
}