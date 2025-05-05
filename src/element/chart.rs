#[derive(Debug, Clone)]
pub struct Chart {
    /// ID of the song for the chart. It refers `Song.id`
    pub song_id: usize,
    /// A type of chart. (e. g. SP/DP at IIDX, 4B/5B/6B/8B at DJMAX) It depends on the game.
    pub chart_type: u8,
    /// A difficulty type of chart. (e. g. Beginner/N/H/A/L at IIDX, NM/HD/MX/SC at DJMAX) It depends on the game.
    pub difficulty_type: u8,
    /// A level of the chart. It depends on the game.
    pub level: f64,
    /// Total notes of the chart. See `TotalNotes` struct
    pub total_notes: TotalNotes,
    /// An unofficial level of the chart. It depends on who made a level table.
    pub user_level: Option<f64>,
    /// A date of when the song updated. It follows RFC 3339
    pub updated_date: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TotalNotes {
    /// A count of normal notes. It include long notes
    pub normal_notes: u16,
    /// A count of special notes. (e. g. Scratch at IIDX, Swipe at GROOVE COASTER)
    pub special_note_01: Option<u16>,
    pub special_note_02: Option<u16>,
    pub special_note_03: Option<u16>,
    pub special_note_04: Option<u16>,
    pub special_note_05: Option<u16>,
    pub special_note_06: Option<u16>,
    pub special_note_07: Option<u16>,
    pub special_note_08: Option<u16>,
    pub special_note_09: Option<u16>,
}
