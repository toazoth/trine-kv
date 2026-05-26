use crate::{
    bucket::DEFAULT_BUCKET_NAME,
    error::{Error, Result},
    types::{KeyRange, Value},
};

/// One operation inside an atomic write batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchOperation {
    Put {
        bucket: String,
        key: Vec<u8>,
        value: Value,
    },
    Delete {
        bucket: String,
        key: Vec<u8>,
    },
    DeleteRange {
        bucket: String,
        range: KeyRange,
    },
}

impl BatchOperation {
    #[must_use]
    pub fn bucket(&self) -> &str {
        match self {
            Self::Put { bucket, .. }
            | Self::Delete { bucket, .. }
            | Self::DeleteRange { bucket, .. } => bucket,
        }
    }
}

/// Atomic group of writes that may span multiple buckets.
///
/// Methods without a bucket suffix target the built-in default bucket. Methods
/// ending in `_bucket` target an optional named bucket returned by `Db::bucket`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WriteBatch {
    operations: Vec<BatchOperation>,
}

impl WriteBatch {
    /// Creates an empty batch.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Adds a key/value write to the default bucket.
    pub fn put(&mut self, key: impl Into<Vec<u8>>, value: impl Into<Value>) {
        self.operations.push(BatchOperation::Put {
            bucket: DEFAULT_BUCKET_NAME.to_owned(),
            key: key.into(),
            value: value.into(),
        });
    }

    /// Adds a key/value write for a named bucket.
    pub fn put_bucket(
        &mut self,
        bucket: impl Into<String>,
        key: impl Into<Vec<u8>>,
        value: impl Into<Value>,
    ) -> Result<()> {
        let bucket = bucket.into();
        validate_named_bucket(&bucket)?;
        self.operations.push(BatchOperation::Put {
            bucket,
            key: key.into(),
            value: value.into(),
        });
        Ok(())
    }

    /// Adds a point delete to the default bucket.
    pub fn delete(&mut self, key: impl Into<Vec<u8>>) {
        self.operations.push(BatchOperation::Delete {
            bucket: DEFAULT_BUCKET_NAME.to_owned(),
            key: key.into(),
        });
    }

    /// Adds a point delete for a named bucket.
    pub fn delete_bucket(
        &mut self,
        bucket: impl Into<String>,
        key: impl Into<Vec<u8>>,
    ) -> Result<()> {
        let bucket = bucket.into();
        validate_named_bucket(&bucket)?;
        self.operations.push(BatchOperation::Delete {
            bucket,
            key: key.into(),
        });
        Ok(())
    }

    /// Adds a range delete to the default bucket.
    pub fn delete_range(&mut self, range: KeyRange) {
        self.operations.push(BatchOperation::DeleteRange {
            bucket: DEFAULT_BUCKET_NAME.to_owned(),
            range,
        });
    }

    /// Adds a range delete for a named bucket.
    pub fn delete_range_bucket(
        &mut self,
        bucket: impl Into<String>,
        range: KeyRange,
    ) -> Result<()> {
        let bucket = bucket.into();
        validate_named_bucket(&bucket)?;
        self.operations
            .push(BatchOperation::DeleteRange { bucket, range });
        Ok(())
    }

    #[must_use]
    pub fn operations(&self) -> &[BatchOperation] {
        &self.operations
    }

    #[must_use]
    pub fn into_operations(self) -> Vec<BatchOperation> {
        self.operations
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

fn validate_named_bucket(bucket: &str) -> Result<()> {
    if bucket.is_empty() {
        return Err(Error::invalid_options("bucket name cannot be empty"));
    }
    if bucket == DEFAULT_BUCKET_NAME {
        return Err(Error::invalid_options(
            "default bucket writes use default batch methods",
        ));
    }
    Ok(())
}
