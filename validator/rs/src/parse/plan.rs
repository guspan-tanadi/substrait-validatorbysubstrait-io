// SPDX-License-Identifier: Apache-2.0

//! Module providing toplevel parse/validation functions for plans.

use crate::input::proto::substrait;
use crate::output::diagnostic;
use crate::parse::context;
use crate::parse::extensions;
use crate::parse::relations;

// Parse a relation root, i.e. a toplevel relation that includes field name
// information.
fn parse_rel_root(x: &substrait::RelRoot, y: &mut context::Context) -> diagnostic::Result<()> {
    proto_required_field!(x, y, input, relations::parse_rel);
    proto_repeated_field!(x, y, names);
    Ok(())
}

// Parse a relation type.
fn parse_rel_type(
    x: &substrait::plan_rel::RelType,
    y: &mut context::Context,
) -> diagnostic::Result<()> {
    match x {
        substrait::plan_rel::RelType::Rel(x) => relations::parse_rel(x, y),
        substrait::plan_rel::RelType::Root(x) => parse_rel_root(x, y),
    }
}

/// Parse a PlanRel node.
fn parse_plan_rel(x: &substrait::PlanRel, y: &mut context::Context) -> diagnostic::Result<()> {
    relation_root!(y, |y| {
        proto_required_field!(x, y, rel_type, parse_rel_type);
    });
    Ok(())
}

/// Toplevel parse function for a plan.
pub fn parse_plan(x: &substrait::Plan, y: &mut context::Context) -> diagnostic::Result<()> {
    // Handle extensions that we need in order to understand the relations
    // first.
    extensions::parse_extensions_before_relations(x, y);

    // Handle the relations.
    let num_relations = proto_repeated_field!(x, y, relations, parse_plan_rel)
        .0
        .len();
    if num_relations == 0 {
        diagnostic!(
            y,
            Error,
            RelationRootMissing,
            "a plan must have at least one relation"
        );
    }

    // Handle the remainder of the extensions after parsing.
    extensions::parse_extensions_after_relations(x, y);

    Ok(())
}