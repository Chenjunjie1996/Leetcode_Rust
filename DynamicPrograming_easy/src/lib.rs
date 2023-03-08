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

    pub fn rob(nums: Vec<i32>) -> i32 {
        /* 打家劫舍
        输入：[2,7,9,3,1]
        输出：12
        解释：偷窃 1 号房屋 (金额 = 2), 偷窃 3 号房屋 (金额 = 9)，接着偷窃 5 号房屋 (金额 = 1)。
        偷窃到的最高金额 = 2 + 9 + 1 = 12 。

        状态转移方程：dp[i]=max(dp[i−2]+nums[i],dp[i−1])  
        1. 偷i不偷i-1，dp[i] = dp[i-2]+nums[i]
        2. 不偷i dp[i] = dp[i-1]
        dp[i] 表示前 i 间房屋能偷窃到的最高总金额
        边界条件：
        dp[0] = nums[0]
        dp[1] = max(nums[0], nums[1])
        答案为：dp[nums.len()-1]
        */
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        if len == 1 {
            return nums[0].max(nums[1]);
        }
        let mut dp = vec![0; len];
        dp[0] = nums[0];
        dp[1] = nums[0].max(nums[1]);
        for i in 2..len {
            dp[i] = (dp[i-2] + nums[i]).max(dp[i-1])
        }
        dp[len-1]
    }

    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        /*
        给你一个整数数组 cost ，其中 cost[i] 是从楼梯第 i 个台阶向上爬需要支付的费用。一旦你支付此费用，即可选择向上爬一个或者两个台阶。
        你可以选择从下标为 0 或下标为 1 的台阶开始爬楼梯。
        输入：cost = [10,15,20]
        输出：15
        解释：你将从下标为 1 的台阶开始。
        - 支付 15 ，向上爬两个台阶，到达楼梯顶部。
        总花费为 15 。
        */
        let n =cost.len();
        let mut dp = vec![0; n+1];
        for i in 2..=n {
            dp[i] = (dp[i-1] + cost[i-1]).min(dp[i-2] + cost[i-2]);
        }
        dp[n]
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        /* 买卖股票最佳时机
        输入：[7,1,5,3,6,4]
        输出：5
        解释：在第 2 天（股票价格 = 1）的时候买入，在第 5 天（股票价格 = 6）的时候卖出
        输入：[7,6,4,3,1]
        输出：0
        */
        let mut ans = 0;
        let mut min_price = i32::MAX;
        for price in prices.into_iter() {
            if price < min_price {
                min_price = price;
            }
            ans = ans.max(price-min_price)
        }
        ans
    }

    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        /*
        // 输入: [1,3,5,4,7]
        // 输出: 3
        // 解释: 最长连续递增序列是 [1,3,5], 长度为3。
        // 尽管 [1,3,5,7] 也是升序的子序列, 但它不是连续的，因为5和7在原数组里被4隔开。
        */
        // let len = nums.len();
        // let mut max_len:i32 = 0;
        // let mut sub_nums: Vec<i32> = vec![];
        // for i in 0..len {
        //     sub_nums.push(nums[i]);
        //     if nums[i+1] < nums[i] {
        //         sub_nums.push(nums[i+1])
        //    } else {
        //         max_len = max_len.max(sub_nums.len() as i32);
        //         sub_nums = vec![];
        //     }
        // }
        // max_len
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        let mut res = 1;
        let mut path = vec![1; nums.len()];
        for i in 1..nums.len() {
            if nums[i] > nums[i-1] {
                path[i] = path[i-1] + 1;
                res = max(res, path[i]);
            }
        }
        res
    }

    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        /*有序数组的平方
        输入：nums = [-7,-3,2,3,11]
        输出：[4,9,9,49,121] 
        */
        // 1.直接排序
        // let res = nums.into_iter().map(|x| x.pow(2)).collect();
        // res
        // 2.双指针
        let mut dp = vec![];
        let mut start = 0;
        let mut end = nums.len() - 1;
        while start <= end {
            let val;
            if nums[start].abs() <= nums[end].abs() {
                val = nums[start];
                start += 1;
            } else {
                val = nums[end];
                end -= 1;
            }
            dp.push(val * val)
        }
        dp
    }

    pub fn fib(n: i32) -> i32 {
        /* 斐波那契数
        F(N) = F(N - 1) + F(N - 2)
        输入：n = 5
        输出：5
        */
        let n = n as usize;
        let mut dp = [0, 1, 1];
        if n <= 2 {
            return dp[n];
        }
        for _i in 2..=n {
            dp[0] = dp[1];
            dp[1] = dp[2];
            dp[2] = dp[0] + dp[1];
        }
        dp[1]
    }
}
