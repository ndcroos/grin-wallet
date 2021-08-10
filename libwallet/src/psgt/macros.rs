// Copyright 2021 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Code adapted from https://github.com/rust-bitcoin/rust-bitcoin/blob/master/src/util/psbt/macros.rs

#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! impl_psgt_insert_pair {
    ($slf:ident.$unkeyed_name:ident <= <$raw_key:ident: _>|<$raw_value:ident: $unkeyed_value_type:ty>) => {
        if $raw_key.key.is_empty() {
            if $slf.$unkeyed_name.is_none() {
                let val: $unkeyed_value_type = $crate::psgt::serialize::Deserialize::deserialize(&$raw_value)?;
                $slf.$unkeyed_name = Some(val)
            } else {
                return Err($crate::psgt::Error::DuplicateKey($raw_key).into());
            }
        } else {
            return Err($crate::psgt::Error::InvalidKey($raw_key).into());
        }
    };
    ($slf:ident.$keyed_name:ident <= <$raw_key:ident: $keyed_key_type:ty>|<$raw_value:ident: $keyed_value_type:ty>) => {
        if !$raw_key.key.is_empty() {
            let key_val: $keyed_key_type = $crate::psgt::serialize::Deserialize::deserialize(&$raw_key.key)?;
            match $slf.$keyed_name.entry(key_val) {
                $crate::prelude::btree_map::Entry::Vacant(empty_key) => {
                    let val: $keyed_value_type = $crate::psgt::serialize::Deserialize::deserialize(&$raw_value)?;
                    empty_key.insert(val);
                }
                $crate::prelude::btree_map::Entry::Occupied(_) => return Err($crate::psgt::Error::DuplicateKey($raw_key).into()),
            }
        } else {
            return Err($crate::psgt::Error::InvalidKey($raw_key).into());
        }
    };
}
