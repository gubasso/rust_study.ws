fn answers_query(nums: Vec<i32>, queries: Vec<Vec<i32>>, limit: i32) -> Vec<bool> {
    let mut prefix = vec![nums[0]];
    for i in 1..nums.len() {
        prefix.push(prefix[prefix.len()-1] + nums[i]);
    }

    let mut answers = vec![];
    for q in queries {
        let (i, j) = (q[0] as usize, q[1] as usize);
        let sum = prefix[j] - prefix[i] + nums[i];
        answers.push(sum < limit);
    }

    answers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let result = answers_query(vec![1, 6, 3, 2, 7, 2], vec![[0, 3].to_vec(), [2, 5].to_vec(), [2, 4].to_vec()], 13);
        assert_eq!(result, vec![true, false, true]);
    }
}
