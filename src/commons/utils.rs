use rand::Rng;

static COMBO_CONSONANTS_CHARACTERS: &str = "bcdfghjklmnpqrstvwxyz";
static COMBO_VOCALS_CHARACTERS: &str = "aeiou";

pub fn get_random_combo(mut length: i8) -> String {
    let mut combo: String =  String::from("");
    let mut i: i8 = 0;

    if length > (COMBO_CONSONANTS_CHARACTERS.len() + COMBO_VOCALS_CHARACTERS.len()) as i8 {
        length = (COMBO_CONSONANTS_CHARACTERS.len() + COMBO_VOCALS_CHARACTERS.len()) as i8 ;
    }

    while i < length {
        let character = get_random_combo_character();
        if !combo.contains(character) {
            combo.push(character);
            i += 1;
        }
    }

    return combo.to_string();

}

fn get_random_combo_character() -> char {
    let mode = rand::thread_rng().gen_range(0..4);
    let mut chain: String = COMBO_CONSONANTS_CHARACTERS.to_owned();

    if mode > 1 {
        chain = COMBO_VOCALS_CHARACTERS.to_owned();
    }

    let index = rand::thread_rng().gen_range(0..chain.len());
    return chain.chars().nth(index).unwrap();
}