extern crate clap;
use std::collections::HashMap;
use clap::{Arg, App};

extern crate linked_hash_map;
use linked_hash_map::LinkedHashMap;

struct Writer {
    pub strings: Vec<String>,
//    is_file_output: bool
    pub file: Option<String>
}

impl Writer {

    pub fn new() -> Writer {
        return Writer{ strings: vec![], file: None }
    }

    fn push(& mut self, string: String) {
        self.strings.push(string);
    }

    fn write(& mut self) {
        for s in self.strings.clone() {
            match self.file.clone() {
                Some(x) => {
                    // TODO: output to file
                    unimplemented!();
                },
                None => println!("{}", s)
            }
        }
    }
}

fn main() {
    let matches = App::new("varconfig")
        .version("0.0.1")
        .author("Jonathan Rowlands <jonrowlands83@gmail.com>")
        .about("Configure unix configuration files with variables!")
        .args_from_usage("
            -o, --output=[FILE]   'Sets an output file'
            -t, --template=[FILE] 'Sets a template base to work from'
            -v...                 'Sets the level of verbosity'
            [INPUT]...
            ")
        .get_matches();

//    "-c, --config=[FILE] 'Sets a custom config file'
//    <INPUT>              'Sets the input file to use'


    // Gets a value for config if supplied by user, or defaults to "default.conf"
//    let config = matches.value_of("config").unwrap_or("default.conf");
//    println!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
//    println!("Using input file: {}", matches.value_of("INPUT").unwrap());


    let mut writer = Writer::new();

    if let Some(o) = matches.value_of("output") {
        writer.file = Some(o.to_string());
    }

    if let Some(kv_strings) = matches.values_of("INPUT") {
        let test = kv_strings
            .map(|kv_string| kv_string.split('='))
            .map(|mut kv| (kv.next().unwrap().into(),
                           kv.next().unwrap().into()))
            .collect::<LinkedHashMap<String, String>>();

        for (k, v) in test {
            writer.push(format!("{}       {}", k, v));
        }
    }

    writer.write();


//    // Vary the output based on how many times the user used the "verbose" flag
//    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
//    match matches.occurrences_of("v") {
//        0 => println!("No verbose info"),
//        1 => println!("Some verbose info"),
//        2 => println!("Tons of verbose info"),
//        3 | _ => println!("Don't be crazy"),
//    }
}
