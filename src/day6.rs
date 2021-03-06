use {
    crate::utils::DayResult,
    std::{
        collections::VecDeque,
        str::{FromStr, Lines},
    },
};

pub(crate) fn main(mut stdin: Lines) -> DayResult {
    let mut deq: VecDeque<u64> = VecDeque::from([0; 9]);
    for n in stdin.next().ok_or("Missing input")?.split(',') {
        let i = usize::from_str(n).map_err(|x| x.to_string())?;
        if i > 8 {
            return Err(format!("Unexpected number: {}", i));
        }
        deq[i] += 1
    }
    const N1: i32 = 80;
    for _ in 0..N1 {
        let births = deq.pop_front().unwrap();
        deq.push_back(births);
        deq[6] += births;
    }
    let res1: u64 = deq.iter().sum();

    for _ in N1..256 {
        let births = deq.pop_front().unwrap();
        deq.push_back(births);
        deq[6] += births;
    }
    let res2: u64 = deq.iter().sum();

    Ok((Ok(res1.to_string()), Ok(res2.to_string())))
}
