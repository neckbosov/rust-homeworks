use std::collections::{VecDeque};
use std::ptr::NonNull;

struct TrieNode {
    terminators: Vec<String>,
    len: usize,
    next: [TrieLink; 10],
    s_link: TrieLink,
    ts_link: TrieLink,
}

type TrieLink = Option<NonNull<TrieNode>>;

impl TrieNode {
    fn new() -> Self {
        Self {
            terminators: Vec::new(),
            len: 0,
            next: [None; 10],
            s_link: None,
            ts_link: None,
        }
    }
    fn is_terminal(&self) -> bool {
        !self.terminators.is_empty()
    }
}

pub struct Trie {
    root: TrieLink,
}

impl Trie {
    pub fn new() -> Self {
        let root_node = Box::new(TrieNode::new());
        let root_node = Box::into_raw(root_node);
        Self {
            root: NonNull::new(root_node),
        }
    }
    pub fn add_seq(&mut self, digits: &[usize], word: String) {
        let mut cur_node = self.root.unwrap().as_ptr();
        for digit in digits {
            unsafe {
                if let Some(node) = (*cur_node).next[*digit] {
                    cur_node = node.as_ptr();
                } else {
                    let mut node = Box::into_raw(Box::new(TrieNode::new()));
                    (*node).len = (*cur_node).len + 1;
                    (*cur_node).next[*digit] = NonNull::new(node);
                    cur_node = node;
                }
            }
        }
        debug_assert_ne!(cur_node, self.root.unwrap().as_ptr());
        unsafe {
            (*cur_node).terminators.push(word);
        }
    }
    pub fn build(&mut self) {
        let mut vertex_queue = VecDeque::new();
        vertex_queue.push_back(self.root.unwrap().as_ptr());
        while let Some(node) = vertex_queue.pop_front() {
            for nx_digit in 0..10 {
                unsafe {
                    if let Some(nx_node) = (*node).next[nx_digit] {
                        let mut nx_node = nx_node.as_ptr();
                        let mut node_s_link = (*node).s_link;
                        while let Some(s_link) = node_s_link {
                            let s_link = s_link.as_ptr();
                            let s_link_nx = (*s_link).next[nx_digit];
                            if s_link_nx.is_some() {
                                (*nx_node).s_link = s_link_nx;
                                break;
                            } else {
                                node_s_link = (*s_link).s_link;
                            }
                        }
                        if (*nx_node).s_link.is_none() {
                            (*nx_node).s_link = self.root;
                        }
                        let nx_s_link = (*nx_node).s_link.unwrap().as_ptr();
                        if (*nx_s_link).is_terminal() {
                            (*nx_node).ts_link = (*nx_node).s_link;
                        } else {
                            (*nx_node).ts_link = (*nx_s_link).ts_link;
                        }
                        vertex_queue.push_back(nx_node);
                    }
                }
            }
        }
    }
    fn get_terminators_recursive(&self, cur_node: TrieLink) -> Vec<(usize, &[String])> {
        let mut cur_node = cur_node.unwrap().as_ptr();
        let mut buf = Vec::new();
        unsafe {
            if (*cur_node).is_terminal() {
                buf.push(((*cur_node).len, (*cur_node).terminators.as_slice()));
            }
        }
        while let Some(node) = unsafe { (*cur_node).ts_link } {
            let node = node.as_ptr();
            unsafe {
                buf.push(((*node).len, (*node).terminators.as_slice()));
            }
            cur_node = node;
        }
        buf
    }
    fn next_node(&self, cur_node: TrieLink, symbol: usize) -> TrieLink {
        let mut node_link = cur_node;
        while let Some(node) = node_link {
            let node = node.as_ptr();
            let nx_node = unsafe { (*node).next[symbol] };
            if nx_node.is_some() {
                return nx_node;
            } else {
                node_link = unsafe { (*node).s_link }
            }
        }
        self.root
    }
    pub fn find_all_occurrences(&self, seq: &[usize]) -> Vec<Vec<(usize, &[String])>> {
        let mut res = Vec::with_capacity(seq.len() + 1);
        res.push(Vec::new());
        let mut cur_node = self.root;
        for item in seq {
            cur_node = self.next_node(cur_node, *item);
            res.push(self.get_terminators_recursive(cur_node));
        }
        res
    }
    fn delete_links(&mut self) {
        let mut vertex_queue = VecDeque::new();
        vertex_queue.push_back(self.root.unwrap().as_ptr());
        while let Some(node) = vertex_queue.pop_front() {
            unsafe {
                (*node).s_link = None;
                (*node).ts_link = None;
            }
            for nx_digit in 0..10 {
                unsafe {
                    if let Some(nx_node) = (*node).next[nx_digit] {
                        let nx_node = nx_node.as_ptr();
                        vertex_queue.push_back(nx_node);
                    }
                }
            }
        }
    }
}

impl Drop for Trie {
    fn drop(&mut self) {
        self.delete_links();
        let mut vertex_queue = VecDeque::new();
        vertex_queue.push_back(self.root.unwrap().as_ptr());
        while let Some(node) = vertex_queue.pop_front() {
            for nx_digit in 0..10 {
                unsafe {
                    if let Some(nx_node) = (*node).next[nx_digit] {
                        let nx_node = nx_node.as_ptr();
                        vertex_queue.push_back(nx_node);
                    }
                }
            }
            unsafe {
                Box::from_raw(node);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::trie::Trie;

    #[test]
    fn test_simple_find_occurrences() {
        let mut trie = Trie::new();
        let buf = vec![1, 2, 3, 1, 2, 4];
        trie.add_seq(&buf[0..2], "kek".to_string());
        trie.add_seq(&buf[0..3], "mem".to_string());
        trie.add_seq(&buf[1..3], "lol".to_string());
        trie.build();
        let occurrences = trie.find_all_occurrences(&buf);

        assert_eq!(
            occurrences,
            vec![
                Vec::new(),
                Vec::new(),
                Vec::from([(2, vec!["kek".to_string()].as_slice())]),
                Vec::from([
                    (3, vec!["mem".to_string()].as_slice()),
                    (2, vec!["lol".to_string()].as_slice())
                ]),
                Vec::from([]),
                Vec::from([(2, vec!["kek".to_string()].as_slice())]),
                Vec::from([]),
            ]
        );
    }

    #[test]
    fn test_another() {
        let mut trie = Trie::new();

        let buf = vec![5, 6, 2, 4, 8, 2];
        trie.add_seq(&buf[0..3], "mix".to_string());
        trie.add_seq(&buf[3..], "Tor".to_string());
        trie.build();
        let occurrences = trie.find_all_occurrences(&buf);
        assert_eq!(
            occurrences,
            vec![
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::from([(3, vec!["mix".to_string()].as_slice())]),
                Vec::new(),
                Vec::new(),
                Vec::from([(3, vec!["Tor".to_string()].as_slice())]),
            ]
        );
    }
}
