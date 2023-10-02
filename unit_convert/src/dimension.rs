use crate::{units::{Space, Conversion}, Num};

pub struct Dimensions {
    /// Assumed to always be simplified, no two dimensions with the same space
    dimensions: Vec<Dimension>,
    units: Vec<Box<dyn Conversion>>
}

#[derive(Debug, PartialEq)]
pub struct Dimension {
    unit_space: Space,
    exponent: Num,
}

impl Dimensions {
    fn get_space(&self, space: Space) -> Option<Num> {
        self.dimensions
            .iter()
            .find(|x| x.unit_space == space)
            .map(|x| x.exponent)
    }
}

impl PartialEq for Dimensions {
    fn eq(&self, other: &Self) -> bool {
        let mut inc = 0;
        for dim in &self.dimensions {
            if let Some(i) = other.get_space(dim.unit_space) {
                if i != dim.exponent {
                    return false;
                }
            } else {
                return false;
            }

            inc += 1;
        }

        if inc != other.dimensions.len() {
            return false;
        }

        true
    }
}
