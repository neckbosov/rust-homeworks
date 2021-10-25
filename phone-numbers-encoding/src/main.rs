use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Stdout, Write};
use crate::trie::Trie;

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
        let mut tmp_string = buf.trim().to_string();
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
    parent_distances: Vec<Vec<usize>>,
    occurrences: Vec<BTreeMap<usize, &'a [String]>>,
}

fn print_answers<'b, 'a: 'b>(
    state: &'b AnswerCalcState<'a>,
    current_pos: usize,
    is_last_digit: bool,
    buf: &mut Vec<&'b str>,
    writer: &mut BufWriter<Stdout>,
) {
    if current_pos == 0 {
        let mut new_buf = buf.clone();
        new_buf.reverse();
        writeln!(writer, "{}: {}", state.source_number, new_buf.join(" ")).expect("failed to write result");
        return;
    }
    if state.correct_positions[current_pos][1] && !is_last_digit {
        buf.push(&state.digits_str[current_pos - 1]);
        print_answers(state, current_pos - 1, true, buf, writer);
        buf.pop();
    }
    if state.correct_positions[current_pos][0] {
        for dist in &state.parent_distances[current_pos] {
            debug_assert_ne!(*dist, 0);
            for word in state.occurrences[current_pos][dist] {
                buf.push(word);
                print_answers(state, current_pos - dist, false, buf, writer);
                buf.pop();
            }
        }
    }
}

fn process_sequence(trie: &Trie, digits: Vec<usize>, source_number: String, writer: &mut BufWriter<Stdout>) {
    let occurrences = trie.find_all_occurrences(&digits);
    debug_assert!(occurrences[0].is_empty());
    let mut correct_positions = vec![[false, false]; occurrences.len()];
    correct_positions[0][0] = true;
    let mut parent_distances = vec![vec![]; occurrences.len()];
    for i in 1..correct_positions.len() {
        for &word_len in occurrences[i].keys() {
            if word_len <= i && (correct_positions[i - word_len][0] || correct_positions[i - word_len][1]) {
                correct_positions[i][0] = true;
                parent_distances[i].push(word_len);
            }
        }
        if i > 0 && correct_positions[i - 1][0] {
            correct_positions[i][1] = true;
        }
    }
    let digits_str: Vec<_> = digits.iter().map(|d| d.to_string()).collect();
    let state = AnswerCalcState {
        source_number,
        digits_str,
        correct_positions,
        parent_distances,
        occurrences,
    };
    let mut buf = Vec::new();
    print_answers(&state, digits.len(), false, &mut buf, writer);
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
        (9, vec!['g', 'h', 'z'])
    ]);
    let chars_encoding = convert_encoding(&digits_coding);
    let trie = read_dict(chars_encoding);
    // dbg!("Finished building");
    // dbg!(&trie);
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
