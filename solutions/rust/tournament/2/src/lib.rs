use std::{cmp::Reverse, collections::HashMap};

pub fn edit_team (team_set: &mut HashMap<String, (u32,u32,u32,u32,u32)>, team: String, change: (u32, u32, u32, u32, u32)) {
    team_set.entry(team)
        .and_modify(|current| {
            current.0 += change.0;
            current.1 += change.1;
            current.2 += change.2;
            current.3 += change.3;
            current.4 += change.4;
        }).or_insert(change);
}

pub fn tally(match_results: &str) -> String {
    let mut score_board = [
        format!("{:<30}","Team").as_str(),
        format!("{:>2}", "MP").as_str(),
        format!("{:>2}", "W").as_str(),
        format!("{:>2}", "D").as_str(),
        format!("{:>2}", "L").as_str(),
        format!("{:>2}", "P").as_str()
    ].join(" | ");
    let mut team_set = HashMap::<String, (u32,u32,u32,u32,u32)>::new();
    match_results.lines().for_each(|line| {
        let mut splitter = line.split(";");
        let home = splitter.next().expect("cannot get Home Team").to_string();
        let away = splitter.next().expect("cannot get Away Team").to_string();
        let result = splitter.next().expect("cannot get Result");
        match result {
            "win" => {
                edit_team(&mut team_set, home, (1, 1, 0, 0, 3));
                edit_team(&mut team_set, away, (1, 0, 0, 1, 0));
            },
            "loss" => {
                edit_team(&mut team_set, home, (1, 0, 0, 1, 0));
                edit_team(&mut team_set, away, (1, 1, 0, 0, 3));
            },
            "draw" => {
                edit_team(&mut team_set, home, (1, 0, 1, 0, 1));
                edit_team(&mut team_set, away, (1, 0, 1, 0, 1));
            },
            _ => {}
        }
    });

    // sort by point then team name alphabet
    let mut rank = team_set.into_iter().collect::<Vec<_>>();
    rank.sort_by_key(|(team, (_mp, _w, _d, _l , p)) | {
        (Reverse(*p), team.clone())
    });

    rank.into_iter().for_each(|(team,(mp,w,d,l,p))| {
        score_board += "\n";
        score_board += [
            format!("{:<30}", team).as_str(),
            format!("{:>2}", mp).as_str(),
            format!("{:>2}", w).as_str(),
            format!("{:>2}", d).as_str(),
            format!("{:>2}", l).as_str(),
            format!("{:>2}", p).as_str(),
        ].join(" | ").as_str();
    });

    score_board
}
