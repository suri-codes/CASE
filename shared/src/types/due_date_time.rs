use std::ops::Deref;

use autosurgeon::{Hydrate, Reconcile, reconcile::NoKey};
use chrono::{NaiveDateTime, format::StrftimeItems};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DueDateTime(Option<NaiveDateTime>);

impl DueDateTime {
    //TODO: could make this take in a string and then return a parse error for all
    // of our supported datetime stuff.
    pub(crate) const fn new(inner: Option<NaiveDateTime>) -> Self {
        Self(inner)
    }
}

const NO_DUE_DATE: &str = "No Due Date";

const DATE_TIME_COMPRESSED_FMT: &str = "%Y%m%d%H%M%S";

impl Reconcile for DueDateTime {
    type Key<'a> = NoKey;

    fn reconcile<R: autosurgeon::Reconciler>(&self, mut reconciler: R) -> Result<(), R::Error> {
        let date_time_as_compressed_str = self.0.map_or_else(
            || NO_DUE_DATE.to_owned(),
            |d| {
                d.format_with_items(StrftimeItems::new(DATE_TIME_COMPRESSED_FMT))
                    .to_string()
            },
        );

        reconciler.str(date_time_as_compressed_str)
    }
}

impl Hydrate for DueDateTime {
    fn hydrate_string(string: &'_ str) -> Result<Self, autosurgeon::HydrateError> {
        let inner = match string {
            NO_DUE_DATE => None,
            date_time_as_compressed_str => Some(
                NaiveDateTime::parse_from_str(
                    date_time_as_compressed_str,
                    DATE_TIME_COMPRESSED_FMT,
                )
                .expect("Expecting this to be a valid string."),
            ),
        };
        Ok(Self(inner))
    }
}

impl Deref for DueDateTime {
    type Target = Option<NaiveDateTime>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use automerge::AutoCommit;
    use autosurgeon::{Hydrate, Reconcile, hydrate, reconcile};
    use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};

    use crate::data::DueDateTime;

    #[test]
    fn reconcile_due_date_time() {
        #[derive(Debug, Reconcile, Hydrate, Clone, PartialEq, Eq)]
        // A "map" encoded struct for automerge, as the root of any document
        // must be presentable as a "map", i.e. a struct.
        struct Map {
            due_date: DueDateTime,
        }
        // testing no due date
        {
            let due_date_time = DueDateTime::new(None);

            let map = Map {
                due_date: due_date_time,
            };

            let expected = map.clone();

            let mut doc = AutoCommit::new();

            reconcile(&mut doc, &map).unwrap();

            let result: Map = hydrate(&doc).unwrap();

            assert_eq!(result, expected);
        }

        // Testing a real due date.
        {
            let due_date_time = DueDateTime::new(Some(NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2006, 1, 31).unwrap(),
                NaiveTime::from_hms_opt(1, 2, 3).unwrap(),
            )));

            let map = Map {
                due_date: due_date_time,
            };

            let expected = map.clone();

            let mut doc = AutoCommit::new();

            reconcile(&mut doc, &map).unwrap();

            let result: Map = hydrate(&doc).unwrap();

            assert_eq!(result, expected);
        }
    }
}
