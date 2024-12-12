use super::{utils::read_disk_map_input, DiskChecksum, DiskMap};

pub fn disk_fragmenter(input: &str) -> DiskChecksum {
    let disk_map = read_disk_map_input(input);

    let organized_disk_map = organize_disk_map(disk_map);

    calc_disk_check_sum(&organized_disk_map)
}

pub fn organize_disk_map(mut disk_map: DiskMap) -> DiskMap {
    let mut end_index: usize = disk_map.len() - 1;
    let mut start_index: usize = 0;

    'outer: loop {
        if end_index <= start_index || start_index > disk_map.len() - 1 {
            break 'outer;
        }

        if let None = disk_map[start_index] {
            'inner: loop {
                if end_index <= start_index {
                    break 'inner;
                }
                if let Some(_) = disk_map[end_index] {
                    disk_map.swap(start_index, end_index);
                    break 'inner;
                }
                end_index -= 1;
            }
        }

        start_index += 1;
    }

    disk_map
}

pub fn calc_disk_check_sum(disk_map: &DiskMap) -> DiskChecksum {
    let mut checksum: DiskChecksum = 0;

    for (block_index, block) in disk_map.iter().enumerate() {
        if let Some(id) = block {
            checksum += *id as DiskChecksum * block_index as DiskChecksum;
        } else {
            break;
        }
    }

    checksum as DiskChecksum
}
