use std::cmp::max;


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

    pub fn message(nums: Vec<i32>) -> i32 {
        /* 按摩师
        一个有名的按摩师会收到源源不断的预约请求，每个预约都可以选择接或不接。在每次预约服务之间要有休息时间，因此她不能接受相邻的预约。
        给定一个预约请求序列，替按摩师找到最优的预约集合（总预约时间最长），返回总的分钟数。
        输入： [2,1,4,5,3,1,1,3]
        输出： 12
        解释： 选择 1 号预约、 3 号预约、 5 号预约和 8 号预约，总时长 = 2 + 4 + 3 + 3 = 12。
        */

        if nums.len() == 0 {
            return 0;
          }
          let mut res = nums[0];
          let len = nums.len();
          let mut dp: Vec<Vec<i32>> = vec![vec![0, nums[0]]; len]; // 设定length定义dp二维数组
          for i in 1..len {
            if dp.get(i).is_none() {
              dp[i] = vec![0, 0]
            }
        
            dp[i][0] = max(dp[i - 1][0], dp[i - 1][1]); // dp[i][0] 表示前i个预约第i个预约不接的最长时间
            dp[i][1] = dp[i - 1][0] + nums[i]; // dp[i][1] 表示前i个预约第i个预约接的最长时间
            res = max(max(res, dp[i][0]), dp[i][1])
          }
          res
    }
}
