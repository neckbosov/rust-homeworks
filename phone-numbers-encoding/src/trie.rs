use std::collections::VecDeque;
use std::ptr::NonNull;

struct TrieNode<'a> {
    terminators: Vec<&'a str>,
    len: usize,
    next: [TrieLink<'a>; 10],
    s_link: TrieLink<'a>,
    ts_link: TrieLink<'a>,
}

type TrieLink<'a> = Option<NonNull<TrieNode<'a>>>;

impl<'a> TrieNode<'a> {
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

pub struct Trie<'a> {
    root: TrieLink<'a>,
}


impl<'a> Trie<'a> {
    pub fn new() -> Self {
        let root_node = Box::new(TrieNode::new());
        let root_node = Box::into_raw(root_node);
        Self {
            root: NonNull::new(root_node),
        }
    }
    pub fn add_seq(&mut self, digits: &[usize], word: &'a str) {
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
    fn get_terminators_recursive(&self, cur_node: TrieLink<'a>) -> Vec<(usize, &[&'a str])> {
        let mut cur_node = cur_node.unwrap().as_ptr();
        let mut buf = Vec::new();
        unsafe {
            if (*cur_node).is_terminal() {
                buf.push(((*cur_node).len, (*cur_node).terminators.as_slice()))
            }
        }
        while let Some(node) = unsafe { (*cur_node).ts_link } {
            let node = node.as_ptr();
            buf.push(unsafe { ((*node).len, (*node).terminators.as_slice()) });
            cur_node = node;
        }
        buf
    }
    fn next_node(&self, cur_node: TrieLink<'a>, symbol: usize) -> TrieLink<'a> {
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
    pub fn find_all_occurrences(&self, seq: &[usize]) -> Vec<Vec<(usize, &[&'a str])>> {
        let mut res = Vec::with_capacity(seq.len());
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

impl<'a> Drop for Trie<'a> {
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
        trie.add_seq(&buf[0..2], "kek");
        trie.add_seq(&buf[0..3], "mem");
        trie.add_seq(&buf[1..3], "lol");
        trie.build();
        let occurrences = trie.find_all_occurrences(&buf);
        assert_eq!(occurrences, vec![
            vec![],
            vec![(2, vec!["kek"].as_slice())],
            vec![(3, vec!["mem"].as_slice()), (2, vec!["lol"].as_slice())],
            vec![],
            vec![(2, vec!["kek"].as_slice())],
            vec![],
        ]);
    }
}