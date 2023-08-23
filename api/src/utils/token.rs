use nanoid::nanoid;

const CODE_ALPHABETS: [char; 20] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', '(', ')'];
const ID_ALPHABETS: [char; 64] = ['_', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

pub enum Token {
    ResetPassword,
    SuperUserToken,
    RandomPassword,
}

impl Token {
    pub fn generate(&self) -> String {
        match self {
            Token::ResetPassword => nanoid!(32, &CODE_ALPHABETS),
            Token::SuperUserToken => nanoid!(64, &ID_ALPHABETS),
            Token::RandomPassword => nanoid!(64, &CODE_ALPHABETS),
        }
    }
}