fn longest_common_subsequence(s1: &str, s2: &str) -> i32 {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();

    let m = chars1.len();
    let n = chars2.len();
    let mut dp: Vec<Vec<i32>> = vec![vec![0; m]; n];

    for i in 0..(n - 1) {
        for j in 0..(m - 1) {
            if chars1[j] == chars2[i] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
            }
        }
    }

    dp[n - 1][m - 1]
}

fn main() {
    let s1 = "abcde";
    let s2 = "ace";

    println!("lcs:{}", longest_common_subsequence(s1, s2));
}
