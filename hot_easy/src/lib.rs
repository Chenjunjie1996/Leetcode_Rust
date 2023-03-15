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

    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        /*
        输入：nums = [2,6,4,8,10,9,15]
        输出：5
        解释：只需要对 [6, 4, 8, 10, 9] 进行升序排序，那么整个表都会变为升序排序。
        数组分为3段，左右为升序，找中段的左右边界 中段最小值大于左段的最大值，最大值小于右段的最小值。
        */
        let n = nums.len();
        let (mut begin, mut end) = (-1, -1);
        let (mut min, mut max) = (i32::MIN, i32::MAX);

        for i in 0..n {
            if max > nums[i] { //从左到右维持最大值，寻找右边界end
                end = i as i32;
            } else {
                max = nums[i];
            }

            if min < nums[n-i-1] { //从右到左维持最小值，寻找左边界begin
                begin = n as i32 - i as i32 - 1;
            } else {
                min = nums[n-i-1];
            }
        }

        if end == -1 {
            0
        } else {
            end - begin + 1
        }
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
