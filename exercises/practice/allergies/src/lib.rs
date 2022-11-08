use enum_iterator::{reverse_all, Sequence};

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Sequence, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}
const MAX_SCORE: u32 = 2^8-1;       //La somma di tutti è 255

pub struct Allergies{
    //allergies : [Allergen; 8],
    allergies : Vec<Allergen>,
}

impl Allergies {

    pub fn new(score: u32) -> Self {
        let mut _allergies = Allergies{ allergies: vec!()};
        let mut actual_score = score;
        println!("Actual score prima del while:{:?}", actual_score);

        while actual_score > MAX_SCORE{
            actual_score -= MAX_SCORE+1;
        }
        let allergens = reverse_all::<Allergen>().collect::<Vec<_>>();
        println!("{:?}", allergens);
        println!("Actual score:{:?}", actual_score);

        for val in allergens{
            println!("Val:{:?}", val as u32);
            if actual_score >= (val as u32) {
                println!("Allergy added: {:?}", val);
                _allergies.allergies.push(val);
                actual_score -= val as u32;
            }
            else if actual_score == 0 {
                break;
            }
        }

        return _allergies;
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
