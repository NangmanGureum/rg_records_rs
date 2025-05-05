#[derive(Debug, Clone)]
pub struct Record {
    /// ID of the song for the record. It refers `Song.id`
    pub song_id: usize,
    /// A type for the chart for the record. It refers `Chart.chart_type`
    pub chart_type: u8,
    /// A difficulty type of the chart for the record. It refers `Chart.difficulty_type`
    pub difficulty_type: u8,
    /// A score of the record
    pub score: u32,
    /// Maximum combos of the record, not 'MAX COMBO' at DJMAX series
    pub maximum_combo: u32,
    /// A clear type of the record
    pub clear_type: u8,
    /// A date of the record. It follows RFC 3339
    pub date: String,
}
