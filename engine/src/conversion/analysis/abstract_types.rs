// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::fun::{FnAnalysis, FnAnalysisBody, FnKind, MethodKind};
use crate::conversion::api::ApiDetail;
use crate::conversion::api::{Api, TypeKind};
use std::collections::HashSet;

/// Spot types with pure virtual functions and mark them abstract.
pub(crate) fn mark_types_abstract(apis: &mut Vec<Api<FnAnalysis>>) {
    let abstract_types: HashSet<_> = apis
        .iter()
        .filter_map(|api| match &api.detail {
            ApiDetail::Function {
                fun: _,
                analysis:
                    FnAnalysisBody {
                        kind: FnKind::Method(self_ty_name, MethodKind::PureVirtual),
                        ..
                    },
            } => Some(self_ty_name.clone()),
            _ => None,
        })
        .collect();
    if abstract_types.is_empty() {
        return;
    }

    for api in apis.iter_mut() {
        let tyname = api.typename();
        match &mut api.detail {
            ApiDetail::Type { analysis, .. } if abstract_types.contains(&tyname) => {
                *analysis = TypeKind::Abstract;
            }
            _ => {}
        }
    }

    // We also need to remove any constructors belonging to these
    // abstract types.
    apis.retain(|api| {
        !matches!(&api.detail,
        ApiDetail::Function {
            fun: _,
            analysis:
                FnAnalysisBody {
                    kind: FnKind::Method(self_ty, MethodKind::Constructor),
                    ..
                },
        } if abstract_types.contains(&self_ty))
    })
}
