use std::fmt;

/// Represents a possible board position
pub struct Board {
    /// The pits for players to play on, with six pits per player. Positions 0-5 are player 1's pits, while positions 6-11 are player 2's pits.
    pub pits: [u8; 12],
    /// The places where players store the stones they have collected. Position 0 is player 1, while position 1 is player 2's pit.
    /// These positions are equivalent to after position 5, and after position 11 respectively.
    pub stores: [u8; 2],
}

impl Board {
    pub fn new() -> Board {
        Board {
            pits: [4; 12],
            stores: [0; 2],
        }
    }

    pub fn with_custom_pits(pits: [u8; 12], stores: [u8; 2]) -> Self {
        Board { pits, stores }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // displays the pits for a custom range
        // with a gap of three spaces at the start
        let display_pits = |itr: &mut dyn Iterator<Item = usize>| {
            let lines = itr
                .map(|i| {
                    let stones = self.pits[i];
                    let pit_id = format!("#{}", i);

                    format!("{:>4}: {}", pit_id, stones)
                })
                .collect::<Vec<_>>()
                .join(" |");

            format!("{}{lines}", " ".repeat(3), lines = lines)
        };
        writeln!(f)?;
        // line 1: display the second players board pits
        // from position 6 -> 11
        // example:
        //      #6: 4 |  #7: 4 |  #8: 4 |  #9: 4 | #10: 4 | #11: 4
        writeln!(f, "{}", display_pits(&mut (6..=11)))?;

        // line 2: display player 1's store on the left, followed by
        // a separating line
        // and player 2's store on the right
        // example:
        // 0  --------+--------+--------+--------+--------+-------- 0
        write!(f, "{:<2} ", self.stores[0])?;
        for i in 0..6 {
            write!(f, "{}", "-".repeat(8))?;

            if i != 5 {
                write!(f, "+")?;
            }
        }
        write!(f, " {}", self.stores[1])?;
        writeln!(f)?;

        // line 3: display the first players board pits
        // from positions 5 --> 1
        // example:
        //      #5: 4 |  #4: 4 |  #3: 4 |  #2: 4 |  #1: 4 | #0: 4
        writeln!(f, "{}", display_pits(&mut (0..=5).rev()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_empty_board() {
        let lines = Vec::from([
            "",
            "     #6: 4 |  #7: 4 |  #8: 4 |  #9: 4 | #10: 4 | #11: 4",
            "0  --------+--------+--------+--------+--------+-------- 0",
            "     #5: 4 |  #4: 4 |  #3: 4 |  #2: 4 |  #1: 4 |  #0: 4",
            "",
        ]);

        let board = Board::new();
        assert_eq!(format!("{:?}", board), lines.join("\n"));
    }

    #[test]
    fn formats_complete_board() {
        let lines = Vec::from([
            "",
            "     #6: 0 |  #7: 0 |  #8: 0 |  #9: 0 | #10: 0 | #11: 0",
            "24 --------+--------+--------+--------+--------+-------- 24",
            "     #5: 0 |  #4: 0 |  #3: 0 |  #2: 0 |  #1: 0 |  #0: 0",
            "",
        ]);

        let board = Board::with_custom_pits([0; 12], [24; 2]);
        assert_eq!(format!("{:?}", board), lines.join("\n"));
    }
}
