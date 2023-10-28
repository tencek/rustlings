// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

use std::collections::HashMap;

// A structure to store the goal details of a team.
struct TeamResult {
    goals_scored: u8,
    goals_conceded: u8,
}

impl TeamResult {
    pub fn Add(&self, otherTeamResult: TeamResult) -> TeamResult {
        TeamResult {
            goals_scored: self.goals_scored + otherTeamResult.goals_scored,
            goals_conceded: self.goals_conceded + otherTeamResult.goals_conceded,
        }
    }
}

fn parse_results(results: String) -> Vec<(String, TeamResult)> {
    results
        .lines()
        .flat_map(|line| {
            let v: Vec<&str> = line.split(',').collect();
            let team_1_name = v[0].to_string();
            let team_1_scored: u8 = v[2].parse().unwrap();
            let team_2_name = v[1].to_string();
            let team_2_scored: u8 = v[3].parse().unwrap();
            let team_1_conceded: u8 = team_2_scored;
            let team_2_conceded: u8 = team_1_scored;
            [
                (
                    team_1_name,
                    TeamResult {
                        goals_scored: team_1_scored,
                        goals_conceded: team_1_conceded,
                    },
                ),
                (
                    team_2_name,
                    TeamResult {
                        goals_scored: team_2_scored,
                        goals_conceded: team_2_conceded,
                    },
                ),
            ]
        })
        .collect()
}

fn build_scores_table(results: String) -> HashMap<String, TeamResult> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, TeamResult> = HashMap::new();

    let parsed_results: Vec<(String, TeamResult)> = parse_results(results);
    for (team_name, team_result) in parsed_results {
        println!(
            "{}: {}:{}",
            team_name, team_result.goals_scored, team_result.goals_conceded
        );
        match scores.get(&team_name) {
            None => scores.insert(team_name, team_result),
            Some(current_team_result) => {
                scores.insert(team_name, current_team_result.Add(team_result))
            }
        };
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
