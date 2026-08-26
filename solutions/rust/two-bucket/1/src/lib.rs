use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut history: HashSet<_> = HashSet::<(u8, u8)>::new();
    if goal > capacity_1 && goal > capacity_2 {
        return None;
    }

    let mut moves = 1;
    let mut curr_1 = if start_bucket == &Bucket::One {
        capacity_1
    } else {
        0
    };
    let mut curr_2 = if start_bucket == &Bucket::Two {
        capacity_2
    } else {
        0
    };

    if capacity_1 == goal {
        return Some(BucketStats {
            moves: if curr_1 == 0 { 2 } else { 1 },
            goal_bucket: Bucket::One,
            other_bucket: curr_2,
        });
    }

    if capacity_2 == goal {
        return Some(BucketStats {
            moves: if curr_2 == 0 { 2 } else { 1 },
            goal_bucket: Bucket::Two,
            other_bucket: curr_1,
        });
    }

    let start = (curr_1, curr_2);
    // possible action
    // 1. pour to other one
    // 2. empty one
    // 3. full fill one

    history.insert((curr_1, curr_2));
    loop {
        if curr_1 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: curr_2,
            });
        }
        if curr_2 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: curr_1,
            });
        }
        if start_bucket == &Bucket::One {
            if curr_1 == 0 {
                // big is empty
                curr_1 = capacity_1;
                moves += 1;
            }
            if curr_2 == capacity_2 {
                // small full
                curr_2 = 0; // empty small
                moves += 1;
            }
            // pour big to small
            let big = if curr_1 + curr_2 > capacity_2 {
                curr_1 - (capacity_2 - curr_2)
            } else {
                0
            };
            let small = if curr_1 + curr_2 > capacity_2 {
                capacity_2
            } else {
                curr_1 + curr_2
            };
            curr_1 = big;
            curr_2 = small;
            moves += 1;
            if !history.insert((curr_1, curr_2)) {
                return None;
            }
        } else {
            if curr_2 == 0 {
                // big is empty
                curr_2 = capacity_2;
                moves += 1;
            }
            if curr_1 == capacity_1 {
                // small full
                curr_1 = 0; // empty small
                moves += 1;
            }
            // pour big to small
            let big = if curr_1 + curr_2 > capacity_1 {
                curr_2 - (capacity_1 - curr_1)
            } else {
                0
            };
            let small = if curr_1 + curr_2 > capacity_1 {
                capacity_1
            } else {
                curr_1 + curr_2
            };
            curr_2 = big;
            curr_1 = small;
            moves += 1;
            if !history.insert((curr_1, curr_2)) {
                return None;
            }
        }
        if start == (curr_1, curr_2) {
            return None;
        }
    }
}
