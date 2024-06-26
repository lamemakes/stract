// Stract is an open source web search engine.
// Copyright (C) 2023 Stract ApS
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{collections::HashMap, sync::Arc};

use tantivy::{columnar::ColumnValues, DocId, SegmentId};

use crate::{
    enum_map::EnumMap,
    schema::{fast_field::FastField, DataType, FastFieldEnum, Field},
};

#[derive(Default, Clone)]
struct InnerFastFieldReader {
    segments: HashMap<SegmentId, Arc<SegmentReader>>,
}

#[derive(Default, Clone)]
pub struct FastFieldReader {
    inner: Arc<InnerFastFieldReader>,
}

impl FastFieldReader {
    pub fn get_segment(&self, segment: &SegmentId) -> Arc<SegmentReader> {
        Arc::clone(self.inner.segments.get(segment).unwrap())
    }

    pub fn borrow_segment(&self, segment: &SegmentId) -> &SegmentReader {
        self.inner.segments.get(segment).unwrap()
    }
}

impl FastFieldReader {
    pub fn new(tv_searcher: &tantivy::Searcher) -> Self {
        let mut segments = HashMap::new();

        for reader in tv_searcher.segment_readers() {
            let fastfield_readers = reader.fast_fields();

            let mut u64s = EnumMap::new();
            let mut bytes = EnumMap::new();

            for field in Field::all().filter_map(|f| f.as_fast()) {
                match field.data_type() {
                    DataType::U64 => {
                        let num_docs = reader.max_doc() as usize;
                        let mut data = vec![0; num_docs];
                        if let Ok(field_reader) = fastfield_readers.u64(field.name()) {
                            for (doc, elem) in data.iter_mut().enumerate() {
                                *elem = field_reader.values.get_val(doc as u32);
                            }
                        }

                        u64s.insert(field, data);
                    }
                    DataType::Bytes => {
                        if let Some(reader) = fastfield_readers.bytes(field.name()).ok().flatten() {
                            bytes.insert(field, reader);
                        }
                    }
                };
            }

            segments.insert(
                reader.segment_id(),
                Arc::new(SegmentReader {
                    field_readers: AllReaders { u64s, bytes },
                }),
            );
        }

        Self {
            inner: Arc::new(InnerFastFieldReader { segments }),
        }
    }
}

struct AllReaders {
    u64s: EnumMap<FastFieldEnum, Vec<u64>>,
    bytes: EnumMap<FastFieldEnum, tantivy::columnar::BytesColumn>,
}

pub enum Value {
    U64(u64),
    Bytes(Vec<u8>),
}

impl Value {
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::U64(val) => Some(*val),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Value::Bytes(val) => Some(val),
            _ => None,
        }
    }
}

impl From<u64> for Value {
    fn from(val: u64) -> Self {
        Value::U64(val)
    }
}

impl From<Vec<u8>> for Value {
    fn from(val: Vec<u8>) -> Self {
        Value::Bytes(val)
    }
}

impl From<Value> for Option<u64> {
    fn from(val: Value) -> Self {
        val.as_u64()
    }
}

impl<'a> From<&'a Value> for Option<&'a [u8]> {
    fn from(val: &'a Value) -> Self {
        val.as_bytes()
    }
}

impl From<Value> for Option<Vec<u8>> {
    fn from(val: Value) -> Self {
        match val {
            Value::Bytes(val) => Some(val),
            _ => None,
        }
    }
}

pub struct FieldReader<'a> {
    readers: &'a AllReaders,
    doc: DocId,
}

impl<'a> FieldReader<'a> {
    pub fn get(&self, field: FastFieldEnum) -> Option<Value> {
        match field.data_type() {
            DataType::U64 => Some(
                self.readers
                    .u64s
                    .get(field)?
                    .get(self.doc as usize)
                    .copied()?
                    .into(),
            ),

            DataType::Bytes => {
                let reader = self.readers.bytes.get(field)?;
                let ord = reader.ords().values.get_val(self.doc);

                if ord > reader.num_terms() as u64 || reader.num_terms() == 0 {
                    return None;
                }

                let mut bytes = Vec::new();
                reader.ord_to_bytes(ord, &mut bytes).unwrap();

                if bytes.is_empty() {
                    None
                } else {
                    Some(bytes.into())
                }
            }
        }
    }
}

pub struct SegmentReader {
    field_readers: AllReaders,
}

impl SegmentReader {
    pub fn get_field_reader(&self, doc: DocId) -> FieldReader<'_> {
        FieldReader {
            readers: &self.field_readers,
            doc,
        }
    }
}
