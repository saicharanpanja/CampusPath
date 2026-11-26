//! Engine: findet den kürzesten Weg zwischen zwei Räumen im `BuildingGraph`.
//!
//! Sprint 1: Gerüst, Ergebnis-Typ, Raum-Prüfung und `reconstruct_path`.
//! (Schritt 2 von 3: `reconstruct_path` mit Tests)


type NodeId = u32; 

/// Ergebnis einer Wegsuche.
#[derive(Debug, PartialEq)]
pub struct PathResult {
    pub nodes: Vec<NodeId>, // Ordered Start -> Goal
    pub total_cost: u32,    // Total weighted pixel cost
}

/// Baut den Weg rückwärts vom Ziel zum Start zusammen.
///
/// `came_from[n]` sagt, von welchem Knoten wir zu `n` gekommen sind.
/// Der Startknoten hat `None`.
#[allow(dead_code)] // wird ab Sprint 2 von find_path benutzt
fn reconstruct_path(came_from: &[Option<NodeId>], goal: NodeId) -> Vec<NodeId> {
    let mut path = vec![goal];
    let mut current = goal;

    while let Some(previous) = came_from[current as usize] {
        path.push(previous);
        current = previous;
    }

    path.reverse(); // jetzt Start -> Ziel
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconstruct_path_follows_came_from() {
        // 0 -> 1 -> 2: Knoten 2 kam von 1, Knoten 1 kam von 0.
        let came_from = vec![None, Some(0), Some(1)];
        assert_eq!(reconstruct_path(&came_from, 2), vec![0, 1, 2]);
    }

    #[test]
    fn reconstruct_path_start_is_goal() {
        let came_from = vec![None, Some(0)];
        assert_eq!(reconstruct_path(&came_from, 0), vec![0]);
    }
}