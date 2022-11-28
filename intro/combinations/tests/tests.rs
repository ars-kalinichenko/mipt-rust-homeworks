use combinations::combinations;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[test]
fn simple() {
    assert_eq!(combinations(&[1, 2, 3], 0), vec![vec![]]);
    assert_eq!(combinations(&[], 42), Vec::<Vec<i32>>::new());
    assert_eq!(combinations(&[], 0), vec![vec![]]);
    assert_eq!(combinations(&[1], 1), vec![vec![1]]);
    assert_eq!(
        combinations(&[1, 2, 3], 2),
        vec![vec![1, 2], vec![1, 3], vec![2, 3]]
    );
    assert_eq!(
        combinations(&[1, 2, 3, 4, 5], 3),
        vec![
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 2, 5],
            vec![1, 3, 4],
            vec![1, 3, 5],
            vec![1, 4, 5],
            vec![2, 3, 4],
            vec![2, 3, 5],
            vec![2, 4, 5],
            vec![3, 4, 5],
        ]
    );
}

#[test]
fn small_unsorted() {
    assert_eq!(
        combinations(&[1, 1488, 42, 228, 7], 3),
        vec![
            vec![1, 1488, 42],
            vec![1, 1488, 228],
            vec![1, 1488, 7],
            vec![1, 42, 228],
            vec![1, 42, 7],
            vec![1, 228, 7],
            vec![1488, 42, 228],
            vec![1488, 42, 7],
            vec![1488, 228, 7],
            vec![42, 228, 7],
        ]
    );
}

#[test]
fn bigger_unsorted() {
    assert_eq!(
        combinations(&[8, 7, 10, 1, 3, 2, 9, 5, 4, 6], 8),
        vec![
            vec![8, 7, 10, 1, 3, 2, 9, 5],
            vec![8, 7, 10, 1, 3, 2, 9, 4],
            vec![8, 7, 10, 1, 3, 2, 9, 6],
            vec![8, 7, 10, 1, 3, 2, 5, 4],
            vec![8, 7, 10, 1, 3, 2, 5, 6],
            vec![8, 7, 10, 1, 3, 2, 4, 6],
            vec![8, 7, 10, 1, 3, 9, 5, 4],
            vec![8, 7, 10, 1, 3, 9, 5, 6],
            vec![8, 7, 10, 1, 3, 9, 4, 6],
            vec![8, 7, 10, 1, 3, 5, 4, 6],
            vec![8, 7, 10, 1, 2, 9, 5, 4],
            vec![8, 7, 10, 1, 2, 9, 5, 6],
            vec![8, 7, 10, 1, 2, 9, 4, 6],
            vec![8, 7, 10, 1, 2, 5, 4, 6],
            vec![8, 7, 10, 1, 9, 5, 4, 6],
            vec![8, 7, 10, 3, 2, 9, 5, 4],
            vec![8, 7, 10, 3, 2, 9, 5, 6],
            vec![8, 7, 10, 3, 2, 9, 4, 6],
            vec![8, 7, 10, 3, 2, 5, 4, 6],
            vec![8, 7, 10, 3, 9, 5, 4, 6],
            vec![8, 7, 10, 2, 9, 5, 4, 6],
            vec![8, 7, 1, 3, 2, 9, 5, 4],
            vec![8, 7, 1, 3, 2, 9, 5, 6],
            vec![8, 7, 1, 3, 2, 9, 4, 6],
            vec![8, 7, 1, 3, 2, 5, 4, 6],
            vec![8, 7, 1, 3, 9, 5, 4, 6],
            vec![8, 7, 1, 2, 9, 5, 4, 6],
            vec![8, 7, 3, 2, 9, 5, 4, 6],
            vec![8, 10, 1, 3, 2, 9, 5, 4],
            vec![8, 10, 1, 3, 2, 9, 5, 6],
            vec![8, 10, 1, 3, 2, 9, 4, 6],
            vec![8, 10, 1, 3, 2, 5, 4, 6],
            vec![8, 10, 1, 3, 9, 5, 4, 6],
            vec![8, 10, 1, 2, 9, 5, 4, 6],
            vec![8, 10, 3, 2, 9, 5, 4, 6],
            vec![8, 1, 3, 2, 9, 5, 4, 6],
            vec![7, 10, 1, 3, 2, 9, 5, 4],
            vec![7, 10, 1, 3, 2, 9, 5, 6],
            vec![7, 10, 1, 3, 2, 9, 4, 6],
            vec![7, 10, 1, 3, 2, 5, 4, 6],
            vec![7, 10, 1, 3, 9, 5, 4, 6],
            vec![7, 10, 1, 2, 9, 5, 4, 6],
            vec![7, 10, 3, 2, 9, 5, 4, 6],
            vec![7, 1, 3, 2, 9, 5, 4, 6],
            vec![10, 1, 3, 2, 9, 5, 4, 6],
        ]
    );
}

#[test]
fn rand_five() {
    let mut rng = thread_rng();
    let mut arr: Vec<_> = (1..=20).collect();
    arr.shuffle(&mut rng);
    let mut comb = vec![];
    for a in 0..arr.len() {
        for b in (a + 1)..arr.len() {
            for c in (b + 1)..arr.len() {
                for d in (c + 1)..arr.len() {
                    for e in (d + 1)..arr.len() {
                        comb.push(vec![arr[a], arr[b], arr[c], arr[d], arr[e]]);
                    }
                }
            }
        }
    }
    assert_eq!(combinations(arr.as_slice(), 5), comb);
}
