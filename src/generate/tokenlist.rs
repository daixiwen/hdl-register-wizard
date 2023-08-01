//! List of VHDL tokens
//! 
//! Converts a unicode string to a valid vhdl token, and uses a list to make sure
//! there aren't any doubles

use std::collections::HashSet;
use std::iter::FromIterator;

/// iterator used to change any non alphanumeric character to an underscore, and remove any
/// beginning or double underscore
struct Vhdlify<'a> {
    /// true if the character from the previous iteration was an underscore, or we are at the beginning
    remove_underscore : bool,

    /// iterator from the original string
    original_iterator : std::str::Chars<'a>
}

impl <'a> Vhdlify<'a> {
    /// create a new iterator from a string
    fn new(ascii_string : &'a String) -> Self {
        Self {
            remove_underscore : true,
            original_iterator: ascii_string.chars()
        }
    }
}

impl Iterator for Vhdlify<'_> {
    type Item = char;

    /// iterator step
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(character) = self.original_iterator.next() {
                if character.is_ascii_alphanumeric() {
                    // this character is always valid
                    self.remove_underscore = false;
                    return Some(character);
                } else {
                    // only return an underscore if it's the first one (or we aren't at the beginning)
                    if self.remove_underscore {
                        continue;
                    } else {
                        self.remove_underscore = true;
                        return Some('_');
                    }
                }
            } else {
                // we reached the end of the string
                return None;
            }
        }
    }
}

/// convert a unicode string to a valid vhdl token. There are probably better ways to do this
pub fn to_vhdl_token(unicode: &str) -> String {
    // convert any special unicode character to ascii
    let ascii_string = unidecode::unidecode(unicode);

    // remove any remainder special characters (replaced by _), starting '_' and double "__"
    let mut vhdl_token : String = Vhdlify::new(&ascii_string).collect();

    // remove any trailing _ . We know there can only be one since we removed any doubles
    if vhdl_token.ends_with('_') {
        vhdl_token.pop();
    }

    // if the string starts with a digit (or if it's empty, but at that point you were
    // really looking for trouble), prefix the string with an 'x'. I like 'x'
    if match vhdl_token.chars().next() {
        None => true,
        Some(x) => x.is_ascii_digit()
    } {
        vhdl_token.insert(0,'x');
    }

    vhdl_token
}

/// Holds a list of tokens, including the reserved words, to be sure to generate valid vhdl identifiers
pub struct TokenList {
    list : HashSet<String>
}

impl TokenList {
    /// Create a new token list, filled with the VHDL reserved words
    pub fn new() -> Self {
        Self {
            list : HashSet::from_iter([
                "abs", "after", "alias", "all", "and", "architecture", "array", "assert", "attribute", 
                "begin", "block", "body", "buffer", "bus", "case", "component", "configuration", "constant", 
                "disconnect", "downto", "else", "elsif", "end", "entity", "exit", "file", "for", "function", 
                "generate", "generic", "group", "guarded", "if", "impure", "in", "inertial", "inout", "is",
                "label", "library", "linkage", "literal", "loop", "map", "mod", "nand", "new", "next", 
                "not", "null", "of", "on", "open", "or", "others", "out", "package", "port", "postponed",
                "procedure", "process", "pure", "range", "record", "register", "reject", "rem", "report",
                "return", "rol", "ror", "select", "severity", "signal", "shared", "sla", "sll", "sra",
                "srl", "subtype", "then", "to", "transport", "type", "unaffected", "units", "until", "use",
                "variable", "wait", "when", "while", "with", "xnor", "xor",
                // new reserved words for VHDL 2008
                "context", "default", "force", "parameter", "release",
                // new reserved words for PSL
                "assert", "assume", "assume_guarantee", "civer", "fairness", "property", "restrict", 
                "restrict_property", "sequence", "strong", "vmode", "vprop", "vunit"
            ].iter().map(|x| x.to_string()))
        }
    }

    /// Add a token to the list. Returns Ok if the name could be added and Err if it was already on the list
    pub fn add_token(&mut self, new_token: &str) -> Result<(),()> {
        let lower_token = new_token.to_string().to_lowercase();
        
        if self.list.contains(&lower_token) {
            Err(())
        } else {
            self.list.insert(lower_token);
            Ok(())
        }
    }

    /// generate a unique token and add it to the list. The given token must include the pattern "{}"
    /// which will either be removed or replaced by an underscore and a number to provide uniqueness.
    /// panics if pattern is not present
    pub fn generate_token(&mut self, pattern : &str) -> String {
        assert!(pattern.contains("{}"));

        let mut sequence = 1;
        loop {
            let token = pattern.replace("{}", &match sequence {
                1 => "".to_owned(),
                _ => format!("_{}",sequence)
            });

            let token = to_vhdl_token(&token);

            if self.add_token(&token).is_ok() {
                return token;
            } else {
                sequence = sequence + 1;
            }
        }
    }
}

impl Default for TokenList {
    fn default() -> Self {
        TokenList::new()
    }
}
