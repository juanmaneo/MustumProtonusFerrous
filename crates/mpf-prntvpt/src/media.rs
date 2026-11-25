use crate::{
    DMPAPER_A3, DMPAPER_A4, DMPAPER_A5, DMPAPER_B4, DMPAPER_B5, DMPAPER_EXECUTIVE, DMPAPER_LEGAL,
    DMPAPER_LETTER,
};

/// `psk_media` from `ticket.c`: PrintSchema media name and its DEVMODE paper code.
const PSK_MEDIA: [(&str, i32); 8] = [
    ("psk:NorthAmericaLetter", DMPAPER_LETTER),
    ("psk:NorthAmericaLegal", DMPAPER_LEGAL),
    ("psk:NorthAmericaExecutive", DMPAPER_EXECUTIVE),
    ("psk:ISOA3", DMPAPER_A3),
    ("psk:ISOA4", DMPAPER_A4),
    ("psk:ISOA5", DMPAPER_A5),
    ("psk:JISB4", DMPAPER_B4),
    ("psk:JISB5", DMPAPER_B5),
];

/// `media_to_paper`: the DEVMODE paper code for a PrintSchema media name. Names not in
/// the table give `DMPAPER_A4`, as Wine does (Wine also logs a FIXME, which is not ported).
pub fn media_to_paper(name: &str) -> i32 {
    PSK_MEDIA
        .iter()
        .find(|(media, _)| *media == name)
        .map_or(DMPAPER_A4, |&(_, paper)| paper)
}

/// `paper_to_media`: the PrintSchema media name for a DEVMODE paper code, or `None`.
pub fn paper_to_media(paper: i32) -> Option<&'static str> {
    PSK_MEDIA
        .iter()
        .find(|(_, code)| *code == paper)
        .map(|&(media, _)| media)
}
