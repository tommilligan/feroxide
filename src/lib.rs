// macros \\

#[macro_use]
pub mod parse_macros;

#[macro_use]
extern crate lazy_static;

#[macro_export]
macro_rules! molecule_from_atom {
    ($atom:expr) => (
        Molecule { compounds: vec! { MoleculeCompound::from_atom($atom) } }
    )
}

#[macro_export]
macro_rules! ion_from_molecule {
    ($molecule:expr) => (
        Ion { molecule: $molecule, charge: None }
    )
}

#[macro_export]
macro_rules! ion_from_atom {
    ($atom:expr) => (
        ion_from_molecule!(molecule_from_atom!($atom))
    )
}


mod atom;
mod container;
mod electron;
mod ion;
mod math;
mod molecule;
mod namings;
mod reaction;
mod redox;
mod trait_element;
mod trait_properties;
mod trait_reaction;
mod types;

pub use atom::*;
pub use container::*;
pub use electron::*;
pub use ion::*;
pub use math::*;
pub use molecule::*;
pub use namings::*;
pub use reaction::*;
pub use redox::*;
pub use trait_element::*;
pub use trait_properties::*;
pub use trait_reaction::*;
pub use types::*;

pub mod data_atoms;
pub mod data_ions;
pub mod data_molecules;
pub mod data_sep;
pub mod data_trivials;

pub mod display_impls;




// tests \\
#[test]
fn trivial_names() {
    use data_atoms::*;

    macro_rules! molecule_string_from_trivial {
        ($string: expr) => (
            Molecule::from_trivial($string.to_owned()).or(Some(molecule_from_atom!(HYDROGEN))).unwrap().symbol()
        )
    }

    assert_eq!(molecule_string_from_trivial!("water"), "H₂O");
    assert_eq!(molecule_string_from_trivial!("ammonia"), "NH₃");
    assert_eq!(molecule_string_from_trivial!("pyrite"), "FeS₂");
}


#[test]
fn molecule_namings() {
    macro_rules! molecule_name_from_string {
        ($string: expr) => (
            Molecule::from_string($string.to_owned()).unwrap().name()
        )
    }

    assert_eq!(molecule_name_from_string!("NH3"), "nitrogentrihydride");  // ammonia
    assert_eq!(molecule_name_from_string!("H2O"), "dihydrogenmonoxide"); // water, DHMO
    assert_eq!(molecule_name_from_string!("PbO"), "leadoxide");
    assert_eq!(molecule_name_from_string!("PCl5"), "phosphoruspentachloride");
}


#[test]
fn display_types_equals_value() {
    assert_eq!(format!("{}", AtomNumber::from(1)), "1");
    assert_eq!(format!("{}", AtomNumber::from(2)), "2");
    assert_eq!(format!("{}", Energy::from(1.23)), "1.23");
}


#[test]
fn ion_calculate_charge_from_string() {
    use trait_element::*;

    assert_eq!(
        AtomCharge::from(-1),
        Ion::from_string("OH".to_owned())
            .unwrap()
            .get_charge()
            .unwrap()
    );
}


#[test]
fn reaction_from_string() {
    use data_atoms::*;
    use data_molecules::*;


    let reaction = ElemReaction {
        lhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    element: ion_from_atom!(HYDROGEN),
                    amount: 2,
                },
                ReactionCompound {
                    element: ion_from_atom!(OXYGEN),
                    amount: 1,
                },
            ],
        },

        rhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    element: ion_from_molecule!(WATER.clone()),
                    amount: 2,
                },
            ],
        },

        is_equilibrium: true,
    };

    let reaction_from_string = ElemReaction::<Ion>::ion_from_string("2H2 + O2 <> 2H2O".to_owned())
        .unwrap();

    assert_eq!(reaction, reaction_from_string);


    let reaction_from_string =
        ElemReaction::<Ion>::ion_from_string("2 H2 + O2 ⇌ 2 H2O".to_owned()).unwrap();

    assert_eq!(reaction, reaction_from_string);
}


#[test]
fn atom_from_string() {
    use data_atoms::*;

    assert_eq!(HYDROGEN, Atom::from_string("H".to_owned()).unwrap());
    assert_eq!(CARBON, Atom::from_string("C".to_owned()).unwrap());
    assert_eq!(COBALT, Atom::from_string("Co".to_owned()).unwrap());
    assert_eq!(FLUORINE, Atom::from_string("F".to_owned()).unwrap());
    assert_eq!(IRON, Atom::from_string("Fe".to_owned()).unwrap());
}


#[test]
fn molecule_from_string() {
    use data_molecules::*;

    assert_eq!(
        WATER.clone(),
        Molecule::from_string("H2O".to_owned()).unwrap()
    );
    assert_eq!(
        CO2.clone(),
        Molecule::from_string("CO2".to_owned()).unwrap()
    );
    assert_eq!(
        SUGAR.clone(),
        Molecule::from_string("C12H22O11".to_owned()).unwrap()
    );
}


#[test]
fn test_gcd() {
    assert_eq!(2, gcd(4, 6));
    assert_eq!(5, gcd(10, 25));
    assert_eq!(12, gcd(48, 36));
    assert_eq!(1, gcd(7, 13));
}


#[test]
fn total_atoms() {
    use data_atoms::*;
    use data_molecules::*;

    let side = ReactionSide {
        compounds: vec![
            ReactionCompound {
                element: WATER.clone(),
                amount: 8,
            },
            ReactionCompound {
                element: molecule_from_atom!(OXYGEN),
                amount: 5,
            },
        ],
    };

    // 16 Hydrogen atoms
    assert_eq!(16, *side.total_atoms().get(&AtomNumber::from(1)).unwrap());

    // 8 + 10 = 18 Oxygen atoms
    assert_eq!(18, *side.total_atoms().get(&AtomNumber::from(8)).unwrap());
}


#[test]
fn container_add_and_remove_elements() {
    use data_atoms::*;
    use data_molecules::*;


    let mut container = Container {
        contents: vec![
            ContainerCompound {
                element: WATER.clone(),
                moles: Moles::from(6.0),
            },
        ],

        available_energy: Energy::from(1e5), // Should be enough
    };


    let reaction = ElemReaction {
        lhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    element: WATER.clone(),
                    amount: 2,
                },
            ],
        },

        rhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    element: molecule_from_atom!(HYDROGEN),
                    amount: 2,
                },
                ReactionCompound {
                    element: molecule_from_atom!(OXYGEN),
                    amount: 1,
                },
            ],
        },

        is_equilibrium: false,
    };

    // Make sure reaction is valid
    assert!(reaction.is_valid());

    // 6 moles water
    assert_eq!(1, container.contents.len());

    // 4 moles water, 2 moles hydrogen, 1 mole oxygen
    assert!(container.react(&reaction));
    assert_eq!(3, container.contents.len());

    // 2 moles water, 4 moles hydrogen, 2 moles oxygen
    assert!(container.react(&reaction));
    assert_eq!(3, container.contents.len());

    // 6 moles hydrogen, 3 moles oxygen
    assert!(container.react(&reaction));
    assert_eq!(2, container.contents.len());

    // No water left, so should fail
    assert!(!container.react(&reaction));

    // Remove 6 moles of hydrogen and 3 moles of oxygen (all contents)
    container.remove_elements(&vec![
        ContainerCompound {
            element: molecule_from_atom!(HYDROGEN),
            moles: Moles::from(6.0),
        },
        ContainerCompound {
            element: molecule_from_atom!(OXYGEN),
            moles: Moles::from(3.0),
        },
    ]);

    // Now it should be empty
    assert_eq!(0, container.contents.len());
}


#[test]
fn check_display() {
    use data_atoms::*;
    use data_molecules::*;
    use data_ions::*;
    use electron::*;
    use reaction::*;
    use container::*;


    let reactioncompound = ReactionCompound {
        amount: 1,
        element: SUGAR.clone(),
    };

    let reactionside = ReactionSide { compounds: vec![reactioncompound.clone()] };

    let reaction = ElemReaction {
        lhs: reactionside.clone(),

        rhs: ReactionSide { compounds: vec![] },

        is_equilibrium: false,
    };

    let containercompound = rc_to_cc(reactioncompound.clone());

    let container = Container {
        contents: vec![containercompound.clone()],
        available_energy: Energy::from(0.0),
    };


    format!("{}", HYDROGEN); // Atom
    format!("{}", SUGAR.clone()); // Molecule
    format!("{}", AMMONIUM.clone()); // Ion
    format!("{}", ELECTRON.clone()); // Electron
    format!("{}", reactioncompound); // ReactionCompound
    format!("{}", reactionside); // ReactionSide
    format!("{}", reaction); // Reaction
    format!("{}", containercompound); // ContainerCompound
    format!("{}", container); // Container
}


#[test]
fn container_reaction_cost() {
    use data_atoms::*;
    use data_molecules::*;


    let hydrogen = molecule_from_atom!(HYDROGEN);
    let oxygen = molecule_from_atom!(OXYGEN);


    let mut container = Container {
        contents: vec![
            ContainerCompound {
                moles: Moles::from(100.0),
                element: hydrogen.clone(),
            },
            ContainerCompound {
                moles: Moles::from(200.0),
                element: oxygen.clone(),
            },
        ],

        available_energy: Energy::from(1000.0),
    };


    let reaction = ElemReaction {
        lhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    amount: 2,
                    element: hydrogen.clone(),
                },
                ReactionCompound {
                    amount: 1,
                    element: oxygen.clone(),
                },
            ],
        },

        rhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    element: WATER.clone(),
                    amount: 2,
                },
            ],
        },

        is_equilibrium: false, // actually true, but false for this test
    };


    assert_eq!(Energy::from(1000.0), container.available_energy);
    assert_eq!(Energy::from(-100.0), reaction.energy_cost());

    // Repeadably try this reaction

    assert!(container.react(&reaction));
    assert_eq!(Energy::from(1100.0), container.available_energy);

    assert!(container.react(&reaction));
    assert_eq!(Energy::from(1200.0), container.available_energy);

    assert!(container.react(&reaction));
    assert_eq!(Energy::from(1300.0), container.available_energy);
}

#[test]
fn ion_notation_check() {
    use data_atoms::*;


    // P₄²⁻  tetraphosphorus(-II)
    #[allow(non_snake_case)]
    let P4 = Ion {
        molecule: Molecule {
            compounds: vec![
                MoleculeCompound {
                    atom: PHOSPHORUS,
                    amount: 4,
                },
            ],
        },

        charge: Some(AtomCharge::from(-2)),
    };


    assert_eq!("P₄²⁻", P4.symbol());
    assert_eq!("tetraphosphorus(-II)", P4.name());
}


#[test]
fn ion_charge_calculation() {
    use data_ions::*;


    assert_eq!(AtomCharge::from(-2), SULPHATE.get_charge().unwrap());
    assert_eq!(AtomCharge::from(-1), HYDROXIDE.get_charge().unwrap());
    assert_eq!(AtomCharge::from(1), AMMONIUM.get_charge().unwrap());
}


#[test]
fn electron_data() {
    use electron::*;


    assert_eq!(AtomCharge::from(-1), ELECTRON.get_charge().unwrap());
    assert_eq!("e⁻", ELECTRON.symbol());
}


#[test]
fn diatomic_check() {
    use data_atoms::*;


    assert_eq!(2, MoleculeCompound::from_atom(HYDROGEN).amount);
    assert_eq!(2, MoleculeCompound::from_atom(NITROGEN).amount);
    assert_eq!(2, MoleculeCompound::from_atom(OXYGEN).amount);
    assert_eq!(2, MoleculeCompound::from_atom(FLUORINE).amount);
    assert_eq!(2, MoleculeCompound::from_atom(CHLORINE).amount);
    assert_eq!(2, MoleculeCompound::from_atom(BROMINE).amount);
    assert_eq!(2, MoleculeCompound::from_atom(IODINE).amount);

    assert_eq!(1, MoleculeCompound::from_atom(CARBON).amount);
    assert_eq!(1, MoleculeCompound::from_atom(LITHIUM).amount);
    assert_eq!(1, MoleculeCompound::from_atom(SULFUR).amount);
}


#[test]
fn atoms_database_check() {
    use data_atoms::*;


    assert_eq!(AtomNumber::from(1), HYDROGEN.number);
    assert_eq!("oxygen", OXYGEN.name);
    assert_eq!("He", HELIUM.symbol);
    assert_eq!(AtomMass::from(35.45), CHLORINE.mass);

    assert!(HYDROGEN.diatomic);
    assert!(OXYGEN.diatomic);
    assert!(!CARBON.diatomic);
    assert!(!SULFUR.diatomic);

    // Very important
    assert_eq!("aluminium", ALUMINIUM.name);
}


#[test]
fn greek_namings() {
    use namings::number_to_greek as to_greek;


    assert_eq!("di", to_greek(2));
    assert_eq!("undeca", to_greek(11));
    assert_eq!("dodeca", to_greek(12));
    assert_eq!("tetradeca", to_greek(14));
    assert_eq!("icosa", to_greek(20));
    assert_eq!("henicosa", to_greek(21));
    assert_eq!("tricosa", to_greek(23));
    assert_eq!("triaconta", to_greek(30));
    assert_eq!("pentatriaconta", to_greek(35));
    assert_eq!("pentaconta", to_greek(50));
    assert_eq!("dohexaconta", to_greek(62));
    assert_eq!("nonanonaconta", to_greek(99));
}


#[test]
fn roman_namings() {
    use namings::number_to_roman as to_roman;


    assert_eq!("0", to_roman(0));
    assert_eq!("IV", to_roman(4));
    assert_eq!("-III", to_roman(-3));
    assert_eq!("VII", to_roman(7));
    assert_eq!("-VII", to_roman(-7));
    assert_eq!("XI", to_roman(11));
    assert_eq!("XIV", to_roman(14));
    assert_eq!("-XVI", to_roman(-16));
}


#[test]
fn reaction_check() {
    use data_atoms::*;
    use data_molecules::*;


    let good_reaction = ElemReaction {
        lhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    amount: 1,
                    element: molecule_from_atom!(CARBON),
                },
                ReactionCompound {
                    amount: 1,
                    element: molecule_from_atom!(OXYGEN),
                },
            ],
        },

        rhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    amount: 1,
                    element: CO2.clone(),
                },
            ],
        },

        is_equilibrium: false,
    };


    let wrong_reaction_0 = ElemReaction {
        lhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    amount: 9,
                    element: molecule_from_atom!(CARBON),
                },
                ReactionCompound {
                    amount: 5,
                    element: molecule_from_atom!(OXYGEN),
                },
            ],
        },

        rhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    amount: 3,
                    element: CO2.clone(),
                },
            ],
        },

        is_equilibrium: false,
    };


    let wrong_reaction_1 = ElemReaction {
        lhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    amount: 1,
                    element: molecule_from_atom!(LITHIUM),
                },
            ],
        },

        rhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    amount: 3,
                    element: molecule_from_atom!(HYDROGEN),
                },
            ],
        },

        is_equilibrium: false,
    };


    let equilibrium_reaction = ElemReaction {
        lhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    amount: 1,
                    element: molecule_from_atom!(HYDROGEN),
                },
            ],
        },

        rhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    amount: 2,
                    element: Molecule {
                        compounds: vec![
                            MoleculeCompound {
                                atom: HYDROGEN,
                                amount: 1,
                            },
                        ],
                    },
                },
            ],
        },

        is_equilibrium: true,
    };


    // Test validity
    assert!(good_reaction.is_valid());
    assert!(!wrong_reaction_0.is_valid());
    assert!(!wrong_reaction_1.is_valid());


    // Test display
    assert_eq!("C + O₂ → CO₂", format!("{}", good_reaction));
    assert_eq!("H₂ ⇌ 2H", format!("{}", equilibrium_reaction));
}


#[test]
fn equalise() {
    use data_atoms::*;
    use data_molecules::*;


    let water_reaction = ElemReaction {
        lhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    element: molecule_from_atom!(HYDROGEN),
                    amount: 0,
                },
                ReactionCompound {
                    element: molecule_from_atom!(OXYGEN),
                    amount: 0,
                },
            ],
        },

        rhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    element: WATER.clone(),
                    amount: 0,
                },
            ],
        },

        is_equilibrium: false,
    };


    assert!(water_reaction.equalise());
}


#[test]
fn only_compare_similiar_elements() {
    use data_molecules::*;
    use data_ions::*;


    let _ = ElemReaction {
        lhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    element: WATER.clone(),
                    amount: 1,
                },
            ],
        },

        rhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    element: CO2.clone(),
                    amount: 1,
                },
            ],
        },

        is_equilibrium: true,
    };


    let _ = ElemReaction {
        lhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    element: AMMONIUM.clone(),
                    amount: 1,
                },
            ],
        },

        rhs: ReactionSide {
            compounds: vec![
                ReactionCompound {
                    element: SULPHATE.clone(),
                    amount: 1,
                },
            ],
        },

        is_equilibrium: true,
    };
}


#[test]
fn molecule_mass_calculation() {
    use data_molecules::*;


    assert_eq!(AtomMass::from(18.015), WATER.mass());
    assert_eq!(AtomMass::from(342.297), SUGAR.mass());
    assert_eq!(AtomMass::from(44.009), CO2.mass());
}
