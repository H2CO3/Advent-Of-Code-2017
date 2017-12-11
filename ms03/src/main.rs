// (2k)^2     is at (-k + 1, +k)
// (2k + 1)^2 is at (+k,     -k)
// k-th perimeter contains [(2k - 1)^2 + 1, (2k + 1)^2]
// => (2k - 1)^2 <= n - 1 < (2k + 1)^2
// k-th perimeter starts at (k, -k + 1)
// Heads north, then west, south, and east for 2k steps each:
// (k, [-k + 1..k + 1))
// ()
// Therefore, it's 8k long.
use std::isize;

const INPUT: isize = 361527;

fn part_1(n: isize) -> isize {
    assert!(n > 0); // there's no 0 in the list

    if n == 1 {
        return 0;
    }

    let mut k = 1;

    loop {
        let k_n_1 = 2 * k - 1;
        let k_p_1 = 2 * k + 1;

        if k_n_1 * k_n_1 + 1 <= n && n <= k_p_1 * k_p_1 {
            break;
        }

        k += 1;
    }

    let base = (2 * k - 1) * (2 * k - 1) + 1;
    let i = n - base; // index within the perimeter

    let coord = if 0 * k <= i && i < 2 * k {
        // right vertical
        i - 0 * k - k + 1
    } else if 2 * k <= i && i < 4 * k {
        // upper horizontal
        k - 1 - (i - 2 * k)
    } else if 4 * k <= i && i < 6 * k {
        // left vertical
        k - 1 - (i - 4 * k)
    } else if 6 * k <= i && i < 8 * k {
        // lower horizontal
        i - 6 * k - k + 1
    } else {
        unreachable!()
    };

    // the other coordinate is always k, in either dimension
    k + isize::abs(coord)
}

fn main() {
    println!("part 1: {}", part_1(INPUT));
}
