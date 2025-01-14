// Copyright 2020 Google Inc. All Rights Reserved.
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

use crate::io::{Decode, DecodeError, DecodeErrorKind, Encode};
use crate::visit::Visit;

encode_decode_as!(bool, {
    false <=> 0_u8,
    true <=> 1_u8,
}, |discriminant| {
    Err(DecodeErrorKind::UnsupportedDiscriminant { ty: "bool", discriminant: discriminant.into() }.into())
});

impl Visit for bool {}
