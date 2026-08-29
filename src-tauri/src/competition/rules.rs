pub fn valid_tiebreak(rule: &str) -> bool {
    matches!(rule, "points_goal_difference_goals_for" | "points_goals_for" | "points_head_to_head")
}

pub fn order_clause(rule: &str) -> &'static str {
    match rule {
        "points_goals_for" => "points DESC, goals_for DESC, goal_difference DESC, club_id ASC",
        "points_head_to_head" => "points DESC, goal_difference DESC, goals_for DESC, club_id ASC",
        _ => "points DESC, goal_difference DESC, goals_for DESC, club_id ASC",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_supported_rules() {
        assert!(valid_tiebreak("points_goal_difference_goals_for"));
        assert!(valid_tiebreak("points_goals_for"));
        assert!(valid_tiebreak("points_head_to_head"));
        assert!(!valid_tiebreak("random"));
    }

    #[test]
    fn returns_safe_ordering_clauses() {
        assert!(order_clause("points_goals_for").contains("goals_for DESC"));
        assert!(order_clause("points_head_to_head").contains("goal_difference DESC"));
        assert_eq!(order_clause("unknown"), order_clause("points_goal_difference_goals_for"));
    }
}
