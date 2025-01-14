// Copyright 2023 Greptime Team
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

use async_trait::async_trait;
use common_meta::RegionIdent;
use common_procedure::Status;
use serde::{Deserialize, Serialize};

use super::{RegionFailoverContext, State};
use crate::error::Result;

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct RegionFailoverEnd;

#[async_trait]
#[typetag::serde]
impl State for RegionFailoverEnd {
    async fn next(
        mut self: Box<Self>,
        _: &RegionFailoverContext,
        _: &RegionIdent,
    ) -> Result<Box<dyn State>> {
        Ok(self)
    }

    fn status(&self) -> Status {
        Status::Done
    }
}
