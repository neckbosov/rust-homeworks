use crate::trie::Trie;
use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Stdout, Write};

mod trie;

fn convert_encoding(digits_coding: &HashMap<usize, Vec<char>>) -> HashMap<char, usize> {
    let mut chars_coding = HashMap::new();
    for (k, v) in digits_coding {
        for symbol in v {
            chars_coding.insert(*symbol, *k);
        }
    }
    chars_coding
}

fn convert_string(s: &str, chars_encoding: &HashMap<char, usize>) -> Vec<usize> {
    let mut res = Vec::new();
    for c in s.chars() {
        if c.is_alphabetic() && c.is_ascii() {
            res.push(chars_encoding[&c.to_ascii_lowercase()]);
        }
    }
    res
}

fn read_dict(chars_encoding: HashMap<char, usize>) -> Trie {
    let file = File::open("dictionary.txt").expect("Fail to open dictionary");
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut trie = Trie::new();
    while reader.read_line(&mut buf).is_ok() {
        let digits = convert_string(&buf, &chars_encoding);
        if digits.is_empty() {
            break;
        }
        let tmp_string = buf.trim().to_string();
        trie.add_seq(&digits, tmp_string);
        buf.clear();
    }
    trie.build();
    trie
}

struct AnswerCalcState<'a> {
    source_number: String,
    digits_str: Vec<String>,
    correct_positions: Vec<[bool; 2]>,
    occurrences: Vec<Vec<(usize, &'a [String])>>,
}

fn print_answers<'b, 'a: 'b>(
    state: &'b AnswerCalcState<'a>,
    current_pos: usize,
    is_last_digit: bool,
    buf: &mut Vec<&'b str>,
    writer: &mut BufWriter<Stdout>,
) {
    if current_pos == state.digits_str.len() {
        writeln!(writer, "{}: {}", state.source_number, buf.join(" "))
            .expect("failed to write result");
        return;
    }
    if state.correct_positions[current_pos][0] {
        for (dist, words) in &state.occurrences[current_pos] {
            for word in *words {
                buf.push(word);
                print_answers(state, current_pos + dist, false, buf, writer);
                buf.pop();
            }
        }
    } else if state.correct_positions[current_pos][1] && !is_last_digit {
        buf.push(&state.digits_str[current_pos]);
        print_answers(state, current_pos + 1, true, buf, writer);
        buf.pop();
    }
}

fn process_sequence(
    trie: &Trie,
    digits: Vec<usize>,
    source_number: String,
    writer: &mut BufWriter<Stdout>,
) {
    let end_occurrences = trie.find_all_occurrences(&digits);
    debug_assert!(end_occurrences[0].is_empty());
    let mut correct_positions = vec![[false, false]; end_occurrences.len() + 1];
    let mut start_occurrences = vec![Vec::new(); end_occurrences.len()];
    for (i, occur) in end_occurrences.into_iter().enumerate() {
        for (l, s) in occur {
            start_occurrences[i - l].push((l, s));
        }
    }
    correct_positions[digits.len()][0] = true;
    for i in (0..start_occurrences.len()).rev() {
        for (word_len, _) in &start_occurrences[i] {
            if i + word_len < correct_positions.len()
                && (correct_positions[i + word_len][0] || correct_positions[i + word_len][1])
            {
                correct_positions[i][0] = true;
            }
        }
        if i + 1 < correct_positions.len()
            && !correct_positions[i][0]
            && start_occurrences[i].is_empty()
        {
            correct_positions[i][1] = true;
        }
    }
    let digits_str: Vec<_> = digits.iter().map(|d| d.to_string()).collect();
    let state = AnswerCalcState {
        source_number,
        digits_str,
        correct_positions,
        occurrences: start_occurrences,
    };
    let mut buf = Vec::new();
    print_answers(&state, 0, false, &mut buf, writer);
}

fn main() {
    let digits_coding: HashMap<usize, Vec<char>> = HashMap::from([
        (0, vec!['e']),
        (1, vec!['j', 'n', 'q']),
        (2, vec!['r', 'w', 'x']),
        (3, vec!['d', 's', 'y']),
        (4, vec!['f', 't']),
        (5, vec!['a', 'm']),
        (6, vec!['c', 'i', 'v']),
        (7, vec!['b', 'k', 'u']),
        (8, vec!['l', 'o', 'p']),
        (9, vec!['g', 'h', 'z']),
    ]);
    let chars_encoding = convert_encoding(&digits_coding);
    let trie = read_dict(chars_encoding);
    let input_file = File::open("input.txt").expect("Failed to find input file");
    let mut input_reader = BufReader::new(input_file);
    let mut buf = String::new();
    let mut output_writer = BufWriter::new(std::io::stdout());
    while input_reader.read_line(&mut buf).is_ok() {
        if buf.is_empty() {
            break;
        }
        let digits: Vec<_> = buf
            .trim()
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        process_sequence(&trie, digits, buf.trim().to_string(), &mut output_writer);
        buf.clear();
    }
}
