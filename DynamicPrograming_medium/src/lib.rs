use std::cmp::max;
use std::cmp::min;

struct Solution{}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        /*
        输入：nums = [10,9,2,5,3,7,101,18]
        输出：4
        解释：最长递增子序列是 [2,3,7,101]，因此长度为 4 。
        */
        if nums.len() <= 1{
            return nums.len() as i32;
        }

        let mut dp = vec![1; nums.len()];
        // dp[i]代表到i为止最长的递增子序列长度
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = max(dp[j]+1, dp[i]);
                }
            }
        }
        let mut res = dp[0];
        for i in dp {
            res = max(res, i)
        }
        res
    }

    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        /*给定一个未排序的整数数组 nums ， 返回最长递增子序列的个数 。
        输入: [1,3,5,4,7]
        输出: 2
        解释: 有两个最长递增子序列，分别是 [1, 3, 4, 7] 和[1, 3, 5, 7]。
        状态转移方程： dp[i]=max(dp[j])+1,其中0≤j<i且num[j]<num[i]
        */
        let n = nums.len();
        let mut max = -1; // nums的最长上升子序列的长度
        let mut dp = vec![1;n]; // 以nums[i]结尾的最长上升子序列的长度
        let mut count = vec![1;n]; // 以nums[i]结尾的最长上升子序列的个数
        // 答案为所有满足dp[i]=max的i所对应的count[i]之和
        for i in 0..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    if dp[i] < dp[j] + 1 {
                        dp[i] = dp[j] + 1;
                        count[i] = count[j];
                    } else if dp[i] == dp[j] + 1 {
                        count[i] += count[j];
                    }
                }
            }
            if max < dp[i] {
                max = dp[i]
            }
        }
        let mut res = 0;
        for i in 0..n {
            if dp[i] == max {
                res += count[i];
            }
        }
        res
    }

    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        // 输入：grid = [[1,3,1],[1,5,1],[4,2,1]]
        // 输出：7
        // 解释：路径 1→3→1→1→1 的总和最小。
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![1; n]; m]; // dp代表到m*n坐标时候的最小路径和
        dp[0][0] = grid[0][0];

        for i in 1..m{
            dp[i][0] = grid[i][0] + dp[i-1][0]
        }
        for j in 1..n {
            dp[0][j] = grid[0][j] + dp[0][j-1]
        }
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = min(dp[i-1][j], dp[i][j-1]) + grid[i][j]
            }
        }
        println!("{:?}", dp);
        dp[m-1][n-1]
    }

    pub fn longest_palindrome(s: String) -> String {
        "s".to_string()
    }
}


