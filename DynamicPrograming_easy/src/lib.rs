use std::{cmp::max, ops::Range};

struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        /*
        输入：n = 3
        输出：3
        解释：有三种方法可以爬到楼顶。
        1. 1 阶 + 1 阶 + 1 阶
        2. 1 阶 + 2 阶
        3. 2 阶 + 1 阶
        dp[n]=dp[n−1]+dp[n−2]
        f(x) = f(x - 1) + f(x - 2)
        */
        let nsize = n as usize;
        let mut dp = vec![0; nsize];
        dp[0] = 1;
        dp[1] = 2;
        let mut i: usize = 2;
        while i < nsize {
            dp[i] = dp[i-2] + dp[i-1];
        }
        dp[nsize - 1]

    }

    pub fn three_steps(n: i32) -> i32 {
        /*
        输入：n = 3 
        输出：4
        说明: 有四种走法
        */
        let nsize = n as usize;
        let mut dp = vec![0; nsize];
        dp[0] = 1;
        dp[1] = 2;
        dp[2] = 4;
        let mut i: usize = 2;
        while i < nsize {
            dp[i] = dp[i-3] + dp[i-2] + dp[i-1];
        }
        dp[nsize-1]
    }

    pub fn max_sub_array(mut nums: Vec<u32>) -> u32 {
        /* 连续数列
        输入： [-2,1,-3,4,-1,2,1,-5,4]
        输出： 6
        解释： 连续子数组 [4,-1,2,1] 的和最大，为 6。
        */

        for i in 1..nums.len() {
            nums[i] += max(nums[i-1], 0)
        }
        *nums.iter().max().unwrap()
    }
}
