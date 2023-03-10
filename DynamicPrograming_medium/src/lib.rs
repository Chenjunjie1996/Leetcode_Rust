use std::cmp::max;
use std::cmp::min;

pub fn valid_palindromic(s: String, mut left: usize, mut right: usize) -> bool {
    while left < right {
        if &s[left..left+1] != &s[right..right+1] {
            return false;
        }
        left+=1;
        right-=1;
    }
    true
}

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

    pub fn longest_palindrome_1(s: String) -> String {
        let len = s.len();
        if len < 2 {return s;}
        let mut maxlen = 1;
        let mut begin = 0;
        // 枚举所有长度大于 1 的子串
        for i in 0..(len-1) {
            for j in (i+1)..len {
                if (j-i+1)>maxlen && valid_palindromic(s.clone(), i, j) {
                    maxlen = j-i+1;
                    begin = i;
                }
            }
        }
        s[begin..begin+maxlen].to_string()
    }

    pub fn longest_palindrome_2(s: String) -> String {
        // 状态转移方程 dp[i][j] = (s[i] == s[j]) and dp[i + 1][j - 1]
        if s.len() == 0 {return "".to_string();}
        if s.len() == 1 {return s;}
        let mut res = &s[..1];
        let mut dp = vec![vec![false; s.len()]; s.len()];
        for i in 1..s.len() {
            for j in 0..i {
                if &s[i..i+1] != &s[j..j+1] {
                    dp[j][i] = false;
                } else {
                    if i - j < 3 {
                        dp[j][i] = true;
                    } else {
                        dp[j][i] = dp[j+1][i-1]
                    }
                }
                // 只要 dp[i][j] == true 成立，就表示子串 s[i..j] 是回文
                if dp[j][i] && i-j+1 > res.len() {
                    res = &s[j..=i]
                }
            }
        }
        println!("{:?}", res);
        res.to_string()
    }

    pub fn rob2(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {return 0;}
        if len == 1 {return nums[0];}
        // 只需单独处理第一个和最后一个房屋，中间的房屋仍然和打家劫舍1一样处理
        return max(
            Self::sub_rob2(nums[0..=len-2].to_vec()),
            Self::sub_rob2(nums[1..=len-1].to_vec()),
        )
    }
    fn sub_rob2(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {return 0;}
        if len == 1 {return nums[0];}
        let mut dp = vec![0; len];
        dp[0] = nums[0];
        dp[1] = max(nums[0], nums[1]);
        for i in 2..len {
            dp[i] = max(dp[i-2] + nums[i-1], dp[i-1])
        }
        dp[len-1]
    }

    pub fn unique_paths(m:i32, n:i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0;n];m];
        for i in 0..m {
            dp[i][0] = 1;
        }
        for j in 0..n {
            dp[0][j] = 1;
        }
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i-1][j] + dp[i][j-1];
            }
        }
        dp[m-1][n-1]
    }

    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 0 {return 0;}
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut dp = vec![vec![0; n]; m];
        for i in 0..m {
            if obstacle_grid[i][0] == 0 {
                dp[i][0] = 1;
            } else {
                break;
            }
        }
        for j in 0..n {
            if obstacle_grid[0][j] == 0 {
                dp[0][j] = 1;
            } else {
                break;
            }
        }
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = if obstacle_grid[i][j] == 0 {dp[i-1][j] + dp[i][j-1]} else {0};
            }
        }
        dp[m-1][n-1]
    }

    fn num_squares(n: i32) -> i32 {
        // 完全平方数
        // 输入 12 12=4+4+4
        // 输出 3
        let nusize = n as usize;
        let mut dp = vec![0; nusize+1];
        for i in 1..=nusize {
            dp[i] = i;
            let mut j = 1;
            while i >= j*j {
                // 找到相差一个完全平方数的最小结果+1
                dp[i] = min(dp[i], dp[i - j * j] + 1);
                j += 1;
            }
        }
        dp[nusize] as i32
    }

    fn can_jump(nums: Vec<i32>) -> bool {
        // 跳跃游戏
        let n = nums.len();
        let mut dp = vec![false; n];
        dp[0] = true;
        for i in 1..n {
            for j in 0..i {
                if dp[j] && nums[j] >= (i-j) as i32 {
                    dp[i] = true;
                } else {
                    dp[i] = false;
                }
                if dp[i] {
                    break;
                }
            }
        }
        dp[n-1]
    }

    fn can_jump2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![i32::MAX; n]; // 跳到 i 的最小步数
        dp[0] = 0;
        for i in 1..n {
            for j in 0..i {
                if nums[j] >= (i-j) as i32 {
                    dp[i] = min(dp[i], dp[j]+1)
                }
            }
        }
        dp[n-1]
    }

    fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        // 输入：nums = [1,2,3,4]
        // 输出：3
        // 解释：nums 中有三个子等差数组：[1, 2, 3]、[2, 3, 4] 和 [1,2,3,4] 自身。
        let n = nums.len();
        let mut dp = vec![false; n];
        let mut res = 0;
        for i in 0..n {
            for j in (i+1)..n {
                if j-i <= 1 {
                    dp[j] = true;
                    continue;
                }
                if dp[j-1] && nums[j]-nums[j-1] == nums[j-1]-nums[j-2] {
                    dp[j] = true;
                } else {
                    dp[j] = false;
                }
                if dp[j] && j >= 2 {
                    res += 1;
                }
            }
        }
        res
    }

    fn max_square(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![0;n];m];
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    if i==0 || j==0 {
                        dp[i][j] = 1
                    } else {
                        dp[i][j] = dp[i-1][j].min(dp[i][j-1]).min(dp[i-1][j-1]) + 1;
                    }
                    ans = ans.max(dp[i][j]);
                }
            }
        }
        ans*ans
    }

}

struct NumMatrix {
    sum: Vec<Vec<i32>>,
}
// https://leetcode.cn/problems/range-sum-query-2d-immutable/solution/ru-he-qiu-er-wei-de-qian-zhui-he-yi-ji-y-6c21/
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut sum = vec![vec![0;m+1]; n+1];
        for i in 0..m {
            for j in 0..n {
                sum[i+1][j+1] = sum[i+1][j] + sum[i][j+1] - sum[i][j] + matrix[i][j];
            }
        }
        Self { sum }
    }

    fn sum_region(&self, row1:i32, col1:i32, row2:i32, col2:i32) -> i32 {
        let r1 = row1 as usize;
        let c1 = col1 as usize;
        let r2 = row2 as usize;
        let c2 = col2 as usize;
        let sum = &self.sum;
        sum[r2+1][c2+1] - sum[r2+1][c1] - sum[r1][c2+1] + sum[r1][c1]
    }


}
