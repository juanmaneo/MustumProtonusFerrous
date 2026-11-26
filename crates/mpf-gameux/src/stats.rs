//! `IGameStatistics` category and statistic store from Wine's `dlls/gameux/gamestatistics.c`.
//!
//! Wine keeps fixed `WCHAR` arrays with a NUL terminator. This model keeps only the
//! used units in a `Vec<u16>`, so an empty vector stands for the empty C string.

use crate::{wstr, Hresult, E_INVALIDARG, S_FALSE, S_OK};

pub const MAX_CATEGORY_LENGTH: usize = 60;
pub const MAX_NAME_LENGTH: usize = 30;
pub const MAX_VALUE_LENGTH: usize = 30;
pub const MAX_CATEGORIES: usize = 10;
pub const MAX_STATS_PER_CATEGORY: usize = 10;

/// A statistic's name and value. `None` stands for an empty string.
pub type StatText = (Option<Vec<u16>>, Option<Vec<u16>>);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stat {
    pub name: Vec<u16>,
    pub value: Vec<u16>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Category {
    pub title: Vec<u16>,
    pub stats: [Stat; MAX_STATS_PER_CATEGORY],
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GameStatistics {
    pub categories: [Category; MAX_CATEGORIES],
}

impl GameStatistics {
    /// `GameStatisticsImpl_SetCategoryTitle`. A `None` title or an out-of-range index
    /// is `E_INVALIDARG`. A title over 60 units is cut to 60 and gives `S_FALSE`.
    pub fn set_category_title(
        &mut self,
        index: u16,
        title: Option<&[u16]>,
    ) -> Result<Hresult, Hresult> {
        let title = match title {
            Some(title) if usize::from(index) < MAX_CATEGORIES => wstr(title),
            _ => return Err(E_INVALIDARG),
        };
        let (kept, hr) = clamp(title, MAX_CATEGORY_LENGTH);
        self.categories[usize::from(index)].title = kept.to_vec();
        Ok(hr)
    }

    /// `GameStatisticsImpl_GetCategoryTitle`. An empty title gives `None`, as `*pTitle = NULL`.
    pub fn get_category_title(&self, index: u16) -> Result<Option<Vec<u16>>, Hresult> {
        let category = self
            .categories
            .get(usize::from(index))
            .ok_or(E_INVALIDARG)?;
        Ok(non_empty(&category.title))
    }

    /// `GameStatisticsImpl_SetStatistic`.
    ///
    /// A `None` name gives `S_FALSE` and changes nothing. Wine checks it before the
    /// index, so that holds even for an out-of-range index. A `None` value clears the
    /// stored value. Names over 30 units and values over 30 units are cut and give `S_FALSE`.
    pub fn set_statistic(
        &mut self,
        category: u16,
        stat: u16,
        name: Option<&[u16]>,
        value: Option<&[u16]>,
    ) -> Result<Hresult, Hresult> {
        let Some(name) = name else {
            return Ok(S_FALSE);
        };
        let slot = self
            .categories
            .get_mut(usize::from(category))
            .and_then(|c| c.stats.get_mut(usize::from(stat)))
            .ok_or(E_INVALIDARG)?;
        let (name, mut hr) = clamp(wstr(name), MAX_NAME_LENGTH);
        slot.name = name.to_vec();
        slot.value = match value {
            Some(value) => {
                let (value, value_hr) = clamp(wstr(value), MAX_VALUE_LENGTH);
                if value_hr != S_OK {
                    hr = value_hr;
                }
                value.to_vec()
            }
            None => Vec::new(),
        };
        Ok(hr)
    }

    /// `GameStatisticsImpl_GetStatistic`. Empty name and value each give `None`.
    pub fn get_statistic(&self, category: u16, stat: u16) -> Result<StatText, Hresult> {
        let slot = self
            .categories
            .get(usize::from(category))
            .and_then(|c| c.stats.get(usize::from(stat)))
            .ok_or(E_INVALIDARG)?;
        Ok((non_empty(&slot.name), non_empty(&slot.value)))
    }
}

fn clamp(s: &[u16], max: usize) -> (&[u16], Hresult) {
    if s.len() > max {
        (&s[..max], S_FALSE)
    } else {
        (s, S_OK)
    }
}

fn non_empty(s: &[u16]) -> Option<Vec<u16>> {
    (!s.is_empty()).then(|| s.to_vec())
}
