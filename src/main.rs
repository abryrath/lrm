#[macro_use]
extern crate clap;
extern crate lipsum;
use clap::App;
use lipsum::lipsum_words;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut word_count: usize;
    let mut char_count: usize = 0;
    let mut content;
    if matches.is_present("words") {
        word_count = matches.value_of("words").unwrap().parse().unwrap();
    }  else {
        word_count = 50;
    }

    if matches.is_present("chars") {
        // Generate chars/3 words and then cut it off at the number of chars specified.
        char_count = matches.value_of("chars").unwrap().parse().unwrap();
        word_count = char_count.checked_div(3).unwrap();
    }

    content = lipsum_words(word_count);
    if char_count > 0 {
        content = (&content[..char_count]).to_string();
    }

    if matches.is_present("breaks") {
        let mut break_width: usize = 80;
        if matches.is_present("break_width") {
            break_width = matches.value_of("break_width").unwrap().parse().unwrap();
        }
        let mut i = 0;
        let mut bound: usize = break_width;
        while bound <= content.len() {
            if !content.is_char_boundary(bound) {
                bound -= 1;
                continue;
            }
            content.insert_str((i+1)*break_width, "\n");
            i += 1;
            bound = (i+1) * break_width;
        }
    }
    println!("{}", content);
}
