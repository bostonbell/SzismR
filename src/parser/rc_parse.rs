/*
 * This is a portion for parsing into a hashmap
 * our parsed rc and allowing the user to call
 * pass it back to our main caller for the user
 * to do stuff with.
*/



pub mod rc_parser{

    extern crate regex;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::Read;
    use std::process;

    pub enum ParseState{
        SOP,
        ParsePack(String),
        ParseScripts(String),
        ParseEntry(String),
        ParseScript(String),
        ParseEnd(String),
        EOP(String),
    }

    pub struct RCObject{
        tokens:  Vec<String>,
    }

    impl RCObject{
        pub fn new() -> RCObject{
                RCObject{
                tokens : Vec::new(),
            }
        }

        pub fn parse_iteration(&mut self, state: &ParseState, token : String, scope: &mut i32) -> Option<ParseState>
        {
            //A little slow, refactor as borrowed regex later.
            let p_pack = regex::Regex::new(r"(.*)\.parsePack").unwrap();
            let p_ms_entry = regex::Regex::new(r"(.*)\.parseScripts").unwrap();
            let p_s_entry = regex::Regex::new(r"(?P<script_caller>.*)\^entryPoint").unwrap();
            let p_descope = regex::Regex::new(r"(.*)End").unwrap();
            let s_entry = regex::Regex::new(r"(?P<script_entry>.*)").unwrap();
            let s_dependancy = regex::Regex::new(r"(.*)").unwrap();


            //Refactor by adding change in state here.
            match state
            {
                &ParseState::SOP =>
                {
                    if p_pack.is_match(token.as_str())
                    {
                        Some(ParseState::ParsePack(token))
                    }
                    else
                    {
                        None
                    }
                },
                &ParseState::ParsePack(ref data) =>
                {
                    if p_ms_entry.is_match(token.as_str())
                    {
                        Some(ParseState::ParseScripts(token))
                    }
                    else
                    {
                        None
                    }
                },
                &ParseState::ParseScripts(ref data) =>
                {
                    if p_s_entry.is_match(token.as_str())
                    {
                        Some(ParseState::ParseEntry(token))
                    }
                    else
                    {
                        None
                    }
                },
                &ParseState::ParseEntry(ref data) =>
                {
                    
                    if s_entry.is_match(token.as_str())
                    {
                        Some(ParseState::ParseScript(token))
                    }
                    else
                    {
                        None
                    }
                },
                &ParseState::ParseScript(ref data) =>
                {
                    if p_descope.is_match(token.as_str())
                    {
                        Some(ParseState::ParseEnd(token))
                    }
                    else if s_entry.is_match(token.as_str())
                    {
                        Some(ParseState::ParseScript(token))
                    }
                    else if s_dependancy.is_match(token.as_str())
                    {
                        Some(ParseState::ParseScript(token))
                    }
                    else
                    {
                        None
                    }
                },
                &ParseState::ParseEnd(ref data) =>
                {
                    if p_ms_entry.is_match(token.as_str())
                    {
                        Some(ParseState::ParseScripts(token))
                    }
                    else if p_pack.is_match(token.as_str())
                    {
                        Some(ParseState::ParsePack(token))
                    }
                    else if p_descope.is_match(token.as_str())
                    {
                        Some(ParseState::ParseEnd(token))
                    }
                    else 
                    {
                        Some(ParseState::EOP(token))
                    }
                        
                },
                &ParseState::EOP(ref data) =>
                {
                    if p_pack.is_match(token.as_str())
                    {
                        Some(ParseState::ParsePack(token))
                    }
                    else
                    {
                        None
                    }
                },
            }
        }

        pub fn parse(&mut self, root : &PathBuf, debug : bool) -> HashMap<String, String>
        {
            //We're gonna return a new path without destroying our old root.
            let mut string_portion = &mut String::new();
            let mut ret_hash : HashMap<String, String> = HashMap::new();

            match File::open(&root){
                Ok(mut f) => {


                    f.read_to_string(string_portion).unwrap();
                    {
                        //Parse tokens by new line.
                        let tokens : Vec<&str> = string_portion.split('\n').collect();

                                                let mut scope = 0;

                        let mut current_key = String::new();
                        let mut current_path = String::new();
                        let mut start_buffer = false;
                        let mut scope = 0;

                        let mut state: ParseState = ParseState::SOP;

                        for i in tokens
                        {
                            match self.parse_iteration(&state, i.to_string(), &mut scope)
                            {
                                Some(ParseState::ParseScripts(someScript)) =>
                                {
                                    state = ParseState::ParseScripts(someScript.clone());
                                    current_key = someScript;
                                }

                                Some(ParseState::ParseEntry(Entry)) =>
                                {
                                    state = ParseState::ParseEntry(Entry);
                                    start_buffer = true;
                                }

                                Some(ParseState::ParseScript(someScript)) =>
                                {
                                    if start_buffer == true
                                    {
                                        let p_ms_entry = regex::Regex::new(r"(.*)\.parseScripts").unwrap();
                                        start_buffer = false;
                                        current_path = someScript.clone();
                                        let mut key = current_key.clone().split_whitespace().next().unwrap().to_string();
                                        for i in 0..13
                                        {
                                            key.pop();
                                        }

                                        ret_hash.insert(key , current_path.split_whitespace().next().unwrap().to_string());
                                        state = ParseState::ParseScript(someScript);
                                    }
                                    else
                                    {
                                        state = ParseState::ParseScript(someScript);
                                    }
                                }

                                Some(ParseState::ParseEnd(someScript)) =>
                                {
                                    state = ParseState::ParseEnd(someScript);
                                }

                                Some(ParseState::ParsePack(someScript)) =>
                                {
                                    state = ParseState::ParsePack(someScript);
                                }

                                Some(someState) => {state = someState}
                                None => {panic!("Bad RC.")}
                            }
                        }

                        if debug
                        {
                            println!(" ");
                            println!("      _____PASSED RC_________");
                            println!(" ");
                        }
                    }
                },
                Err(e) => {
                     println!("Failed to open RC file: {}", e);
                }
            };
            ret_hash
        }


    }
}
