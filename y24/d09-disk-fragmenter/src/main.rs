use aoc::{input_str, stringstuff::CharExt, time};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty,
    File(usize),
}

impl Block {
    pub fn is_empty(&self) -> bool {
        matches!(self, Block::Empty)
    }

    pub fn file_id(self) -> Option<usize> {
        match self {
            Block::File(id) => Some(id),
            _ => None,
        }
    }
}

fn part1(input: &str) -> usize {
    let mut file_id = 0;
    let mut is_file = true;
    let mut blocks = vec![];

    for len in input.trim().chars().map(|b| b.digit_to_num()) {
        if is_file {
            blocks.extend((0..len).map(|_| Block::File(file_id)));
            file_id += 1
        } else {
            blocks.extend((0..len).map(|_| Block::Empty));
        }
        is_file = !is_file
    }

    // 2 pointer solution
    let mut start = 0;
    let mut end = blocks.len() - 1;

    loop {
        // move the end pointer to a block
        while blocks[end].is_empty() {
            end -= 1;
        }

        // move the start point until it hits empty space
        while !blocks[start].is_empty() {
            start += 1;
        }

        if start >= end {
            break;
        }

        blocks.swap(start, end);
    }

    #[cfg(debug_assertions)]
    {
        let mut blocks = blocks.iter().copied();
        while let Some(Block::File(_)) = blocks.next() {}
        debug_assert!(blocks.all(|b| b == Block::Empty));
    }

    // checksum
    blocks
        .into_iter()
        .take_while(|b| !b.is_empty())
        .enumerate()
        .map(|(idx, block)| block.file_id().unwrap() * idx)
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Page {
    // length
    Empty(usize),
    // length, id
    File(usize, usize),
}

impl Page {
    pub fn is_empty_page(&self) -> bool {
        matches!(self, Page::Empty(_))
    }

    pub fn shrink_empty_space(&mut self, by: usize) {
        match self {
            Page::Empty(len) => *len -= by,
            Page::File(_, _) => panic!("shrink_empty_space requires an empty page"),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Page::Empty(len) => *len,
            Page::File(len, _) => *len,
        }
    }
}

fn part2(input: &str) -> usize {
    let mut file_id = 0;
    let mut is_file = true;
    let mut pages = vec![];

    for len in input.trim().chars().map(|b| b.digit_to_num()) {
        if is_file {
            pages.push(Page::File(len, file_id));
            file_id += 1
        } else {
            pages.push(Page::Empty(len));
        }
        is_file = !is_file
    }

    let mut end = pages.len() - 1;

    loop {
        #[cfg(debug_assertions)]
        {
            let _debug = pages
                .iter()
                .copied()
                .flat_map(|p| match p {
                    Page::Empty(len) => (0..len).map(|_| Block::Empty).collect::<Vec<_>>(),
                    Page::File(len, id) => (0..len).map(|_| Block::File(id)).collect::<Vec<_>>(),
                })
                .map(|b| match b {
                    Block::Empty => String::from("."),
                    Block::File(id) => id.to_string(),
                })
                .collect::<String>();
            println!("{:?}", _debug);
        }

        // move the end pointer to a block
        while end > 0 && pages[end].is_empty_page() {
            end -= 1;
        }
        if end == 0 {
            break;
        }
        let end_page = pages[end];

        for start in 0..end {
            let start_page = pages[start];
            if start_page.is_empty_page() && start_page.len() >= end_page.len() {
                pages.insert(start, end_page);
                debug_assert_eq!(pages[start + 1], start_page);
                pages[start + 1].shrink_empty_space(end_page.len());
                pages[end + 1] = Page::Empty(end_page.len());
                break;
            }
        }
        end -= 1;
    }

    pages
        .into_iter()
        .flat_map(|p| match p {
            Page::Empty(len) => (0..len).map(|_| Block::Empty).collect::<Vec<_>>(),
            Page::File(len, id) => (0..len).map(|_| Block::File(id)).collect::<Vec<_>>(),
        })
        .enumerate()
        .flat_map(|(idx, block)| block.file_id().map(|id| id * idx))
        .sum()
}

fn main() {
    let input = input_str!(2024, 9);

    let part1 = time("Part 1", || part1(input));
    println!("Part 1: {}", part1);

    let part2 = time("Part 2", || part2(input));
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn example() {
        let input = "2333133121414131402";
        assert_eq!(part1(input), 1928);
        assert_eq!(part2(input), 2858);
    }
}
