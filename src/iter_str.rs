use core::fmt;

use crate::generated::{
    LEXICON, LEXICON_OFFSETS, LEXICON_ORDERED_LENGTHS, LEXICON_SHORT_LENGTHS, PHRASEBOOK,
    PHRASEBOOK_SHORT,
};

#[derive(Clone)]
struct PhrasebookIter {
    index: u32,
}

impl PhrasebookIter {
    const EMPTY: Self = Self {
        index: PHRASEBOOK.len() as u32,
    };
}

impl Iterator for PhrasebookIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let b = *PHRASEBOOK.get(self.index as usize)?;
        self.index += 1;
        Some(b)
    }
}

#[derive(Clone)]
pub struct IterStr {
    phrasebook: PhrasebookIter,
    last_was_word: bool,
}

impl IterStr {
    pub fn new(start_index: u32) -> IterStr {
        IterStr {
            phrasebook: PhrasebookIter { index: start_index },
            last_was_word: false,
        }
    }
}

const HYPHEN: u8 = 127;

impl Iterator for IterStr {
    type Item = &'static str;
    fn next(&mut self) -> Option<&'static str> {
        let mut tmp = self.phrasebook.clone();
        tmp.next().map(|raw_b| {
            // the first byte includes if it is the last in this name
            // in the high bit.
            let (is_end, b) = (raw_b & 0b1000_0000 != 0, raw_b & 0b0111_1111);

            let ret = if b == HYPHEN {
                // have to handle this before the case below, because a -
                // replaces the space entirely.
                self.last_was_word = false;
                "-"
            } else if self.last_was_word {
                self.last_was_word = false;
                // early return, we don't want to update the
                // phrasebook (i.e. we're pretending we didn't touch
                // this byte).
                return " ";
            } else {
                self.last_was_word = true;

                let idx;
                let length = if b < PHRASEBOOK_SHORT {
                    idx = b as usize;
                    // these lengths are hard-coded
                    LEXICON_SHORT_LENGTHS[idx] as usize
                } else {
                    idx = (b - PHRASEBOOK_SHORT) as usize * 256 + (tmp.next().unwrap()) as usize;

                    // search for the right place: the first one where
                    // the end-point is after our current index.
                    match LEXICON_ORDERED_LENGTHS.binary_search_by_key(&idx, |&(end, _)| end - 1) {
                        Ok(i) | Err(i) => LEXICON_ORDERED_LENGTHS[i].1 as usize,
                    }
                };
                let offset = LEXICON_OFFSETS[idx] as usize;
                &LEXICON[offset..offset + length]
            };
            self.phrasebook = if is_end { PhrasebookIter::EMPTY } else { tmp };
            ret
        })
    }
}

impl fmt::Debug for IterStr {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let printed = self.clone();
        for s in printed {
            write!(fmtr, "{}", s)?
        }
        Ok(())
    }
}
