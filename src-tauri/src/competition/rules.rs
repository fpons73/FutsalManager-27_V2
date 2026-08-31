pub fn valid_tiebreak(rule: &str) -> bool {
    matches!(rule, "points_goal_difference_goals_for" | "points_goals_for" | "points_head_to_head")
}

pub fn order_clause(rule: &str) -> &'static str {
    match rule {
        "points_goals_for" => "points DESC, goals_for DESC, goal_difference DESC, club_id ASC",
        // El enfrentamiento directo se resuelve en Rust porque necesita consultar
        // únicamente los partidos disputados entre los equipos empatados.
        "points_head_to_head" => "points DESC, goal_difference DESC, goals_for DESC, club_id ASC",
        _ => "points DESC, goal_difference DESC, goals_for DESC, club_id ASC",
    }
}

pub fn head_to_head_order(
    entries: &[(i64, i64, i64, i64)],
    matches: &[(i64, i64, i64, i64)],
) -> Vec<i64> {
    let mut ordered = Vec::with_capacity(entries.len());
    let mut groups: std::collections::BTreeMap<i64, Vec<(i64, i64, i64, i64)>> = Default::default();
    for entry in entries.iter().copied() { groups.entry(entry.1).or_default().push(entry); }
    for (_, mut group) in groups.into_iter().rev() {
        let tied: std::collections::HashSet<i64> = group.iter().map(|entry| entry.0).collect();
        let mut head_to_head: std::collections::HashMap<i64, (i64, i64, i64)> = Default::default();
        for club in &tied { head_to_head.insert(*club, (0, 0, 0)); }
        for (home, away, home_goals, away_goals) in matches.iter().copied() {
            if !tied.contains(&home) || !tied.contains(&away) { continue; }
            let (home_points, away_points) = if home_goals > away_goals { (3, 0) } else if home_goals < away_goals { (0, 3) } else { (1, 1) };
            if let Some(stats) = head_to_head.get_mut(&home) { stats.0 += home_points; stats.1 += home_goals - away_goals; stats.2 += home_goals; }
            if let Some(stats) = head_to_head.get_mut(&away) { stats.0 += away_points; stats.1 += away_goals - home_goals; stats.2 += away_goals; }
        }
        group.sort_by(|a, b| {
            let ah = head_to_head.get(&a.0).copied().unwrap_or_default();
            let bh = head_to_head.get(&b.0).copied().unwrap_or_default();
            bh.cmp(&ah).then_with(|| b.2.cmp(&a.2)).then_with(|| b.3.cmp(&a.3)).then_with(|| a.0.cmp(&b.0))
        });
        ordered.extend(group.into_iter().map(|entry| entry.0));
    }
    ordered
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
    fn head_to_head_breaks_points_ties() {
        let entries = vec![(1, 10, 2, 8), (2, 10, 1, 7), (3, 9, 9, 20)];
        let matches = vec![(1, 2, 0, 2), (2, 1, 1, 1), (1, 3, 4, 0), (3, 2, 3, 0)];
        assert_eq!(head_to_head_order(&entries, &matches), vec![2, 1, 3]);
    }

    #[test]
    fn returns_safe_ordering_clauses() {
        assert!(order_clause("points_goals_for").contains("goals_for DESC"));
        assert!(order_clause("points_head_to_head").contains("goal_difference DESC"));
        assert_eq!(order_clause("unknown"), order_clause("points_goal_difference_goals_for"));
    }
}
