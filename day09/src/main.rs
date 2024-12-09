use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq)]
enum Block {
    Empty,
    Used(usize),
}

#[derive(Debug, Clone)]
struct DiskMap {
    blocks: Vec<Block>,
}

impl DiskMap {
    fn from_dense(dense: String) -> DiskMap {
        let mut blocks = Vec::new();
        dense.chars().enumerate().for_each(|(i, c)| {
            let length: usize = c.to_digit(10).unwrap() as usize;
            (0..length).for_each(|_c| {
                blocks.push(if i % 2 == 0 {
                    Block::Used(i / 2)
                } else {
                    Block::Empty
                });
            });
        });
        DiskMap { blocks }
    }

    // For the A solution, we walk the blocks from left to right until we find an empty block and thenwe walk from right
    // to left until we find a used block. Then we swap the two. We continue this process until the two pointers meet.
    fn optimize_a(&mut self) {
        let mut free_index = 0;
        let mut used_index = self.blocks.len() - 1;

        while free_index < used_index {
            match self.blocks[free_index] {
                Block::Used(_) => {
                    free_index += 1;
                }
                Block::Empty => match self.blocks[used_index] {
                    Block::Empty => {
                        used_index -= 1;
                    }
                    Block::Used(_) => {
                        self.blocks.swap(free_index, used_index);
                        free_index += 1;
                        used_index -= 1;
                    }
                },
            }
        }
    }

    // So in part B we have to iterate in reverse over each block exactly once. We do this by iterating in reverse over
    // the block id's, finding the block and it's size, then finding the first available empty space of at least that
    // size and swap the two, but only if the first available empty space is to the left of the block.
    fn optimize_b(&mut self) {
        for id in (0..(self.get_highest_id() + 1)).rev() {
            let (index, size) = self.get_last_block_with_id(id);
            if let Some(free_index) = self.get_first_free_n_blocks(size) {
                if free_index < index {
                    for i in 0..size {
                        self.blocks.swap(index + i, free_index + i);
                    }
                }
            }
        }
    }

    fn get_last_block_with_id(&self, id: usize) -> (usize, usize) {
        let mut block_end = self.blocks.len();
        let mut count = 0;

        while block_end > 0 {
            block_end -= 1;

            match self.blocks[block_end] {
                Block::Used(block_id) if block_id == id => {
                    count += 1;
                }
                Block::Used(_) | Block::Empty if count == 0 => continue,
                _ => break,
            }
        }

        (block_end + 1, count)
    }

    fn get_first_free_n_blocks(&self, n: usize) -> Option<usize> {
        let mut block_index = 0;
        let mut count = 0;
        while block_index < self.blocks.len() {
            match self.blocks[block_index] {
                Block::Used(_) => {
                    block_index += 1;
                    count = 0;
                }
                Block::Empty => {
                    block_index += 1;
                    count += 1;
                }
            }

            // index now points to the end of the count free blocks so don't forget to subtract count. Because if you
            // forget you'll spend an hour debugging.
            if count == n {
                return Some(block_index - count);
            }
        }
        None
    }

    fn get_highest_id(&self) -> usize {
        self.blocks
            .iter()
            .rev()
            .find_map(|block| {
                if let Block::Used(block_id) = block {
                    Some(*block_id)
                } else {
                    None
                }
            })
            .unwrap()
    }

    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .fold(0, |acc, (index, block)| match block {
                Block::Empty => acc,
                Block::Used(id) => acc + index * id,
            })
    }
}

fn main() {
    // As the AoC progresses I expect to have more and more unwraps creeping in
    let mut disk_map_a = DiskMap::from_dense(read_to_string("input").unwrap());

    let mut disk_map_b = disk_map_a.clone();
    disk_map_a.optimize_a();
    println!("Part A: {}", disk_map_a.checksum());

    disk_map_b.optimize_b();
    println!("Part B: {}", disk_map_b.checksum());
}
