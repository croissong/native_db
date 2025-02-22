use crate::db_type::{DatabaseKey, Input, KeyOptions, Result, ToKey};
use crate::watch;
use crate::watch::query::internal;
use crate::watch::MpscReceiver;

/// Watch only one value.
pub struct WatchGet<'db, 'w> {
    pub(crate) internal: &'w internal::InternalWatch<'db>,
}

impl WatchGet<'_, '_> {
    /// Watch the primary key.
    ///
    /// Returns a channel receiver and the watcher id.
    /// The watcher id can be used to unwatch the channel.
    ///
    /// # Example
    /// ```rust
    /// use native_db::*;
    /// use native_model::{native_model, Model};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// #[native_model(id=1, version=1)]
    /// #[native_db]
    /// struct Data {
    ///     #[primary_key]
    ///     id: u64,
    /// }
    ///
    /// fn main() -> Result<(), db_type::Error> {
    ///     let mut builder = DatabaseBuilder::new();
    ///     builder.define::<Data>()?;
    ///     let db = builder.create_in_memory()?;
    ///     
    ///     // Watch the primary key
    ///     let (_recv, _id) = db.watch().get().primary::<Data>(1u64)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn primary<T: Input>(&self, key: impl ToKey) -> Result<(MpscReceiver<watch::Event>, u64)> {
        self.internal.watch_primary::<T>(key)
    }

    /// Watch the secondary key.
    ///
    /// Returns a channel receiver and the watcher id.
    /// The watcher id can be used to unwatch the channel.
    ///
    /// # Example
    /// ```rust
    /// use native_db::*;
    /// use native_model::{native_model, Model};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// #[native_model(id=1, version=1)]
    /// #[native_db]
    /// struct Data {
    ///     #[primary_key]
    ///     id: u64,
    ///     #[secondary_key]
    ///    name: String,
    /// }
    ///
    /// fn main() -> Result<(), db_type::Error> {
    ///     let mut builder = DatabaseBuilder::new();
    ///     builder.define::<Data>()?;
    ///     let db = builder.create_in_memory()?;
    ///     
    ///     // Watch the secondary key name
    ///     let (_recv, _id) = db.watch().get().secondary::<Data>(DataKey::name, "test")?;
    ///     Ok(())
    /// }
    /// ```
    pub fn secondary<T: Input>(
        &self,
        key_def: impl DatabaseKey<KeyOptions>,
        key: impl ToKey,
    ) -> Result<(MpscReceiver<watch::Event>, u64)> {
        self.internal.watch_secondary::<T>(&key_def, key)
    }
}
