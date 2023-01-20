use std::fmt::Display;

use crate::statematrix::StateMatrix;

#[derive(Debug, Clone, Default)]
pub enum FlowTableValue {
    #[default] DontCare,
    Value(usize)
}

impl FlowTableValue {
    fn is_dont_care(&self) -> bool {
        match self {
            Self::DontCare => true,
            _ => false
        }
    }

    fn unwrap(&self) -> usize {
        match self {
            Self::DontCare => panic!("Unwrapped dont care value"),
            Self::Value(x) => *x
        }
    }
}

impl Display for FlowTableValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DontCare => write!(f, "-"),
            Self::Value(val) => write!(f, "{val}")
        }
    }
}

impl PartialEq for FlowTableValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::DontCare => true,

            Self::Value(left) => match other {
                Self::DontCare => true,
                Self::Value(right) => left == right
            }
        }
    }
}

impl Eq for FlowTableValue {}

#[derive(Debug, Clone, Default)]
struct FlowTableEntry {
    to: FlowTableValue,
    output: FlowTableValue
}


#[derive(Debug, Clone)]
pub struct FlowTable {
    entries: Vec<FlowTableEntry>,
    num_states: usize,
    num_inputs: usize
}

impl FlowTable {
    pub fn new(num_states: usize, num_inputs: usize) -> FlowTable {
        FlowTable {
            entries: vec![FlowTableEntry::default(); num_states * num_inputs],
            num_states: num_states,
            num_inputs: num_inputs
        }
    }

    pub fn set_entry(&mut self, state: usize, input: usize, next_state: FlowTableValue, output: FlowTableValue) {
        self.entries[state * self.num_inputs + input] = FlowTableEntry {
            to: next_state,
            output: output
        }
    }

    fn add_rows_to_matrix(&self, matrix: &mut StateMatrix, first_row: usize, second_row: usize) {
        for input in 0..self.num_inputs {
            let left = &self.entries[first_row * self.num_inputs + input];
            let right = &self.entries[second_row * self.num_inputs + input];

            if left.output != right.output {
                matrix.set_incompatible(first_row, second_row)
            } else {
                if left.to.is_dont_care() || right.to.is_dont_care() {
                    continue;
                }

                if left.to.unwrap() != right.to.unwrap() {
                    matrix.add_pair(first_row, second_row, (left.to.unwrap(), right.to.unwrap()))
                }
            }
        }
    }

    pub fn reduce(self) -> FlowTable {
        let mut matrix = StateMatrix::new(self.num_states);

        for first in 0..self.num_states {
            for second in (first + 1)..self.num_states {
                self.add_rows_to_matrix(&mut matrix, first, second)                
            }
        }
        
        let c_list = matrix.to_c_list();
        matrix = dbg!(matrix);
        todo!()
    }
}

impl Display for FlowTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "     |")?;
        for i in 0..self.num_inputs {
            write!(f, "|  {}  ", char::from_digit((i + 10) as u32, 36).unwrap().to_ascii_uppercase())?;
        }
        writeln!(f)?;
        
        write!(f, "-----+")?;
        for _ in 0..self.num_inputs {
            write!(f, "+-----")?;
        }
        writeln!(f)?;

        for state in 0..self.num_states {
            write!(f, "  {state}  |")?;
            for i in 0..self.num_inputs {
                write!(f, "| {},{} ", self.entries[state * self.num_inputs + i].to, self.entries[state * self.num_inputs + i].output)?;
            }
            writeln!(f)?;

            if state == self.num_states - 1 {
                break;
            }
            
            write!(f, "-----+")?;
            for _ in 0..self.num_inputs {
                write!(f, "+-----")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}