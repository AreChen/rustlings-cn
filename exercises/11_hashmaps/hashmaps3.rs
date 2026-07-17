// 给出一场足球比赛的比分列表（每行一条）。每行的格式为
// 格式为 "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"。
// 例如："England,France,4,2"（England 进 4 球，France 进 2 球）。
//
// 你需要构建一个比分表，其中包含球队名称、球队进球总数和球队失球总数。

use std::collections::HashMap;

// 用于存储球队进球详情的结构体。
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_score_table(results: &str) -> HashMap<&str, TeamScores> {
    // 球队名称是键，与之关联的结构体是值。
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // 注意：我们使用 `unwrap`，因为还没有学习错误处理。
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: 使用提取出的详情填充比分表。
        // 注意：球队 1 的进球数就是球队 2 的失球数。同理，球队 2 的进球数
        // 就是球队 1 的失球数。
    }

    scores
}

fn main() {
    // 你可以选择在这里进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_score_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_score_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_score_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
