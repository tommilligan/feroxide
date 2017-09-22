use molecule::Molecule;

lazy_static! {

    pub static ref AMMONIA: Molecule = Molecule::from_string("NH3".to_owned()).unwrap();
    pub static ref WATER: Molecule = Molecule::from_string("H2O".to_owned()).unwrap();
    pub static ref PYRITE: Molecule = Molecule::from_string("FeS2".to_owned()).unwrap();

    pub static ref ALL_TRIVIALS: Vec<(&'static str, &'static Molecule)> = vec! {
        ("AMMONIA", &AMMONIA),
        ("WATER", &WATER),
        ("PYRITE", &PYRITE)
    };
}
