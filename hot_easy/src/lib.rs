pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Solution;

impl Solution {
    // 柠檬水找零
    pub fn lemon_change(bills: Vec<i32>) -> bool {
        let mut five = 0;
        let mut ten = 0;
        for bill in bills {
            if bill == 5 {
                five += 1;
            } else if bill == 10 {
                if five >= 1 {
                    five -= 1;
                    ten += 1
                } else {
                    return false;
                }
            } else {
                if ten >= 1 && five >= 1 {
                    ten -= 1;
                    five -= 1;
                } else if five >= 3 {
                    five -= 3;
                } else {
                    return false;
                }
            }
        }
        true
    }

    pub fn find_disappear_numbers(nums: Vec<i32>) -> Vec<i32> {
        // 消失的数组  原地哈希
        let mut ans = vec![];
        let len = nums.len();
        let mut arr = nums.clone();

        for i in 0..len {
            let cur = arr[i].abs() - 1;
            if arr[cur as usize] > 0 {
                arr[cur as usize] *= -1;
            }
            for i in 0..len {
                if arr[i] > 0 {
                    ans.push(i as i32 + 1);
                }
            }
        }
        ans
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
