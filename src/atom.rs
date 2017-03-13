use properties::*;

use types::*;


#[derive(Debug)]
pub struct Atom {
    pub number: AtomNumber,
    pub mass: AtomMass,
    pub symbol: &'static str,
    pub name: &'static str,
    pub group: AtomGroup,
    pub is_diatomic: bool
}


impl Eq for Atom {}

impl PartialEq for Atom {
    fn eq(&self, rhs: &Atom) -> bool {
        self.number == rhs.number
    }
}


impl Properties for Atom {
    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }

    fn name(&self) -> String {
        // NOTE: This case is temporary
        if self.name == "oxygen" {
            // Special suffix
            return "oxide".to_owned();
        }

        self.name.to_owned()
    }

    fn mass(&self) -> AtomMass {
        self.mass
    }
}