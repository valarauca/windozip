
use std::cell::RefCell;
use super::clap::{Values,App,Arg,ArgMatches};
use super::regex::Regex;

///Create CLI interface with clap
pub fn claptrap<'a>() -> ArgMatches<'a> {
App::new("windozip")
    .author("Cody")
    .version("0.0.1")
    .about("A CLI zip utility for Windows.")
    .usage("windozip <opts> <commands>")
    .arg(Arg::with_name("extract")
        .takes_value(true)
        .multiple(false)
        .value_name("FILE")
        .short("x")
        .long("extract")
        .help("Extract data from archive"))
    .arg(Arg::with_name("inspect")
        .short("i")
        .long("inspect")
        .help("Inspects the contents of a zip")
        .takes_value(true)
        .multiple(false)
        .value_name("FILE")
        .conflicts_with("extract"))
    .arg(Arg::with_name("create")
        .takes_value(true)
        .multiple(false)
        .value_name("FILE")
        .short("c")
        .long("create")
        .help("Create a new zip file")
        .conflicts_with("extract")
        .conflicts_with("view"))
    .arg(Arg::with_name("regex")
        .takes_value(true)
        .multiple(false)
        .short("r")
        .long("regex")
        .help("Filter against a regex"))
    .arg(Arg::with_name("size")
         .takes_value(true)
         .multiple(false)
         .short("s")
         .long("sizes")
         .requires("inspect")
         .help("Display sizes when inspecting"))
    .get_matches()
}

///Define a global to store regexes
thread_local! {
    static REGEX: RefCell<Option<Vec<Regex>>> = RefCell::new(None);
    static CLEAN: Regex = Regex::new(r#"^['"]?([\s\S]*?)['"]?$"#).unwrap();
    static SIZE: RefCell<bool> = RefCell::new(false);
}

///clean regex
fn clean_str<'a>(x: &'a str) -> &'a str {
    CLEAN.with(|regex|{
        match regex.captures(x) {
            Option::None => panic!("Your regex is so bad it triggered the edge case panic."),
            Option::Some(caps) => caps.at(1).unwrap()
        }
    })
}


///build regex
fn to_regex(x: &str) -> Regex {
    match Regex::new(clean_str(x)) {
        Err(e) => {
            panic!("\n\n\nFailed to created regex on:\n{}\n{:?}\n\n",x,e);
        },
        Ok(x) => x
    }
}

///set regex
fn set_regex(v: Values) {
    REGEX.with(|cell| {
        let mut item = cell.borrow_mut();
        *item = Some(v.map(to_regex).collect());
    })
}


///set flag
fn set_size() {
    SIZE.with(|cell| {
        let mut item = cell.borrow_mut();
        *item = true;
    });
}

///display file size
pub fn display_size() -> bool {
    SIZE.with(|cell|{
        let lambda = |x: &bool| -> bool {
            *x
        };
        lambda(&cell.borrow())
    })
}


///Does our input string match the given regex?
///If there are no regexes it always returnes true
#[inline(always)]
pub fn match_regex(s: &str) -> bool {
    REGEX.with(|cell| {
        let item = cell.borrow();
        //the rust type system can be fun
        let lambda = | x: &Option<Vec<Regex>> | -> bool {
            match x {
                &Option::None => true,
                &Option::Some(ref x) => {
                    let folder = |b: bool, x: &Regex | -> bool {
                        if b {
                            b
                        } else {
                            x.is_match(s)
                        }
                    };
                    x.iter().fold(false,folder)
                }
            }
        };
        lambda(&item)
    })
}



            
///What mode the application is running in
pub enum Mode<'a> {
    Extract(Values<'a>),
    Create(Values<'a>),
    View(&'a str),
}

///Convert clap into a more usable enum
pub fn cli<'a>( args: &'a ArgMatches<'a>) -> Mode<'a> {
    //put clap's args in the proper location
    if args.is_present("size") {
        set_size();
    }
    match args.values_of("regex") {
        Option::None => { },
        Option::Some(x) => set_regex(x)
    };
    match args.values_of("extract") {
        Option::None => { },
        Option::Some(x) => return Mode::Extract(x)
    };
    match args.values_of("create") {
        Option::None => { },
        Option::Some(x) => return Mode::Create(x)
    };
    match args.value_of("inspect") {
        Option::None => { },
        Option::Some(x) => return Mode::View(x)
    };
    ::std::process::exit(0);
}
