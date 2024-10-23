use ::libc;
pub type dwarf_tag = libc::c_uint;
pub const DW_TAG_PGI_interface_block: dwarf_tag = 40992;
pub const DW_TAG_PGI_kanji_type: dwarf_tag = 40960;
pub const DW_TAG_upc_relaxed_type: dwarf_tag = 34663;
pub const DW_TAG_upc_strict_type: dwarf_tag = 34662;
pub const DW_TAG_upc_shared_type: dwarf_tag = 34661;
pub const DW_TAG_GNU_call_site_parameter: dwarf_tag = 16650;
pub const DW_TAG_GNU_call_site: dwarf_tag = 16649;
pub const DW_TAG_GNU_formal_parameter_pack: dwarf_tag = 16648;
pub const DW_TAG_GNU_template_parameter_pack: dwarf_tag = 16647;
pub const DW_TAG_GNU_template_template_param: dwarf_tag = 16646;
pub const DW_TAG_GNU_EINCL: dwarf_tag = 16645;
pub const DW_TAG_GNU_BINCL: dwarf_tag = 16644;
pub const DW_TAG_class_template: dwarf_tag = 16643;
pub const DW_TAG_function_template: dwarf_tag = 16642;
pub const DW_TAG_format_label: dwarf_tag = 16641;
pub const DW_TAG_HP_Bliss_field_set: dwarf_tag = 16530;
pub const DW_TAG_HP_Bliss_field: dwarf_tag = 16529;
pub const DW_TAG_HP_array_descriptor: dwarf_tag = 16528;
pub const DW_TAG_MIPS_loop: dwarf_tag = 16513;
pub const DW_TAG_hi_user: dwarf_tag = 65535;
pub const DW_TAG_lo_user: dwarf_tag = 16512;
pub const DW_TAG_immutable_type: dwarf_tag = 75;
pub const DW_TAG_skeleton_unit: dwarf_tag = 74;
pub const DW_TAG_call_site_parameter: dwarf_tag = 73;
pub const DW_TAG_call_site: dwarf_tag = 72;
pub const DW_TAG_atomic_type: dwarf_tag = 71;
pub const DW_TAG_dynamic_type: dwarf_tag = 70;
pub const DW_TAG_generic_subrange: dwarf_tag = 69;
pub const DW_TAG_coarray_type: dwarf_tag = 68;
pub const DW_TAG_template_alias: dwarf_tag = 67;
pub const DW_TAG_rvalue_reference_type: dwarf_tag = 66;
pub const DW_TAG_type_unit: dwarf_tag = 65;
pub const DW_TAG_shared_type: dwarf_tag = 64;
pub const DW_TAG_condition: dwarf_tag = 63;
pub const DW_TAG_imported_unit: dwarf_tag = 61;
pub const DW_TAG_partial_unit: dwarf_tag = 60;
pub const DW_TAG_unspecified_type: dwarf_tag = 59;
pub const DW_TAG_imported_module: dwarf_tag = 58;
pub const DW_TAG_namespace: dwarf_tag = 57;
pub const DW_TAG_interface_type: dwarf_tag = 56;
pub const DW_TAG_restrict_type: dwarf_tag = 55;
pub const DW_TAG_dwarf_procedure: dwarf_tag = 54;
pub const DW_TAG_volatile_type: dwarf_tag = 53;
pub const DW_TAG_variable: dwarf_tag = 52;
pub const DW_TAG_variant_part: dwarf_tag = 51;
pub const DW_TAG_try_block: dwarf_tag = 50;
pub const DW_TAG_thrown_type: dwarf_tag = 49;
pub const DW_TAG_template_value_param: dwarf_tag = 48;
pub const DW_TAG_template_type_param: dwarf_tag = 47;
pub const DW_TAG_subprogram: dwarf_tag = 46;
pub const DW_TAG_packed_type: dwarf_tag = 45;
pub const DW_TAG_namelist_item: dwarf_tag = 44;
pub const DW_TAG_namelist: dwarf_tag = 43;
pub const DW_TAG_friend: dwarf_tag = 42;
pub const DW_TAG_file_type: dwarf_tag = 41;
pub const DW_TAG_enumerator: dwarf_tag = 40;
pub const DW_TAG_constant: dwarf_tag = 39;
pub const DW_TAG_const_type: dwarf_tag = 38;
pub const DW_TAG_catch_block: dwarf_tag = 37;
pub const DW_TAG_base_type: dwarf_tag = 36;
pub const DW_TAG_access_declaration: dwarf_tag = 35;
pub const DW_TAG_with_stmt: dwarf_tag = 34;
pub const DW_TAG_subrange_type: dwarf_tag = 33;
pub const DW_TAG_set_type: dwarf_tag = 32;
pub const DW_TAG_ptr_to_member_type: dwarf_tag = 31;
pub const DW_TAG_module: dwarf_tag = 30;
pub const DW_TAG_inlined_subroutine: dwarf_tag = 29;
pub const DW_TAG_inheritance: dwarf_tag = 28;
pub const DW_TAG_common_inclusion: dwarf_tag = 27;
pub const DW_TAG_common_block: dwarf_tag = 26;
pub const DW_TAG_variant: dwarf_tag = 25;
pub const DW_TAG_unspecified_parameters: dwarf_tag = 24;
pub const DW_TAG_union_type: dwarf_tag = 23;
pub const DW_TAG_typedef: dwarf_tag = 22;
pub const DW_TAG_subroutine_type: dwarf_tag = 21;
pub const DW_TAG_structure_type: dwarf_tag = 19;
pub const DW_TAG_string_type: dwarf_tag = 18;
pub const DW_TAG_compile_unit: dwarf_tag = 17;
pub const DW_TAG_reference_type: dwarf_tag = 16;
pub const DW_TAG_pointer_type: dwarf_tag = 15;
pub const DW_TAG_member: dwarf_tag = 13;
pub const DW_TAG_lexical_block: dwarf_tag = 11;
pub const DW_TAG_label: dwarf_tag = 10;
pub const DW_TAG_imported_declaration: dwarf_tag = 8;
pub const DW_TAG_formal_parameter: dwarf_tag = 5;
pub const DW_TAG_enumeration_type: dwarf_tag = 4;
pub const DW_TAG_entry_point: dwarf_tag = 3;
pub const DW_TAG_class_type: dwarf_tag = 2;
pub const DW_TAG_array_type: dwarf_tag = 1;
pub const DW_TAG_padding: dwarf_tag = 0;
pub type dwarf_form = libc::c_uint;
pub const DW_FORM_GNU_strp_alt: dwarf_form = 7969;
pub const DW_FORM_GNU_ref_alt: dwarf_form = 7968;
pub const DW_FORM_GNU_str_index: dwarf_form = 7938;
pub const DW_FORM_GNU_addr_index: dwarf_form = 7937;
pub const DW_FORM_addrx4: dwarf_form = 44;
pub const DW_FORM_addrx3: dwarf_form = 43;
pub const DW_FORM_addrx2: dwarf_form = 42;
pub const DW_FORM_addrx1: dwarf_form = 41;
pub const DW_FORM_strx4: dwarf_form = 40;
pub const DW_FORM_strx3: dwarf_form = 39;
pub const DW_FORM_strx2: dwarf_form = 38;
pub const DW_FORM_strx1: dwarf_form = 37;
pub const DW_FORM_ref_sup8: dwarf_form = 36;
pub const DW_FORM_rnglistx: dwarf_form = 35;
pub const DW_FORM_loclistx: dwarf_form = 34;
pub const DW_FORM_implicit_const: dwarf_form = 33;
pub const DW_FORM_line_strp: dwarf_form = 31;
pub const DW_FORM_data16: dwarf_form = 30;
pub const DW_FORM_strp_sup: dwarf_form = 29;
pub const DW_FORM_ref_sup4: dwarf_form = 28;
pub const DW_FORM_addrx: dwarf_form = 27;
pub const DW_FORM_strx: dwarf_form = 26;
pub const DW_FORM_ref_sig8: dwarf_form = 32;
pub const DW_FORM_flag_present: dwarf_form = 25;
pub const DW_FORM_exprloc: dwarf_form = 24;
pub const DW_FORM_sec_offset: dwarf_form = 23;
pub const DW_FORM_indirect: dwarf_form = 22;
pub const DW_FORM_ref_udata: dwarf_form = 21;
pub const DW_FORM_ref8: dwarf_form = 20;
pub const DW_FORM_ref4: dwarf_form = 19;
pub const DW_FORM_ref2: dwarf_form = 18;
pub const DW_FORM_ref1: dwarf_form = 17;
pub const DW_FORM_ref_addr: dwarf_form = 16;
pub const DW_FORM_udata: dwarf_form = 15;
pub const DW_FORM_strp: dwarf_form = 14;
pub const DW_FORM_sdata: dwarf_form = 13;
pub const DW_FORM_flag: dwarf_form = 12;
pub const DW_FORM_data1: dwarf_form = 11;
pub const DW_FORM_block1: dwarf_form = 10;
pub const DW_FORM_block: dwarf_form = 9;
pub const DW_FORM_string: dwarf_form = 8;
pub const DW_FORM_data8: dwarf_form = 7;
pub const DW_FORM_data4: dwarf_form = 6;
pub const DW_FORM_data2: dwarf_form = 5;
pub const DW_FORM_block4: dwarf_form = 4;
pub const DW_FORM_block2: dwarf_form = 3;
pub const DW_FORM_addr: dwarf_form = 1;
pub type dwarf_attribute = libc::c_uint;
pub const DW_AT_APPLE_property: dwarf_attribute = 16365;
pub const DW_AT_APPLE_objc_complete_type: dwarf_attribute = 16364;
pub const DW_AT_APPLE_property_attribute: dwarf_attribute = 16363;
pub const DW_AT_APPLE_property_setter: dwarf_attribute = 16362;
pub const DW_AT_APPLE_property_getter: dwarf_attribute = 16361;
pub const DW_AT_APPLE_property_name: dwarf_attribute = 16360;
pub const DW_AT_APPLE_omit_frame_ptr: dwarf_attribute = 16359;
pub const DW_AT_APPLE_runtime_class: dwarf_attribute = 16358;
pub const DW_AT_APPLE_major_runtime_vers: dwarf_attribute = 16357;
pub const DW_AT_APPLE_block: dwarf_attribute = 16356;
pub const DW_AT_APPLE_isa: dwarf_attribute = 16355;
pub const DW_AT_APPLE_flags: dwarf_attribute = 16354;
pub const DW_AT_APPLE_optimized: dwarf_attribute = 16353;
pub const DW_AT_PGI_lstride: dwarf_attribute = 14850;
pub const DW_AT_PGI_soffset: dwarf_attribute = 14849;
pub const DW_AT_PGI_lbase: dwarf_attribute = 14848;
pub const DW_AT_upc_threads_scaled: dwarf_attribute = 12816;
pub const DW_AT_GNU_bias: dwarf_attribute = 8965;
pub const DW_AT_GNU_denominator: dwarf_attribute = 8964;
pub const DW_AT_GNU_numerator: dwarf_attribute = 8963;
pub const DW_AT_GNAT_descriptive_type: dwarf_attribute = 8962;
pub const DW_AT_use_GNAT_descriptive_type: dwarf_attribute = 8961;
pub const DW_AT_VMS_rtnbeg_pd_address: dwarf_attribute = 8705;
pub const DW_AT_GNU_entry_view: dwarf_attribute = 8504;
pub const DW_AT_GNU_locviews: dwarf_attribute = 8503;
pub const DW_AT_GNU_discriminator: dwarf_attribute = 8502;
pub const DW_AT_GNU_pubtypes: dwarf_attribute = 8501;
pub const DW_AT_GNU_pubnames: dwarf_attribute = 8500;
pub const DW_AT_GNU_addr_base: dwarf_attribute = 8499;
pub const DW_AT_GNU_ranges_base: dwarf_attribute = 8498;
pub const DW_AT_GNU_dwo_id: dwarf_attribute = 8497;
pub const DW_AT_GNU_dwo_name: dwarf_attribute = 8496;
pub const DW_AT_GNU_deleted: dwarf_attribute = 8474;
pub const DW_AT_GNU_macros: dwarf_attribute = 8473;
pub const DW_AT_GNU_all_source_call_sites: dwarf_attribute = 8472;
pub const DW_AT_GNU_all_call_sites: dwarf_attribute = 8471;
pub const DW_AT_GNU_all_tail_call_sites: dwarf_attribute = 8470;
pub const DW_AT_GNU_tail_call: dwarf_attribute = 8469;
pub const DW_AT_GNU_call_site_target_clobbered: dwarf_attribute = 8468;
pub const DW_AT_GNU_call_site_target: dwarf_attribute = 8467;
pub const DW_AT_GNU_call_site_data_value: dwarf_attribute = 8466;
pub const DW_AT_GNU_call_site_value: dwarf_attribute = 8465;
pub const DW_AT_GNU_template_name: dwarf_attribute = 8464;
pub const DW_AT_GNU_odr_signature: dwarf_attribute = 8463;
pub const DW_AT_GNU_shared_locks_required: dwarf_attribute = 8462;
pub const DW_AT_GNU_exclusive_locks_required: dwarf_attribute = 8461;
pub const DW_AT_GNU_locks_excluded: dwarf_attribute = 8460;
pub const DW_AT_GNU_pt_guarded: dwarf_attribute = 8459;
pub const DW_AT_GNU_guarded: dwarf_attribute = 8458;
pub const DW_AT_GNU_pt_guarded_by: dwarf_attribute = 8457;
pub const DW_AT_GNU_guarded_by: dwarf_attribute = 8456;
pub const DW_AT_GNU_vector: dwarf_attribute = 8455;
pub const DW_AT_body_end: dwarf_attribute = 8454;
pub const DW_AT_body_begin: dwarf_attribute = 8453;
pub const DW_AT_src_coords: dwarf_attribute = 8452;
pub const DW_AT_mac_info: dwarf_attribute = 8451;
pub const DW_AT_src_info: dwarf_attribute = 8450;
pub const DW_AT_sf_names: dwarf_attribute = 8449;
pub const DW_AT_HP_is_result_param: dwarf_attribute = 8233;
pub const DW_AT_HP_default_location: dwarf_attribute = 8227;
pub const DW_AT_HP_definition_points: dwarf_attribute = 8226;
pub const DW_AT_HP_widened_byte_size: dwarf_attribute = 8225;
pub const DW_AT_HP_unit_size: dwarf_attribute = 8224;
pub const DW_AT_HP_unit_name: dwarf_attribute = 8223;
pub const DW_AT_HP_prof_flags: dwarf_attribute = 8219;
pub const DW_AT_HP_linkage_name: dwarf_attribute = 8218;
pub const DW_AT_HP_all_variables_modifiable: dwarf_attribute = 8217;
pub const DW_AT_HP_cold_region_high_pc: dwarf_attribute = 8216;
pub const DW_AT_HP_cold_region_low_pc: dwarf_attribute = 8215;
pub const DW_AT_HP_opt_flags: dwarf_attribute = 8214;
pub const DW_AT_HP_prof_version_id: dwarf_attribute = 8213;
pub const DW_AT_HP_opt_level: dwarf_attribute = 8212;
pub const DW_AT_HP_pass_by_reference: dwarf_attribute = 8211;
pub const DW_AT_HP_raw_data_ptr: dwarf_attribute = 8210;
pub const DW_AT_HP_proc_per_section: dwarf_attribute = 8209;
pub const DW_AT_HP_actuals_stmt_list: dwarf_attribute = 8208;
pub const DW_AT_HP_epilogue: dwarf_attribute = 8200;
pub const DW_AT_HP_prologue: dwarf_attribute = 8197;
pub const DW_AT_HP_unmodifiable: dwarf_attribute = 8193;
pub const DW_AT_HP_block_index: dwarf_attribute = 8192;
pub const DW_AT_MIPS_has_inlines: dwarf_attribute = 8203;
pub const DW_AT_MIPS_clone_origin: dwarf_attribute = 8202;
pub const DW_AT_MIPS_abstract_name: dwarf_attribute = 8201;
pub const DW_AT_MIPS_stride: dwarf_attribute = 8200;
pub const DW_AT_MIPS_linkage_name: dwarf_attribute = 8199;
pub const DW_AT_MIPS_software_pipeline_depth: dwarf_attribute = 8198;
pub const DW_AT_MIPS_loop_unroll_factor: dwarf_attribute = 8197;
pub const DW_AT_MIPS_epilog_begin: dwarf_attribute = 8196;
pub const DW_AT_MIPS_tail_loop_begin: dwarf_attribute = 8195;
pub const DW_AT_MIPS_loop_begin: dwarf_attribute = 8194;
pub const DW_AT_MIPS_fde: dwarf_attribute = 8193;
pub const DW_AT_hi_user: dwarf_attribute = 16383;
pub const DW_AT_lo_user: dwarf_attribute = 8192;
pub const DW_AT_loclists_base: dwarf_attribute = 140;
pub const DW_AT_defaulted: dwarf_attribute = 139;
pub const DW_AT_deleted: dwarf_attribute = 138;
pub const DW_AT_export_symbols: dwarf_attribute = 137;
pub const DW_AT_alignment: dwarf_attribute = 136;
pub const DW_AT_noreturn: dwarf_attribute = 135;
pub const DW_AT_call_data_value: dwarf_attribute = 134;
pub const DW_AT_call_data_location: dwarf_attribute = 133;
pub const DW_AT_call_target_clobbered: dwarf_attribute = 132;
pub const DW_AT_call_target: dwarf_attribute = 131;
pub const DW_AT_call_tail_call: dwarf_attribute = 130;
pub const DW_AT_call_pc: dwarf_attribute = 129;
pub const DW_AT_call_parameter: dwarf_attribute = 128;
pub const DW_AT_call_origin: dwarf_attribute = 127;
pub const DW_AT_call_value: dwarf_attribute = 126;
pub const DW_AT_call_return_pc: dwarf_attribute = 125;
pub const DW_AT_call_all_tail_calls: dwarf_attribute = 124;
pub const DW_AT_call_all_source_calls: dwarf_attribute = 123;
pub const DW_AT_call_all_calls: dwarf_attribute = 122;
pub const DW_AT_macros: dwarf_attribute = 121;
pub const DW_AT_rvalue_reference: dwarf_attribute = 120;
pub const DW_AT_reference: dwarf_attribute = 119;
pub const DW_AT_dwo_name: dwarf_attribute = 118;
pub const DW_AT_rnglists_base: dwarf_attribute = 116;
pub const DW_AT_addr_base: dwarf_attribute = 115;
pub const DW_AT_str_offsets_base: dwarf_attribute = 114;
pub const DW_AT_rank: dwarf_attribute = 113;
pub const DW_AT_string_length_byte_size: dwarf_attribute = 112;
pub const DW_AT_string_length_bit_size: dwarf_attribute = 111;
pub const DW_AT_linkage_name: dwarf_attribute = 110;
pub const DW_AT_enum_class: dwarf_attribute = 109;
pub const DW_AT_const_expr: dwarf_attribute = 108;
pub const DW_AT_data_bit_offset: dwarf_attribute = 107;
pub const DW_AT_main_subprogram: dwarf_attribute = 106;
pub const DW_AT_signature: dwarf_attribute = 105;
pub const DW_AT_recursive: dwarf_attribute = 104;
pub const DW_AT_pure: dwarf_attribute = 103;
pub const DW_AT_elemental: dwarf_attribute = 102;
pub const DW_AT_endianity: dwarf_attribute = 101;
pub const DW_AT_object_pointer: dwarf_attribute = 100;
pub const DW_AT_explicit: dwarf_attribute = 99;
pub const DW_AT_threads_scaled: dwarf_attribute = 98;
pub const DW_AT_mutable: dwarf_attribute = 97;
pub const DW_AT_picture_string: dwarf_attribute = 96;
pub const DW_AT_digit_count: dwarf_attribute = 95;
pub const DW_AT_decimal_sign: dwarf_attribute = 94;
pub const DW_AT_small: dwarf_attribute = 93;
pub const DW_AT_decimal_scale: dwarf_attribute = 92;
pub const DW_AT_binary_scale: dwarf_attribute = 91;
pub const DW_AT_description: dwarf_attribute = 90;
pub const DW_AT_call_line: dwarf_attribute = 89;
pub const DW_AT_call_file: dwarf_attribute = 88;
pub const DW_AT_call_column: dwarf_attribute = 87;
pub const DW_AT_trampoline: dwarf_attribute = 86;
pub const DW_AT_ranges: dwarf_attribute = 85;
pub const DW_AT_extension: dwarf_attribute = 84;
pub const DW_AT_use_UTF8: dwarf_attribute = 83;
pub const DW_AT_entry_pc: dwarf_attribute = 82;
pub const DW_AT_byte_stride: dwarf_attribute = 81;
pub const DW_AT_data_location: dwarf_attribute = 80;
pub const DW_AT_associated: dwarf_attribute = 79;
pub const DW_AT_allocated: dwarf_attribute = 78;
pub const DW_AT_vtable_elem_location: dwarf_attribute = 77;
pub const DW_AT_virtuality: dwarf_attribute = 76;
pub const DW_AT_variable_parameter: dwarf_attribute = 75;
pub const DW_AT_use_location: dwarf_attribute = 74;
pub const DW_AT_type: dwarf_attribute = 73;
pub const DW_AT_static_link: dwarf_attribute = 72;
pub const DW_AT_specification: dwarf_attribute = 71;
pub const DW_AT_segment: dwarf_attribute = 70;
pub const DW_AT_priority: dwarf_attribute = 69;
pub const DW_AT_namelist_items: dwarf_attribute = 68;
pub const DW_AT_macro_info: dwarf_attribute = 67;
pub const DW_AT_identifier_case: dwarf_attribute = 66;
pub const DW_AT_friend: dwarf_attribute = 65;
pub const DW_AT_frame_base: dwarf_attribute = 64;
pub const DW_AT_external: dwarf_attribute = 63;
pub const DW_AT_encoding: dwarf_attribute = 62;
pub const DW_AT_discr_list: dwarf_attribute = 61;
pub const DW_AT_declaration: dwarf_attribute = 60;
pub const DW_AT_decl_line: dwarf_attribute = 59;
pub const DW_AT_decl_file: dwarf_attribute = 58;
pub const DW_AT_decl_column: dwarf_attribute = 57;
pub const DW_AT_data_member_location: dwarf_attribute = 56;
pub const DW_AT_count: dwarf_attribute = 55;
pub const DW_AT_calling_convention: dwarf_attribute = 54;
pub const DW_AT_base_types: dwarf_attribute = 53;
pub const DW_AT_artificial: dwarf_attribute = 52;
pub const DW_AT_address_class: dwarf_attribute = 51;
pub const DW_AT_accessibility: dwarf_attribute = 50;
pub const DW_AT_abstract_origin: dwarf_attribute = 49;
pub const DW_AT_upper_bound: dwarf_attribute = 47;
pub const DW_AT_bit_stride: dwarf_attribute = 46;
pub const DW_AT_start_scope: dwarf_attribute = 44;
pub const DW_AT_return_addr: dwarf_attribute = 42;
pub const DW_AT_prototyped: dwarf_attribute = 39;
pub const DW_AT_producer: dwarf_attribute = 37;
pub const DW_AT_lower_bound: dwarf_attribute = 34;
pub const DW_AT_is_optional: dwarf_attribute = 33;
pub const DW_AT_inline: dwarf_attribute = 32;
pub const DW_AT_default_value: dwarf_attribute = 30;
pub const DW_AT_containing_type: dwarf_attribute = 29;
pub const DW_AT_const_value: dwarf_attribute = 28;
pub const DW_AT_comp_dir: dwarf_attribute = 27;
pub const DW_AT_common_reference: dwarf_attribute = 26;
pub const DW_AT_string_length: dwarf_attribute = 25;
pub const DW_AT_import: dwarf_attribute = 24;
pub const DW_AT_visibility: dwarf_attribute = 23;
pub const DW_AT_discr_value: dwarf_attribute = 22;
pub const DW_AT_discr: dwarf_attribute = 21;
pub const DW_AT_member: dwarf_attribute = 20;
pub const DW_AT_language: dwarf_attribute = 19;
pub const DW_AT_high_pc: dwarf_attribute = 18;
pub const DW_AT_low_pc: dwarf_attribute = 17;
pub const DW_AT_stmt_list: dwarf_attribute = 16;
pub const DW_AT_element_list: dwarf_attribute = 15;
pub const DW_AT_bit_size: dwarf_attribute = 13;
pub const DW_AT_bit_offset: dwarf_attribute = 12;
pub const DW_AT_byte_size: dwarf_attribute = 11;
pub const DW_AT_subscr_data: dwarf_attribute = 10;
pub const DW_AT_ordering: dwarf_attribute = 9;
pub const DW_AT_name: dwarf_attribute = 3;
pub const DW_AT_location: dwarf_attribute = 2;
pub const DW_AT_sibling: dwarf_attribute = 1;
pub type dwarf_location_atom = libc::c_uint;
pub const DW_OP_AARCH64_operation: dwarf_location_atom = 234;
pub const DW_OP_PGI_omp_thread_num: dwarf_location_atom = 248;
pub const DW_OP_HP_tls: dwarf_location_atom = 230;
pub const DW_OP_HP_unmod_range: dwarf_location_atom = 229;
pub const DW_OP_HP_mod_range: dwarf_location_atom = 228;
pub const DW_OP_HP_fltconst8: dwarf_location_atom = 227;
pub const DW_OP_HP_fltconst4: dwarf_location_atom = 226;
pub const DW_OP_HP_is_value: dwarf_location_atom = 225;
pub const DW_OP_HP_unknown: dwarf_location_atom = 224;
pub const DW_OP_GNU_variable_value: dwarf_location_atom = 253;
pub const DW_OP_GNU_const_index: dwarf_location_atom = 252;
pub const DW_OP_GNU_addr_index: dwarf_location_atom = 251;
pub const DW_OP_GNU_parameter_ref: dwarf_location_atom = 250;
pub const DW_OP_GNU_reinterpret: dwarf_location_atom = 249;
pub const DW_OP_GNU_convert: dwarf_location_atom = 247;
pub const DW_OP_GNU_deref_type: dwarf_location_atom = 246;
pub const DW_OP_GNU_regval_type: dwarf_location_atom = 245;
pub const DW_OP_GNU_const_type: dwarf_location_atom = 244;
pub const DW_OP_GNU_entry_value: dwarf_location_atom = 243;
pub const DW_OP_GNU_implicit_pointer: dwarf_location_atom = 242;
pub const DW_OP_GNU_encoded_addr: dwarf_location_atom = 241;
pub const DW_OP_GNU_uninit: dwarf_location_atom = 240;
pub const DW_OP_GNU_push_tls_address: dwarf_location_atom = 224;
pub const DW_OP_hi_user: dwarf_location_atom = 255;
pub const DW_OP_lo_user: dwarf_location_atom = 224;
pub const DW_OP_reinterpret: dwarf_location_atom = 169;
pub const DW_OP_convert: dwarf_location_atom = 168;
pub const DW_OP_xderef_type: dwarf_location_atom = 167;
pub const DW_OP_deref_type: dwarf_location_atom = 166;
pub const DW_OP_regval_type: dwarf_location_atom = 165;
pub const DW_OP_const_type: dwarf_location_atom = 164;
pub const DW_OP_entry_value: dwarf_location_atom = 163;
pub const DW_OP_constx: dwarf_location_atom = 162;
pub const DW_OP_addrx: dwarf_location_atom = 161;
pub const DW_OP_implicit_pointer: dwarf_location_atom = 160;
pub const DW_OP_stack_value: dwarf_location_atom = 159;
pub const DW_OP_implicit_value: dwarf_location_atom = 158;
pub const DW_OP_bit_piece: dwarf_location_atom = 157;
pub const DW_OP_call_frame_cfa: dwarf_location_atom = 156;
pub const DW_OP_form_tls_address: dwarf_location_atom = 155;
pub const DW_OP_call_ref: dwarf_location_atom = 154;
pub const DW_OP_call4: dwarf_location_atom = 153;
pub const DW_OP_call2: dwarf_location_atom = 152;
pub const DW_OP_push_object_address: dwarf_location_atom = 151;
pub const DW_OP_nop: dwarf_location_atom = 150;
pub const DW_OP_xderef_size: dwarf_location_atom = 149;
pub const DW_OP_deref_size: dwarf_location_atom = 148;
pub const DW_OP_piece: dwarf_location_atom = 147;
pub const DW_OP_bregx: dwarf_location_atom = 146;
pub const DW_OP_fbreg: dwarf_location_atom = 145;
pub const DW_OP_regx: dwarf_location_atom = 144;
pub const DW_OP_breg31: dwarf_location_atom = 143;
pub const DW_OP_breg30: dwarf_location_atom = 142;
pub const DW_OP_breg29: dwarf_location_atom = 141;
pub const DW_OP_breg28: dwarf_location_atom = 140;
pub const DW_OP_breg27: dwarf_location_atom = 139;
pub const DW_OP_breg26: dwarf_location_atom = 138;
pub const DW_OP_breg25: dwarf_location_atom = 137;
pub const DW_OP_breg24: dwarf_location_atom = 136;
pub const DW_OP_breg23: dwarf_location_atom = 135;
pub const DW_OP_breg22: dwarf_location_atom = 134;
pub const DW_OP_breg21: dwarf_location_atom = 133;
pub const DW_OP_breg20: dwarf_location_atom = 132;
pub const DW_OP_breg19: dwarf_location_atom = 131;
pub const DW_OP_breg18: dwarf_location_atom = 130;
pub const DW_OP_breg17: dwarf_location_atom = 129;
pub const DW_OP_breg16: dwarf_location_atom = 128;
pub const DW_OP_breg15: dwarf_location_atom = 127;
pub const DW_OP_breg14: dwarf_location_atom = 126;
pub const DW_OP_breg13: dwarf_location_atom = 125;
pub const DW_OP_breg12: dwarf_location_atom = 124;
pub const DW_OP_breg11: dwarf_location_atom = 123;
pub const DW_OP_breg10: dwarf_location_atom = 122;
pub const DW_OP_breg9: dwarf_location_atom = 121;
pub const DW_OP_breg8: dwarf_location_atom = 120;
pub const DW_OP_breg7: dwarf_location_atom = 119;
pub const DW_OP_breg6: dwarf_location_atom = 118;
pub const DW_OP_breg5: dwarf_location_atom = 117;
pub const DW_OP_breg4: dwarf_location_atom = 116;
pub const DW_OP_breg3: dwarf_location_atom = 115;
pub const DW_OP_breg2: dwarf_location_atom = 114;
pub const DW_OP_breg1: dwarf_location_atom = 113;
pub const DW_OP_breg0: dwarf_location_atom = 112;
pub const DW_OP_reg31: dwarf_location_atom = 111;
pub const DW_OP_reg30: dwarf_location_atom = 110;
pub const DW_OP_reg29: dwarf_location_atom = 109;
pub const DW_OP_reg28: dwarf_location_atom = 108;
pub const DW_OP_reg27: dwarf_location_atom = 107;
pub const DW_OP_reg26: dwarf_location_atom = 106;
pub const DW_OP_reg25: dwarf_location_atom = 105;
pub const DW_OP_reg24: dwarf_location_atom = 104;
pub const DW_OP_reg23: dwarf_location_atom = 103;
pub const DW_OP_reg22: dwarf_location_atom = 102;
pub const DW_OP_reg21: dwarf_location_atom = 101;
pub const DW_OP_reg20: dwarf_location_atom = 100;
pub const DW_OP_reg19: dwarf_location_atom = 99;
pub const DW_OP_reg18: dwarf_location_atom = 98;
pub const DW_OP_reg17: dwarf_location_atom = 97;
pub const DW_OP_reg16: dwarf_location_atom = 96;
pub const DW_OP_reg15: dwarf_location_atom = 95;
pub const DW_OP_reg14: dwarf_location_atom = 94;
pub const DW_OP_reg13: dwarf_location_atom = 93;
pub const DW_OP_reg12: dwarf_location_atom = 92;
pub const DW_OP_reg11: dwarf_location_atom = 91;
pub const DW_OP_reg10: dwarf_location_atom = 90;
pub const DW_OP_reg9: dwarf_location_atom = 89;
pub const DW_OP_reg8: dwarf_location_atom = 88;
pub const DW_OP_reg7: dwarf_location_atom = 87;
pub const DW_OP_reg6: dwarf_location_atom = 86;
pub const DW_OP_reg5: dwarf_location_atom = 85;
pub const DW_OP_reg4: dwarf_location_atom = 84;
pub const DW_OP_reg3: dwarf_location_atom = 83;
pub const DW_OP_reg2: dwarf_location_atom = 82;
pub const DW_OP_reg1: dwarf_location_atom = 81;
pub const DW_OP_reg0: dwarf_location_atom = 80;
pub const DW_OP_lit31: dwarf_location_atom = 79;
pub const DW_OP_lit30: dwarf_location_atom = 78;
pub const DW_OP_lit29: dwarf_location_atom = 77;
pub const DW_OP_lit28: dwarf_location_atom = 76;
pub const DW_OP_lit27: dwarf_location_atom = 75;
pub const DW_OP_lit26: dwarf_location_atom = 74;
pub const DW_OP_lit25: dwarf_location_atom = 73;
pub const DW_OP_lit24: dwarf_location_atom = 72;
pub const DW_OP_lit23: dwarf_location_atom = 71;
pub const DW_OP_lit22: dwarf_location_atom = 70;
pub const DW_OP_lit21: dwarf_location_atom = 69;
pub const DW_OP_lit20: dwarf_location_atom = 68;
pub const DW_OP_lit19: dwarf_location_atom = 67;
pub const DW_OP_lit18: dwarf_location_atom = 66;
pub const DW_OP_lit17: dwarf_location_atom = 65;
pub const DW_OP_lit16: dwarf_location_atom = 64;
pub const DW_OP_lit15: dwarf_location_atom = 63;
pub const DW_OP_lit14: dwarf_location_atom = 62;
pub const DW_OP_lit13: dwarf_location_atom = 61;
pub const DW_OP_lit12: dwarf_location_atom = 60;
pub const DW_OP_lit11: dwarf_location_atom = 59;
pub const DW_OP_lit10: dwarf_location_atom = 58;
pub const DW_OP_lit9: dwarf_location_atom = 57;
pub const DW_OP_lit8: dwarf_location_atom = 56;
pub const DW_OP_lit7: dwarf_location_atom = 55;
pub const DW_OP_lit6: dwarf_location_atom = 54;
pub const DW_OP_lit5: dwarf_location_atom = 53;
pub const DW_OP_lit4: dwarf_location_atom = 52;
pub const DW_OP_lit3: dwarf_location_atom = 51;
pub const DW_OP_lit2: dwarf_location_atom = 50;
pub const DW_OP_lit1: dwarf_location_atom = 49;
pub const DW_OP_lit0: dwarf_location_atom = 48;
pub const DW_OP_skip: dwarf_location_atom = 47;
pub const DW_OP_ne: dwarf_location_atom = 46;
pub const DW_OP_lt: dwarf_location_atom = 45;
pub const DW_OP_le: dwarf_location_atom = 44;
pub const DW_OP_gt: dwarf_location_atom = 43;
pub const DW_OP_ge: dwarf_location_atom = 42;
pub const DW_OP_eq: dwarf_location_atom = 41;
pub const DW_OP_bra: dwarf_location_atom = 40;
pub const DW_OP_xor: dwarf_location_atom = 39;
pub const DW_OP_shra: dwarf_location_atom = 38;
pub const DW_OP_shr: dwarf_location_atom = 37;
pub const DW_OP_shl: dwarf_location_atom = 36;
pub const DW_OP_plus_uconst: dwarf_location_atom = 35;
pub const DW_OP_plus: dwarf_location_atom = 34;
pub const DW_OP_or: dwarf_location_atom = 33;
pub const DW_OP_not: dwarf_location_atom = 32;
pub const DW_OP_neg: dwarf_location_atom = 31;
pub const DW_OP_mul: dwarf_location_atom = 30;
pub const DW_OP_mod: dwarf_location_atom = 29;
pub const DW_OP_minus: dwarf_location_atom = 28;
pub const DW_OP_div: dwarf_location_atom = 27;
pub const DW_OP_and: dwarf_location_atom = 26;
pub const DW_OP_abs: dwarf_location_atom = 25;
pub const DW_OP_xderef: dwarf_location_atom = 24;
pub const DW_OP_rot: dwarf_location_atom = 23;
pub const DW_OP_swap: dwarf_location_atom = 22;
pub const DW_OP_pick: dwarf_location_atom = 21;
pub const DW_OP_over: dwarf_location_atom = 20;
pub const DW_OP_drop: dwarf_location_atom = 19;
pub const DW_OP_dup: dwarf_location_atom = 18;
pub const DW_OP_consts: dwarf_location_atom = 17;
pub const DW_OP_constu: dwarf_location_atom = 16;
pub const DW_OP_const8s: dwarf_location_atom = 15;
pub const DW_OP_const8u: dwarf_location_atom = 14;
pub const DW_OP_const4s: dwarf_location_atom = 13;
pub const DW_OP_const4u: dwarf_location_atom = 12;
pub const DW_OP_const2s: dwarf_location_atom = 11;
pub const DW_OP_const2u: dwarf_location_atom = 10;
pub const DW_OP_const1s: dwarf_location_atom = 9;
pub const DW_OP_const1u: dwarf_location_atom = 8;
pub const DW_OP_deref: dwarf_location_atom = 6;
pub const DW_OP_addr: dwarf_location_atom = 3;
pub type dwarf_type = libc::c_uint;
pub const DW_ATE_HP_VAX_complex_float_d: dwarf_type = 144;
pub const DW_ATE_HP_VAX_complex_float: dwarf_type = 143;
pub const DW_ATE_HP_unsigned_fixed: dwarf_type = 142;
pub const DW_ATE_HP_signed_fixed: dwarf_type = 141;
pub const DW_ATE_HP_edited: dwarf_type = 140;
pub const DW_ATE_HP_zoned_decimal: dwarf_type = 139;
pub const DW_ATE_HP_packed_decimal: dwarf_type = 138;
pub const DW_ATE_HP_VAX_float_d: dwarf_type = 137;
pub const DW_ATE_HP_VAX_float: dwarf_type = 136;
pub const DW_ATE_HP_imaginary_float128: dwarf_type = 134;
pub const DW_ATE_HP_imaginary_float80: dwarf_type = 133;
pub const DW_ATE_HP_floathpintel: dwarf_type = 132;
pub const DW_ATE_HP_complex_float128: dwarf_type = 131;
pub const DW_ATE_HP_float128: dwarf_type = 130;
pub const DW_ATE_HP_complex_float80: dwarf_type = 129;
pub const DW_ATE_HP_float80: dwarf_type = 128;
pub const DW_ATE_hi_user: dwarf_type = 255;
pub const DW_ATE_lo_user: dwarf_type = 128;
pub const DW_ATE_ASCII: dwarf_type = 18;
pub const DW_ATE_UCS: dwarf_type = 17;
pub const DW_ATE_UTF: dwarf_type = 16;
pub const DW_ATE_decimal_float: dwarf_type = 15;
pub const DW_ATE_unsigned_fixed: dwarf_type = 14;
pub const DW_ATE_signed_fixed: dwarf_type = 13;
pub const DW_ATE_edited: dwarf_type = 12;
pub const DW_ATE_numeric_string: dwarf_type = 11;
pub const DW_ATE_packed_decimal: dwarf_type = 10;
pub const DW_ATE_imaginary_float: dwarf_type = 9;
pub const DW_ATE_unsigned_char: dwarf_type = 8;
pub const DW_ATE_unsigned: dwarf_type = 7;
pub const DW_ATE_signed_char: dwarf_type = 6;
pub const DW_ATE_signed: dwarf_type = 5;
pub const DW_ATE_float: dwarf_type = 4;
pub const DW_ATE_complex_float: dwarf_type = 3;
pub const DW_ATE_boolean: dwarf_type = 2;
pub const DW_ATE_address: dwarf_type = 1;
pub const DW_ATE_void: dwarf_type = 0;
pub type dwarf_call_frame_info = libc::c_uint;
pub const DW_CFA_GNU_negative_offset_extended: dwarf_call_frame_info = 47;
pub const DW_CFA_GNU_args_size: dwarf_call_frame_info = 46;
pub const DW_CFA_AARCH64_negate_ra_state: dwarf_call_frame_info = 45;
pub const DW_CFA_GNU_window_save: dwarf_call_frame_info = 45;
pub const DW_CFA_MIPS_advance_loc8: dwarf_call_frame_info = 29;
pub const DW_CFA_hi_user: dwarf_call_frame_info = 63;
pub const DW_CFA_lo_user: dwarf_call_frame_info = 28;
pub const DW_CFA_val_expression: dwarf_call_frame_info = 22;
pub const DW_CFA_val_offset_sf: dwarf_call_frame_info = 21;
pub const DW_CFA_val_offset: dwarf_call_frame_info = 20;
pub const DW_CFA_def_cfa_offset_sf: dwarf_call_frame_info = 19;
pub const DW_CFA_def_cfa_sf: dwarf_call_frame_info = 18;
pub const DW_CFA_offset_extended_sf: dwarf_call_frame_info = 17;
pub const DW_CFA_expression: dwarf_call_frame_info = 16;
pub const DW_CFA_def_cfa_expression: dwarf_call_frame_info = 15;
pub const DW_CFA_def_cfa_offset: dwarf_call_frame_info = 14;
pub const DW_CFA_def_cfa_register: dwarf_call_frame_info = 13;
pub const DW_CFA_def_cfa: dwarf_call_frame_info = 12;
pub const DW_CFA_restore_state: dwarf_call_frame_info = 11;
pub const DW_CFA_remember_state: dwarf_call_frame_info = 10;
pub const DW_CFA_register: dwarf_call_frame_info = 9;
pub const DW_CFA_same_value: dwarf_call_frame_info = 8;
pub const DW_CFA_undefined: dwarf_call_frame_info = 7;
pub const DW_CFA_restore_extended: dwarf_call_frame_info = 6;
pub const DW_CFA_offset_extended: dwarf_call_frame_info = 5;
pub const DW_CFA_advance_loc4: dwarf_call_frame_info = 4;
pub const DW_CFA_advance_loc2: dwarf_call_frame_info = 3;
pub const DW_CFA_advance_loc1: dwarf_call_frame_info = 2;
pub const DW_CFA_set_loc: dwarf_call_frame_info = 1;
pub const DW_CFA_nop: dwarf_call_frame_info = 0;
pub const DW_CFA_restore: dwarf_call_frame_info = 192;
pub const DW_CFA_offset: dwarf_call_frame_info = 128;
pub const DW_CFA_advance_loc: dwarf_call_frame_info = 64;
pub type dwarf_name_index_attribute = libc::c_uint;
pub const DW_IDX_GNU_external: dwarf_name_index_attribute = 8193;
pub const DW_IDX_GNU_internal: dwarf_name_index_attribute = 8192;
pub const DW_IDX_hi_user: dwarf_name_index_attribute = 16383;
pub const DW_IDX_lo_user: dwarf_name_index_attribute = 8192;
pub const DW_IDX_type_hash: dwarf_name_index_attribute = 5;
pub const DW_IDX_parent: dwarf_name_index_attribute = 4;
pub const DW_IDX_die_offset: dwarf_name_index_attribute = 3;
pub const DW_IDX_type_unit: dwarf_name_index_attribute = 2;
pub const DW_IDX_compile_unit: dwarf_name_index_attribute = 1;
pub type dwarf_unit_type = libc::c_uint;
pub const DW_UT_hi_user: dwarf_unit_type = 255;
pub const DW_UT_lo_user: dwarf_unit_type = 128;
pub const DW_UT_split_type: dwarf_unit_type = 6;
pub const DW_UT_split_compile: dwarf_unit_type = 5;
pub const DW_UT_skeleton: dwarf_unit_type = 4;
pub const DW_UT_partial: dwarf_unit_type = 3;
pub const DW_UT_type: dwarf_unit_type = 2;
pub const DW_UT_compile: dwarf_unit_type = 1;
#[no_mangle]
pub unsafe extern "C" fn get_DW_TAG_name(mut tag: libc::c_uint) -> *const libc::c_char {
    match tag {
        0 => return b"DW_TAG_padding\0" as *const u8 as *const libc::c_char,
        1 => return b"DW_TAG_array_type\0" as *const u8 as *const libc::c_char,
        2 => return b"DW_TAG_class_type\0" as *const u8 as *const libc::c_char,
        3 => return b"DW_TAG_entry_point\0" as *const u8 as *const libc::c_char,
        4 => return b"DW_TAG_enumeration_type\0" as *const u8 as *const libc::c_char,
        5 => return b"DW_TAG_formal_parameter\0" as *const u8 as *const libc::c_char,
        8 => return b"DW_TAG_imported_declaration\0" as *const u8 as *const libc::c_char,
        10 => return b"DW_TAG_label\0" as *const u8 as *const libc::c_char,
        11 => return b"DW_TAG_lexical_block\0" as *const u8 as *const libc::c_char,
        13 => return b"DW_TAG_member\0" as *const u8 as *const libc::c_char,
        15 => return b"DW_TAG_pointer_type\0" as *const u8 as *const libc::c_char,
        16 => return b"DW_TAG_reference_type\0" as *const u8 as *const libc::c_char,
        17 => return b"DW_TAG_compile_unit\0" as *const u8 as *const libc::c_char,
        18 => return b"DW_TAG_string_type\0" as *const u8 as *const libc::c_char,
        19 => return b"DW_TAG_structure_type\0" as *const u8 as *const libc::c_char,
        21 => return b"DW_TAG_subroutine_type\0" as *const u8 as *const libc::c_char,
        22 => return b"DW_TAG_typedef\0" as *const u8 as *const libc::c_char,
        23 => return b"DW_TAG_union_type\0" as *const u8 as *const libc::c_char,
        24 => {
            return b"DW_TAG_unspecified_parameters\0" as *const u8 as *const libc::c_char;
        }
        25 => return b"DW_TAG_variant\0" as *const u8 as *const libc::c_char,
        26 => return b"DW_TAG_common_block\0" as *const u8 as *const libc::c_char,
        27 => return b"DW_TAG_common_inclusion\0" as *const u8 as *const libc::c_char,
        28 => return b"DW_TAG_inheritance\0" as *const u8 as *const libc::c_char,
        29 => return b"DW_TAG_inlined_subroutine\0" as *const u8 as *const libc::c_char,
        30 => return b"DW_TAG_module\0" as *const u8 as *const libc::c_char,
        31 => return b"DW_TAG_ptr_to_member_type\0" as *const u8 as *const libc::c_char,
        32 => return b"DW_TAG_set_type\0" as *const u8 as *const libc::c_char,
        33 => return b"DW_TAG_subrange_type\0" as *const u8 as *const libc::c_char,
        34 => return b"DW_TAG_with_stmt\0" as *const u8 as *const libc::c_char,
        35 => return b"DW_TAG_access_declaration\0" as *const u8 as *const libc::c_char,
        36 => return b"DW_TAG_base_type\0" as *const u8 as *const libc::c_char,
        37 => return b"DW_TAG_catch_block\0" as *const u8 as *const libc::c_char,
        38 => return b"DW_TAG_const_type\0" as *const u8 as *const libc::c_char,
        39 => return b"DW_TAG_constant\0" as *const u8 as *const libc::c_char,
        40 => return b"DW_TAG_enumerator\0" as *const u8 as *const libc::c_char,
        41 => return b"DW_TAG_file_type\0" as *const u8 as *const libc::c_char,
        42 => return b"DW_TAG_friend\0" as *const u8 as *const libc::c_char,
        43 => return b"DW_TAG_namelist\0" as *const u8 as *const libc::c_char,
        44 => return b"DW_TAG_namelist_item\0" as *const u8 as *const libc::c_char,
        45 => return b"DW_TAG_packed_type\0" as *const u8 as *const libc::c_char,
        46 => return b"DW_TAG_subprogram\0" as *const u8 as *const libc::c_char,
        47 => return b"DW_TAG_template_type_param\0" as *const u8 as *const libc::c_char,
        48 => return b"DW_TAG_template_value_param\0" as *const u8 as *const libc::c_char,
        49 => return b"DW_TAG_thrown_type\0" as *const u8 as *const libc::c_char,
        50 => return b"DW_TAG_try_block\0" as *const u8 as *const libc::c_char,
        51 => return b"DW_TAG_variant_part\0" as *const u8 as *const libc::c_char,
        52 => return b"DW_TAG_variable\0" as *const u8 as *const libc::c_char,
        53 => return b"DW_TAG_volatile_type\0" as *const u8 as *const libc::c_char,
        54 => return b"DW_TAG_dwarf_procedure\0" as *const u8 as *const libc::c_char,
        55 => return b"DW_TAG_restrict_type\0" as *const u8 as *const libc::c_char,
        56 => return b"DW_TAG_interface_type\0" as *const u8 as *const libc::c_char,
        57 => return b"DW_TAG_namespace\0" as *const u8 as *const libc::c_char,
        58 => return b"DW_TAG_imported_module\0" as *const u8 as *const libc::c_char,
        59 => return b"DW_TAG_unspecified_type\0" as *const u8 as *const libc::c_char,
        60 => return b"DW_TAG_partial_unit\0" as *const u8 as *const libc::c_char,
        61 => return b"DW_TAG_imported_unit\0" as *const u8 as *const libc::c_char,
        63 => return b"DW_TAG_condition\0" as *const u8 as *const libc::c_char,
        64 => return b"DW_TAG_shared_type\0" as *const u8 as *const libc::c_char,
        65 => return b"DW_TAG_type_unit\0" as *const u8 as *const libc::c_char,
        66 => {
            return b"DW_TAG_rvalue_reference_type\0" as *const u8 as *const libc::c_char;
        }
        67 => return b"DW_TAG_template_alias\0" as *const u8 as *const libc::c_char,
        68 => return b"DW_TAG_coarray_type\0" as *const u8 as *const libc::c_char,
        69 => return b"DW_TAG_generic_subrange\0" as *const u8 as *const libc::c_char,
        70 => return b"DW_TAG_dynamic_type\0" as *const u8 as *const libc::c_char,
        71 => return b"DW_TAG_atomic_type\0" as *const u8 as *const libc::c_char,
        72 => return b"DW_TAG_call_site\0" as *const u8 as *const libc::c_char,
        73 => return b"DW_TAG_call_site_parameter\0" as *const u8 as *const libc::c_char,
        74 => return b"DW_TAG_skeleton_unit\0" as *const u8 as *const libc::c_char,
        75 => return b"DW_TAG_immutable_type\0" as *const u8 as *const libc::c_char,
        16513 => return b"DW_TAG_MIPS_loop\0" as *const u8 as *const libc::c_char,
        16528 => {
            return b"DW_TAG_HP_array_descriptor\0" as *const u8 as *const libc::c_char;
        }
        16529 => return b"DW_TAG_HP_Bliss_field\0" as *const u8 as *const libc::c_char,
        16530 => {
            return b"DW_TAG_HP_Bliss_field_set\0" as *const u8 as *const libc::c_char;
        }
        16641 => return b"DW_TAG_format_label\0" as *const u8 as *const libc::c_char,
        16642 => return b"DW_TAG_function_template\0" as *const u8 as *const libc::c_char,
        16643 => return b"DW_TAG_class_template\0" as *const u8 as *const libc::c_char,
        16644 => return b"DW_TAG_GNU_BINCL\0" as *const u8 as *const libc::c_char,
        16645 => return b"DW_TAG_GNU_EINCL\0" as *const u8 as *const libc::c_char,
        16646 => {
            return b"DW_TAG_GNU_template_template_param\0" as *const u8
                as *const libc::c_char;
        }
        16647 => {
            return b"DW_TAG_GNU_template_parameter_pack\0" as *const u8
                as *const libc::c_char;
        }
        16648 => {
            return b"DW_TAG_GNU_formal_parameter_pack\0" as *const u8
                as *const libc::c_char;
        }
        16649 => return b"DW_TAG_GNU_call_site\0" as *const u8 as *const libc::c_char,
        16650 => {
            return b"DW_TAG_GNU_call_site_parameter\0" as *const u8
                as *const libc::c_char;
        }
        34661 => return b"DW_TAG_upc_shared_type\0" as *const u8 as *const libc::c_char,
        34662 => return b"DW_TAG_upc_strict_type\0" as *const u8 as *const libc::c_char,
        34663 => return b"DW_TAG_upc_relaxed_type\0" as *const u8 as *const libc::c_char,
        40960 => return b"DW_TAG_PGI_kanji_type\0" as *const u8 as *const libc::c_char,
        40992 => {
            return b"DW_TAG_PGI_interface_block\0" as *const u8 as *const libc::c_char;
        }
        _ => {}
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn get_DW_AT_name(mut attr: libc::c_uint) -> *const libc::c_char {
    match attr {
        1 => return b"DW_AT_sibling\0" as *const u8 as *const libc::c_char,
        2 => return b"DW_AT_location\0" as *const u8 as *const libc::c_char,
        3 => return b"DW_AT_name\0" as *const u8 as *const libc::c_char,
        9 => return b"DW_AT_ordering\0" as *const u8 as *const libc::c_char,
        10 => return b"DW_AT_subscr_data\0" as *const u8 as *const libc::c_char,
        11 => return b"DW_AT_byte_size\0" as *const u8 as *const libc::c_char,
        12 => return b"DW_AT_bit_offset\0" as *const u8 as *const libc::c_char,
        13 => return b"DW_AT_bit_size\0" as *const u8 as *const libc::c_char,
        15 => return b"DW_AT_element_list\0" as *const u8 as *const libc::c_char,
        16 => return b"DW_AT_stmt_list\0" as *const u8 as *const libc::c_char,
        17 => return b"DW_AT_low_pc\0" as *const u8 as *const libc::c_char,
        18 => return b"DW_AT_high_pc\0" as *const u8 as *const libc::c_char,
        19 => return b"DW_AT_language\0" as *const u8 as *const libc::c_char,
        20 => return b"DW_AT_member\0" as *const u8 as *const libc::c_char,
        21 => return b"DW_AT_discr\0" as *const u8 as *const libc::c_char,
        22 => return b"DW_AT_discr_value\0" as *const u8 as *const libc::c_char,
        23 => return b"DW_AT_visibility\0" as *const u8 as *const libc::c_char,
        24 => return b"DW_AT_import\0" as *const u8 as *const libc::c_char,
        25 => return b"DW_AT_string_length\0" as *const u8 as *const libc::c_char,
        26 => return b"DW_AT_common_reference\0" as *const u8 as *const libc::c_char,
        27 => return b"DW_AT_comp_dir\0" as *const u8 as *const libc::c_char,
        28 => return b"DW_AT_const_value\0" as *const u8 as *const libc::c_char,
        29 => return b"DW_AT_containing_type\0" as *const u8 as *const libc::c_char,
        30 => return b"DW_AT_default_value\0" as *const u8 as *const libc::c_char,
        32 => return b"DW_AT_inline\0" as *const u8 as *const libc::c_char,
        33 => return b"DW_AT_is_optional\0" as *const u8 as *const libc::c_char,
        34 => return b"DW_AT_lower_bound\0" as *const u8 as *const libc::c_char,
        37 => return b"DW_AT_producer\0" as *const u8 as *const libc::c_char,
        39 => return b"DW_AT_prototyped\0" as *const u8 as *const libc::c_char,
        42 => return b"DW_AT_return_addr\0" as *const u8 as *const libc::c_char,
        44 => return b"DW_AT_start_scope\0" as *const u8 as *const libc::c_char,
        46 => return b"DW_AT_bit_stride\0" as *const u8 as *const libc::c_char,
        47 => return b"DW_AT_upper_bound\0" as *const u8 as *const libc::c_char,
        49 => return b"DW_AT_abstract_origin\0" as *const u8 as *const libc::c_char,
        50 => return b"DW_AT_accessibility\0" as *const u8 as *const libc::c_char,
        51 => return b"DW_AT_address_class\0" as *const u8 as *const libc::c_char,
        52 => return b"DW_AT_artificial\0" as *const u8 as *const libc::c_char,
        53 => return b"DW_AT_base_types\0" as *const u8 as *const libc::c_char,
        54 => return b"DW_AT_calling_convention\0" as *const u8 as *const libc::c_char,
        55 => return b"DW_AT_count\0" as *const u8 as *const libc::c_char,
        56 => return b"DW_AT_data_member_location\0" as *const u8 as *const libc::c_char,
        57 => return b"DW_AT_decl_column\0" as *const u8 as *const libc::c_char,
        58 => return b"DW_AT_decl_file\0" as *const u8 as *const libc::c_char,
        59 => return b"DW_AT_decl_line\0" as *const u8 as *const libc::c_char,
        60 => return b"DW_AT_declaration\0" as *const u8 as *const libc::c_char,
        61 => return b"DW_AT_discr_list\0" as *const u8 as *const libc::c_char,
        62 => return b"DW_AT_encoding\0" as *const u8 as *const libc::c_char,
        63 => return b"DW_AT_external\0" as *const u8 as *const libc::c_char,
        64 => return b"DW_AT_frame_base\0" as *const u8 as *const libc::c_char,
        65 => return b"DW_AT_friend\0" as *const u8 as *const libc::c_char,
        66 => return b"DW_AT_identifier_case\0" as *const u8 as *const libc::c_char,
        67 => return b"DW_AT_macro_info\0" as *const u8 as *const libc::c_char,
        68 => return b"DW_AT_namelist_items\0" as *const u8 as *const libc::c_char,
        69 => return b"DW_AT_priority\0" as *const u8 as *const libc::c_char,
        70 => return b"DW_AT_segment\0" as *const u8 as *const libc::c_char,
        71 => return b"DW_AT_specification\0" as *const u8 as *const libc::c_char,
        72 => return b"DW_AT_static_link\0" as *const u8 as *const libc::c_char,
        73 => return b"DW_AT_type\0" as *const u8 as *const libc::c_char,
        74 => return b"DW_AT_use_location\0" as *const u8 as *const libc::c_char,
        75 => return b"DW_AT_variable_parameter\0" as *const u8 as *const libc::c_char,
        76 => return b"DW_AT_virtuality\0" as *const u8 as *const libc::c_char,
        77 => return b"DW_AT_vtable_elem_location\0" as *const u8 as *const libc::c_char,
        78 => return b"DW_AT_allocated\0" as *const u8 as *const libc::c_char,
        79 => return b"DW_AT_associated\0" as *const u8 as *const libc::c_char,
        80 => return b"DW_AT_data_location\0" as *const u8 as *const libc::c_char,
        81 => return b"DW_AT_byte_stride\0" as *const u8 as *const libc::c_char,
        82 => return b"DW_AT_entry_pc\0" as *const u8 as *const libc::c_char,
        83 => return b"DW_AT_use_UTF8\0" as *const u8 as *const libc::c_char,
        84 => return b"DW_AT_extension\0" as *const u8 as *const libc::c_char,
        85 => return b"DW_AT_ranges\0" as *const u8 as *const libc::c_char,
        86 => return b"DW_AT_trampoline\0" as *const u8 as *const libc::c_char,
        87 => return b"DW_AT_call_column\0" as *const u8 as *const libc::c_char,
        88 => return b"DW_AT_call_file\0" as *const u8 as *const libc::c_char,
        89 => return b"DW_AT_call_line\0" as *const u8 as *const libc::c_char,
        90 => return b"DW_AT_description\0" as *const u8 as *const libc::c_char,
        91 => return b"DW_AT_binary_scale\0" as *const u8 as *const libc::c_char,
        92 => return b"DW_AT_decimal_scale\0" as *const u8 as *const libc::c_char,
        93 => return b"DW_AT_small\0" as *const u8 as *const libc::c_char,
        94 => return b"DW_AT_decimal_sign\0" as *const u8 as *const libc::c_char,
        95 => return b"DW_AT_digit_count\0" as *const u8 as *const libc::c_char,
        96 => return b"DW_AT_picture_string\0" as *const u8 as *const libc::c_char,
        97 => return b"DW_AT_mutable\0" as *const u8 as *const libc::c_char,
        98 => return b"DW_AT_threads_scaled\0" as *const u8 as *const libc::c_char,
        99 => return b"DW_AT_explicit\0" as *const u8 as *const libc::c_char,
        100 => return b"DW_AT_object_pointer\0" as *const u8 as *const libc::c_char,
        101 => return b"DW_AT_endianity\0" as *const u8 as *const libc::c_char,
        102 => return b"DW_AT_elemental\0" as *const u8 as *const libc::c_char,
        103 => return b"DW_AT_pure\0" as *const u8 as *const libc::c_char,
        104 => return b"DW_AT_recursive\0" as *const u8 as *const libc::c_char,
        105 => return b"DW_AT_signature\0" as *const u8 as *const libc::c_char,
        106 => return b"DW_AT_main_subprogram\0" as *const u8 as *const libc::c_char,
        107 => return b"DW_AT_data_bit_offset\0" as *const u8 as *const libc::c_char,
        108 => return b"DW_AT_const_expr\0" as *const u8 as *const libc::c_char,
        109 => return b"DW_AT_enum_class\0" as *const u8 as *const libc::c_char,
        110 => return b"DW_AT_linkage_name\0" as *const u8 as *const libc::c_char,
        111 => {
            return b"DW_AT_string_length_bit_size\0" as *const u8 as *const libc::c_char;
        }
        112 => {
            return b"DW_AT_string_length_byte_size\0" as *const u8 as *const libc::c_char;
        }
        113 => return b"DW_AT_rank\0" as *const u8 as *const libc::c_char,
        114 => return b"DW_AT_str_offsets_base\0" as *const u8 as *const libc::c_char,
        115 => return b"DW_AT_addr_base\0" as *const u8 as *const libc::c_char,
        116 => return b"DW_AT_rnglists_base\0" as *const u8 as *const libc::c_char,
        118 => return b"DW_AT_dwo_name\0" as *const u8 as *const libc::c_char,
        119 => return b"DW_AT_reference\0" as *const u8 as *const libc::c_char,
        120 => return b"DW_AT_rvalue_reference\0" as *const u8 as *const libc::c_char,
        121 => return b"DW_AT_macros\0" as *const u8 as *const libc::c_char,
        122 => return b"DW_AT_call_all_calls\0" as *const u8 as *const libc::c_char,
        123 => {
            return b"DW_AT_call_all_source_calls\0" as *const u8 as *const libc::c_char;
        }
        124 => return b"DW_AT_call_all_tail_calls\0" as *const u8 as *const libc::c_char,
        125 => return b"DW_AT_call_return_pc\0" as *const u8 as *const libc::c_char,
        126 => return b"DW_AT_call_value\0" as *const u8 as *const libc::c_char,
        127 => return b"DW_AT_call_origin\0" as *const u8 as *const libc::c_char,
        128 => return b"DW_AT_call_parameter\0" as *const u8 as *const libc::c_char,
        129 => return b"DW_AT_call_pc\0" as *const u8 as *const libc::c_char,
        130 => return b"DW_AT_call_tail_call\0" as *const u8 as *const libc::c_char,
        131 => return b"DW_AT_call_target\0" as *const u8 as *const libc::c_char,
        132 => {
            return b"DW_AT_call_target_clobbered\0" as *const u8 as *const libc::c_char;
        }
        133 => return b"DW_AT_call_data_location\0" as *const u8 as *const libc::c_char,
        134 => return b"DW_AT_call_data_value\0" as *const u8 as *const libc::c_char,
        135 => return b"DW_AT_noreturn\0" as *const u8 as *const libc::c_char,
        136 => return b"DW_AT_alignment\0" as *const u8 as *const libc::c_char,
        137 => return b"DW_AT_export_symbols\0" as *const u8 as *const libc::c_char,
        138 => return b"DW_AT_deleted\0" as *const u8 as *const libc::c_char,
        139 => return b"DW_AT_defaulted\0" as *const u8 as *const libc::c_char,
        140 => return b"DW_AT_loclists_base\0" as *const u8 as *const libc::c_char,
        8193 => return b"DW_AT_MIPS_fde\0" as *const u8 as *const libc::c_char,
        8194 => return b"DW_AT_MIPS_loop_begin\0" as *const u8 as *const libc::c_char,
        8195 => {
            return b"DW_AT_MIPS_tail_loop_begin\0" as *const u8 as *const libc::c_char;
        }
        8196 => return b"DW_AT_MIPS_epilog_begin\0" as *const u8 as *const libc::c_char,
        8197 => {
            return b"DW_AT_MIPS_loop_unroll_factor\0" as *const u8 as *const libc::c_char;
        }
        8198 => {
            return b"DW_AT_MIPS_software_pipeline_depth\0" as *const u8
                as *const libc::c_char;
        }
        8199 => return b"DW_AT_MIPS_linkage_name\0" as *const u8 as *const libc::c_char,
        8200 => return b"DW_AT_MIPS_stride\0" as *const u8 as *const libc::c_char,
        8201 => return b"DW_AT_MIPS_abstract_name\0" as *const u8 as *const libc::c_char,
        8202 => return b"DW_AT_MIPS_clone_origin\0" as *const u8 as *const libc::c_char,
        8203 => return b"DW_AT_MIPS_has_inlines\0" as *const u8 as *const libc::c_char,
        8192 => return b"DW_AT_HP_block_index\0" as *const u8 as *const libc::c_char,
        8208 => {
            return b"DW_AT_HP_actuals_stmt_list\0" as *const u8 as *const libc::c_char;
        }
        8209 => return b"DW_AT_HP_proc_per_section\0" as *const u8 as *const libc::c_char,
        8210 => return b"DW_AT_HP_raw_data_ptr\0" as *const u8 as *const libc::c_char,
        8211 => {
            return b"DW_AT_HP_pass_by_reference\0" as *const u8 as *const libc::c_char;
        }
        8212 => return b"DW_AT_HP_opt_level\0" as *const u8 as *const libc::c_char,
        8213 => return b"DW_AT_HP_prof_version_id\0" as *const u8 as *const libc::c_char,
        8214 => return b"DW_AT_HP_opt_flags\0" as *const u8 as *const libc::c_char,
        8215 => {
            return b"DW_AT_HP_cold_region_low_pc\0" as *const u8 as *const libc::c_char;
        }
        8216 => {
            return b"DW_AT_HP_cold_region_high_pc\0" as *const u8 as *const libc::c_char;
        }
        8217 => {
            return b"DW_AT_HP_all_variables_modifiable\0" as *const u8
                as *const libc::c_char;
        }
        8218 => return b"DW_AT_HP_linkage_name\0" as *const u8 as *const libc::c_char,
        8219 => return b"DW_AT_HP_prof_flags\0" as *const u8 as *const libc::c_char,
        8223 => return b"DW_AT_HP_unit_name\0" as *const u8 as *const libc::c_char,
        8224 => return b"DW_AT_HP_unit_size\0" as *const u8 as *const libc::c_char,
        8225 => {
            return b"DW_AT_HP_widened_byte_size\0" as *const u8 as *const libc::c_char;
        }
        8226 => {
            return b"DW_AT_HP_definition_points\0" as *const u8 as *const libc::c_char;
        }
        8227 => return b"DW_AT_HP_default_location\0" as *const u8 as *const libc::c_char,
        8233 => return b"DW_AT_HP_is_result_param\0" as *const u8 as *const libc::c_char,
        8449 => return b"DW_AT_sf_names\0" as *const u8 as *const libc::c_char,
        8450 => return b"DW_AT_src_info\0" as *const u8 as *const libc::c_char,
        8451 => return b"DW_AT_mac_info\0" as *const u8 as *const libc::c_char,
        8452 => return b"DW_AT_src_coords\0" as *const u8 as *const libc::c_char,
        8453 => return b"DW_AT_body_begin\0" as *const u8 as *const libc::c_char,
        8454 => return b"DW_AT_body_end\0" as *const u8 as *const libc::c_char,
        8455 => return b"DW_AT_GNU_vector\0" as *const u8 as *const libc::c_char,
        8456 => return b"DW_AT_GNU_guarded_by\0" as *const u8 as *const libc::c_char,
        8457 => return b"DW_AT_GNU_pt_guarded_by\0" as *const u8 as *const libc::c_char,
        8458 => return b"DW_AT_GNU_guarded\0" as *const u8 as *const libc::c_char,
        8459 => return b"DW_AT_GNU_pt_guarded\0" as *const u8 as *const libc::c_char,
        8460 => return b"DW_AT_GNU_locks_excluded\0" as *const u8 as *const libc::c_char,
        8461 => {
            return b"DW_AT_GNU_exclusive_locks_required\0" as *const u8
                as *const libc::c_char;
        }
        8462 => {
            return b"DW_AT_GNU_shared_locks_required\0" as *const u8
                as *const libc::c_char;
        }
        8463 => return b"DW_AT_GNU_odr_signature\0" as *const u8 as *const libc::c_char,
        8464 => return b"DW_AT_GNU_template_name\0" as *const u8 as *const libc::c_char,
        8465 => return b"DW_AT_GNU_call_site_value\0" as *const u8 as *const libc::c_char,
        8466 => {
            return b"DW_AT_GNU_call_site_data_value\0" as *const u8
                as *const libc::c_char;
        }
        8467 => {
            return b"DW_AT_GNU_call_site_target\0" as *const u8 as *const libc::c_char;
        }
        8468 => {
            return b"DW_AT_GNU_call_site_target_clobbered\0" as *const u8
                as *const libc::c_char;
        }
        8469 => return b"DW_AT_GNU_tail_call\0" as *const u8 as *const libc::c_char,
        8470 => {
            return b"DW_AT_GNU_all_tail_call_sites\0" as *const u8 as *const libc::c_char;
        }
        8471 => return b"DW_AT_GNU_all_call_sites\0" as *const u8 as *const libc::c_char,
        8472 => {
            return b"DW_AT_GNU_all_source_call_sites\0" as *const u8
                as *const libc::c_char;
        }
        8473 => return b"DW_AT_GNU_macros\0" as *const u8 as *const libc::c_char,
        8474 => return b"DW_AT_GNU_deleted\0" as *const u8 as *const libc::c_char,
        8496 => return b"DW_AT_GNU_dwo_name\0" as *const u8 as *const libc::c_char,
        8497 => return b"DW_AT_GNU_dwo_id\0" as *const u8 as *const libc::c_char,
        8498 => return b"DW_AT_GNU_ranges_base\0" as *const u8 as *const libc::c_char,
        8499 => return b"DW_AT_GNU_addr_base\0" as *const u8 as *const libc::c_char,
        8500 => return b"DW_AT_GNU_pubnames\0" as *const u8 as *const libc::c_char,
        8501 => return b"DW_AT_GNU_pubtypes\0" as *const u8 as *const libc::c_char,
        8502 => return b"DW_AT_GNU_discriminator\0" as *const u8 as *const libc::c_char,
        8503 => return b"DW_AT_GNU_locviews\0" as *const u8 as *const libc::c_char,
        8504 => return b"DW_AT_GNU_entry_view\0" as *const u8 as *const libc::c_char,
        8705 => {
            return b"DW_AT_VMS_rtnbeg_pd_address\0" as *const u8 as *const libc::c_char;
        }
        8961 => {
            return b"DW_AT_use_GNAT_descriptive_type\0" as *const u8
                as *const libc::c_char;
        }
        8962 => {
            return b"DW_AT_GNAT_descriptive_type\0" as *const u8 as *const libc::c_char;
        }
        8963 => return b"DW_AT_GNU_numerator\0" as *const u8 as *const libc::c_char,
        8964 => return b"DW_AT_GNU_denominator\0" as *const u8 as *const libc::c_char,
        8965 => return b"DW_AT_GNU_bias\0" as *const u8 as *const libc::c_char,
        12816 => return b"DW_AT_upc_threads_scaled\0" as *const u8 as *const libc::c_char,
        14848 => return b"DW_AT_PGI_lbase\0" as *const u8 as *const libc::c_char,
        14849 => return b"DW_AT_PGI_soffset\0" as *const u8 as *const libc::c_char,
        14850 => return b"DW_AT_PGI_lstride\0" as *const u8 as *const libc::c_char,
        16353 => return b"DW_AT_APPLE_optimized\0" as *const u8 as *const libc::c_char,
        16354 => return b"DW_AT_APPLE_flags\0" as *const u8 as *const libc::c_char,
        16355 => return b"DW_AT_APPLE_isa\0" as *const u8 as *const libc::c_char,
        16356 => return b"DW_AT_APPLE_block\0" as *const u8 as *const libc::c_char,
        16357 => {
            return b"DW_AT_APPLE_major_runtime_vers\0" as *const u8
                as *const libc::c_char;
        }
        16358 => {
            return b"DW_AT_APPLE_runtime_class\0" as *const u8 as *const libc::c_char;
        }
        16359 => {
            return b"DW_AT_APPLE_omit_frame_ptr\0" as *const u8 as *const libc::c_char;
        }
        16360 => {
            return b"DW_AT_APPLE_property_name\0" as *const u8 as *const libc::c_char;
        }
        16361 => {
            return b"DW_AT_APPLE_property_getter\0" as *const u8 as *const libc::c_char;
        }
        16362 => {
            return b"DW_AT_APPLE_property_setter\0" as *const u8 as *const libc::c_char;
        }
        16363 => {
            return b"DW_AT_APPLE_property_attribute\0" as *const u8
                as *const libc::c_char;
        }
        16364 => {
            return b"DW_AT_APPLE_objc_complete_type\0" as *const u8
                as *const libc::c_char;
        }
        16365 => return b"DW_AT_APPLE_property\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn get_DW_FORM_name(
    mut form: libc::c_uint,
) -> *const libc::c_char {
    match form {
        1 => return b"DW_FORM_addr\0" as *const u8 as *const libc::c_char,
        3 => return b"DW_FORM_block2\0" as *const u8 as *const libc::c_char,
        4 => return b"DW_FORM_block4\0" as *const u8 as *const libc::c_char,
        5 => return b"DW_FORM_data2\0" as *const u8 as *const libc::c_char,
        6 => return b"DW_FORM_data4\0" as *const u8 as *const libc::c_char,
        7 => return b"DW_FORM_data8\0" as *const u8 as *const libc::c_char,
        8 => return b"DW_FORM_string\0" as *const u8 as *const libc::c_char,
        9 => return b"DW_FORM_block\0" as *const u8 as *const libc::c_char,
        10 => return b"DW_FORM_block1\0" as *const u8 as *const libc::c_char,
        11 => return b"DW_FORM_data1\0" as *const u8 as *const libc::c_char,
        12 => return b"DW_FORM_flag\0" as *const u8 as *const libc::c_char,
        13 => return b"DW_FORM_sdata\0" as *const u8 as *const libc::c_char,
        14 => return b"DW_FORM_strp\0" as *const u8 as *const libc::c_char,
        15 => return b"DW_FORM_udata\0" as *const u8 as *const libc::c_char,
        16 => return b"DW_FORM_ref_addr\0" as *const u8 as *const libc::c_char,
        17 => return b"DW_FORM_ref1\0" as *const u8 as *const libc::c_char,
        18 => return b"DW_FORM_ref2\0" as *const u8 as *const libc::c_char,
        19 => return b"DW_FORM_ref4\0" as *const u8 as *const libc::c_char,
        20 => return b"DW_FORM_ref8\0" as *const u8 as *const libc::c_char,
        21 => return b"DW_FORM_ref_udata\0" as *const u8 as *const libc::c_char,
        22 => return b"DW_FORM_indirect\0" as *const u8 as *const libc::c_char,
        23 => return b"DW_FORM_sec_offset\0" as *const u8 as *const libc::c_char,
        24 => return b"DW_FORM_exprloc\0" as *const u8 as *const libc::c_char,
        25 => return b"DW_FORM_flag_present\0" as *const u8 as *const libc::c_char,
        32 => return b"DW_FORM_ref_sig8\0" as *const u8 as *const libc::c_char,
        26 => return b"DW_FORM_strx\0" as *const u8 as *const libc::c_char,
        27 => return b"DW_FORM_addrx\0" as *const u8 as *const libc::c_char,
        28 => return b"DW_FORM_ref_sup4\0" as *const u8 as *const libc::c_char,
        29 => return b"DW_FORM_strp_sup\0" as *const u8 as *const libc::c_char,
        30 => return b"DW_FORM_data16\0" as *const u8 as *const libc::c_char,
        31 => return b"DW_FORM_line_strp\0" as *const u8 as *const libc::c_char,
        33 => return b"DW_FORM_implicit_const\0" as *const u8 as *const libc::c_char,
        34 => return b"DW_FORM_loclistx\0" as *const u8 as *const libc::c_char,
        35 => return b"DW_FORM_rnglistx\0" as *const u8 as *const libc::c_char,
        36 => return b"DW_FORM_ref_sup8\0" as *const u8 as *const libc::c_char,
        37 => return b"DW_FORM_strx1\0" as *const u8 as *const libc::c_char,
        38 => return b"DW_FORM_strx2\0" as *const u8 as *const libc::c_char,
        39 => return b"DW_FORM_strx3\0" as *const u8 as *const libc::c_char,
        40 => return b"DW_FORM_strx4\0" as *const u8 as *const libc::c_char,
        41 => return b"DW_FORM_addrx1\0" as *const u8 as *const libc::c_char,
        42 => return b"DW_FORM_addrx2\0" as *const u8 as *const libc::c_char,
        43 => return b"DW_FORM_addrx3\0" as *const u8 as *const libc::c_char,
        44 => return b"DW_FORM_addrx4\0" as *const u8 as *const libc::c_char,
        7937 => return b"DW_FORM_GNU_addr_index\0" as *const u8 as *const libc::c_char,
        7938 => return b"DW_FORM_GNU_str_index\0" as *const u8 as *const libc::c_char,
        7968 => return b"DW_FORM_GNU_ref_alt\0" as *const u8 as *const libc::c_char,
        7969 => return b"DW_FORM_GNU_strp_alt\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn get_DW_OP_name(mut op: libc::c_uint) -> *const libc::c_char {
    match op {
        3 => return b"DW_OP_addr\0" as *const u8 as *const libc::c_char,
        6 => return b"DW_OP_deref\0" as *const u8 as *const libc::c_char,
        8 => return b"DW_OP_const1u\0" as *const u8 as *const libc::c_char,
        9 => return b"DW_OP_const1s\0" as *const u8 as *const libc::c_char,
        10 => return b"DW_OP_const2u\0" as *const u8 as *const libc::c_char,
        11 => return b"DW_OP_const2s\0" as *const u8 as *const libc::c_char,
        12 => return b"DW_OP_const4u\0" as *const u8 as *const libc::c_char,
        13 => return b"DW_OP_const4s\0" as *const u8 as *const libc::c_char,
        14 => return b"DW_OP_const8u\0" as *const u8 as *const libc::c_char,
        15 => return b"DW_OP_const8s\0" as *const u8 as *const libc::c_char,
        16 => return b"DW_OP_constu\0" as *const u8 as *const libc::c_char,
        17 => return b"DW_OP_consts\0" as *const u8 as *const libc::c_char,
        18 => return b"DW_OP_dup\0" as *const u8 as *const libc::c_char,
        19 => return b"DW_OP_drop\0" as *const u8 as *const libc::c_char,
        20 => return b"DW_OP_over\0" as *const u8 as *const libc::c_char,
        21 => return b"DW_OP_pick\0" as *const u8 as *const libc::c_char,
        22 => return b"DW_OP_swap\0" as *const u8 as *const libc::c_char,
        23 => return b"DW_OP_rot\0" as *const u8 as *const libc::c_char,
        24 => return b"DW_OP_xderef\0" as *const u8 as *const libc::c_char,
        25 => return b"DW_OP_abs\0" as *const u8 as *const libc::c_char,
        26 => return b"DW_OP_and\0" as *const u8 as *const libc::c_char,
        27 => return b"DW_OP_div\0" as *const u8 as *const libc::c_char,
        28 => return b"DW_OP_minus\0" as *const u8 as *const libc::c_char,
        29 => return b"DW_OP_mod\0" as *const u8 as *const libc::c_char,
        30 => return b"DW_OP_mul\0" as *const u8 as *const libc::c_char,
        31 => return b"DW_OP_neg\0" as *const u8 as *const libc::c_char,
        32 => return b"DW_OP_not\0" as *const u8 as *const libc::c_char,
        33 => return b"DW_OP_or\0" as *const u8 as *const libc::c_char,
        34 => return b"DW_OP_plus\0" as *const u8 as *const libc::c_char,
        35 => return b"DW_OP_plus_uconst\0" as *const u8 as *const libc::c_char,
        36 => return b"DW_OP_shl\0" as *const u8 as *const libc::c_char,
        37 => return b"DW_OP_shr\0" as *const u8 as *const libc::c_char,
        38 => return b"DW_OP_shra\0" as *const u8 as *const libc::c_char,
        39 => return b"DW_OP_xor\0" as *const u8 as *const libc::c_char,
        40 => return b"DW_OP_bra\0" as *const u8 as *const libc::c_char,
        41 => return b"DW_OP_eq\0" as *const u8 as *const libc::c_char,
        42 => return b"DW_OP_ge\0" as *const u8 as *const libc::c_char,
        43 => return b"DW_OP_gt\0" as *const u8 as *const libc::c_char,
        44 => return b"DW_OP_le\0" as *const u8 as *const libc::c_char,
        45 => return b"DW_OP_lt\0" as *const u8 as *const libc::c_char,
        46 => return b"DW_OP_ne\0" as *const u8 as *const libc::c_char,
        47 => return b"DW_OP_skip\0" as *const u8 as *const libc::c_char,
        48 => return b"DW_OP_lit0\0" as *const u8 as *const libc::c_char,
        49 => return b"DW_OP_lit1\0" as *const u8 as *const libc::c_char,
        50 => return b"DW_OP_lit2\0" as *const u8 as *const libc::c_char,
        51 => return b"DW_OP_lit3\0" as *const u8 as *const libc::c_char,
        52 => return b"DW_OP_lit4\0" as *const u8 as *const libc::c_char,
        53 => return b"DW_OP_lit5\0" as *const u8 as *const libc::c_char,
        54 => return b"DW_OP_lit6\0" as *const u8 as *const libc::c_char,
        55 => return b"DW_OP_lit7\0" as *const u8 as *const libc::c_char,
        56 => return b"DW_OP_lit8\0" as *const u8 as *const libc::c_char,
        57 => return b"DW_OP_lit9\0" as *const u8 as *const libc::c_char,
        58 => return b"DW_OP_lit10\0" as *const u8 as *const libc::c_char,
        59 => return b"DW_OP_lit11\0" as *const u8 as *const libc::c_char,
        60 => return b"DW_OP_lit12\0" as *const u8 as *const libc::c_char,
        61 => return b"DW_OP_lit13\0" as *const u8 as *const libc::c_char,
        62 => return b"DW_OP_lit14\0" as *const u8 as *const libc::c_char,
        63 => return b"DW_OP_lit15\0" as *const u8 as *const libc::c_char,
        64 => return b"DW_OP_lit16\0" as *const u8 as *const libc::c_char,
        65 => return b"DW_OP_lit17\0" as *const u8 as *const libc::c_char,
        66 => return b"DW_OP_lit18\0" as *const u8 as *const libc::c_char,
        67 => return b"DW_OP_lit19\0" as *const u8 as *const libc::c_char,
        68 => return b"DW_OP_lit20\0" as *const u8 as *const libc::c_char,
        69 => return b"DW_OP_lit21\0" as *const u8 as *const libc::c_char,
        70 => return b"DW_OP_lit22\0" as *const u8 as *const libc::c_char,
        71 => return b"DW_OP_lit23\0" as *const u8 as *const libc::c_char,
        72 => return b"DW_OP_lit24\0" as *const u8 as *const libc::c_char,
        73 => return b"DW_OP_lit25\0" as *const u8 as *const libc::c_char,
        74 => return b"DW_OP_lit26\0" as *const u8 as *const libc::c_char,
        75 => return b"DW_OP_lit27\0" as *const u8 as *const libc::c_char,
        76 => return b"DW_OP_lit28\0" as *const u8 as *const libc::c_char,
        77 => return b"DW_OP_lit29\0" as *const u8 as *const libc::c_char,
        78 => return b"DW_OP_lit30\0" as *const u8 as *const libc::c_char,
        79 => return b"DW_OP_lit31\0" as *const u8 as *const libc::c_char,
        80 => return b"DW_OP_reg0\0" as *const u8 as *const libc::c_char,
        81 => return b"DW_OP_reg1\0" as *const u8 as *const libc::c_char,
        82 => return b"DW_OP_reg2\0" as *const u8 as *const libc::c_char,
        83 => return b"DW_OP_reg3\0" as *const u8 as *const libc::c_char,
        84 => return b"DW_OP_reg4\0" as *const u8 as *const libc::c_char,
        85 => return b"DW_OP_reg5\0" as *const u8 as *const libc::c_char,
        86 => return b"DW_OP_reg6\0" as *const u8 as *const libc::c_char,
        87 => return b"DW_OP_reg7\0" as *const u8 as *const libc::c_char,
        88 => return b"DW_OP_reg8\0" as *const u8 as *const libc::c_char,
        89 => return b"DW_OP_reg9\0" as *const u8 as *const libc::c_char,
        90 => return b"DW_OP_reg10\0" as *const u8 as *const libc::c_char,
        91 => return b"DW_OP_reg11\0" as *const u8 as *const libc::c_char,
        92 => return b"DW_OP_reg12\0" as *const u8 as *const libc::c_char,
        93 => return b"DW_OP_reg13\0" as *const u8 as *const libc::c_char,
        94 => return b"DW_OP_reg14\0" as *const u8 as *const libc::c_char,
        95 => return b"DW_OP_reg15\0" as *const u8 as *const libc::c_char,
        96 => return b"DW_OP_reg16\0" as *const u8 as *const libc::c_char,
        97 => return b"DW_OP_reg17\0" as *const u8 as *const libc::c_char,
        98 => return b"DW_OP_reg18\0" as *const u8 as *const libc::c_char,
        99 => return b"DW_OP_reg19\0" as *const u8 as *const libc::c_char,
        100 => return b"DW_OP_reg20\0" as *const u8 as *const libc::c_char,
        101 => return b"DW_OP_reg21\0" as *const u8 as *const libc::c_char,
        102 => return b"DW_OP_reg22\0" as *const u8 as *const libc::c_char,
        103 => return b"DW_OP_reg23\0" as *const u8 as *const libc::c_char,
        104 => return b"DW_OP_reg24\0" as *const u8 as *const libc::c_char,
        105 => return b"DW_OP_reg25\0" as *const u8 as *const libc::c_char,
        106 => return b"DW_OP_reg26\0" as *const u8 as *const libc::c_char,
        107 => return b"DW_OP_reg27\0" as *const u8 as *const libc::c_char,
        108 => return b"DW_OP_reg28\0" as *const u8 as *const libc::c_char,
        109 => return b"DW_OP_reg29\0" as *const u8 as *const libc::c_char,
        110 => return b"DW_OP_reg30\0" as *const u8 as *const libc::c_char,
        111 => return b"DW_OP_reg31\0" as *const u8 as *const libc::c_char,
        112 => return b"DW_OP_breg0\0" as *const u8 as *const libc::c_char,
        113 => return b"DW_OP_breg1\0" as *const u8 as *const libc::c_char,
        114 => return b"DW_OP_breg2\0" as *const u8 as *const libc::c_char,
        115 => return b"DW_OP_breg3\0" as *const u8 as *const libc::c_char,
        116 => return b"DW_OP_breg4\0" as *const u8 as *const libc::c_char,
        117 => return b"DW_OP_breg5\0" as *const u8 as *const libc::c_char,
        118 => return b"DW_OP_breg6\0" as *const u8 as *const libc::c_char,
        119 => return b"DW_OP_breg7\0" as *const u8 as *const libc::c_char,
        120 => return b"DW_OP_breg8\0" as *const u8 as *const libc::c_char,
        121 => return b"DW_OP_breg9\0" as *const u8 as *const libc::c_char,
        122 => return b"DW_OP_breg10\0" as *const u8 as *const libc::c_char,
        123 => return b"DW_OP_breg11\0" as *const u8 as *const libc::c_char,
        124 => return b"DW_OP_breg12\0" as *const u8 as *const libc::c_char,
        125 => return b"DW_OP_breg13\0" as *const u8 as *const libc::c_char,
        126 => return b"DW_OP_breg14\0" as *const u8 as *const libc::c_char,
        127 => return b"DW_OP_breg15\0" as *const u8 as *const libc::c_char,
        128 => return b"DW_OP_breg16\0" as *const u8 as *const libc::c_char,
        129 => return b"DW_OP_breg17\0" as *const u8 as *const libc::c_char,
        130 => return b"DW_OP_breg18\0" as *const u8 as *const libc::c_char,
        131 => return b"DW_OP_breg19\0" as *const u8 as *const libc::c_char,
        132 => return b"DW_OP_breg20\0" as *const u8 as *const libc::c_char,
        133 => return b"DW_OP_breg21\0" as *const u8 as *const libc::c_char,
        134 => return b"DW_OP_breg22\0" as *const u8 as *const libc::c_char,
        135 => return b"DW_OP_breg23\0" as *const u8 as *const libc::c_char,
        136 => return b"DW_OP_breg24\0" as *const u8 as *const libc::c_char,
        137 => return b"DW_OP_breg25\0" as *const u8 as *const libc::c_char,
        138 => return b"DW_OP_breg26\0" as *const u8 as *const libc::c_char,
        139 => return b"DW_OP_breg27\0" as *const u8 as *const libc::c_char,
        140 => return b"DW_OP_breg28\0" as *const u8 as *const libc::c_char,
        141 => return b"DW_OP_breg29\0" as *const u8 as *const libc::c_char,
        142 => return b"DW_OP_breg30\0" as *const u8 as *const libc::c_char,
        143 => return b"DW_OP_breg31\0" as *const u8 as *const libc::c_char,
        144 => return b"DW_OP_regx\0" as *const u8 as *const libc::c_char,
        145 => return b"DW_OP_fbreg\0" as *const u8 as *const libc::c_char,
        146 => return b"DW_OP_bregx\0" as *const u8 as *const libc::c_char,
        147 => return b"DW_OP_piece\0" as *const u8 as *const libc::c_char,
        148 => return b"DW_OP_deref_size\0" as *const u8 as *const libc::c_char,
        149 => return b"DW_OP_xderef_size\0" as *const u8 as *const libc::c_char,
        150 => return b"DW_OP_nop\0" as *const u8 as *const libc::c_char,
        151 => return b"DW_OP_push_object_address\0" as *const u8 as *const libc::c_char,
        152 => return b"DW_OP_call2\0" as *const u8 as *const libc::c_char,
        153 => return b"DW_OP_call4\0" as *const u8 as *const libc::c_char,
        154 => return b"DW_OP_call_ref\0" as *const u8 as *const libc::c_char,
        155 => return b"DW_OP_form_tls_address\0" as *const u8 as *const libc::c_char,
        156 => return b"DW_OP_call_frame_cfa\0" as *const u8 as *const libc::c_char,
        157 => return b"DW_OP_bit_piece\0" as *const u8 as *const libc::c_char,
        158 => return b"DW_OP_implicit_value\0" as *const u8 as *const libc::c_char,
        159 => return b"DW_OP_stack_value\0" as *const u8 as *const libc::c_char,
        160 => return b"DW_OP_implicit_pointer\0" as *const u8 as *const libc::c_char,
        161 => return b"DW_OP_addrx\0" as *const u8 as *const libc::c_char,
        162 => return b"DW_OP_constx\0" as *const u8 as *const libc::c_char,
        163 => return b"DW_OP_entry_value\0" as *const u8 as *const libc::c_char,
        164 => return b"DW_OP_const_type\0" as *const u8 as *const libc::c_char,
        165 => return b"DW_OP_regval_type\0" as *const u8 as *const libc::c_char,
        166 => return b"DW_OP_deref_type\0" as *const u8 as *const libc::c_char,
        167 => return b"DW_OP_xderef_type\0" as *const u8 as *const libc::c_char,
        168 => return b"DW_OP_convert\0" as *const u8 as *const libc::c_char,
        169 => return b"DW_OP_reinterpret\0" as *const u8 as *const libc::c_char,
        224 => return b"DW_OP_GNU_push_tls_address\0" as *const u8 as *const libc::c_char,
        240 => return b"DW_OP_GNU_uninit\0" as *const u8 as *const libc::c_char,
        241 => return b"DW_OP_GNU_encoded_addr\0" as *const u8 as *const libc::c_char,
        242 => return b"DW_OP_GNU_implicit_pointer\0" as *const u8 as *const libc::c_char,
        243 => return b"DW_OP_GNU_entry_value\0" as *const u8 as *const libc::c_char,
        244 => return b"DW_OP_GNU_const_type\0" as *const u8 as *const libc::c_char,
        245 => return b"DW_OP_GNU_regval_type\0" as *const u8 as *const libc::c_char,
        246 => return b"DW_OP_GNU_deref_type\0" as *const u8 as *const libc::c_char,
        247 => return b"DW_OP_GNU_convert\0" as *const u8 as *const libc::c_char,
        249 => return b"DW_OP_GNU_reinterpret\0" as *const u8 as *const libc::c_char,
        250 => return b"DW_OP_GNU_parameter_ref\0" as *const u8 as *const libc::c_char,
        251 => return b"DW_OP_GNU_addr_index\0" as *const u8 as *const libc::c_char,
        252 => return b"DW_OP_GNU_const_index\0" as *const u8 as *const libc::c_char,
        253 => return b"DW_OP_GNU_variable_value\0" as *const u8 as *const libc::c_char,
        225 => return b"DW_OP_HP_is_value\0" as *const u8 as *const libc::c_char,
        226 => return b"DW_OP_HP_fltconst4\0" as *const u8 as *const libc::c_char,
        227 => return b"DW_OP_HP_fltconst8\0" as *const u8 as *const libc::c_char,
        228 => return b"DW_OP_HP_mod_range\0" as *const u8 as *const libc::c_char,
        229 => return b"DW_OP_HP_unmod_range\0" as *const u8 as *const libc::c_char,
        230 => return b"DW_OP_HP_tls\0" as *const u8 as *const libc::c_char,
        248 => return b"DW_OP_PGI_omp_thread_num\0" as *const u8 as *const libc::c_char,
        234 => return b"DW_OP_AARCH64_operation\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn get_DW_ATE_name(mut enc: libc::c_uint) -> *const libc::c_char {
    match enc {
        0 => return b"DW_ATE_void\0" as *const u8 as *const libc::c_char,
        1 => return b"DW_ATE_address\0" as *const u8 as *const libc::c_char,
        2 => return b"DW_ATE_boolean\0" as *const u8 as *const libc::c_char,
        3 => return b"DW_ATE_complex_float\0" as *const u8 as *const libc::c_char,
        4 => return b"DW_ATE_float\0" as *const u8 as *const libc::c_char,
        5 => return b"DW_ATE_signed\0" as *const u8 as *const libc::c_char,
        6 => return b"DW_ATE_signed_char\0" as *const u8 as *const libc::c_char,
        7 => return b"DW_ATE_unsigned\0" as *const u8 as *const libc::c_char,
        8 => return b"DW_ATE_unsigned_char\0" as *const u8 as *const libc::c_char,
        9 => return b"DW_ATE_imaginary_float\0" as *const u8 as *const libc::c_char,
        10 => return b"DW_ATE_packed_decimal\0" as *const u8 as *const libc::c_char,
        11 => return b"DW_ATE_numeric_string\0" as *const u8 as *const libc::c_char,
        12 => return b"DW_ATE_edited\0" as *const u8 as *const libc::c_char,
        13 => return b"DW_ATE_signed_fixed\0" as *const u8 as *const libc::c_char,
        14 => return b"DW_ATE_unsigned_fixed\0" as *const u8 as *const libc::c_char,
        15 => return b"DW_ATE_decimal_float\0" as *const u8 as *const libc::c_char,
        16 => return b"DW_ATE_UTF\0" as *const u8 as *const libc::c_char,
        17 => return b"DW_ATE_UCS\0" as *const u8 as *const libc::c_char,
        18 => return b"DW_ATE_ASCII\0" as *const u8 as *const libc::c_char,
        128 => return b"DW_ATE_HP_float80\0" as *const u8 as *const libc::c_char,
        129 => return b"DW_ATE_HP_complex_float80\0" as *const u8 as *const libc::c_char,
        130 => return b"DW_ATE_HP_float128\0" as *const u8 as *const libc::c_char,
        131 => return b"DW_ATE_HP_complex_float128\0" as *const u8 as *const libc::c_char,
        132 => return b"DW_ATE_HP_floathpintel\0" as *const u8 as *const libc::c_char,
        133 => {
            return b"DW_ATE_HP_imaginary_float80\0" as *const u8 as *const libc::c_char;
        }
        134 => {
            return b"DW_ATE_HP_imaginary_float128\0" as *const u8 as *const libc::c_char;
        }
        136 => return b"DW_ATE_HP_VAX_float\0" as *const u8 as *const libc::c_char,
        137 => return b"DW_ATE_HP_VAX_float_d\0" as *const u8 as *const libc::c_char,
        138 => return b"DW_ATE_HP_packed_decimal\0" as *const u8 as *const libc::c_char,
        139 => return b"DW_ATE_HP_zoned_decimal\0" as *const u8 as *const libc::c_char,
        140 => return b"DW_ATE_HP_edited\0" as *const u8 as *const libc::c_char,
        141 => return b"DW_ATE_HP_signed_fixed\0" as *const u8 as *const libc::c_char,
        142 => return b"DW_ATE_HP_unsigned_fixed\0" as *const u8 as *const libc::c_char,
        143 => {
            return b"DW_ATE_HP_VAX_complex_float\0" as *const u8 as *const libc::c_char;
        }
        144 => {
            return b"DW_ATE_HP_VAX_complex_float_d\0" as *const u8 as *const libc::c_char;
        }
        _ => {}
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn get_DW_CFA_name(mut opc: libc::c_uint) -> *const libc::c_char {
    match opc {
        64 => return b"DW_CFA_advance_loc\0" as *const u8 as *const libc::c_char,
        128 => return b"DW_CFA_offset\0" as *const u8 as *const libc::c_char,
        192 => return b"DW_CFA_restore\0" as *const u8 as *const libc::c_char,
        0 => return b"DW_CFA_nop\0" as *const u8 as *const libc::c_char,
        1 => return b"DW_CFA_set_loc\0" as *const u8 as *const libc::c_char,
        2 => return b"DW_CFA_advance_loc1\0" as *const u8 as *const libc::c_char,
        3 => return b"DW_CFA_advance_loc2\0" as *const u8 as *const libc::c_char,
        4 => return b"DW_CFA_advance_loc4\0" as *const u8 as *const libc::c_char,
        5 => return b"DW_CFA_offset_extended\0" as *const u8 as *const libc::c_char,
        6 => return b"DW_CFA_restore_extended\0" as *const u8 as *const libc::c_char,
        7 => return b"DW_CFA_undefined\0" as *const u8 as *const libc::c_char,
        8 => return b"DW_CFA_same_value\0" as *const u8 as *const libc::c_char,
        9 => return b"DW_CFA_register\0" as *const u8 as *const libc::c_char,
        10 => return b"DW_CFA_remember_state\0" as *const u8 as *const libc::c_char,
        11 => return b"DW_CFA_restore_state\0" as *const u8 as *const libc::c_char,
        12 => return b"DW_CFA_def_cfa\0" as *const u8 as *const libc::c_char,
        13 => return b"DW_CFA_def_cfa_register\0" as *const u8 as *const libc::c_char,
        14 => return b"DW_CFA_def_cfa_offset\0" as *const u8 as *const libc::c_char,
        15 => return b"DW_CFA_def_cfa_expression\0" as *const u8 as *const libc::c_char,
        16 => return b"DW_CFA_expression\0" as *const u8 as *const libc::c_char,
        17 => return b"DW_CFA_offset_extended_sf\0" as *const u8 as *const libc::c_char,
        18 => return b"DW_CFA_def_cfa_sf\0" as *const u8 as *const libc::c_char,
        19 => return b"DW_CFA_def_cfa_offset_sf\0" as *const u8 as *const libc::c_char,
        20 => return b"DW_CFA_val_offset\0" as *const u8 as *const libc::c_char,
        21 => return b"DW_CFA_val_offset_sf\0" as *const u8 as *const libc::c_char,
        22 => return b"DW_CFA_val_expression\0" as *const u8 as *const libc::c_char,
        28 => return b"DW_CFA_lo_user\0" as *const u8 as *const libc::c_char,
        63 => return b"DW_CFA_hi_user\0" as *const u8 as *const libc::c_char,
        29 => return b"DW_CFA_MIPS_advance_loc8\0" as *const u8 as *const libc::c_char,
        45 => return b"DW_CFA_GNU_window_save\0" as *const u8 as *const libc::c_char,
        46 => return b"DW_CFA_GNU_args_size\0" as *const u8 as *const libc::c_char,
        47 => {
            return b"DW_CFA_GNU_negative_offset_extended\0" as *const u8
                as *const libc::c_char;
        }
        _ => {}
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn get_DW_IDX_name(mut idx: libc::c_uint) -> *const libc::c_char {
    match idx {
        1 => return b"DW_IDX_compile_unit\0" as *const u8 as *const libc::c_char,
        2 => return b"DW_IDX_type_unit\0" as *const u8 as *const libc::c_char,
        3 => return b"DW_IDX_die_offset\0" as *const u8 as *const libc::c_char,
        4 => return b"DW_IDX_parent\0" as *const u8 as *const libc::c_char,
        5 => return b"DW_IDX_type_hash\0" as *const u8 as *const libc::c_char,
        16383 => return b"DW_IDX_hi_user\0" as *const u8 as *const libc::c_char,
        8192 => return b"DW_IDX_GNU_internal\0" as *const u8 as *const libc::c_char,
        8193 => return b"DW_IDX_GNU_external\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn get_DW_UT_name(mut ut: libc::c_uint) -> *const libc::c_char {
    match ut {
        1 => return b"DW_UT_compile\0" as *const u8 as *const libc::c_char,
        2 => return b"DW_UT_type\0" as *const u8 as *const libc::c_char,
        3 => return b"DW_UT_partial\0" as *const u8 as *const libc::c_char,
        4 => return b"DW_UT_skeleton\0" as *const u8 as *const libc::c_char,
        5 => return b"DW_UT_split_compile\0" as *const u8 as *const libc::c_char,
        6 => return b"DW_UT_split_type\0" as *const u8 as *const libc::c_char,
        128 => return b"DW_UT_lo_user\0" as *const u8 as *const libc::c_char,
        255 => return b"DW_UT_hi_user\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return 0 as *const libc::c_char;
}
