//MorseCode
#[derive(Debug, PartialEq)]


enum Pulse {
    Short,
    Long,
}

//Representes a single character 
type Letter = Vec<Pulse>;
/// Represents a string of character
type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        let mut msg = Vec::with_capacity(self.len());

        for c in self.chars(){
     let pulses =  match c    {

            
            'A' | 'a' => vec![Pulse::Short, Pulse::Long],
            'B' | 'b' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short],
            // 'C' | 'c' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short],
            // 'D' | 'd' => vec![Pulse::Long, Pulse::Short, Pulse::Short],
            // 'E' | 'e' => vec![Pulse::Short],
            // 'F' | 'f' =>
            _=> continue,

            };
            msg.push(pulses);
        }
        msg
    }
}

impl std::fmt::Display for Pulse {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}


fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        };
        print!(" ");
    };
    println!();
}
fn main() {
    let greeting = "Hello, world"
        .to_string()
        .to_morse_code();
    
    print_morse_code(&greeting);
}



// impl std::fmt::Display for Pulse {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Short => write!(f, "."), 
//             Self::Long => write!(f, "_"),
//         }
//     }
// }


#[test]
fn hello_world() {
    use Pulse::*;

    let expected = vec![
        vec![Short, Short, Short, Short],
        vec![Short],
        vec![Short, Long, Short, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Long, Long],
        vec![Short, Long, Long],
        vec![Long, Long, Long],
        vec![Short, Long, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Short, Short],
    ];

    let actual = "Hello, world".to_string().to_morse_code();
    assert_eq!(actual, expected);
}

#[test]
fn whole_alphabet() {
    // let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890".to_string();
    let alphabet = "ab".to_string();
    alphabet.to_morse_code();
    alphabet.to_uppercase().to_morse_code();
}