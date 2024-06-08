#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Default)]
#[serde(default)]
pub struct CargoGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_common_metadata: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_crate_versions: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_feature_names: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_feature_names: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wildcard_dependencies: Option<crate::lints::LintLevel>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Default)]
#[serde(default)]
pub struct ComplexityGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_instead_of_map: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool_comparison: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_deref_ref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrowed_box: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_count_to_len: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_lit_as_u8: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_on_copy: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crosspointer_transmute: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_constructed_unit_structs: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_cfg_attr: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deref_addrof: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derivable_impls: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diverging_sub_expression: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_comparisons: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_parens: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_subsec: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excessive_nesting: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_auto_deref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_counter_loop: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_write: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_unused_lifetimes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_unused_type_parameters: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_map_identity: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_next: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_map_identity: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_last_with_len: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_op: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implied_bounds_in_impls: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inspect_for_each: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_plus_one: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_count: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_kv_map: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub let_with_type_underscore: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_clamp: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_filter: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_filter_map: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_find: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_find_map: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_flatten: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_hash_one: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_main_separator_str: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_range_patterns: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_rem_euclid: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_slice_size_calculation: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_split_once: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_strip: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_swap: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_unwrap_or: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_flatten: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_identity: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_as_ref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_single_binding: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_arbitrary_self_type: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_bool: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_bool_assign: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_borrowed_reference: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_if: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_lifetimes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_match: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_option_as_deref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_option_take: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_question_mark: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_splitn: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_update: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neg_cmp_op_on_partial_ord: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_effect: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonminimal_bool: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_used_in_recursion: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_as_ref_deref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_filter_map: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_map_unit_fn: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_then_unwrap: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overflow_check_conditional: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partialeq_ne_impl: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precedence: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ptr_offset_with_cast: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_zip_with_len: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_as_str: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_async_block: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_at_rest_pattern: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_closure_call: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_guards: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_slicing: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_once: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserve_after_initialization: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_filter_map: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_map_unit_fn: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_is_some: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seek_from_current: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seek_to_start_instead_of_rewind: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_circuit_statement: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_element_loop: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_while_next: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_from_utf8_as_bytes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strlen_on_c_strings: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary_assignment: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub too_many_arguments: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmute_bytes_to_str: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmute_float_to_int: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmute_int_to_bool: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmute_int_to_char: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmute_int_to_float: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmute_int_to_non_zero: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmute_num_to_bytes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmute_ptr_to_ref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmutes_expressible_as_ptr_casts: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_complexity: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_arg: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_cast: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_filter_map: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_find_map: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_literal_unwrap: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_map_on_constructor: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_operation: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_sort_by: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_unwrap: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unneeded_wildcard_pattern: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_format_specs: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub useless_asref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub useless_conversion: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub useless_format: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub useless_transmute: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vec_box: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub while_let_loop: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wildcard_in_or_patterns: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_divided_by_zero: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_prefixed_literal: Option<crate::lints::LintLevel>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Default)]
#[serde(default)]
pub struct CorrectnessGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absurd_extreme_comparisons: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub almost_swapped: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approx_constant: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_yields_async: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bad_bit_mask: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_slice_different_sizes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_semver: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derive_ord_xor_partial_ord: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derived_hash_with_manual_eq: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eager_transmute: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_clike_unportable_variant: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq_op: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub erasing_op: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fn_address_comparisons: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_let_mutex: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifs_same_cond: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impl_hash_borrow_with_str_and_bytes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impossible_comparisons: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ineffective_bit_mask: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infinite_iter: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherent_to_string_shadow_display: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_fn_without_body: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_null_ptr_usage: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_regex: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invisible_characters: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_next_loop: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_skip_zero: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator_step_by_zero: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub let_underscore_lock: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lint_groups_priority: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_str_case_mismatch: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_replace_with_uninit: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_max: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mistyped_literal_suffixes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modulo_one: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mut_from_ref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_loop: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_octal_unix_permissions: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonsensical_open_options: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_unsafe_ptr_arg_deref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_env_unwrap: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_bounds_indexing: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overly_complex_bool_expr: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panicking_unwrap: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible_missing_comma: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_line_without_trim: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive_format_impl: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_comparisons: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_locals: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversed_empty_ranges: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_assignment: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serde_api_misuse: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_of_in_element_count: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious_splitn: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmute_null_to_fn: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmuting_null: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uninit_assumed_init: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uninit_vec: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cmp: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_hash: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_return_expecting_ord: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsound_collection_transmute: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_io_amount: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub useless_attribute: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vec_resize_to_zero: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub while_immutable_condition: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrong_transmute: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zst_offset: Option<crate::lints::LintLevel>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Default)]
#[serde(default)]
pub struct DeprecatedGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_ops: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend_from_slice: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_map: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub find_map: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_let_redundant_pattern_matching: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_misused_cfg: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misaligned_transmute: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mismatched_target_os: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_enum_variant_names: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_step_by_zero: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_macro: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_consts: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_assert_eq: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsafe_vector_initialization: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstable_as_mut_slice: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unstable_as_slice: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_collect: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrong_pub_self_convention: Option<crate::lints::LintLevel>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Default)]
#[serde(default)]
pub struct NurseryGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_ptr_cast_mut: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches_sharing_code: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_with_drain: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognitive_complexity: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_is_never_read: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_assert_with_mut_call: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derive_partial_eq_without_eq: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_line_after_doc_comments: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_line_after_outer_attr: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equatable_if_let: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallible_impl_from: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_not_send: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imprecise_flops: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_on_empty_collections: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_on_single_items: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_with_drain: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_stack_frames: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_const_for_fn: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutex_integer: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_collect: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_pass_by_ref_mut: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_send_fields_in_send_ty: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonstandard_macro_braces: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_if_let_else: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_fun_call: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_buf_push_overwrite: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_zero_byte_vec: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_clone: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_pub_crate: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub significant_drop_in_scrutinee: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub significant_drop_tightening: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_lit_as_bytes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suboptimal_flops: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious_operation_groupings: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_empty_array: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trait_duplication_in_bounds: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmute_undefined_repr: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trivial_regex: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuple_array_conversions: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_repetition_in_bounds: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uninhabited_references: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_struct_initialization: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_peekable: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_rounding: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_self: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub useless_let_if_seq: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub while_float: Option<crate::lints::LintLevel>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Default)]
#[serde(default)]
pub struct PedanticGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigning_clones: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool_to_int_with_if: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_as_ptr: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive_file_extension_comparisons: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_lossless: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_possible_truncation: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_possible_wrap: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_precision_loss: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_ptr_alignment: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_sign_loss: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_conversions: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloned_instead_of_copied: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_iterator: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_trait_access: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_link_with_quotes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_markdown: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_enum: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_glob_use: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expl_impl_clone_on_copy: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_deref_methods: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_into_iter_loop: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_iter_loop: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_map_next: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_map_option: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_cmp: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fn_params_excessive_bools: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_iter_instead_of_collect: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_not_else: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_unit_patterns: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_clone: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_hasher: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inconsistent_struct_constructor: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_refutable_slice: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inefficient_to_string: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_always: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub into_iter_without_iter: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_upcast_comparisons: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_after_statements: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_filter_is_ok: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_filter_is_some: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_not_returning_iterator: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_without_into_iter: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_digit_groups: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_futures: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_stack_arrays: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_types_passed_by_value: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedlist: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macro_use_imports: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_assert: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_c_str_literals: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_instant_elapsed: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_is_variant_and: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_let_else: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_ok_or: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_string_new: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub many_single_char_names: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_unwrap_or: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_bool: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_on_vec_items: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_same_arms: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_wild_err_arm: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_wildcard_for_single_variants: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_infinite_iter: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mismatching_type_param_order: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_errors_doc: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_fields_in_debug: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_panics_doc: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name_repetitions: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_use_candidate: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mut_mut: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naive_bytecount: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_bitwise_bool: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_continue: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_for_each: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_pass_by_value: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_raw_string_hashes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_effect_underscore_binding: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_mangle_with_rust_abi: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_as_ref_cloned: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_option: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ptr_as_ptr: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ptr_cast_constness: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_underscore_fields: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_minus_one: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_plus_one: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_closure_for_method_calls: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_else: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_as_ptr: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_binding_to_reference: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_option_ref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_self_not_must_use: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_functions_in_if_condition: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semicolon_if_nothing_returned: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_panic_without_expect: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similar_names: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_char_pattern: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_match_else: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stable_sort_primitive: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str_split_at_newline: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_add_assign: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub struct_excessive_bools: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub struct_field_names: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub too_many_lines: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmute_ptr_to_ptr: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trivially_copy_pass_by_ref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unchecked_duration_subtraction: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_not_nfc: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uninlined_format_args: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_box_returns: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_join: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_wraps: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnested_or_patterns: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreadable_literal: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsafe_derive_deserialize: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_async: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_self: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_underscore_binding: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose_bit_mask: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wildcard_imports: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_sized_map_values: Option<crate::lints::LintLevel>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Default)]
#[serde(default)]
pub struct PerfGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_collection: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boxed_local: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmp_owned: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapsible_str_replace: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drain_collect: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_fun_call: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend_with_drain: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_collect: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_in_format_args: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_overeager_cloned: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_const_arrays: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_enum_variant: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_memcpy: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_retain: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_str_repeat: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_try_fold: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_entry: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_spin_loop: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_write_lock: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_allocation: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_large_err: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_vector_initialization: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_local_initializer_can_be_made_const: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_string_in_format_args: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_to_owned: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub useless_vec: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vec_init_then_push: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waker_clone_wake: Option<crate::lints::LintLevel>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Default)]
#[serde(default)]
pub struct RestrictionGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_paths: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_instead_of_core: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_attributes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_attributes_without_reason: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arithmetic_side_effects: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_conversions: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_underscore: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertions_on_result_states: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub big_endian_bytes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_on_ref_ptr: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_dir: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbg_macro: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_literal_representation: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_numeric_fallback: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_union_representation: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deref_by_slicing: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disallowed_script_idents: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub else_if_without_else: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_drop: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_enum_variants_with_brackets: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_structs_with_brackets: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_impl_error: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exhaustive_enums: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exhaustive_structs: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_used: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filetype_is_file: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_arithmetic: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_cmp_const: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fn_to_numeric_cast_any: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_push_string: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_unwrap: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_endian_bytes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_then_some_else_none: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impl_trait_in_params: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_return: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexing_slicing: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infinite_loop: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_asm_x86_att_syntax: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_asm_x86_intel_syntax: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_division: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_division_remainder_used: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_over_hash_type: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_include_file: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub let_underscore_must_use: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub let_underscore_untyped: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub little_endian_bytes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lossy_float_literal: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_err_ignore: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_forget: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ident_chars: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_assert_message: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_asserts_for_indexing: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_docs_in_private_items: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_inline_in_public_items: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_trait_methods: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed_read_write_in_expression: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_module_files: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modulo_arithmetic: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_inherent_impl: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_unsafe_ops_per_block: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutex_atomic: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_raw_strings: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_ascii_literal: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panic: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panic_in_result_fn: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_pub_fields: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_type_mismatch: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_stderr: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_stdout: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_use: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_with_shorthand: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_without_shorthand: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_mark_used: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rc_buffer: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rc_mutex: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_type_annotations: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_patterns: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renamed_function_params: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_pat_in_fully_bound_structs: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_name_method: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_named_module_files: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semicolon_inside_block: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semicolon_outside_block: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separated_literal_suffix: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_reuse: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_same: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_unrelated: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_call_fn: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_char_lifetime_names: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_instead_of_alloc: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_instead_of_core: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str_to_string: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_add: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_lit_chars_any: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_slice: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_to_string: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious_xor_used_as_pow: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tests_outside_test_module: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub todo: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub try_err: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undocumented_unsafe_blocks: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unimplemented: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_safety_comment: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_safety_doc: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_self_imports: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unneeded_field_pattern: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreachable: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unseparated_literal_suffix: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unwrap_in_result: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unwrap_used: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_debug: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose_file_reads: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wildcard_enum_match_arm: Option<crate::lints::LintLevel>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Default)]
#[serde(default)]
pub struct StyleGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertions_on_constants: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_op_pattern: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks_in_conditions: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool_assert_comparison: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_interior_mutable_const: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_default: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builtin_type_shadow: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_nth: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chars_last_cmp: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chars_next_cmp: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmp_null: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapsible_else_if: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapsible_if: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapsible_match: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_chain: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_to_empty: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declare_interior_mutable_const: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_instead_of_iter_empty: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disallowed_macros: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disallowed_methods: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disallowed_names: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disallowed_types: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_lazy_continuation: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_must_use: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_neg: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_underscore_argument: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_variant_names: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_expect: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excessive_precision: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_reassign_with_default: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_map_bool_then: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fn_to_numeric_cast: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fn_to_numeric_cast_with_truncation: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_kv_map: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_over_into: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_str_radix_10: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_first: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_same_then_else: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_saturating_add: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_saturating_sub: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inconsistent_digit_grouping: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infallible_destructuring_match: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherent_to_string: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_numbered_fields: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub into_iter_on_ref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_digit_ascii_radix: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_after_test_module: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_cloned_collect: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_next_slice: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_nth: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_nth_zero: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_skip_next: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub just_underscores_and_digits: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_numeric_constants: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len_without_is_empty: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len_zero: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub let_and_return: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub let_unit_value: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_recursion: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_async_fn: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_bits: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_is_ascii_check: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_is_finite: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_is_infinite: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_map: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_next_back: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_non_exhaustive: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_range_contains: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_saturating_arithmetic: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_while_let_some: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_clone: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_collect_result_unit: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_like_matches_macro: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_overlapping_arm: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_ref_pats: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_result_ok: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_replace_option_with_none: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_replace_with_default: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_enforced_import_renames: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_safety_doc: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed_attributes_style: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed_case_hex_literals: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_inception: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_use_unit: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mut_mutex_lock: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_borrow: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_borrows_for_generic_args: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_doctest_main: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_else: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_late_init: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_parens_on_range_literals: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_pub_self: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_range_loop: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_return: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_return_with_question_mark: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neg_multiply: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_ret_no_self: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_without_default: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_minimal_cfg: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated_if_else: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok_expect: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_ref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_map_or_err_ok: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_map_or_none: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partialeq_to_none: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_literal: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_with_newline: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub println_empty_string: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ptr_arg: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ptr_eq: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_mark: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_closure: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_field_names: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_pattern: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_pattern_matching: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_static_lifetimes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_map_or_into_option: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_unit_err: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_item_push: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_named_constructors: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_implement_trait: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_char_add_str: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_component_path_imports: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_match: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_extend_chars: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tabs_in_doc_comments: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_digit_is_some: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_string_trait_impl: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toplevel_ref_arg: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_split_whitespace: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_fallible_conversions: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_fold: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_lazy_evaluations: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_mut_passed: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_owned_empty_strings: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsafe_removed_from_name: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_enumerate_index: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_unit: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unusual_byte_groupings: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unwrap_or_default: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_case_acronyms: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub while_let_on_iterator: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_literal: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_with_newline: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writeln_empty_string: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrong_self_convention: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_ptr: Option<crate::lints::LintLevel>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Default)]
#[serde(default)]
pub struct SuspiciousGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub almost_complete_range: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_with_non_send_sync: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub await_holding_invalid_type: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub await_holding_lock: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub await_holding_refcell_ref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blanket_clippy_restriction_lints: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_abs_to_unsigned: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_enum_constructor: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_enum_truncation: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_nan_to_int: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_slice_from_raw_parts: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub const_is_empty: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crate_in_macro_def: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_clippy_cfg_attr: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_non_drop: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_mod: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicated_attributes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_docs: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_loop: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_equality_without_abs: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forget_non_drop: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub four_forward_slashes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_raw_with_void_ptr: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incompatible_msrv: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ineffective_open_options: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_out_of_bounds: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_absolute_paths: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub let_underscore_future: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_filter_map_ok: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macro_metavars_in_unsafe: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_unwrap_or_default: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misnamed_getters: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misrefactored_assign_op: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_transmute_annotations: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_assignments: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_bound_locations: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mut_range_bound: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutable_key_type: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_character_iteration: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needless_maybe_sized: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_effect_replace: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_canonical_clone_impl: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_canonical_partial_ord_impl: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub octal_escapes: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_ends_with_ext: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_set_readonly_false: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_in_format_impl: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rc_clone_in_vec_init: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_vec_with_capacity: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_range_in_vec_init: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_of_ref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious_arithmetic_impl: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious_assignment_formatting: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious_command_arg_space: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious_doc_comments: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious_else_formatting: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious_map: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious_op_assign_impl: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious_open_options: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious_to_owned: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious_unary_op_formatting: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_ptr_to_ref: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_attr_in_doctest: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_id_on_box: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unconditional_recursion: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_clippy_cfg: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_get_then_check: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unnecessary_result_map_or_else: Option<crate::lints::LintLevel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_repeat_side_effects: Option<crate::lints::LintLevel>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Default)]
pub struct Config {
    #[serde(rename = "$schema", default = "crate::config::default_schema_location")]
    pub schema: String,

    pub cargo: CargoGroup,
    pub complexity: ComplexityGroup,
    pub correctness: CorrectnessGroup,
    pub deprecated: DeprecatedGroup,
    pub nursery: NurseryGroup,
    pub pedantic: PedanticGroup,
    pub perf: PerfGroup,
    pub restriction: RestrictionGroup,
    pub style: StyleGroup,
    pub suspicious: SuspiciousGroup,
}
