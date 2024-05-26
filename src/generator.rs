use std::ops::Rem;
use rand;

static LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
static UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static NUMBERS: &str = "0123456789";
static SPECIAL: &str = "!@#$%^&*=+-_";

pub enum STRENTH {
    Weak,
    Average,
    Strong
}

fn collect_alphabet(pwd_diff: STRENTH) -> String {
    let mut charset: String = String::new();
    match pwd_diff {
        STRENTH::Weak => {
            charset.push_str(LOWERCASE);
            charset.push_str(UPPERCASE);
        },
        STRENTH::Average => {
            charset.push_str(LOWERCASE);
            charset.push_str(UPPERCASE);
            charset.push_str(NUMBERS);
        },
        STRENTH::Strong => {
            charset.push_str(LOWERCASE);
            charset.push_str(UPPERCASE);
            charset.push_str(NUMBERS);
            charset.push_str(SPECIAL);
        }
    };
    return charset;
}

fn get_rand_char(charset: &str) -> char {
    let rand_num: i32 = (rand::random::<i32>() % charset.chars().count() as i32).abs();
    charset.chars().nth(rand_num as usize).expect("SHIT")
}

pub fn genpwd(len: i32, pwd_diff: STRENTH) -> String{
    let mut result = String::new();
    let charset = String::from(collect_alphabet(pwd_diff));
    
    for _ in 0..len {
        result.push(get_rand_char(&charset));
    }
    return result;
}
