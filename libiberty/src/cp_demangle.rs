use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type demangle_callbackref = Option::<
    unsafe extern "C" fn(*const libc::c_char, size_t, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct demangle_component {
    pub type_0: demangle_component_type,
    pub d_printing: libc::c_int,
    pub d_counting: libc::c_int,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s_name: C2RustUnnamed_11,
    pub s_operator: C2RustUnnamed_10,
    pub s_extended_operator: C2RustUnnamed_9,
    pub s_fixed: C2RustUnnamed_8,
    pub s_ctor: C2RustUnnamed_7,
    pub s_dtor: C2RustUnnamed_6,
    pub s_builtin: C2RustUnnamed_5,
    pub s_string: C2RustUnnamed_4,
    pub s_number: C2RustUnnamed_3,
    pub s_character: C2RustUnnamed_2,
    pub s_binary: C2RustUnnamed_1,
    pub s_unary_num: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub sub: *mut demangle_component,
    pub num: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub left: *mut demangle_component,
    pub right: *mut demangle_component,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub character: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub number: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub string: *const libc::c_char,
    pub len: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub type_0: *const demangle_builtin_type_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct demangle_builtin_type_info {
    pub name: *const libc::c_char,
    pub len: libc::c_int,
    pub java_name: *const libc::c_char,
    pub java_len: libc::c_int,
    pub print: d_builtin_type_print,
}
pub type d_builtin_type_print = libc::c_uint;
pub const D_PRINT_VOID: d_builtin_type_print = 9;
pub const D_PRINT_FLOAT: d_builtin_type_print = 8;
pub const D_PRINT_BOOL: d_builtin_type_print = 7;
pub const D_PRINT_UNSIGNED_LONG_LONG: d_builtin_type_print = 6;
pub const D_PRINT_LONG_LONG: d_builtin_type_print = 5;
pub const D_PRINT_UNSIGNED_LONG: d_builtin_type_print = 4;
pub const D_PRINT_LONG: d_builtin_type_print = 3;
pub const D_PRINT_UNSIGNED: d_builtin_type_print = 2;
pub const D_PRINT_INT: d_builtin_type_print = 1;
pub const D_PRINT_DEFAULT: d_builtin_type_print = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub kind: gnu_v3_dtor_kinds,
    pub name: *mut demangle_component,
}
pub type gnu_v3_dtor_kinds = libc::c_uint;
pub const gnu_v3_object_dtor_group: gnu_v3_dtor_kinds = 5;
pub const gnu_v3_unified_dtor: gnu_v3_dtor_kinds = 4;
pub const gnu_v3_base_object_dtor: gnu_v3_dtor_kinds = 3;
pub const gnu_v3_complete_object_dtor: gnu_v3_dtor_kinds = 2;
pub const gnu_v3_deleting_dtor: gnu_v3_dtor_kinds = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub kind: gnu_v3_ctor_kinds,
    pub name: *mut demangle_component,
}
pub type gnu_v3_ctor_kinds = libc::c_uint;
pub const gnu_v3_object_ctor_group: gnu_v3_ctor_kinds = 5;
pub const gnu_v3_unified_ctor: gnu_v3_ctor_kinds = 4;
pub const gnu_v3_complete_object_allocating_ctor: gnu_v3_ctor_kinds = 3;
pub const gnu_v3_base_object_ctor: gnu_v3_ctor_kinds = 2;
pub const gnu_v3_complete_object_ctor: gnu_v3_ctor_kinds = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub length: *mut demangle_component,
    pub accum: libc::c_short,
    pub sat: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub args: libc::c_int,
    pub name: *mut demangle_component,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub op: *const demangle_operator_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct demangle_operator_info {
    pub code: *const libc::c_char,
    pub name: *const libc::c_char,
    pub len: libc::c_int,
    pub args: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub s: *const libc::c_char,
    pub len: libc::c_int,
}
pub type demangle_component_type = libc::c_uint;
pub const DEMANGLE_COMPONENT_THROW_SPEC: demangle_component_type = 81;
pub const DEMANGLE_COMPONENT_NOEXCEPT: demangle_component_type = 80;
pub const DEMANGLE_COMPONENT_CLONE: demangle_component_type = 79;
pub const DEMANGLE_COMPONENT_TRANSACTION_SAFE: demangle_component_type = 78;
pub const DEMANGLE_COMPONENT_TAGGED_NAME: demangle_component_type = 77;
pub const DEMANGLE_COMPONENT_PACK_EXPANSION: demangle_component_type = 76;
pub const DEMANGLE_COMPONENT_NONTRANSACTION_CLONE: demangle_component_type = 75;
pub const DEMANGLE_COMPONENT_TRANSACTION_CLONE: demangle_component_type = 74;
pub const DEMANGLE_COMPONENT_UNNAMED_TYPE: demangle_component_type = 73;
pub const DEMANGLE_COMPONENT_DEFAULT_ARG: demangle_component_type = 72;
pub const DEMANGLE_COMPONENT_LAMBDA: demangle_component_type = 71;
pub const DEMANGLE_COMPONENT_GLOBAL_DESTRUCTORS: demangle_component_type = 70;
pub const DEMANGLE_COMPONENT_GLOBAL_CONSTRUCTORS: demangle_component_type = 69;
pub const DEMANGLE_COMPONENT_DECLTYPE: demangle_component_type = 68;
pub const DEMANGLE_COMPONENT_NUMBER: demangle_component_type = 67;
pub const DEMANGLE_COMPONENT_CHARACTER: demangle_component_type = 66;
pub const DEMANGLE_COMPONENT_COMPOUND_NAME: demangle_component_type = 65;
pub const DEMANGLE_COMPONENT_JAVA_RESOURCE: demangle_component_type = 64;
pub const DEMANGLE_COMPONENT_VENDOR_EXPR: demangle_component_type = 63;
pub const DEMANGLE_COMPONENT_LITERAL_NEG: demangle_component_type = 62;
pub const DEMANGLE_COMPONENT_LITERAL: demangle_component_type = 61;
pub const DEMANGLE_COMPONENT_TRINARY_ARG2: demangle_component_type = 60;
pub const DEMANGLE_COMPONENT_TRINARY_ARG1: demangle_component_type = 59;
pub const DEMANGLE_COMPONENT_TRINARY: demangle_component_type = 58;
pub const DEMANGLE_COMPONENT_BINARY_ARGS: demangle_component_type = 57;
pub const DEMANGLE_COMPONENT_BINARY: demangle_component_type = 56;
pub const DEMANGLE_COMPONENT_UNARY: demangle_component_type = 55;
pub const DEMANGLE_COMPONENT_NULLARY: demangle_component_type = 54;
pub const DEMANGLE_COMPONENT_CONVERSION: demangle_component_type = 53;
pub const DEMANGLE_COMPONENT_CAST: demangle_component_type = 52;
pub const DEMANGLE_COMPONENT_EXTENDED_OPERATOR: demangle_component_type = 51;
pub const DEMANGLE_COMPONENT_OPERATOR: demangle_component_type = 50;
pub const DEMANGLE_COMPONENT_INITIALIZER_LIST: demangle_component_type = 49;
pub const DEMANGLE_COMPONENT_TPARM_OBJ: demangle_component_type = 48;
pub const DEMANGLE_COMPONENT_TEMPLATE_ARGLIST: demangle_component_type = 47;
pub const DEMANGLE_COMPONENT_ARGLIST: demangle_component_type = 46;
pub const DEMANGLE_COMPONENT_VECTOR_TYPE: demangle_component_type = 45;
pub const DEMANGLE_COMPONENT_FIXED_TYPE: demangle_component_type = 44;
pub const DEMANGLE_COMPONENT_PTRMEM_TYPE: demangle_component_type = 43;
pub const DEMANGLE_COMPONENT_ARRAY_TYPE: demangle_component_type = 42;
pub const DEMANGLE_COMPONENT_FUNCTION_TYPE: demangle_component_type = 41;
pub const DEMANGLE_COMPONENT_VENDOR_TYPE: demangle_component_type = 40;
pub const DEMANGLE_COMPONENT_BUILTIN_TYPE: demangle_component_type = 39;
pub const DEMANGLE_COMPONENT_IMAGINARY: demangle_component_type = 38;
pub const DEMANGLE_COMPONENT_COMPLEX: demangle_component_type = 37;
pub const DEMANGLE_COMPONENT_RVALUE_REFERENCE: demangle_component_type = 36;
pub const DEMANGLE_COMPONENT_REFERENCE: demangle_component_type = 35;
pub const DEMANGLE_COMPONENT_POINTER: demangle_component_type = 34;
pub const DEMANGLE_COMPONENT_VENDOR_TYPE_QUAL: demangle_component_type = 33;
pub const DEMANGLE_COMPONENT_RVALUE_REFERENCE_THIS: demangle_component_type = 32;
pub const DEMANGLE_COMPONENT_REFERENCE_THIS: demangle_component_type = 31;
pub const DEMANGLE_COMPONENT_CONST_THIS: demangle_component_type = 30;
pub const DEMANGLE_COMPONENT_VOLATILE_THIS: demangle_component_type = 29;
pub const DEMANGLE_COMPONENT_RESTRICT_THIS: demangle_component_type = 28;
pub const DEMANGLE_COMPONENT_CONST: demangle_component_type = 27;
pub const DEMANGLE_COMPONENT_VOLATILE: demangle_component_type = 26;
pub const DEMANGLE_COMPONENT_RESTRICT: demangle_component_type = 25;
pub const DEMANGLE_COMPONENT_SUB_STD: demangle_component_type = 24;
pub const DEMANGLE_COMPONENT_HIDDEN_ALIAS: demangle_component_type = 23;
pub const DEMANGLE_COMPONENT_REFTEMP: demangle_component_type = 22;
pub const DEMANGLE_COMPONENT_TLS_WRAPPER: demangle_component_type = 21;
pub const DEMANGLE_COMPONENT_TLS_INIT: demangle_component_type = 20;
pub const DEMANGLE_COMPONENT_GUARD: demangle_component_type = 19;
pub const DEMANGLE_COMPONENT_JAVA_CLASS: demangle_component_type = 18;
pub const DEMANGLE_COMPONENT_COVARIANT_THUNK: demangle_component_type = 17;
pub const DEMANGLE_COMPONENT_VIRTUAL_THUNK: demangle_component_type = 16;
pub const DEMANGLE_COMPONENT_THUNK: demangle_component_type = 15;
pub const DEMANGLE_COMPONENT_TYPEINFO_FN: demangle_component_type = 14;
pub const DEMANGLE_COMPONENT_TYPEINFO_NAME: demangle_component_type = 13;
pub const DEMANGLE_COMPONENT_TYPEINFO: demangle_component_type = 12;
pub const DEMANGLE_COMPONENT_CONSTRUCTION_VTABLE: demangle_component_type = 11;
pub const DEMANGLE_COMPONENT_VTT: demangle_component_type = 10;
pub const DEMANGLE_COMPONENT_VTABLE: demangle_component_type = 9;
pub const DEMANGLE_COMPONENT_DTOR: demangle_component_type = 8;
pub const DEMANGLE_COMPONENT_CTOR: demangle_component_type = 7;
pub const DEMANGLE_COMPONENT_FUNCTION_PARAM: demangle_component_type = 6;
pub const DEMANGLE_COMPONENT_TEMPLATE_PARAM: demangle_component_type = 5;
pub const DEMANGLE_COMPONENT_TEMPLATE: demangle_component_type = 4;
pub const DEMANGLE_COMPONENT_TYPED_NAME: demangle_component_type = 3;
pub const DEMANGLE_COMPONENT_LOCAL_NAME: demangle_component_type = 2;
pub const DEMANGLE_COMPONENT_QUAL_NAME: demangle_component_type = 1;
pub const DEMANGLE_COMPONENT_NAME: demangle_component_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct d_print_info {
    pub buf: [libc::c_char; 256],
    pub len: size_t,
    pub last_char: libc::c_char,
    pub callback: demangle_callbackref,
    pub opaque: *mut libc::c_void,
    pub templates: *mut d_print_template,
    pub modifiers: *mut d_print_mod,
    pub demangle_failure: libc::c_int,
    pub recursion: libc::c_int,
    pub is_lambda_arg: libc::c_int,
    pub pack_index: libc::c_int,
    pub flush_count: libc::c_ulong,
    pub component_stack: *const d_component_stack,
    pub saved_scopes: *mut d_saved_scope,
    pub next_saved_scope: libc::c_int,
    pub num_saved_scopes: libc::c_int,
    pub copy_templates: *mut d_print_template,
    pub next_copy_template: libc::c_int,
    pub num_copy_templates: libc::c_int,
    pub current_template: *const demangle_component,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct d_print_template {
    pub next: *mut d_print_template,
    pub template_decl: *const demangle_component,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct d_saved_scope {
    pub container: *const demangle_component,
    pub templates: *mut d_print_template,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct d_component_stack {
    pub dc: *const demangle_component,
    pub parent: *const d_component_stack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct d_print_mod {
    pub next: *mut d_print_mod,
    pub mod_0: *mut demangle_component,
    pub printed: libc::c_int,
    pub templates: *mut d_print_template,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct d_info {
    pub s: *const libc::c_char,
    pub send: *const libc::c_char,
    pub options: libc::c_int,
    pub n: *const libc::c_char,
    pub comps: *mut demangle_component,
    pub next_comp: libc::c_int,
    pub num_comps: libc::c_int,
    pub subs: *mut *mut demangle_component,
    pub next_sub: libc::c_int,
    pub num_subs: libc::c_int,
    pub last_name: *mut demangle_component,
    pub expansion: libc::c_int,
    pub is_expression: libc::c_int,
    pub is_conversion: libc::c_int,
    pub unresolved_name_state: libc::c_int,
    pub recursion_level: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct d_standard_sub_info {
    pub code: libc::c_char,
    pub simple_expansion: *const libc::c_char,
    pub simple_len: libc::c_int,
    pub full_expansion: *const libc::c_char,
    pub full_len: libc::c_int,
    pub set_last_name: *const libc::c_char,
    pub set_last_name_len: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct d_info_checkpoint {
    pub n: *const libc::c_char,
    pub next_comp: libc::c_int,
    pub next_sub: libc::c_int,
    pub expansion: libc::c_int,
}
pub const DCT_GLOBAL_CTORS: C2RustUnnamed_12 = 2;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const DCT_GLOBAL_DTORS: C2RustUnnamed_12 = 3;
pub const DCT_MANGLED: C2RustUnnamed_12 = 1;
pub const DCT_TYPE: C2RustUnnamed_12 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct d_growable_string {
    pub buf: *mut libc::c_char,
    pub len: size_t,
    pub alc: size_t,
    pub allocation_failure: libc::c_int,
}
unsafe extern "C" fn is_fnqual_component_type(
    mut type_0: demangle_component_type,
) -> libc::c_int {
    match type_0 as libc::c_uint {
        28 | 29 | 30 | 31 | 32 | 78 | 80 | 81 => return 1 as libc::c_int,
        _ => {}
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_fill_name(
    mut p: *mut demangle_component,
    mut s: *const libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    if p.is_null() || s.is_null() || len <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*p).d_printing = 0 as libc::c_int;
    (*p).d_counting = 0 as libc::c_int;
    (*p).type_0 = DEMANGLE_COMPONENT_NAME;
    (*p).u.s_name.s = s;
    (*p).u.s_name.len = len;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_fill_extended_operator(
    mut p: *mut demangle_component,
    mut args: libc::c_int,
    mut name: *mut demangle_component,
) -> libc::c_int {
    if p.is_null() || args < 0 as libc::c_int || name.is_null() {
        return 0 as libc::c_int;
    }
    (*p).d_printing = 0 as libc::c_int;
    (*p).d_counting = 0 as libc::c_int;
    (*p).type_0 = DEMANGLE_COMPONENT_EXTENDED_OPERATOR;
    (*p).u.s_extended_operator.args = args;
    (*p).u.s_extended_operator.name = name;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_fill_ctor(
    mut p: *mut demangle_component,
    mut kind: gnu_v3_ctor_kinds,
    mut name: *mut demangle_component,
) -> libc::c_int {
    if p.is_null() || name.is_null()
        || (kind as libc::c_int) < gnu_v3_complete_object_ctor as libc::c_int
        || kind as libc::c_int > gnu_v3_object_ctor_group as libc::c_int
    {
        return 0 as libc::c_int;
    }
    (*p).d_printing = 0 as libc::c_int;
    (*p).d_counting = 0 as libc::c_int;
    (*p).type_0 = DEMANGLE_COMPONENT_CTOR;
    (*p).u.s_ctor.kind = kind;
    (*p).u.s_ctor.name = name;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_fill_dtor(
    mut p: *mut demangle_component,
    mut kind: gnu_v3_dtor_kinds,
    mut name: *mut demangle_component,
) -> libc::c_int {
    if p.is_null() || name.is_null()
        || (kind as libc::c_int) < gnu_v3_deleting_dtor as libc::c_int
        || kind as libc::c_int > gnu_v3_object_dtor_group as libc::c_int
    {
        return 0 as libc::c_int;
    }
    (*p).d_printing = 0 as libc::c_int;
    (*p).d_counting = 0 as libc::c_int;
    (*p).type_0 = DEMANGLE_COMPONENT_DTOR;
    (*p).u.s_dtor.kind = kind;
    (*p).u.s_dtor.name = name;
    return 1 as libc::c_int;
}
unsafe extern "C" fn d_make_empty(mut di: *mut d_info) -> *mut demangle_component {
    let mut p: *mut demangle_component = 0 as *mut demangle_component;
    if (*di).next_comp >= (*di).num_comps {
        return 0 as *mut demangle_component;
    }
    p = &mut *((*di).comps).offset((*di).next_comp as isize) as *mut demangle_component;
    (*p).d_printing = 0 as libc::c_int;
    (*p).d_counting = 0 as libc::c_int;
    (*di).next_comp += 1;
    (*di).next_comp;
    return p;
}
unsafe extern "C" fn d_make_comp(
    mut di: *mut d_info,
    mut type_0: demangle_component_type,
    mut left: *mut demangle_component,
    mut right: *mut demangle_component,
) -> *mut demangle_component {
    let mut p: *mut demangle_component = 0 as *mut demangle_component;
    match type_0 as libc::c_uint {
        1 | 2 | 3 | 77 | 4 | 11 | 33 | 43 | 55 | 56 | 57 | 58 | 59 | 61 | 62 | 63 | 65
        | 45 | 79 => {
            if left.is_null() || right.is_null() {
                return 0 as *mut demangle_component;
            }
        }
        9 | 10 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 74 | 75 | 34
        | 35 | 36 | 37 | 38 | 40 | 52 | 53 | 64 | 68 | 76 | 69 | 70 | 54 | 60 | 48 => {
            if left.is_null() {
                return 0 as *mut demangle_component;
            }
        }
        42 | 49 => {
            if right.is_null() {
                return 0 as *mut demangle_component;
            }
        }
        41 | 25 | 26 | 27 | 46 | 47 | 28 | 29 | 30 | 31 | 32 | 78 | 80 | 81 => {}
        _ => return 0 as *mut demangle_component,
    }
    p = d_make_empty(di);
    if !p.is_null() {
        (*p).type_0 = type_0;
        (*p).u.s_binary.left = left;
        (*p).u.s_binary.right = right;
    }
    return p;
}
unsafe extern "C" fn d_make_demangle_mangled_name(
    mut di: *mut d_info,
    mut s: *const libc::c_char,
) -> *mut demangle_component {
    if *(*di).n as libc::c_int != '_' as i32
        || *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int != 'Z' as i32
    {
        return d_make_name(di, s, strlen(s) as libc::c_int);
    }
    (*di).n = ((*di).n).offset(2 as libc::c_int as isize);
    return d_encoding(di, 0 as libc::c_int);
}
unsafe extern "C" fn d_make_name(
    mut di: *mut d_info,
    mut s: *const libc::c_char,
    mut len: libc::c_int,
) -> *mut demangle_component {
    let mut p: *mut demangle_component = 0 as *mut demangle_component;
    p = d_make_empty(di);
    if cplus_demangle_fill_name(p, s, len) == 0 {
        return 0 as *mut demangle_component;
    }
    return p;
}
unsafe extern "C" fn d_make_builtin_type(
    mut di: *mut d_info,
    mut type_0: *const demangle_builtin_type_info,
) -> *mut demangle_component {
    let mut p: *mut demangle_component = 0 as *mut demangle_component;
    if type_0.is_null() {
        return 0 as *mut demangle_component;
    }
    p = d_make_empty(di);
    if !p.is_null() {
        (*p).type_0 = DEMANGLE_COMPONENT_BUILTIN_TYPE;
        (*p).u.s_builtin.type_0 = type_0;
    }
    return p;
}
unsafe extern "C" fn d_make_operator(
    mut di: *mut d_info,
    mut op: *const demangle_operator_info,
) -> *mut demangle_component {
    let mut p: *mut demangle_component = 0 as *mut demangle_component;
    p = d_make_empty(di);
    if !p.is_null() {
        (*p).type_0 = DEMANGLE_COMPONENT_OPERATOR;
        (*p).u.s_operator.op = op;
    }
    return p;
}
unsafe extern "C" fn d_make_extended_operator(
    mut di: *mut d_info,
    mut args: libc::c_int,
    mut name: *mut demangle_component,
) -> *mut demangle_component {
    let mut p: *mut demangle_component = 0 as *mut demangle_component;
    p = d_make_empty(di);
    if cplus_demangle_fill_extended_operator(p, args, name) == 0 {
        return 0 as *mut demangle_component;
    }
    return p;
}
unsafe extern "C" fn d_make_default_arg(
    mut di: *mut d_info,
    mut num: libc::c_int,
    mut sub: *mut demangle_component,
) -> *mut demangle_component {
    let mut p: *mut demangle_component = d_make_empty(di);
    if !p.is_null() {
        (*p).type_0 = DEMANGLE_COMPONENT_DEFAULT_ARG;
        (*p).u.s_unary_num.num = num;
        (*p).u.s_unary_num.sub = sub;
    }
    return p;
}
unsafe extern "C" fn d_make_ctor(
    mut di: *mut d_info,
    mut kind: gnu_v3_ctor_kinds,
    mut name: *mut demangle_component,
) -> *mut demangle_component {
    let mut p: *mut demangle_component = 0 as *mut demangle_component;
    p = d_make_empty(di);
    if cplus_demangle_fill_ctor(p, kind, name) == 0 {
        return 0 as *mut demangle_component;
    }
    return p;
}
unsafe extern "C" fn d_make_dtor(
    mut di: *mut d_info,
    mut kind: gnu_v3_dtor_kinds,
    mut name: *mut demangle_component,
) -> *mut demangle_component {
    let mut p: *mut demangle_component = 0 as *mut demangle_component;
    p = d_make_empty(di);
    if cplus_demangle_fill_dtor(p, kind, name) == 0 {
        return 0 as *mut demangle_component;
    }
    return p;
}
unsafe extern "C" fn d_make_template_param(
    mut di: *mut d_info,
    mut i: libc::c_int,
) -> *mut demangle_component {
    let mut p: *mut demangle_component = 0 as *mut demangle_component;
    p = d_make_empty(di);
    if !p.is_null() {
        (*p).type_0 = DEMANGLE_COMPONENT_TEMPLATE_PARAM;
        (*p).u.s_number.number = i as libc::c_long;
    }
    return p;
}
unsafe extern "C" fn d_make_function_param(
    mut di: *mut d_info,
    mut i: libc::c_int,
) -> *mut demangle_component {
    let mut p: *mut demangle_component = 0 as *mut demangle_component;
    p = d_make_empty(di);
    if !p.is_null() {
        (*p).type_0 = DEMANGLE_COMPONENT_FUNCTION_PARAM;
        (*p).u.s_number.number = i as libc::c_long;
    }
    return p;
}
unsafe extern "C" fn d_make_sub(
    mut di: *mut d_info,
    mut name: *const libc::c_char,
    mut len: libc::c_int,
) -> *mut demangle_component {
    let mut p: *mut demangle_component = 0 as *mut demangle_component;
    p = d_make_empty(di);
    if !p.is_null() {
        (*p).type_0 = DEMANGLE_COMPONENT_SUB_STD;
        (*p).u.s_string.string = name;
        (*p).u.s_string.len = len;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_mangled_name(
    mut di: *mut d_info,
    mut top_level: libc::c_int,
) -> *mut demangle_component {
    let mut p: *mut demangle_component = 0 as *mut demangle_component;
    if (if *(*di).n as libc::c_int == '_' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) == 0 && top_level != 0
    {
        return 0 as *mut demangle_component;
    }
    if if *(*di).n as libc::c_int == 'Z' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    p = d_encoding(di, top_level);
    if top_level != 0
        && (*di).options & (1 as libc::c_int) << 0 as libc::c_int != 0 as libc::c_int
    {
        while *(*di).n as libc::c_int == '.' as i32
            && (*((*di).n).offset(1 as libc::c_int as isize) as libc::c_int >= 'a' as i32
                && *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int
                    <= 'z' as i32
                || *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int
                    == '_' as i32
                || *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int
                    >= '0' as i32
                    && *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int
                        <= '9' as i32)
        {
            p = d_clone_suffix(di, p);
        }
    }
    return p;
}
unsafe extern "C" fn has_return_type(mut dc: *mut demangle_component) -> libc::c_int {
    if dc.is_null() {
        return 0 as libc::c_int;
    }
    match (*dc).type_0 as libc::c_uint {
        2 => return has_return_type((*dc).u.s_binary.right),
        4 => {
            return (is_ctor_dtor_or_conversion((*dc).u.s_binary.left) == 0)
                as libc::c_int;
        }
        28 | 29 | 30 | 31 | 32 | 78 | 80 | 81 => {
            return has_return_type((*dc).u.s_binary.left);
        }
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn is_ctor_dtor_or_conversion(
    mut dc: *mut demangle_component,
) -> libc::c_int {
    if dc.is_null() {
        return 0 as libc::c_int;
    }
    match (*dc).type_0 as libc::c_uint {
        1 | 2 => return is_ctor_dtor_or_conversion((*dc).u.s_binary.right),
        7 | 8 | 53 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn d_encoding(
    mut di: *mut d_info,
    mut top_level: libc::c_int,
) -> *mut demangle_component {
    let mut peek: libc::c_char = *(*di).n;
    let mut dc: *mut demangle_component = 0 as *mut demangle_component;
    if peek as libc::c_int == 'G' as i32 || peek as libc::c_int == 'T' as i32 {
        dc = d_special_name(di);
    } else {
        dc = d_name(di);
        if !dc.is_null() {
            if top_level != 0
                && (*di).options & (1 as libc::c_int) << 0 as libc::c_int
                    == 0 as libc::c_int
            {
                while is_fnqual_component_type((*dc).type_0) != 0 {
                    dc = (*dc).u.s_binary.left;
                }
                if (*dc).type_0 as libc::c_uint
                    == DEMANGLE_COMPONENT_LOCAL_NAME as libc::c_int as libc::c_uint
                {
                    while !((*dc).u.s_binary.right).is_null()
                        && is_fnqual_component_type((*(*dc).u.s_binary.right).type_0)
                            != 0
                    {
                        (*dc)
                            .u
                            .s_binary
                            .right = (*(*dc).u.s_binary.right).u.s_binary.left;
                    }
                    if ((*dc).u.s_binary.right).is_null() {
                        dc = 0 as *mut demangle_component;
                    }
                }
            } else {
                peek = *(*di).n;
                if peek as libc::c_int != '\0' as i32
                    && peek as libc::c_int != 'E' as i32
                {
                    let mut ftype: *mut demangle_component = 0
                        as *mut demangle_component;
                    ftype = d_bare_function_type(di, has_return_type(dc));
                    if !ftype.is_null() {
                        if top_level == 0
                            && (*dc).type_0 as libc::c_uint
                                == DEMANGLE_COMPONENT_LOCAL_NAME as libc::c_int
                                    as libc::c_uint
                            && (*ftype).type_0 as libc::c_uint
                                == DEMANGLE_COMPONENT_FUNCTION_TYPE as libc::c_int
                                    as libc::c_uint
                        {
                            (*ftype).u.s_binary.left = 0 as *mut demangle_component;
                        }
                        dc = d_make_comp(di, DEMANGLE_COMPONENT_TYPED_NAME, dc, ftype);
                    } else {
                        dc = 0 as *mut demangle_component;
                    }
                }
            }
        }
    }
    return dc;
}
unsafe extern "C" fn d_abi_tags(
    mut di: *mut d_info,
    mut dc: *mut demangle_component,
) -> *mut demangle_component {
    let mut hold_last_name: *mut demangle_component = 0 as *mut demangle_component;
    let mut peek: libc::c_char = 0;
    hold_last_name = (*di).last_name;
    loop {
        peek = *(*di).n;
        if !(peek as libc::c_int == 'B' as i32) {
            break;
        }
        let mut tag: *mut demangle_component = 0 as *mut demangle_component;
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        tag = d_source_name(di);
        dc = d_make_comp(di, DEMANGLE_COMPONENT_TAGGED_NAME, dc, tag);
    }
    (*di).last_name = hold_last_name;
    return dc;
}
unsafe extern "C" fn d_name(mut di: *mut d_info) -> *mut demangle_component {
    let mut peek: libc::c_char = *(*di).n;
    let mut dc: *mut demangle_component = 0 as *mut demangle_component;
    match peek as libc::c_int {
        78 => return d_nested_name(di),
        90 => return d_local_name(di),
        85 => return d_unqualified_name(di),
        83 => {
            let mut subst: libc::c_int = 0;
            if *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int != 't' as i32
            {
                dc = d_substitution(di, 0 as libc::c_int);
                subst = 1 as libc::c_int;
            } else {
                (*di).n = ((*di).n).offset(2 as libc::c_int as isize);
                dc = d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_QUAL_NAME,
                    d_make_name(
                        di,
                        b"std\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int,
                    ),
                    d_unqualified_name(di),
                );
                (*di).expansion += 3 as libc::c_int;
                subst = 0 as libc::c_int;
            }
            if !(*(*di).n as libc::c_int != 'I' as i32) {
                if subst == 0 {
                    if d_add_substitution(di, dc) == 0 {
                        return 0 as *mut demangle_component;
                    }
                }
                dc = d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_TEMPLATE,
                    dc,
                    d_template_args(di),
                );
            }
            return dc;
        }
        76 | _ => {
            dc = d_unqualified_name(di);
            if *(*di).n as libc::c_int == 'I' as i32 {
                if d_add_substitution(di, dc) == 0 {
                    return 0 as *mut demangle_component;
                }
                dc = d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_TEMPLATE,
                    dc,
                    d_template_args(di),
                );
            }
            return dc;
        }
    };
}
unsafe extern "C" fn d_nested_name(mut di: *mut d_info) -> *mut demangle_component {
    let mut ret: *mut demangle_component = 0 as *mut demangle_component;
    let mut pret: *mut *mut demangle_component = 0 as *mut *mut demangle_component;
    let mut rqual: *mut demangle_component = 0 as *mut demangle_component;
    if if *(*di).n as libc::c_int == 'N' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    pret = d_cv_qualifiers(di, &mut ret, 1 as libc::c_int);
    if pret.is_null() {
        return 0 as *mut demangle_component;
    }
    rqual = d_ref_qualifier(di, 0 as *mut demangle_component);
    *pret = d_prefix(di, 1 as libc::c_int);
    if (*pret).is_null() {
        return 0 as *mut demangle_component;
    }
    if !rqual.is_null() {
        (*rqual).u.s_binary.left = ret;
        ret = rqual;
    }
    if if *(*di).n as libc::c_int == 'E' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    return ret;
}
unsafe extern "C" fn d_prefix(
    mut di: *mut d_info,
    mut subst: libc::c_int,
) -> *mut demangle_component {
    let mut ret: *mut demangle_component = 0 as *mut demangle_component;
    loop {
        let mut peek: libc::c_char = 0;
        let mut comb_type: demangle_component_type = DEMANGLE_COMPONENT_NAME;
        let mut dc: *mut demangle_component = 0 as *mut demangle_component;
        peek = *(*di).n;
        if peek as libc::c_int == '\0' as i32 {
            return 0 as *mut demangle_component;
        }
        comb_type = DEMANGLE_COMPONENT_QUAL_NAME;
        if peek as libc::c_int == 'D' as i32 {
            let mut peek2: libc::c_char = *((*di).n).offset(1 as libc::c_int as isize);
            if peek2 as libc::c_int == 'T' as i32 || peek2 as libc::c_int == 't' as i32 {
                dc = cplus_demangle_type(di);
            } else {
                dc = d_unqualified_name(di);
            }
        } else if peek as libc::c_int >= '0' as i32 && peek as libc::c_int <= '9' as i32
            || peek as libc::c_int >= 'a' as i32 && peek as libc::c_int <= 'z' as i32
            || peek as libc::c_int == 'C' as i32 || peek as libc::c_int == 'U' as i32
            || peek as libc::c_int == 'L' as i32
        {
            dc = d_unqualified_name(di);
        } else if peek as libc::c_int == 'S' as i32 {
            dc = d_substitution(di, 1 as libc::c_int);
        } else if peek as libc::c_int == 'I' as i32 {
            if ret.is_null() {
                return 0 as *mut demangle_component;
            }
            comb_type = DEMANGLE_COMPONENT_TEMPLATE;
            dc = d_template_args(di);
        } else if peek as libc::c_int == 'T' as i32 {
            dc = d_template_param(di);
        } else if peek as libc::c_int == 'E' as i32 {
            return ret
        } else if peek as libc::c_int == 'M' as i32 {
            if ret.is_null() {
                return 0 as *mut demangle_component;
            }
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
            continue;
        } else {
            return 0 as *mut demangle_component
        }
        if ret.is_null() {
            ret = dc;
        } else {
            ret = d_make_comp(di, comb_type, ret, dc);
        }
        if peek as libc::c_int != 'S' as i32 && *(*di).n as libc::c_int != 'E' as i32
            && subst != 0
        {
            if d_add_substitution(di, ret) == 0 {
                return 0 as *mut demangle_component;
            }
        }
    };
}
unsafe extern "C" fn d_unqualified_name(mut di: *mut d_info) -> *mut demangle_component {
    let mut ret: *mut demangle_component = 0 as *mut demangle_component;
    let mut peek: libc::c_char = 0;
    peek = *(*di).n;
    if peek as libc::c_int >= '0' as i32 && peek as libc::c_int <= '9' as i32 {
        ret = d_source_name(di);
    } else if peek as libc::c_int >= 'a' as i32 && peek as libc::c_int <= 'z' as i32 {
        let mut was_expr: libc::c_int = (*di).is_expression;
        if peek as libc::c_int == 'o' as i32
            && *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int == 'n' as i32
        {
            (*di).n = ((*di).n).offset(2 as libc::c_int as isize);
            (*di).is_expression = 0 as libc::c_int;
        }
        ret = d_operator_name(di);
        (*di).is_expression = was_expr;
        if !ret.is_null()
            && (*ret).type_0 as libc::c_uint
                == DEMANGLE_COMPONENT_OPERATOR as libc::c_int as libc::c_uint
        {
            (*di)
                .expansion = ((*di).expansion as libc::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                        .wrapping_add((*(*ret).u.s_operator.op).len as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong),
                ) as libc::c_int as libc::c_int;
            if strcmp(
                (*(*ret).u.s_operator.op).code,
                b"li\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                ret = d_make_comp(di, DEMANGLE_COMPONENT_UNARY, ret, d_source_name(di));
            }
        }
    } else if peek as libc::c_int == 'C' as i32 || peek as libc::c_int == 'D' as i32 {
        ret = d_ctor_dtor_name(di);
    } else if peek as libc::c_int == 'L' as i32 {
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        ret = d_source_name(di);
        if ret.is_null() {
            return 0 as *mut demangle_component;
        }
        if d_discriminator(di) == 0 {
            return 0 as *mut demangle_component;
        }
    } else if peek as libc::c_int == 'U' as i32 {
        match *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int {
            108 => {
                ret = d_lambda(di);
            }
            116 => {
                ret = d_unnamed_type(di);
            }
            _ => return 0 as *mut demangle_component,
        }
    } else {
        return 0 as *mut demangle_component
    }
    if *(*di).n as libc::c_int == 'B' as i32 {
        ret = d_abi_tags(di, ret);
    }
    return ret;
}
unsafe extern "C" fn d_source_name(mut di: *mut d_info) -> *mut demangle_component {
    let mut len: libc::c_int = 0;
    let mut ret: *mut demangle_component = 0 as *mut demangle_component;
    len = d_number(di);
    if len <= 0 as libc::c_int {
        return 0 as *mut demangle_component;
    }
    ret = d_identifier(di, len);
    (*di).last_name = ret;
    return ret;
}
unsafe extern "C" fn d_number(mut di: *mut d_info) -> libc::c_int {
    let mut negative: libc::c_int = 0;
    let mut peek: libc::c_char = 0;
    let mut ret: libc::c_int = 0;
    negative = 0 as libc::c_int;
    peek = *(*di).n;
    if peek as libc::c_int == 'n' as i32 {
        negative = 1 as libc::c_int;
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        peek = *(*di).n;
    }
    ret = 0 as libc::c_int;
    loop {
        if !(peek as libc::c_int >= '0' as i32 && peek as libc::c_int <= '9' as i32) {
            if negative != 0 {
                ret = -ret;
            }
            return ret;
        }
        if ret
            > (2147483647 as libc::c_int - (peek as libc::c_int - '0' as i32))
                / 10 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        ret = ret * 10 as libc::c_int + (peek as libc::c_int - '0' as i32);
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        peek = *(*di).n;
    };
}
unsafe extern "C" fn d_number_component(mut di: *mut d_info) -> *mut demangle_component {
    let mut ret: *mut demangle_component = d_make_empty(di);
    if !ret.is_null() {
        (*ret).type_0 = DEMANGLE_COMPONENT_NUMBER;
        (*ret).u.s_number.number = d_number(di) as libc::c_long;
    }
    return ret;
}
unsafe extern "C" fn d_identifier(
    mut di: *mut d_info,
    mut len: libc::c_int,
) -> *mut demangle_component {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    name = (*di).n;
    if (((*di).send).offset_from(name) as libc::c_long) < len as libc::c_long {
        return 0 as *mut demangle_component;
    }
    (*di).n = ((*di).n).offset(len as isize);
    if (*di).options & (1 as libc::c_int) << 2 as libc::c_int != 0 as libc::c_int
        && *(*di).n as libc::c_int == '$' as i32
    {
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
    }
    if len
        >= (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int
            + 2 as libc::c_int
        && memcmp(
            name as *const libc::c_void,
            b"_GLOBAL_\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
    {
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        s = name
            .offset(
                (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
        if (*s as libc::c_int == '.' as i32 || *s as libc::c_int == '_' as i32
            || *s as libc::c_int == '$' as i32)
            && *s.offset(1 as libc::c_int as isize) as libc::c_int == 'N' as i32
        {
            (*di)
                .expansion = ((*di).expansion as libc::c_ulong)
                .wrapping_sub(
                    (len as libc::c_ulong)
                        .wrapping_sub(
                            ::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong,
                        ),
                ) as libc::c_int as libc::c_int;
            return d_make_name(
                di,
                b"(anonymous namespace)\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            );
        }
    }
    return d_make_name(di, name, len);
}
#[no_mangle]
pub static mut cplus_demangle_operators: [demangle_operator_info; 73] = [demangle_operator_info {
    code: 0 as *const libc::c_char,
    name: 0 as *const libc::c_char,
    len: 0,
    args: 0,
}; 73];
unsafe extern "C" fn d_operator_name(mut di: *mut d_info) -> *mut demangle_component {
    let mut c1: libc::c_char = 0;
    let mut c2: libc::c_char = 0;
    c1 = (if *(*di).n as libc::c_int == '\0' as i32 {
        '\0' as i32
    } else {
        let fresh0 = (*di).n;
        (*di).n = ((*di).n).offset(1);
        *fresh0 as libc::c_int
    }) as libc::c_char;
    c2 = (if *(*di).n as libc::c_int == '\0' as i32 {
        '\0' as i32
    } else {
        let fresh1 = (*di).n;
        (*di).n = ((*di).n).offset(1);
        *fresh1 as libc::c_int
    }) as libc::c_char;
    if c1 as libc::c_int == 'v' as i32
        && (c2 as libc::c_int >= '0' as i32 && c2 as libc::c_int <= '9' as i32)
    {
        return d_make_extended_operator(
            di,
            c2 as libc::c_int - '0' as i32,
            d_source_name(di),
        )
    } else if c1 as libc::c_int == 'c' as i32 && c2 as libc::c_int == 'v' as i32 {
        let mut type_0: *mut demangle_component = 0 as *mut demangle_component;
        let mut was_conversion: libc::c_int = (*di).is_conversion;
        let mut res: *mut demangle_component = 0 as *mut demangle_component;
        (*di).is_conversion = ((*di).is_expression == 0) as libc::c_int;
        type_0 = cplus_demangle_type(di);
        if (*di).is_conversion != 0 {
            res = d_make_comp(
                di,
                DEMANGLE_COMPONENT_CONVERSION,
                type_0,
                0 as *mut demangle_component,
            );
        } else {
            res = d_make_comp(
                di,
                DEMANGLE_COMPONENT_CAST,
                type_0,
                0 as *mut demangle_component,
            );
        }
        (*di).is_conversion = was_conversion;
        return res;
    } else {
        let mut low: libc::c_int = 0 as libc::c_int;
        let mut high: libc::c_int = (::core::mem::size_of::<
            [demangle_operator_info; 73],
        >() as libc::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<demangle_operator_info>() as libc::c_ulong,
            )
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        loop {
            let mut i: libc::c_int = 0;
            let mut p: *const demangle_operator_info = 0
                as *const demangle_operator_info;
            i = low + (high - low) / 2 as libc::c_int;
            p = cplus_demangle_operators.as_ptr().offset(i as isize);
            if c1 as libc::c_int
                == *((*p).code).offset(0 as libc::c_int as isize) as libc::c_int
                && c2 as libc::c_int
                    == *((*p).code).offset(1 as libc::c_int as isize) as libc::c_int
            {
                return d_make_operator(di, p);
            }
            if (c1 as libc::c_int)
                < *((*p).code).offset(0 as libc::c_int as isize) as libc::c_int
                || c1 as libc::c_int
                    == *((*p).code).offset(0 as libc::c_int as isize) as libc::c_int
                    && (c2 as libc::c_int)
                        < *((*p).code).offset(1 as libc::c_int as isize) as libc::c_int
            {
                high = i;
            } else {
                low = i + 1 as libc::c_int;
            }
            if low == high {
                return 0 as *mut demangle_component;
            }
        }
    };
}
unsafe extern "C" fn d_make_character(
    mut di: *mut d_info,
    mut c: libc::c_int,
) -> *mut demangle_component {
    let mut p: *mut demangle_component = 0 as *mut demangle_component;
    p = d_make_empty(di);
    if !p.is_null() {
        (*p).type_0 = DEMANGLE_COMPONENT_CHARACTER;
        (*p).u.s_character.character = c;
    }
    return p;
}
unsafe extern "C" fn d_java_resource(mut di: *mut d_info) -> *mut demangle_component {
    let mut p: *mut demangle_component = 0 as *mut demangle_component;
    let mut next: *mut demangle_component = 0 as *mut demangle_component;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    len = d_number(di);
    if len <= 1 as libc::c_int {
        return 0 as *mut demangle_component;
    }
    if (if *(*di).n as libc::c_int == '\0' as i32 {
        '\0' as i32
    } else {
        let fresh2 = (*di).n;
        (*di).n = ((*di).n).offset(1);
        *fresh2 as libc::c_int
    }) != '_' as i32
    {
        return 0 as *mut demangle_component;
    }
    len -= 1;
    len;
    str = (*di).n;
    i = 0 as libc::c_int;
    while len > 0 as libc::c_int {
        c = *str.offset(i as isize);
        if c == 0 {
            return 0 as *mut demangle_component;
        }
        if c as libc::c_int == '$' as i32 {
            i += 1;
            i;
            let fresh3 = i;
            i = i + 1;
            match *str.offset(fresh3 as isize) as libc::c_int {
                83 => {
                    c = '/' as i32 as libc::c_char;
                }
                95 => {
                    c = '.' as i32 as libc::c_char;
                }
                36 => {
                    c = '$' as i32 as libc::c_char;
                }
                _ => return 0 as *mut demangle_component,
            }
            next = d_make_character(di, c as libc::c_int);
            (*di).n = ((*di).n).offset(i as isize);
            str = (*di).n;
            len -= i;
            i = 0 as libc::c_int;
            if next.is_null() {
                return 0 as *mut demangle_component;
            }
        } else {
            while i < len && *str.offset(i as isize) as libc::c_int != 0
                && *str.offset(i as isize) as libc::c_int != '$' as i32
            {
                i += 1;
                i;
            }
            next = d_make_name(di, str, i);
            (*di).n = ((*di).n).offset(i as isize);
            str = (*di).n;
            len -= i;
            i = 0 as libc::c_int;
            if next.is_null() {
                return 0 as *mut demangle_component;
            }
        }
        if p.is_null() {
            p = next;
        } else {
            p = d_make_comp(di, DEMANGLE_COMPONENT_COMPOUND_NAME, p, next);
            if p.is_null() {
                return 0 as *mut demangle_component;
            }
        }
    }
    p = d_make_comp(
        di,
        DEMANGLE_COMPONENT_JAVA_RESOURCE,
        p,
        0 as *mut demangle_component,
    );
    return p;
}
unsafe extern "C" fn d_special_name(mut di: *mut d_info) -> *mut demangle_component {
    (*di).expansion += 20 as libc::c_int;
    if if *(*di).n as libc::c_int == 'T' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        match if *(*di).n as libc::c_int == '\0' as i32 {
            '\0' as i32
        } else {
            let fresh4 = (*di).n;
            (*di).n = ((*di).n).offset(1);
            *fresh4 as libc::c_int
        } {
            86 => {
                (*di).expansion -= 5 as libc::c_int;
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_VTABLE,
                    cplus_demangle_type(di),
                    0 as *mut demangle_component,
                );
            }
            84 => {
                (*di).expansion -= 10 as libc::c_int;
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_VTT,
                    cplus_demangle_type(di),
                    0 as *mut demangle_component,
                );
            }
            73 => {
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_TYPEINFO,
                    cplus_demangle_type(di),
                    0 as *mut demangle_component,
                );
            }
            83 => {
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_TYPEINFO_NAME,
                    cplus_demangle_type(di),
                    0 as *mut demangle_component,
                );
            }
            104 => {
                if d_call_offset(di, 'h' as i32) == 0 {
                    return 0 as *mut demangle_component;
                }
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_THUNK,
                    d_encoding(di, 0 as libc::c_int),
                    0 as *mut demangle_component,
                );
            }
            118 => {
                if d_call_offset(di, 'v' as i32) == 0 {
                    return 0 as *mut demangle_component;
                }
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_VIRTUAL_THUNK,
                    d_encoding(di, 0 as libc::c_int),
                    0 as *mut demangle_component,
                );
            }
            99 => {
                if d_call_offset(di, '\0' as i32) == 0 {
                    return 0 as *mut demangle_component;
                }
                if d_call_offset(di, '\0' as i32) == 0 {
                    return 0 as *mut demangle_component;
                }
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_COVARIANT_THUNK,
                    d_encoding(di, 0 as libc::c_int),
                    0 as *mut demangle_component,
                );
            }
            67 => {
                let mut derived_type: *mut demangle_component = 0
                    as *mut demangle_component;
                let mut offset: libc::c_int = 0;
                let mut base_type: *mut demangle_component = 0
                    as *mut demangle_component;
                derived_type = cplus_demangle_type(di);
                offset = d_number(di);
                if offset < 0 as libc::c_int {
                    return 0 as *mut demangle_component;
                }
                if if *(*di).n as libc::c_int == '_' as i32 {
                    (*di).n = ((*di).n).offset(1);
                    (*di).n;
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                } == 0
                {
                    return 0 as *mut demangle_component;
                }
                base_type = cplus_demangle_type(di);
                (*di).expansion += 5 as libc::c_int;
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_CONSTRUCTION_VTABLE,
                    base_type,
                    derived_type,
                );
            }
            70 => {
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_TYPEINFO_FN,
                    cplus_demangle_type(di),
                    0 as *mut demangle_component,
                );
            }
            74 => {
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_JAVA_CLASS,
                    cplus_demangle_type(di),
                    0 as *mut demangle_component,
                );
            }
            72 => {
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_TLS_INIT,
                    d_name(di),
                    0 as *mut demangle_component,
                );
            }
            87 => {
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_TLS_WRAPPER,
                    d_name(di),
                    0 as *mut demangle_component,
                );
            }
            65 => {
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_TPARM_OBJ,
                    d_template_arg(di),
                    0 as *mut demangle_component,
                );
            }
            _ => return 0 as *mut demangle_component,
        }
    } else if if *(*di).n as libc::c_int == 'G' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        match if *(*di).n as libc::c_int == '\0' as i32 {
            '\0' as i32
        } else {
            let fresh5 = (*di).n;
            (*di).n = ((*di).n).offset(1);
            *fresh5 as libc::c_int
        } {
            86 => {
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_GUARD,
                    d_name(di),
                    0 as *mut demangle_component,
                );
            }
            82 => {
                let mut name: *mut demangle_component = d_name(di);
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_REFTEMP,
                    name,
                    d_number_component(di),
                );
            }
            65 => {
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_HIDDEN_ALIAS,
                    d_encoding(di, 0 as libc::c_int),
                    0 as *mut demangle_component,
                );
            }
            84 => {
                match if *(*di).n as libc::c_int == '\0' as i32 {
                    '\0' as i32
                } else {
                    let fresh6 = (*di).n;
                    (*di).n = ((*di).n).offset(1);
                    *fresh6 as libc::c_int
                } {
                    110 => {
                        return d_make_comp(
                            di,
                            DEMANGLE_COMPONENT_NONTRANSACTION_CLONE,
                            d_encoding(di, 0 as libc::c_int),
                            0 as *mut demangle_component,
                        );
                    }
                    116 | _ => {
                        return d_make_comp(
                            di,
                            DEMANGLE_COMPONENT_TRANSACTION_CLONE,
                            d_encoding(di, 0 as libc::c_int),
                            0 as *mut demangle_component,
                        );
                    }
                }
            }
            114 => return d_java_resource(di),
            _ => return 0 as *mut demangle_component,
        }
    } else {
        return 0 as *mut demangle_component
    };
}
unsafe extern "C" fn d_call_offset(
    mut di: *mut d_info,
    mut c: libc::c_int,
) -> libc::c_int {
    if c == '\0' as i32 {
        c = if *(*di).n as libc::c_int == '\0' as i32 {
            '\0' as i32
        } else {
            let fresh7 = (*di).n;
            (*di).n = ((*di).n).offset(1);
            *fresh7 as libc::c_int
        };
    }
    if c == 'h' as i32 {
        d_number(di);
    } else if c == 'v' as i32 {
        d_number(di);
        if if *(*di).n as libc::c_int == '_' as i32 {
            (*di).n = ((*di).n).offset(1);
            (*di).n;
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            return 0 as libc::c_int;
        }
        d_number(di);
    } else {
        return 0 as libc::c_int
    }
    if if *(*di).n as libc::c_int == '_' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn d_ctor_dtor_name(mut di: *mut d_info) -> *mut demangle_component {
    if !((*di).last_name).is_null() {
        if (*(*di).last_name).type_0 as libc::c_uint
            == DEMANGLE_COMPONENT_NAME as libc::c_int as libc::c_uint
        {
            (*di).expansion += (*(*di).last_name).u.s_name.len;
        } else if (*(*di).last_name).type_0 as libc::c_uint
            == DEMANGLE_COMPONENT_SUB_STD as libc::c_int as libc::c_uint
        {
            (*di).expansion += (*(*di).last_name).u.s_string.len;
        }
    }
    match *(*di).n as libc::c_int {
        67 => {
            let mut kind: gnu_v3_ctor_kinds = 0 as gnu_v3_ctor_kinds;
            let mut inheriting: libc::c_int = 0 as libc::c_int;
            if *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int == 'I' as i32
            {
                inheriting = 1 as libc::c_int;
                (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
            }
            match *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int {
                49 => {
                    kind = gnu_v3_complete_object_ctor;
                }
                50 => {
                    kind = gnu_v3_base_object_ctor;
                }
                51 => {
                    kind = gnu_v3_complete_object_allocating_ctor;
                }
                52 => {
                    kind = gnu_v3_unified_ctor;
                }
                53 => {
                    kind = gnu_v3_object_ctor_group;
                }
                _ => return 0 as *mut demangle_component,
            }
            (*di).n = ((*di).n).offset(2 as libc::c_int as isize);
            if inheriting != 0 {
                cplus_demangle_type(di);
            }
            return d_make_ctor(di, kind, (*di).last_name);
        }
        68 => {
            let mut kind_0: gnu_v3_dtor_kinds = 0 as gnu_v3_dtor_kinds;
            match *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int {
                48 => {
                    kind_0 = gnu_v3_deleting_dtor;
                }
                49 => {
                    kind_0 = gnu_v3_complete_object_dtor;
                }
                50 => {
                    kind_0 = gnu_v3_base_object_dtor;
                }
                52 => {
                    kind_0 = gnu_v3_unified_dtor;
                }
                53 => {
                    kind_0 = gnu_v3_object_dtor_group;
                }
                _ => return 0 as *mut demangle_component,
            }
            (*di).n = ((*di).n).offset(2 as libc::c_int as isize);
            return d_make_dtor(di, kind_0, (*di).last_name);
        }
        _ => return 0 as *mut demangle_component,
    };
}
unsafe extern "C" fn next_is_type_qual(mut di: *mut d_info) -> libc::c_int {
    let mut peek: libc::c_char = *(*di).n;
    if peek as libc::c_int == 'r' as i32 || peek as libc::c_int == 'V' as i32
        || peek as libc::c_int == 'K' as i32
    {
        return 1 as libc::c_int;
    }
    if peek as libc::c_int == 'D' as i32 {
        peek = *((*di).n).offset(1 as libc::c_int as isize);
        if peek as libc::c_int == 'x' as i32 || peek as libc::c_int == 'o' as i32
            || peek as libc::c_int == 'O' as i32 || peek as libc::c_int == 'w' as i32
        {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut cplus_demangle_builtin_types: [demangle_builtin_type_info; 34] = [demangle_builtin_type_info {
    name: 0 as *const libc::c_char,
    len: 0,
    java_name: 0 as *const libc::c_char,
    java_len: 0,
    print: D_PRINT_DEFAULT,
}; 34];
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_type(
    mut di: *mut d_info,
) -> *mut demangle_component {
    let mut peek: libc::c_char = 0;
    let mut ret: *mut demangle_component = 0 as *mut demangle_component;
    let mut can_subst: libc::c_int = 0;
    if next_is_type_qual(di) != 0 {
        let mut pret: *mut *mut demangle_component = 0 as *mut *mut demangle_component;
        pret = d_cv_qualifiers(di, &mut ret, 0 as libc::c_int);
        if pret.is_null() {
            return 0 as *mut demangle_component;
        }
        if *(*di).n as libc::c_int == 'F' as i32 {
            *pret = d_function_type(di);
        } else {
            *pret = cplus_demangle_type(di);
        }
        if (*pret).is_null() {
            return 0 as *mut demangle_component;
        }
        if (**pret).type_0 as libc::c_uint
            == DEMANGLE_COMPONENT_RVALUE_REFERENCE_THIS as libc::c_int as libc::c_uint
            || (**pret).type_0 as libc::c_uint
                == DEMANGLE_COMPONENT_REFERENCE_THIS as libc::c_int as libc::c_uint
        {
            let mut fn_0: *mut demangle_component = (**pret).u.s_binary.left;
            (**pret).u.s_binary.left = ret;
            ret = *pret;
            *pret = fn_0;
        }
        if d_add_substitution(di, ret) == 0 {
            return 0 as *mut demangle_component;
        }
        return ret;
    }
    can_subst = 1 as libc::c_int;
    peek = *(*di).n;
    match peek as libc::c_int {
        97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 108 | 109 | 110 | 111
        | 115 | 116 | 118 | 119 | 120 | 121 | 122 => {
            ret = d_make_builtin_type(
                di,
                &*cplus_demangle_builtin_types
                    .as_ptr()
                    .offset((peek as libc::c_int - 'a' as i32) as isize),
            );
            (*di).expansion += (*(*ret).u.s_builtin.type_0).len;
            can_subst = 0 as libc::c_int;
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        }
        117 => {
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
            ret = d_make_comp(
                di,
                DEMANGLE_COMPONENT_VENDOR_TYPE,
                d_source_name(di),
                0 as *mut demangle_component,
            );
        }
        70 => {
            ret = d_function_type(di);
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 78 | 90 => {
            ret = d_class_enum_type(di);
        }
        65 => {
            ret = d_array_type(di);
        }
        77 => {
            ret = d_pointer_to_member_type(di);
        }
        84 => {
            ret = d_template_param(di);
            if *(*di).n as libc::c_int == 'I' as i32 {
                if (*di).is_conversion == 0 {
                    if d_add_substitution(di, ret) == 0 {
                        return 0 as *mut demangle_component;
                    }
                    ret = d_make_comp(
                        di,
                        DEMANGLE_COMPONENT_TEMPLATE,
                        ret,
                        d_template_args(di),
                    );
                } else {
                    let mut args: *mut demangle_component = 0 as *mut demangle_component;
                    let mut checkpoint: d_info_checkpoint = d_info_checkpoint {
                        n: 0 as *const libc::c_char,
                        next_comp: 0,
                        next_sub: 0,
                        expansion: 0,
                    };
                    d_checkpoint(di, &mut checkpoint);
                    args = d_template_args(di);
                    if *(*di).n as libc::c_int == 'I' as i32 {
                        if d_add_substitution(di, ret) == 0 {
                            return 0 as *mut demangle_component;
                        }
                        ret = d_make_comp(di, DEMANGLE_COMPONENT_TEMPLATE, ret, args);
                    } else {
                        d_backtrack(di, &mut checkpoint);
                    }
                }
            }
        }
        83 => {
            let mut peek_next: libc::c_char = 0;
            peek_next = *((*di).n).offset(1 as libc::c_int as isize);
            if peek_next as libc::c_int >= '0' as i32
                && peek_next as libc::c_int <= '9' as i32
                || peek_next as libc::c_int == '_' as i32
                || peek_next as libc::c_int >= 'A' as i32
                    && peek_next as libc::c_int <= 'Z' as i32
            {
                ret = d_substitution(di, 0 as libc::c_int);
                if *(*di).n as libc::c_int == 'I' as i32 {
                    ret = d_make_comp(
                        di,
                        DEMANGLE_COMPONENT_TEMPLATE,
                        ret,
                        d_template_args(di),
                    );
                } else {
                    can_subst = 0 as libc::c_int;
                }
            } else {
                ret = d_class_enum_type(di);
                if !ret.is_null()
                    && (*ret).type_0 as libc::c_uint
                        == DEMANGLE_COMPONENT_SUB_STD as libc::c_int as libc::c_uint
                {
                    can_subst = 0 as libc::c_int;
                }
            }
        }
        79 => {
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
            ret = d_make_comp(
                di,
                DEMANGLE_COMPONENT_RVALUE_REFERENCE,
                cplus_demangle_type(di),
                0 as *mut demangle_component,
            );
        }
        80 => {
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
            ret = d_make_comp(
                di,
                DEMANGLE_COMPONENT_POINTER,
                cplus_demangle_type(di),
                0 as *mut demangle_component,
            );
        }
        82 => {
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
            ret = d_make_comp(
                di,
                DEMANGLE_COMPONENT_REFERENCE,
                cplus_demangle_type(di),
                0 as *mut demangle_component,
            );
        }
        67 => {
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
            ret = d_make_comp(
                di,
                DEMANGLE_COMPONENT_COMPLEX,
                cplus_demangle_type(di),
                0 as *mut demangle_component,
            );
        }
        71 => {
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
            ret = d_make_comp(
                di,
                DEMANGLE_COMPONENT_IMAGINARY,
                cplus_demangle_type(di),
                0 as *mut demangle_component,
            );
        }
        85 => {
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
            ret = d_source_name(di);
            if *(*di).n as libc::c_int == 'I' as i32 {
                ret = d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_TEMPLATE,
                    ret,
                    d_template_args(di),
                );
            }
            ret = d_make_comp(
                di,
                DEMANGLE_COMPONENT_VENDOR_TYPE_QUAL,
                cplus_demangle_type(di),
                ret,
            );
        }
        68 => {
            can_subst = 0 as libc::c_int;
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
            peek = (if *(*di).n as libc::c_int == '\0' as i32 {
                '\0' as i32
            } else {
                let fresh8 = (*di).n;
                (*di).n = ((*di).n).offset(1);
                *fresh8 as libc::c_int
            }) as libc::c_char;
            match peek as libc::c_int {
                84 | 116 => {
                    ret = d_make_comp(
                        di,
                        DEMANGLE_COMPONENT_DECLTYPE,
                        d_expression(di),
                        0 as *mut demangle_component,
                    );
                    if !ret.is_null()
                        && (if *(*di).n as libc::c_int == '\0' as i32 {
                            '\0' as i32
                        } else {
                            let fresh9 = (*di).n;
                            (*di).n = ((*di).n).offset(1);
                            *fresh9 as libc::c_int
                        }) != 'E' as i32
                    {
                        ret = 0 as *mut demangle_component;
                    }
                    can_subst = 1 as libc::c_int;
                }
                112 => {
                    ret = d_make_comp(
                        di,
                        DEMANGLE_COMPONENT_PACK_EXPANSION,
                        cplus_demangle_type(di),
                        0 as *mut demangle_component,
                    );
                    can_subst = 1 as libc::c_int;
                }
                97 => {
                    ret = d_make_name(
                        di,
                        b"auto\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int,
                    );
                }
                99 => {
                    ret = d_make_name(
                        di,
                        b"decltype(auto)\0" as *const u8 as *const libc::c_char,
                        14 as libc::c_int,
                    );
                }
                102 => {
                    ret = d_make_builtin_type(
                        di,
                        &*cplus_demangle_builtin_types
                            .as_ptr()
                            .offset(26 as libc::c_int as isize),
                    );
                    (*di).expansion += (*(*ret).u.s_builtin.type_0).len;
                }
                100 => {
                    ret = d_make_builtin_type(
                        di,
                        &*cplus_demangle_builtin_types
                            .as_ptr()
                            .offset(27 as libc::c_int as isize),
                    );
                    (*di).expansion += (*(*ret).u.s_builtin.type_0).len;
                }
                101 => {
                    ret = d_make_builtin_type(
                        di,
                        &*cplus_demangle_builtin_types
                            .as_ptr()
                            .offset(28 as libc::c_int as isize),
                    );
                    (*di).expansion += (*(*ret).u.s_builtin.type_0).len;
                }
                104 => {
                    ret = d_make_builtin_type(
                        di,
                        &*cplus_demangle_builtin_types
                            .as_ptr()
                            .offset(29 as libc::c_int as isize),
                    );
                    (*di).expansion += (*(*ret).u.s_builtin.type_0).len;
                }
                117 => {
                    ret = d_make_builtin_type(
                        di,
                        &*cplus_demangle_builtin_types
                            .as_ptr()
                            .offset(30 as libc::c_int as isize),
                    );
                    (*di).expansion += (*(*ret).u.s_builtin.type_0).len;
                }
                115 => {
                    ret = d_make_builtin_type(
                        di,
                        &*cplus_demangle_builtin_types
                            .as_ptr()
                            .offset(31 as libc::c_int as isize),
                    );
                    (*di).expansion += (*(*ret).u.s_builtin.type_0).len;
                }
                105 => {
                    ret = d_make_builtin_type(
                        di,
                        &*cplus_demangle_builtin_types
                            .as_ptr()
                            .offset(32 as libc::c_int as isize),
                    );
                    (*di).expansion += (*(*ret).u.s_builtin.type_0).len;
                }
                70 => {
                    ret = d_make_empty(di);
                    (*ret).type_0 = DEMANGLE_COMPONENT_FIXED_TYPE;
                    (*ret)
                        .u
                        .s_fixed
                        .accum = (*(*di).n as libc::c_int >= '0' as i32
                        && *(*di).n as libc::c_int <= '9' as i32) as libc::c_int
                        as libc::c_short;
                    if (*ret).u.s_fixed.accum != 0 {
                        d_number(di);
                    }
                    (*ret).u.s_fixed.length = cplus_demangle_type(di);
                    if ((*ret).u.s_fixed.length).is_null() {
                        return 0 as *mut demangle_component;
                    }
                    d_number(di);
                    peek = (if *(*di).n as libc::c_int == '\0' as i32 {
                        '\0' as i32
                    } else {
                        let fresh10 = (*di).n;
                        (*di).n = ((*di).n).offset(1);
                        *fresh10 as libc::c_int
                    }) as libc::c_char;
                    (*ret)
                        .u
                        .s_fixed
                        .sat = (peek as libc::c_int == 's' as i32) as libc::c_int
                        as libc::c_short;
                }
                118 => {
                    ret = d_vector_type(di);
                    can_subst = 1 as libc::c_int;
                }
                110 => {
                    ret = d_make_builtin_type(
                        di,
                        &*cplus_demangle_builtin_types
                            .as_ptr()
                            .offset(33 as libc::c_int as isize),
                    );
                    (*di).expansion += (*(*ret).u.s_builtin.type_0).len;
                }
                _ => return 0 as *mut demangle_component,
            }
        }
        _ => return 0 as *mut demangle_component,
    }
    if can_subst != 0 {
        if d_add_substitution(di, ret) == 0 {
            return 0 as *mut demangle_component;
        }
    }
    return ret;
}
unsafe extern "C" fn d_cv_qualifiers(
    mut di: *mut d_info,
    mut pret: *mut *mut demangle_component,
    mut member_fn: libc::c_int,
) -> *mut *mut demangle_component {
    let mut pstart: *mut *mut demangle_component = 0 as *mut *mut demangle_component;
    let mut peek: libc::c_char = 0;
    pstart = pret;
    peek = *(*di).n;
    while next_is_type_qual(di) != 0 {
        let mut t: demangle_component_type = DEMANGLE_COMPONENT_NAME;
        let mut right: *mut demangle_component = 0 as *mut demangle_component;
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        if peek as libc::c_int == 'r' as i32 {
            t = (if member_fn != 0 {
                DEMANGLE_COMPONENT_RESTRICT_THIS as libc::c_int
            } else {
                DEMANGLE_COMPONENT_RESTRICT as libc::c_int
            }) as demangle_component_type;
            (*di)
                .expansion = ((*di).expansion as libc::c_ulong)
                .wrapping_add(
                    ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
                ) as libc::c_int as libc::c_int;
        } else if peek as libc::c_int == 'V' as i32 {
            t = (if member_fn != 0 {
                DEMANGLE_COMPONENT_VOLATILE_THIS as libc::c_int
            } else {
                DEMANGLE_COMPONENT_VOLATILE as libc::c_int
            }) as demangle_component_type;
            (*di)
                .expansion = ((*di).expansion as libc::c_ulong)
                .wrapping_add(
                    ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
                ) as libc::c_int as libc::c_int;
        } else if peek as libc::c_int == 'K' as i32 {
            t = (if member_fn != 0 {
                DEMANGLE_COMPONENT_CONST_THIS as libc::c_int
            } else {
                DEMANGLE_COMPONENT_CONST as libc::c_int
            }) as demangle_component_type;
            (*di)
                .expansion = ((*di).expansion as libc::c_ulong)
                .wrapping_add(
                    ::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
                ) as libc::c_int as libc::c_int;
        } else {
            peek = (if *(*di).n as libc::c_int == '\0' as i32 {
                '\0' as i32
            } else {
                let fresh11 = (*di).n;
                (*di).n = ((*di).n).offset(1);
                *fresh11 as libc::c_int
            }) as libc::c_char;
            if peek as libc::c_int == 'x' as i32 {
                t = DEMANGLE_COMPONENT_TRANSACTION_SAFE;
                (*di)
                    .expansion = ((*di).expansion as libc::c_ulong)
                    .wrapping_add(
                        ::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong,
                    ) as libc::c_int as libc::c_int;
            } else if peek as libc::c_int == 'o' as i32
                || peek as libc::c_int == 'O' as i32
            {
                t = DEMANGLE_COMPONENT_NOEXCEPT;
                (*di)
                    .expansion = ((*di).expansion as libc::c_ulong)
                    .wrapping_add(
                        ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
                    ) as libc::c_int as libc::c_int;
                if peek as libc::c_int == 'O' as i32 {
                    right = d_expression(di);
                    if right.is_null() {
                        return 0 as *mut *mut demangle_component;
                    }
                    if if *(*di).n as libc::c_int == 'E' as i32 {
                        (*di).n = ((*di).n).offset(1);
                        (*di).n;
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } == 0
                    {
                        return 0 as *mut *mut demangle_component;
                    }
                }
            } else if peek as libc::c_int == 'w' as i32 {
                t = DEMANGLE_COMPONENT_THROW_SPEC;
                (*di)
                    .expansion = ((*di).expansion as libc::c_ulong)
                    .wrapping_add(
                        ::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
                    ) as libc::c_int as libc::c_int;
                right = d_parmlist(di);
                if right.is_null() {
                    return 0 as *mut *mut demangle_component;
                }
                if if *(*di).n as libc::c_int == 'E' as i32 {
                    (*di).n = ((*di).n).offset(1);
                    (*di).n;
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                } == 0
                {
                    return 0 as *mut *mut demangle_component;
                }
            } else {
                return 0 as *mut *mut demangle_component
            }
        }
        *pret = d_make_comp(di, t, 0 as *mut demangle_component, right);
        if (*pret).is_null() {
            return 0 as *mut *mut demangle_component;
        }
        pret = &mut (**pret).u.s_binary.left;
        peek = *(*di).n;
    }
    if member_fn == 0 && peek as libc::c_int == 'F' as i32 {
        while pstart != pret {
            match (**pstart).type_0 as libc::c_uint {
                25 => {
                    (**pstart).type_0 = DEMANGLE_COMPONENT_RESTRICT_THIS;
                }
                26 => {
                    (**pstart).type_0 = DEMANGLE_COMPONENT_VOLATILE_THIS;
                }
                27 => {
                    (**pstart).type_0 = DEMANGLE_COMPONENT_CONST_THIS;
                }
                _ => {}
            }
            pstart = &mut (**pstart).u.s_binary.left;
        }
    }
    return pret;
}
unsafe extern "C" fn d_ref_qualifier(
    mut di: *mut d_info,
    mut sub: *mut demangle_component,
) -> *mut demangle_component {
    let mut ret: *mut demangle_component = sub;
    let mut peek: libc::c_char = 0;
    peek = *(*di).n;
    if peek as libc::c_int == 'R' as i32 || peek as libc::c_int == 'O' as i32 {
        let mut t: demangle_component_type = DEMANGLE_COMPONENT_NAME;
        if peek as libc::c_int == 'R' as i32 {
            t = DEMANGLE_COMPONENT_REFERENCE_THIS;
            (*di)
                .expansion = ((*di).expansion as libc::c_ulong)
                .wrapping_add(
                    ::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
                ) as libc::c_int as libc::c_int;
        } else {
            t = DEMANGLE_COMPONENT_RVALUE_REFERENCE_THIS;
            (*di)
                .expansion = ((*di).expansion as libc::c_ulong)
                .wrapping_add(
                    ::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong,
                ) as libc::c_int as libc::c_int;
        }
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        ret = d_make_comp(di, t, ret, 0 as *mut demangle_component);
    }
    return ret;
}
unsafe extern "C" fn d_function_type(mut di: *mut d_info) -> *mut demangle_component {
    let mut ret: *mut demangle_component = 0 as *mut demangle_component;
    if (*di).options & (1 as libc::c_int) << 18 as libc::c_int == 0 as libc::c_int {
        if (*di).recursion_level > 2048 as libc::c_int as libc::c_uint {
            return 0 as *mut demangle_component;
        }
        (*di).recursion_level = ((*di).recursion_level).wrapping_add(1);
        (*di).recursion_level;
    }
    if if *(*di).n as libc::c_int == 'F' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        if *(*di).n as libc::c_int == 'Y' as i32 {
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        }
        ret = d_bare_function_type(di, 1 as libc::c_int);
        ret = d_ref_qualifier(di, ret);
        if if *(*di).n as libc::c_int == 'E' as i32 {
            (*di).n = ((*di).n).offset(1);
            (*di).n;
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            ret = 0 as *mut demangle_component;
        }
    }
    if (*di).options & (1 as libc::c_int) << 18 as libc::c_int == 0 as libc::c_int {
        (*di).recursion_level = ((*di).recursion_level).wrapping_sub(1);
        (*di).recursion_level;
    }
    return ret;
}
unsafe extern "C" fn d_parmlist(mut di: *mut d_info) -> *mut demangle_component {
    let mut tl: *mut demangle_component = 0 as *mut demangle_component;
    let mut ptl: *mut *mut demangle_component = 0 as *mut *mut demangle_component;
    tl = 0 as *mut demangle_component;
    ptl = &mut tl;
    loop {
        let mut type_0: *mut demangle_component = 0 as *mut demangle_component;
        let mut peek: libc::c_char = *(*di).n;
        if peek as libc::c_int == '\0' as i32 || peek as libc::c_int == 'E' as i32
            || peek as libc::c_int == '.' as i32
        {
            break;
        }
        if (peek as libc::c_int == 'R' as i32 || peek as libc::c_int == 'O' as i32)
            && *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int == 'E' as i32
        {
            break;
        }
        type_0 = cplus_demangle_type(di);
        if type_0.is_null() {
            return 0 as *mut demangle_component;
        }
        *ptl = d_make_comp(
            di,
            DEMANGLE_COMPONENT_ARGLIST,
            type_0,
            0 as *mut demangle_component,
        );
        if (*ptl).is_null() {
            return 0 as *mut demangle_component;
        }
        ptl = &mut (**ptl).u.s_binary.right;
    }
    if tl.is_null() {
        return 0 as *mut demangle_component;
    }
    if ((*tl).u.s_binary.right).is_null()
        && (*(*tl).u.s_binary.left).type_0 as libc::c_uint
            == DEMANGLE_COMPONENT_BUILTIN_TYPE as libc::c_int as libc::c_uint
        && (*(*(*tl).u.s_binary.left).u.s_builtin.type_0).print as libc::c_uint
            == D_PRINT_VOID as libc::c_int as libc::c_uint
    {
        (*di).expansion -= (*(*(*tl).u.s_binary.left).u.s_builtin.type_0).len;
        (*tl).u.s_binary.left = 0 as *mut demangle_component;
    }
    return tl;
}
unsafe extern "C" fn d_bare_function_type(
    mut di: *mut d_info,
    mut has_return_type_0: libc::c_int,
) -> *mut demangle_component {
    let mut return_type: *mut demangle_component = 0 as *mut demangle_component;
    let mut tl: *mut demangle_component = 0 as *mut demangle_component;
    let mut peek: libc::c_char = 0;
    peek = *(*di).n;
    if peek as libc::c_int == 'J' as i32 {
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        has_return_type_0 = 1 as libc::c_int;
    }
    if has_return_type_0 != 0 {
        return_type = cplus_demangle_type(di);
        if return_type.is_null() {
            return 0 as *mut demangle_component;
        }
    } else {
        return_type = 0 as *mut demangle_component;
    }
    tl = d_parmlist(di);
    if tl.is_null() {
        return 0 as *mut demangle_component;
    }
    return d_make_comp(di, DEMANGLE_COMPONENT_FUNCTION_TYPE, return_type, tl);
}
unsafe extern "C" fn d_class_enum_type(mut di: *mut d_info) -> *mut demangle_component {
    return d_name(di);
}
unsafe extern "C" fn d_array_type(mut di: *mut d_info) -> *mut demangle_component {
    let mut peek: libc::c_char = 0;
    let mut dim: *mut demangle_component = 0 as *mut demangle_component;
    if if *(*di).n as libc::c_int == 'A' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    peek = *(*di).n;
    if peek as libc::c_int == '_' as i32 {
        dim = 0 as *mut demangle_component;
    } else if peek as libc::c_int >= '0' as i32 && peek as libc::c_int <= '9' as i32 {
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        s = (*di).n;
        loop {
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
            peek = *(*di).n;
            if !(peek as libc::c_int >= '0' as i32 && peek as libc::c_int <= '9' as i32)
            {
                break;
            }
        }
        dim = d_make_name(
            di,
            s,
            ((*di).n).offset_from(s) as libc::c_long as libc::c_int,
        );
        if dim.is_null() {
            return 0 as *mut demangle_component;
        }
    } else {
        dim = d_expression(di);
        if dim.is_null() {
            return 0 as *mut demangle_component;
        }
    }
    if if *(*di).n as libc::c_int == '_' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    return d_make_comp(di, DEMANGLE_COMPONENT_ARRAY_TYPE, dim, cplus_demangle_type(di));
}
unsafe extern "C" fn d_vector_type(mut di: *mut d_info) -> *mut demangle_component {
    let mut peek: libc::c_char = 0;
    let mut dim: *mut demangle_component = 0 as *mut demangle_component;
    peek = *(*di).n;
    if peek as libc::c_int == '_' as i32 {
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        dim = d_expression(di);
    } else {
        dim = d_number_component(di);
    }
    if dim.is_null() {
        return 0 as *mut demangle_component;
    }
    if if *(*di).n as libc::c_int == '_' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    return d_make_comp(di, DEMANGLE_COMPONENT_VECTOR_TYPE, dim, cplus_demangle_type(di));
}
unsafe extern "C" fn d_pointer_to_member_type(
    mut di: *mut d_info,
) -> *mut demangle_component {
    let mut cl: *mut demangle_component = 0 as *mut demangle_component;
    let mut mem: *mut demangle_component = 0 as *mut demangle_component;
    if if *(*di).n as libc::c_int == 'M' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    cl = cplus_demangle_type(di);
    if cl.is_null() {
        return 0 as *mut demangle_component;
    }
    mem = cplus_demangle_type(di);
    if mem.is_null() {
        return 0 as *mut demangle_component;
    }
    return d_make_comp(di, DEMANGLE_COMPONENT_PTRMEM_TYPE, cl, mem);
}
unsafe extern "C" fn d_compact_number(mut di: *mut d_info) -> libc::c_int {
    let mut num: libc::c_int = 0;
    if *(*di).n as libc::c_int == '_' as i32 {
        num = 0 as libc::c_int;
    } else if *(*di).n as libc::c_int == 'n' as i32 {
        return -(1 as libc::c_int)
    } else {
        num = d_number(di) + 1 as libc::c_int;
    }
    if num < 0 as libc::c_int
        || (if *(*di).n as libc::c_int == '_' as i32 {
            (*di).n = ((*di).n).offset(1);
            (*di).n;
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) == 0
    {
        return -(1 as libc::c_int);
    }
    return num;
}
unsafe extern "C" fn d_template_param(mut di: *mut d_info) -> *mut demangle_component {
    let mut param: libc::c_int = 0;
    if if *(*di).n as libc::c_int == 'T' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    param = d_compact_number(di);
    if param < 0 as libc::c_int {
        return 0 as *mut demangle_component;
    }
    return d_make_template_param(di, param);
}
unsafe extern "C" fn d_template_args(mut di: *mut d_info) -> *mut demangle_component {
    if *(*di).n as libc::c_int != 'I' as i32 && *(*di).n as libc::c_int != 'J' as i32 {
        return 0 as *mut demangle_component;
    }
    (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
    return d_template_args_1(di);
}
unsafe extern "C" fn d_template_args_1(mut di: *mut d_info) -> *mut demangle_component {
    let mut hold_last_name: *mut demangle_component = 0 as *mut demangle_component;
    let mut al: *mut demangle_component = 0 as *mut demangle_component;
    let mut pal: *mut *mut demangle_component = 0 as *mut *mut demangle_component;
    hold_last_name = (*di).last_name;
    if *(*di).n as libc::c_int == 'E' as i32 {
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        return d_make_comp(
            di,
            DEMANGLE_COMPONENT_TEMPLATE_ARGLIST,
            0 as *mut demangle_component,
            0 as *mut demangle_component,
        );
    }
    al = 0 as *mut demangle_component;
    pal = &mut al;
    loop {
        let mut a: *mut demangle_component = 0 as *mut demangle_component;
        a = d_template_arg(di);
        if a.is_null() {
            return 0 as *mut demangle_component;
        }
        *pal = d_make_comp(
            di,
            DEMANGLE_COMPONENT_TEMPLATE_ARGLIST,
            a,
            0 as *mut demangle_component,
        );
        if (*pal).is_null() {
            return 0 as *mut demangle_component;
        }
        pal = &mut (**pal).u.s_binary.right;
        if !(*(*di).n as libc::c_int == 'E' as i32) {
            continue;
        }
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        break;
    }
    (*di).last_name = hold_last_name;
    return al;
}
unsafe extern "C" fn d_template_arg(mut di: *mut d_info) -> *mut demangle_component {
    let mut ret: *mut demangle_component = 0 as *mut demangle_component;
    match *(*di).n as libc::c_int {
        88 => {
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
            ret = d_expression(di);
            if if *(*di).n as libc::c_int == 'E' as i32 {
                (*di).n = ((*di).n).offset(1);
                (*di).n;
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                return 0 as *mut demangle_component;
            }
            return ret;
        }
        76 => return d_expr_primary(di),
        73 | 74 => return d_template_args(di),
        _ => return cplus_demangle_type(di),
    };
}
unsafe extern "C" fn d_exprlist(
    mut di: *mut d_info,
    mut terminator: libc::c_char,
) -> *mut demangle_component {
    let mut list: *mut demangle_component = 0 as *mut demangle_component;
    let mut p: *mut *mut demangle_component = &mut list;
    if *(*di).n as libc::c_int == terminator as libc::c_int {
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        return d_make_comp(
            di,
            DEMANGLE_COMPONENT_ARGLIST,
            0 as *mut demangle_component,
            0 as *mut demangle_component,
        );
    }
    loop {
        let mut arg: *mut demangle_component = d_expression(di);
        if arg.is_null() {
            return 0 as *mut demangle_component;
        }
        *p = d_make_comp(
            di,
            DEMANGLE_COMPONENT_ARGLIST,
            arg,
            0 as *mut demangle_component,
        );
        if (*p).is_null() {
            return 0 as *mut demangle_component;
        }
        p = &mut (**p).u.s_binary.right;
        if !(*(*di).n as libc::c_int == terminator as libc::c_int) {
            continue;
        }
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        break;
    }
    return list;
}
unsafe extern "C" fn op_is_new_cast(mut op: *mut demangle_component) -> libc::c_int {
    let mut code: *const libc::c_char = (*(*op).u.s_operator.op).code;
    return (*code.offset(1 as libc::c_int as isize) as libc::c_int == 'c' as i32
        && (*code.offset(0 as libc::c_int as isize) as libc::c_int == 's' as i32
            || *code.offset(0 as libc::c_int as isize) as libc::c_int == 'd' as i32
            || *code.offset(0 as libc::c_int as isize) as libc::c_int == 'c' as i32
            || *code.offset(0 as libc::c_int as isize) as libc::c_int == 'r' as i32))
        as libc::c_int;
}
unsafe extern "C" fn d_unresolved_name(mut di: *mut d_info) -> *mut demangle_component {
    let mut type_0: *mut demangle_component = 0 as *mut demangle_component;
    let mut name: *mut demangle_component = 0 as *mut demangle_component;
    let mut peek: libc::c_char = 0;
    (*di).n = ((*di).n).offset(2 as libc::c_int as isize);
    peek = *(*di).n;
    if (*di).unresolved_name_state != 0
        && (peek as libc::c_int >= '0' as i32 && peek as libc::c_int <= '9' as i32
            || peek as libc::c_int >= 'a' as i32 && peek as libc::c_int <= 'z' as i32
            || peek as libc::c_int == 'C' as i32 || peek as libc::c_int == 'U' as i32
            || peek as libc::c_int == 'L' as i32)
    {
        (*di).unresolved_name_state = -(1 as libc::c_int);
        type_0 = d_prefix(di, 0 as libc::c_int);
        if *(*di).n as libc::c_int == 'E' as i32 {
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        }
    } else {
        type_0 = cplus_demangle_type(di);
    }
    name = d_unqualified_name(di);
    if *(*di).n as libc::c_int == 'I' as i32 {
        name = d_make_comp(di, DEMANGLE_COMPONENT_TEMPLATE, name, d_template_args(di));
    }
    return d_make_comp(di, DEMANGLE_COMPONENT_QUAL_NAME, type_0, name);
}
unsafe extern "C" fn d_expression_1(mut di: *mut d_info) -> *mut demangle_component {
    let mut peek: libc::c_char = 0;
    peek = *(*di).n;
    if peek as libc::c_int == 'L' as i32 {
        return d_expr_primary(di)
    } else if peek as libc::c_int == 'T' as i32 {
        return d_template_param(di)
    } else if peek as libc::c_int == 's' as i32
        && *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int == 'r' as i32
    {
        return d_unresolved_name(di)
    } else if peek as libc::c_int == 's' as i32
        && *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int == 'p' as i32
    {
        (*di).n = ((*di).n).offset(2 as libc::c_int as isize);
        return d_make_comp(
            di,
            DEMANGLE_COMPONENT_PACK_EXPANSION,
            d_expression_1(di),
            0 as *mut demangle_component,
        );
    } else if peek as libc::c_int == 'f' as i32
        && *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int == 'p' as i32
    {
        let mut index: libc::c_int = 0;
        (*di).n = ((*di).n).offset(2 as libc::c_int as isize);
        if *(*di).n as libc::c_int == 'T' as i32 {
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
            index = 0 as libc::c_int;
        } else {
            index = d_compact_number(di);
            if index == 2147483647 as libc::c_int || index == -(1 as libc::c_int) {
                return 0 as *mut demangle_component;
            }
            index += 1;
            index;
        }
        return d_make_function_param(di, index);
    } else if peek as libc::c_int >= '0' as i32 && peek as libc::c_int <= '9' as i32
        || peek as libc::c_int == 'o' as i32
            && *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int == 'n' as i32
    {
        let mut name: *mut demangle_component = 0 as *mut demangle_component;
        if peek as libc::c_int == 'o' as i32 {
            (*di).n = ((*di).n).offset(2 as libc::c_int as isize);
        }
        name = d_unqualified_name(di);
        if name.is_null() {
            return 0 as *mut demangle_component;
        }
        if *(*di).n as libc::c_int == 'I' as i32 {
            return d_make_comp(
                di,
                DEMANGLE_COMPONENT_TEMPLATE,
                name,
                d_template_args(di),
            )
        } else {
            return name
        }
    } else if (peek as libc::c_int == 'i' as i32 || peek as libc::c_int == 't' as i32)
        && *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int == 'l' as i32
    {
        let mut type_0: *mut demangle_component = 0 as *mut demangle_component;
        (*di).n = ((*di).n).offset(2 as libc::c_int as isize);
        if peek as libc::c_int == 't' as i32 {
            type_0 = cplus_demangle_type(di);
        }
        if *(*di).n == 0 || *((*di).n).offset(1 as libc::c_int as isize) == 0 {
            return 0 as *mut demangle_component;
        }
        return d_make_comp(
            di,
            DEMANGLE_COMPONENT_INITIALIZER_LIST,
            type_0,
            d_exprlist(di, 'E' as i32 as libc::c_char),
        );
    } else if peek as libc::c_int == 'u' as i32 {
        let mut name_0: *mut demangle_component = 0 as *mut demangle_component;
        let mut args: *mut demangle_component = 0 as *mut demangle_component;
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        name_0 = d_source_name(di);
        args = d_template_args_1(di);
        return d_make_comp(di, DEMANGLE_COMPONENT_VENDOR_EXPR, name_0, args);
    } else {
        let mut op: *mut demangle_component = 0 as *mut demangle_component;
        let mut code: *const libc::c_char = 0 as *const libc::c_char;
        let mut args_0: libc::c_int = 0;
        op = d_operator_name(di);
        if op.is_null() {
            return 0 as *mut demangle_component;
        }
        if (*op).type_0 as libc::c_uint
            == DEMANGLE_COMPONENT_OPERATOR as libc::c_int as libc::c_uint
        {
            code = (*(*op).u.s_operator.op).code;
            (*di).expansion += (*(*op).u.s_operator.op).len - 2 as libc::c_int;
            if strcmp(code, b"st\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_UNARY,
                    op,
                    cplus_demangle_type(di),
                );
            }
        }
        match (*op).type_0 as libc::c_uint {
            50 => {
                args_0 = (*(*op).u.s_operator.op).args;
            }
            51 => {
                args_0 = (*op).u.s_extended_operator.args;
            }
            52 => {
                args_0 = 1 as libc::c_int;
            }
            _ => return 0 as *mut demangle_component,
        }
        match args_0 {
            0 => {
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_NULLARY,
                    op,
                    0 as *mut demangle_component,
                );
            }
            1 => {
                let mut operand: *mut demangle_component = 0 as *mut demangle_component;
                let mut suffix: libc::c_int = 0 as libc::c_int;
                if !code.is_null()
                    && (*code.offset(0 as libc::c_int as isize) as libc::c_int
                        == 'p' as i32
                        || *code.offset(0 as libc::c_int as isize) as libc::c_int
                            == 'm' as i32)
                    && *code.offset(1 as libc::c_int as isize) as libc::c_int
                        == *code.offset(0 as libc::c_int as isize) as libc::c_int
                {
                    suffix = (if *(*di).n as libc::c_int == '_' as i32 {
                        (*di).n = ((*di).n).offset(1);
                        (*di).n;
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } == 0) as libc::c_int;
                }
                if (*op).type_0 as libc::c_uint
                    == DEMANGLE_COMPONENT_CAST as libc::c_int as libc::c_uint
                    && (if *(*di).n as libc::c_int == '_' as i32 {
                        (*di).n = ((*di).n).offset(1);
                        (*di).n;
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) != 0
                {
                    operand = d_exprlist(di, 'E' as i32 as libc::c_char);
                } else if !code.is_null()
                    && strcmp(code, b"sP\0" as *const u8 as *const libc::c_char) == 0
                {
                    operand = d_template_args_1(di);
                } else {
                    operand = d_expression_1(di);
                }
                if suffix != 0 {
                    operand = d_make_comp(
                        di,
                        DEMANGLE_COMPONENT_BINARY_ARGS,
                        operand,
                        operand,
                    );
                }
                return d_make_comp(di, DEMANGLE_COMPONENT_UNARY, op, operand);
            }
            2 => {
                let mut left: *mut demangle_component = 0 as *mut demangle_component;
                let mut right: *mut demangle_component = 0 as *mut demangle_component;
                if code.is_null() {
                    return 0 as *mut demangle_component;
                }
                if op_is_new_cast(op) != 0 {
                    left = cplus_demangle_type(di);
                } else if *code.offset(0 as libc::c_int as isize) as libc::c_int
                    == 'f' as i32
                {
                    left = d_operator_name(di);
                } else if strcmp(code, b"di\0" as *const u8 as *const libc::c_char) == 0
                {
                    left = d_unqualified_name(di);
                } else {
                    left = d_expression_1(di);
                }
                if strcmp(code, b"cl\0" as *const u8 as *const libc::c_char) == 0 {
                    right = d_exprlist(di, 'E' as i32 as libc::c_char);
                } else if strcmp(code, b"dt\0" as *const u8 as *const libc::c_char) == 0
                    || strcmp(code, b"pt\0" as *const u8 as *const libc::c_char) == 0
                {
                    peek = *(*di).n;
                    if peek as libc::c_int == 'g' as i32
                        && *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int
                            == 's' as i32
                        || peek as libc::c_int == 's' as i32
                            && *((*di).n).offset(1 as libc::c_int as isize)
                                as libc::c_int == 'r' as i32
                    {
                        right = d_expression_1(di);
                    } else {
                        right = d_unqualified_name(di);
                        if *(*di).n as libc::c_int == 'I' as i32 {
                            right = d_make_comp(
                                di,
                                DEMANGLE_COMPONENT_TEMPLATE,
                                right,
                                d_template_args(di),
                            );
                        }
                    }
                } else {
                    right = d_expression_1(di);
                }
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_BINARY,
                    op,
                    d_make_comp(di, DEMANGLE_COMPONENT_BINARY_ARGS, left, right),
                );
            }
            3 => {
                let mut first: *mut demangle_component = 0 as *mut demangle_component;
                let mut second: *mut demangle_component = 0 as *mut demangle_component;
                let mut third: *mut demangle_component = 0 as *mut demangle_component;
                if code.is_null() {
                    return 0 as *mut demangle_component
                } else if strcmp(code, b"qu\0" as *const u8 as *const libc::c_char) == 0
                    || strcmp(code, b"dX\0" as *const u8 as *const libc::c_char) == 0
                {
                    first = d_expression_1(di);
                    second = d_expression_1(di);
                    third = d_expression_1(di);
                    if third.is_null() {
                        return 0 as *mut demangle_component;
                    }
                } else if *code.offset(0 as libc::c_int as isize) as libc::c_int
                    == 'f' as i32
                {
                    first = d_operator_name(di);
                    second = d_expression_1(di);
                    third = d_expression_1(di);
                    if third.is_null() {
                        return 0 as *mut demangle_component;
                    }
                } else if *code.offset(0 as libc::c_int as isize) as libc::c_int
                    == 'n' as i32
                {
                    if *code.offset(1 as libc::c_int as isize) as libc::c_int
                        != 'w' as i32
                        && *code.offset(1 as libc::c_int as isize) as libc::c_int
                            != 'a' as i32
                    {
                        return 0 as *mut demangle_component;
                    }
                    first = d_exprlist(di, '_' as i32 as libc::c_char);
                    second = cplus_demangle_type(di);
                    if *(*di).n as libc::c_int == 'E' as i32 {
                        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
                        third = 0 as *mut demangle_component;
                    } else if *(*di).n as libc::c_int == 'p' as i32
                        && *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int
                            == 'i' as i32
                    {
                        (*di).n = ((*di).n).offset(2 as libc::c_int as isize);
                        third = d_exprlist(di, 'E' as i32 as libc::c_char);
                    } else if *(*di).n as libc::c_int == 'i' as i32
                        && *((*di).n).offset(1 as libc::c_int as isize) as libc::c_int
                            == 'l' as i32
                    {
                        third = d_expression_1(di);
                    } else {
                        return 0 as *mut demangle_component
                    }
                } else {
                    return 0 as *mut demangle_component
                }
                return d_make_comp(
                    di,
                    DEMANGLE_COMPONENT_TRINARY,
                    op,
                    d_make_comp(
                        di,
                        DEMANGLE_COMPONENT_TRINARY_ARG1,
                        first,
                        d_make_comp(di, DEMANGLE_COMPONENT_TRINARY_ARG2, second, third),
                    ),
                );
            }
            _ => return 0 as *mut demangle_component,
        }
    };
}
unsafe extern "C" fn d_expression(mut di: *mut d_info) -> *mut demangle_component {
    let mut ret: *mut demangle_component = 0 as *mut demangle_component;
    let mut was_expression: libc::c_int = (*di).is_expression;
    (*di).is_expression = 1 as libc::c_int;
    ret = d_expression_1(di);
    (*di).is_expression = was_expression;
    return ret;
}
unsafe extern "C" fn d_expr_primary(mut di: *mut d_info) -> *mut demangle_component {
    let mut ret: *mut demangle_component = 0 as *mut demangle_component;
    if if *(*di).n as libc::c_int == 'L' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    if *(*di).n as libc::c_int == '_' as i32 || *(*di).n as libc::c_int == 'Z' as i32 {
        ret = cplus_demangle_mangled_name(di, 0 as libc::c_int);
    } else {
        let mut type_0: *mut demangle_component = 0 as *mut demangle_component;
        let mut t: demangle_component_type = DEMANGLE_COMPONENT_NAME;
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        type_0 = cplus_demangle_type(di);
        if type_0.is_null() {
            return 0 as *mut demangle_component;
        }
        if (*type_0).type_0 as libc::c_uint
            == DEMANGLE_COMPONENT_BUILTIN_TYPE as libc::c_int as libc::c_uint
            && (*(*type_0).u.s_builtin.type_0).print as libc::c_uint
                != D_PRINT_DEFAULT as libc::c_int as libc::c_uint
        {
            (*di).expansion -= (*(*type_0).u.s_builtin.type_0).len;
        }
        if (*type_0).type_0 as libc::c_uint
            == DEMANGLE_COMPONENT_BUILTIN_TYPE as libc::c_int as libc::c_uint
            && strcmp(
                (*(*type_0).u.s_builtin.type_0).name,
                cplus_demangle_builtin_types[33 as libc::c_int as usize].name,
            ) == 0 as libc::c_int
        {
            if *(*di).n as libc::c_int == 'E' as i32 {
                (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
                return type_0;
            }
        }
        t = DEMANGLE_COMPONENT_LITERAL;
        if *(*di).n as libc::c_int == 'n' as i32 {
            t = DEMANGLE_COMPONENT_LITERAL_NEG;
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        }
        s = (*di).n;
        while *(*di).n as libc::c_int != 'E' as i32 {
            if *(*di).n as libc::c_int == '\0' as i32 {
                return 0 as *mut demangle_component;
            }
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        }
        ret = d_make_comp(
            di,
            t,
            type_0,
            d_make_name(di, s, ((*di).n).offset_from(s) as libc::c_long as libc::c_int),
        );
    }
    if if *(*di).n as libc::c_int == 'E' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    return ret;
}
unsafe extern "C" fn d_local_name(mut di: *mut d_info) -> *mut demangle_component {
    let mut function: *mut demangle_component = 0 as *mut demangle_component;
    let mut name: *mut demangle_component = 0 as *mut demangle_component;
    if if *(*di).n as libc::c_int == 'Z' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    function = d_encoding(di, 0 as libc::c_int);
    if function.is_null() {
        return 0 as *mut demangle_component;
    }
    if if *(*di).n as libc::c_int == 'E' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    if *(*di).n as libc::c_int == 's' as i32 {
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        if d_discriminator(di) == 0 {
            return 0 as *mut demangle_component;
        }
        name = d_make_name(
            di,
            b"string literal\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        );
    } else {
        let mut num: libc::c_int = -(1 as libc::c_int);
        if *(*di).n as libc::c_int == 'd' as i32 {
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
            num = d_compact_number(di);
            if num < 0 as libc::c_int {
                return 0 as *mut demangle_component;
            }
        }
        name = d_name(di);
        if !name.is_null()
            && (*name).type_0 as libc::c_uint
                != DEMANGLE_COMPONENT_LAMBDA as libc::c_int as libc::c_uint
            && (*name).type_0 as libc::c_uint
                != DEMANGLE_COMPONENT_UNNAMED_TYPE as libc::c_int as libc::c_uint
        {
            if d_discriminator(di) == 0 {
                return 0 as *mut demangle_component;
            }
        }
        if num >= 0 as libc::c_int {
            name = d_make_default_arg(di, num, name);
        }
    }
    if (*function).type_0 as libc::c_uint
        == DEMANGLE_COMPONENT_TYPED_NAME as libc::c_int as libc::c_uint
        && (*(*function).u.s_binary.right).type_0 as libc::c_uint
            == DEMANGLE_COMPONENT_FUNCTION_TYPE as libc::c_int as libc::c_uint
    {
        (*(*function).u.s_binary.right).u.s_binary.left = 0 as *mut demangle_component;
    }
    return d_make_comp(di, DEMANGLE_COMPONENT_LOCAL_NAME, function, name);
}
unsafe extern "C" fn d_discriminator(mut di: *mut d_info) -> libc::c_int {
    let mut discrim: libc::c_int = 0;
    let mut num_underscores: libc::c_int = 1 as libc::c_int;
    if *(*di).n as libc::c_int != '_' as i32 {
        return 1 as libc::c_int;
    }
    (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
    if *(*di).n as libc::c_int == '_' as i32 {
        num_underscores += 1;
        num_underscores;
        (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
    }
    discrim = d_number(di);
    if discrim < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if num_underscores > 1 as libc::c_int && discrim >= 10 as libc::c_int {
        if *(*di).n as libc::c_int == '_' as i32 {
            (*di).n = ((*di).n).offset(1 as libc::c_int as isize);
        } else {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn d_lambda(mut di: *mut d_info) -> *mut demangle_component {
    let mut tl: *mut demangle_component = 0 as *mut demangle_component;
    let mut ret: *mut demangle_component = 0 as *mut demangle_component;
    let mut num: libc::c_int = 0;
    if if *(*di).n as libc::c_int == 'U' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    if if *(*di).n as libc::c_int == 'l' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    tl = d_parmlist(di);
    if tl.is_null() {
        return 0 as *mut demangle_component;
    }
    if if *(*di).n as libc::c_int == 'E' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    num = d_compact_number(di);
    if num < 0 as libc::c_int {
        return 0 as *mut demangle_component;
    }
    ret = d_make_empty(di);
    if !ret.is_null() {
        (*ret).type_0 = DEMANGLE_COMPONENT_LAMBDA;
        (*ret).u.s_unary_num.sub = tl;
        (*ret).u.s_unary_num.num = num;
    }
    return ret;
}
unsafe extern "C" fn d_unnamed_type(mut di: *mut d_info) -> *mut demangle_component {
    let mut ret: *mut demangle_component = 0 as *mut demangle_component;
    let mut num: libc::c_int = 0;
    if if *(*di).n as libc::c_int == 'U' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    if if *(*di).n as libc::c_int == 't' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    num = d_compact_number(di);
    if num < 0 as libc::c_int {
        return 0 as *mut demangle_component;
    }
    ret = d_make_empty(di);
    if !ret.is_null() {
        (*ret).type_0 = DEMANGLE_COMPONENT_UNNAMED_TYPE;
        (*ret).u.s_number.number = num as libc::c_long;
    }
    if d_add_substitution(di, ret) == 0 {
        return 0 as *mut demangle_component;
    }
    return ret;
}
unsafe extern "C" fn d_clone_suffix(
    mut di: *mut d_info,
    mut encoding: *mut demangle_component,
) -> *mut demangle_component {
    let mut suffix: *const libc::c_char = (*di).n;
    let mut pend: *const libc::c_char = suffix;
    let mut n: *mut demangle_component = 0 as *mut demangle_component;
    if *pend as libc::c_int == '.' as i32
        && (*pend.offset(1 as libc::c_int as isize) as libc::c_int >= 'a' as i32
            && *pend.offset(1 as libc::c_int as isize) as libc::c_int <= 'z' as i32
            || *pend.offset(1 as libc::c_int as isize) as libc::c_int == '_' as i32)
    {
        pend = pend.offset(2 as libc::c_int as isize);
        while *pend as libc::c_int >= 'a' as i32 && *pend as libc::c_int <= 'z' as i32
            || *pend as libc::c_int == '_' as i32
        {
            pend = pend.offset(1);
            pend;
        }
    }
    while *pend as libc::c_int == '.' as i32
        && (*pend.offset(1 as libc::c_int as isize) as libc::c_int >= '0' as i32
            && *pend.offset(1 as libc::c_int as isize) as libc::c_int <= '9' as i32)
    {
        pend = pend.offset(2 as libc::c_int as isize);
        while *pend as libc::c_int >= '0' as i32 && *pend as libc::c_int <= '9' as i32 {
            pend = pend.offset(1);
            pend;
        }
    }
    (*di).n = ((*di).n).offset(pend.offset_from(suffix) as libc::c_long as isize);
    n = d_make_name(di, suffix, pend.offset_from(suffix) as libc::c_long as libc::c_int);
    return d_make_comp(di, DEMANGLE_COMPONENT_CLONE, encoding, n);
}
unsafe extern "C" fn d_add_substitution(
    mut di: *mut d_info,
    mut dc: *mut demangle_component,
) -> libc::c_int {
    if dc.is_null() {
        return 0 as libc::c_int;
    }
    if (*di).next_sub >= (*di).num_subs {
        return 0 as libc::c_int;
    }
    let ref mut fresh12 = *((*di).subs).offset((*di).next_sub as isize);
    *fresh12 = dc;
    (*di).next_sub += 1;
    (*di).next_sub;
    return 1 as libc::c_int;
}
static mut standard_subs: [d_standard_sub_info; 7] = [d_standard_sub_info {
    code: 0,
    simple_expansion: 0 as *const libc::c_char,
    simple_len: 0,
    full_expansion: 0 as *const libc::c_char,
    full_len: 0,
    set_last_name: 0 as *const libc::c_char,
    set_last_name_len: 0,
}; 7];
unsafe extern "C" fn d_substitution(
    mut di: *mut d_info,
    mut prefix: libc::c_int,
) -> *mut demangle_component {
    let mut c: libc::c_char = 0;
    if if *(*di).n as libc::c_int == 'S' as i32 {
        (*di).n = ((*di).n).offset(1);
        (*di).n;
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        return 0 as *mut demangle_component;
    }
    c = (if *(*di).n as libc::c_int == '\0' as i32 {
        '\0' as i32
    } else {
        let fresh13 = (*di).n;
        (*di).n = ((*di).n).offset(1);
        *fresh13 as libc::c_int
    }) as libc::c_char;
    if c as libc::c_int == '_' as i32
        || c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32
        || c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32
    {
        let mut id: libc::c_uint = 0;
        id = 0 as libc::c_int as libc::c_uint;
        if c as libc::c_int != '_' as i32 {
            loop {
                let mut new_id: libc::c_uint = 0;
                if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
                    new_id = id
                        .wrapping_mul(36 as libc::c_int as libc::c_uint)
                        .wrapping_add(c as libc::c_uint)
                        .wrapping_sub('0' as i32 as libc::c_uint);
                } else if c as libc::c_int >= 'A' as i32
                    && c as libc::c_int <= 'Z' as i32
                {
                    new_id = id
                        .wrapping_mul(36 as libc::c_int as libc::c_uint)
                        .wrapping_add(c as libc::c_uint)
                        .wrapping_sub('A' as i32 as libc::c_uint)
                        .wrapping_add(10 as libc::c_int as libc::c_uint);
                } else {
                    return 0 as *mut demangle_component
                }
                if new_id < id {
                    return 0 as *mut demangle_component;
                }
                id = new_id;
                c = (if *(*di).n as libc::c_int == '\0' as i32 {
                    '\0' as i32
                } else {
                    let fresh14 = (*di).n;
                    (*di).n = ((*di).n).offset(1);
                    *fresh14 as libc::c_int
                }) as libc::c_char;
                if !(c as libc::c_int != '_' as i32) {
                    break;
                }
            }
            id = id.wrapping_add(1);
            id;
        }
        if id >= (*di).next_sub as libc::c_uint {
            return 0 as *mut demangle_component;
        }
        return *((*di).subs).offset(id as isize);
    } else {
        let mut verbose: libc::c_int = 0;
        let mut p: *const d_standard_sub_info = 0 as *const d_standard_sub_info;
        let mut pend: *const d_standard_sub_info = 0 as *const d_standard_sub_info;
        verbose = ((*di).options & (1 as libc::c_int) << 3 as libc::c_int
            != 0 as libc::c_int) as libc::c_int;
        if verbose == 0 && prefix != 0 {
            let mut peek: libc::c_char = 0;
            peek = *(*di).n;
            if peek as libc::c_int == 'C' as i32 || peek as libc::c_int == 'D' as i32 {
                verbose = 1 as libc::c_int;
            }
        }
        pend = (&*standard_subs.as_ptr().offset(0 as libc::c_int as isize)
            as *const d_standard_sub_info)
            .offset(
                (::core::mem::size_of::<[d_standard_sub_info; 7]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<d_standard_sub_info>() as libc::c_ulong,
                    ) as isize,
            );
        p = &*standard_subs.as_ptr().offset(0 as libc::c_int as isize)
            as *const d_standard_sub_info;
        while p < pend {
            if c as libc::c_int == (*p).code as libc::c_int {
                let mut s: *const libc::c_char = 0 as *const libc::c_char;
                let mut len: libc::c_int = 0;
                let mut dc: *mut demangle_component = 0 as *mut demangle_component;
                if !((*p).set_last_name).is_null() {
                    (*di)
                        .last_name = d_make_sub(
                        di,
                        (*p).set_last_name,
                        (*p).set_last_name_len,
                    );
                }
                if verbose != 0 {
                    s = (*p).full_expansion;
                    len = (*p).full_len;
                } else {
                    s = (*p).simple_expansion;
                    len = (*p).simple_len;
                }
                (*di).expansion += len;
                dc = d_make_sub(di, s, len);
                if *(*di).n as libc::c_int == 'B' as i32 {
                    dc = d_abi_tags(di, dc);
                    if d_add_substitution(di, dc) == 0 {
                        return 0 as *mut demangle_component;
                    }
                }
                return dc;
            }
            p = p.offset(1);
            p;
        }
        return 0 as *mut demangle_component;
    };
}
unsafe extern "C" fn d_checkpoint(
    mut di: *mut d_info,
    mut checkpoint: *mut d_info_checkpoint,
) {
    (*checkpoint).n = (*di).n;
    (*checkpoint).next_comp = (*di).next_comp;
    (*checkpoint).next_sub = (*di).next_sub;
    (*checkpoint).expansion = (*di).expansion;
}
unsafe extern "C" fn d_backtrack(
    mut di: *mut d_info,
    mut checkpoint: *mut d_info_checkpoint,
) {
    (*di).n = (*checkpoint).n;
    (*di).next_comp = (*checkpoint).next_comp;
    (*di).next_sub = (*checkpoint).next_sub;
    (*di).expansion = (*checkpoint).expansion;
}
unsafe extern "C" fn d_growable_string_init(
    mut dgs: *mut d_growable_string,
    mut estimate: size_t,
) {
    (*dgs).buf = 0 as *mut libc::c_char;
    (*dgs).len = 0 as libc::c_int as size_t;
    (*dgs).alc = 0 as libc::c_int as size_t;
    (*dgs).allocation_failure = 0 as libc::c_int;
    if estimate > 0 as libc::c_int as libc::c_ulong {
        d_growable_string_resize(dgs, estimate);
    }
}
#[inline]
unsafe extern "C" fn d_growable_string_resize(
    mut dgs: *mut d_growable_string,
    mut need: size_t,
) {
    let mut newalc: size_t = 0;
    let mut newbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*dgs).allocation_failure != 0 {
        return;
    }
    newalc = if (*dgs).alc > 0 as libc::c_int as libc::c_ulong {
        (*dgs).alc
    } else {
        2 as libc::c_int as libc::c_ulong
    };
    while newalc < need {
        newalc <<= 1 as libc::c_int;
    }
    newbuf = realloc((*dgs).buf as *mut libc::c_void, newalc) as *mut libc::c_char;
    if newbuf.is_null() {
        free((*dgs).buf as *mut libc::c_void);
        (*dgs).buf = 0 as *mut libc::c_char;
        (*dgs).len = 0 as libc::c_int as size_t;
        (*dgs).alc = 0 as libc::c_int as size_t;
        (*dgs).allocation_failure = 1 as libc::c_int;
        return;
    }
    (*dgs).buf = newbuf;
    (*dgs).alc = newalc;
}
#[inline]
unsafe extern "C" fn d_growable_string_append_buffer(
    mut dgs: *mut d_growable_string,
    mut s: *const libc::c_char,
    mut l: size_t,
) {
    let mut need: size_t = 0;
    need = ((*dgs).len).wrapping_add(l).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if need > (*dgs).alc {
        d_growable_string_resize(dgs, need);
    }
    if (*dgs).allocation_failure != 0 {
        return;
    }
    memcpy(
        ((*dgs).buf).offset((*dgs).len as isize) as *mut libc::c_void,
        s as *const libc::c_void,
        l,
    );
    *((*dgs).buf)
        .offset(((*dgs).len).wrapping_add(l) as isize) = '\0' as i32 as libc::c_char;
    (*dgs).len = ((*dgs).len as libc::c_ulong).wrapping_add(l) as size_t as size_t;
}
unsafe extern "C" fn d_growable_string_callback_adapter(
    mut s: *const libc::c_char,
    mut l: size_t,
    mut opaque: *mut libc::c_void,
) {
    let mut dgs: *mut d_growable_string = opaque as *mut d_growable_string;
    d_growable_string_append_buffer(dgs, s, l);
}
unsafe extern "C" fn d_count_templates_scopes(
    mut dpi: *mut d_print_info,
    mut dc: *mut demangle_component,
) {
    let mut current_block: u64;
    if dc.is_null() || (*dc).d_counting > 1 as libc::c_int
        || (*dpi).recursion > 1024 as libc::c_int
    {
        return;
    }
    (*dc).d_counting += 1;
    (*dc).d_counting;
    match (*dc).type_0 as libc::c_uint {
        4 => {
            (*dpi).num_copy_templates += 1;
            (*dpi).num_copy_templates;
            current_block = 1015251370922701328;
        }
        35 | 36 => {
            if (*(*dc).u.s_binary.left).type_0 as libc::c_uint
                == DEMANGLE_COMPONENT_TEMPLATE_PARAM as libc::c_int as libc::c_uint
            {
                (*dpi).num_saved_scopes += 1;
                (*dpi).num_saved_scopes;
            }
            current_block = 1015251370922701328;
        }
        1 | 2 | 3 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22
        | 23 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 78 | 80 | 81 | 33 | 34 | 37 | 38
        | 40 | 41 | 42 | 43 | 45 | 46 | 47 | 48 | 49 | 52 | 53 | 54 | 55 | 56 | 57 | 58
        | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 68 | 74 | 75 | 76 | 77 | 79 => {
            current_block = 1015251370922701328;
        }
        7 => {
            d_count_templates_scopes(dpi, (*dc).u.s_ctor.name);
            current_block = 313581471991351815;
        }
        8 => {
            d_count_templates_scopes(dpi, (*dc).u.s_dtor.name);
            current_block = 313581471991351815;
        }
        51 => {
            d_count_templates_scopes(dpi, (*dc).u.s_extended_operator.name);
            current_block = 313581471991351815;
        }
        44 => {
            d_count_templates_scopes(dpi, (*dc).u.s_fixed.length);
            current_block = 313581471991351815;
        }
        69 | 70 => {
            d_count_templates_scopes(dpi, (*dc).u.s_binary.left);
            current_block = 313581471991351815;
        }
        71 | 72 => {
            d_count_templates_scopes(dpi, (*dc).u.s_unary_num.sub);
            current_block = 313581471991351815;
        }
        0 | 5 | 6 | 24 | 39 | 50 | 66 | 67 | 73 | _ => {
            current_block = 313581471991351815;
        }
    }
    match current_block {
        1015251370922701328 => {
            if (*dpi).recursion > 2048 as libc::c_int {
                return;
            }
            (*dpi).recursion += 1;
            (*dpi).recursion;
            d_count_templates_scopes(dpi, (*dc).u.s_binary.left);
            d_count_templates_scopes(dpi, (*dc).u.s_binary.right);
            (*dpi).recursion -= 1;
            (*dpi).recursion;
        }
        _ => {}
    };
}
unsafe extern "C" fn d_print_init(
    mut dpi: *mut d_print_info,
    mut callback: demangle_callbackref,
    mut opaque: *mut libc::c_void,
    mut dc: *mut demangle_component,
) {
    (*dpi).len = 0 as libc::c_int as size_t;
    (*dpi).last_char = '\0' as i32 as libc::c_char;
    (*dpi).templates = 0 as *mut d_print_template;
    (*dpi).modifiers = 0 as *mut d_print_mod;
    (*dpi).pack_index = 0 as libc::c_int;
    (*dpi).flush_count = 0 as libc::c_int as libc::c_ulong;
    (*dpi).callback = callback;
    (*dpi).opaque = opaque;
    (*dpi).demangle_failure = 0 as libc::c_int;
    (*dpi).recursion = 0 as libc::c_int;
    (*dpi).is_lambda_arg = 0 as libc::c_int;
    (*dpi).component_stack = 0 as *const d_component_stack;
    (*dpi).saved_scopes = 0 as *mut d_saved_scope;
    (*dpi).next_saved_scope = 0 as libc::c_int;
    (*dpi).num_saved_scopes = 0 as libc::c_int;
    (*dpi).copy_templates = 0 as *mut d_print_template;
    (*dpi).next_copy_template = 0 as libc::c_int;
    (*dpi).num_copy_templates = 0 as libc::c_int;
    d_count_templates_scopes(dpi, dc);
    if (*dpi).recursion < 2048 as libc::c_int {
        (*dpi).recursion = 0 as libc::c_int;
    }
    (*dpi).num_copy_templates *= (*dpi).num_saved_scopes;
    (*dpi).current_template = 0 as *const demangle_component;
}
#[inline]
unsafe extern "C" fn d_print_error(mut dpi: *mut d_print_info) {
    (*dpi).demangle_failure = 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn d_print_saw_error(mut dpi: *mut d_print_info) -> libc::c_int {
    return ((*dpi).demangle_failure != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn d_print_flush(mut dpi: *mut d_print_info) {
    (*dpi).buf[(*dpi).len as usize] = '\0' as i32 as libc::c_char;
    ((*dpi).callback)
        .expect(
            "non-null function pointer",
        )(((*dpi).buf).as_mut_ptr(), (*dpi).len, (*dpi).opaque);
    (*dpi).len = 0 as libc::c_int as size_t;
    (*dpi).flush_count = ((*dpi).flush_count).wrapping_add(1);
    (*dpi).flush_count;
}
#[inline]
unsafe extern "C" fn d_append_char(mut dpi: *mut d_print_info, mut c: libc::c_char) {
    if (*dpi).len
        == (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        d_print_flush(dpi);
    }
    let fresh15 = (*dpi).len;
    (*dpi).len = ((*dpi).len).wrapping_add(1);
    (*dpi).buf[fresh15 as usize] = c;
    (*dpi).last_char = c;
}
#[inline]
unsafe extern "C" fn d_append_buffer(
    mut dpi: *mut d_print_info,
    mut s: *const libc::c_char,
    mut l: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < l {
        d_append_char(dpi, *s.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
}
#[inline]
unsafe extern "C" fn d_append_string(
    mut dpi: *mut d_print_info,
    mut s: *const libc::c_char,
) {
    d_append_buffer(dpi, s, strlen(s));
}
#[inline]
unsafe extern "C" fn d_append_num(mut dpi: *mut d_print_info, mut l: libc::c_int) {
    let mut buf: [libc::c_char; 25] = [0; 25];
    sprintf(buf.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, l);
    d_append_string(dpi, buf.as_mut_ptr());
}
#[inline]
unsafe extern "C" fn d_last_char(mut dpi: *mut d_print_info) -> libc::c_char {
    return (*dpi).last_char;
}
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_print_callback(
    mut options: libc::c_int,
    mut dc: *mut demangle_component,
    mut callback: demangle_callbackref,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    let mut dpi: d_print_info = d_print_info {
        buf: [0; 256],
        len: 0,
        last_char: 0,
        callback: None,
        opaque: 0 as *mut libc::c_void,
        templates: 0 as *mut d_print_template,
        modifiers: 0 as *mut d_print_mod,
        demangle_failure: 0,
        recursion: 0,
        is_lambda_arg: 0,
        pack_index: 0,
        flush_count: 0,
        component_stack: 0 as *const d_component_stack,
        saved_scopes: 0 as *mut d_saved_scope,
        next_saved_scope: 0,
        num_saved_scopes: 0,
        copy_templates: 0 as *mut d_print_template,
        next_copy_template: 0,
        num_copy_templates: 0,
        current_template: 0 as *const demangle_component,
    };
    d_print_init(&mut dpi, callback, opaque, dc);
    let vla = (if dpi.num_saved_scopes > 0 as libc::c_int {
        dpi.num_saved_scopes
    } else {
        1 as libc::c_int
    }) as usize;
    let mut scopes: Vec::<d_saved_scope> = ::std::vec::from_elem(
        d_saved_scope {
            container: 0 as *const demangle_component,
            templates: 0 as *mut d_print_template,
        },
        vla,
    );
    let vla_0 = (if dpi.num_copy_templates > 0 as libc::c_int {
        dpi.num_copy_templates
    } else {
        1 as libc::c_int
    }) as usize;
    let mut temps: Vec::<d_print_template> = ::std::vec::from_elem(
        d_print_template {
            next: 0 as *mut d_print_template,
            template_decl: 0 as *const demangle_component,
        },
        vla_0,
    );
    dpi.saved_scopes = scopes.as_mut_ptr();
    dpi.copy_templates = temps.as_mut_ptr();
    d_print_comp(&mut dpi, options, dc);
    d_print_flush(&mut dpi);
    return (d_print_saw_error(&mut dpi) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_print(
    mut options: libc::c_int,
    mut dc: *mut demangle_component,
    mut estimate: libc::c_int,
    mut palc: *mut size_t,
) -> *mut libc::c_char {
    let mut dgs: d_growable_string = d_growable_string {
        buf: 0 as *mut libc::c_char,
        len: 0,
        alc: 0,
        allocation_failure: 0,
    };
    d_growable_string_init(&mut dgs, estimate as size_t);
    if cplus_demangle_print_callback(
        options,
        dc,
        Some(
            d_growable_string_callback_adapter
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    size_t,
                    *mut libc::c_void,
                ) -> (),
        ),
        &mut dgs as *mut d_growable_string as *mut libc::c_void,
    ) == 0
    {
        free(dgs.buf as *mut libc::c_void);
        *palc = 0 as libc::c_int as size_t;
        return 0 as *mut libc::c_char;
    }
    *palc = if dgs.allocation_failure != 0 {
        1 as libc::c_int as libc::c_ulong
    } else {
        dgs.alc
    };
    return dgs.buf;
}
unsafe extern "C" fn d_index_template_argument(
    mut args: *mut demangle_component,
    mut i: libc::c_int,
) -> *mut demangle_component {
    let mut a: *mut demangle_component = 0 as *mut demangle_component;
    if i < 0 as libc::c_int {
        return args;
    }
    a = args;
    while !a.is_null() {
        if (*a).type_0 as libc::c_uint
            != DEMANGLE_COMPONENT_TEMPLATE_ARGLIST as libc::c_int as libc::c_uint
        {
            return 0 as *mut demangle_component;
        }
        if i <= 0 as libc::c_int {
            break;
        }
        i -= 1;
        i;
        a = (*a).u.s_binary.right;
    }
    if i != 0 as libc::c_int || a.is_null() {
        return 0 as *mut demangle_component;
    }
    return (*a).u.s_binary.left;
}
unsafe extern "C" fn d_lookup_template_argument(
    mut dpi: *mut d_print_info,
    mut dc: *const demangle_component,
) -> *mut demangle_component {
    if ((*dpi).templates).is_null() {
        d_print_error(dpi);
        return 0 as *mut demangle_component;
    }
    return d_index_template_argument(
        (*(*(*dpi).templates).template_decl).u.s_binary.right,
        (*dc).u.s_number.number as libc::c_int,
    );
}
unsafe extern "C" fn d_find_pack(
    mut dpi: *mut d_print_info,
    mut dc: *const demangle_component,
) -> *mut demangle_component {
    let mut a: *mut demangle_component = 0 as *mut demangle_component;
    if dc.is_null() {
        return 0 as *mut demangle_component;
    }
    match (*dc).type_0 as libc::c_uint {
        5 => {
            a = d_lookup_template_argument(dpi, dc);
            if !a.is_null()
                && (*a).type_0 as libc::c_uint
                    == DEMANGLE_COMPONENT_TEMPLATE_ARGLIST as libc::c_int as libc::c_uint
            {
                return a;
            }
            return 0 as *mut demangle_component;
        }
        76 => return 0 as *mut demangle_component,
        71 | 0 | 77 | 50 | 39 | 24 | 66 | 6 | 73 | 44 | 72 | 67 => {
            return 0 as *mut demangle_component;
        }
        51 => return d_find_pack(dpi, (*dc).u.s_extended_operator.name),
        7 => return d_find_pack(dpi, (*dc).u.s_ctor.name),
        8 => return d_find_pack(dpi, (*dc).u.s_dtor.name),
        _ => {
            a = d_find_pack(dpi, (*dc).u.s_binary.left);
            if !a.is_null() {
                return a;
            }
            return d_find_pack(dpi, (*dc).u.s_binary.right);
        }
    };
}
unsafe extern "C" fn d_pack_length(mut dc: *const demangle_component) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    while !dc.is_null()
        && (*dc).type_0 as libc::c_uint
            == DEMANGLE_COMPONENT_TEMPLATE_ARGLIST as libc::c_int as libc::c_uint
        && !((*dc).u.s_binary.left).is_null()
    {
        count += 1;
        count;
        dc = (*dc).u.s_binary.right;
    }
    return count;
}
unsafe extern "C" fn d_args_length(
    mut dpi: *mut d_print_info,
    mut dc: *const demangle_component,
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    while !dc.is_null()
        && (*dc).type_0 as libc::c_uint
            == DEMANGLE_COMPONENT_TEMPLATE_ARGLIST as libc::c_int as libc::c_uint
    {
        let mut elt: *mut demangle_component = (*dc).u.s_binary.left;
        if elt.is_null() {
            break;
        }
        if (*elt).type_0 as libc::c_uint
            == DEMANGLE_COMPONENT_PACK_EXPANSION as libc::c_int as libc::c_uint
        {
            let mut a: *mut demangle_component = d_find_pack(
                dpi,
                (*elt).u.s_binary.left,
            );
            count += d_pack_length(a);
        } else {
            count += 1;
            count;
        }
        dc = (*dc).u.s_binary.right;
    }
    return count;
}
unsafe extern "C" fn d_print_subexpr(
    mut dpi: *mut d_print_info,
    mut options: libc::c_int,
    mut dc: *mut demangle_component,
) {
    let mut simple: libc::c_int = 0 as libc::c_int;
    if (*dc).type_0 as libc::c_uint
        == DEMANGLE_COMPONENT_NAME as libc::c_int as libc::c_uint
        || (*dc).type_0 as libc::c_uint
            == DEMANGLE_COMPONENT_QUAL_NAME as libc::c_int as libc::c_uint
        || (*dc).type_0 as libc::c_uint
            == DEMANGLE_COMPONENT_INITIALIZER_LIST as libc::c_int as libc::c_uint
        || (*dc).type_0 as libc::c_uint
            == DEMANGLE_COMPONENT_FUNCTION_PARAM as libc::c_int as libc::c_uint
    {
        simple = 1 as libc::c_int;
    }
    if simple == 0 {
        d_append_char(dpi, '(' as i32 as libc::c_char);
    }
    d_print_comp(dpi, options, dc);
    if simple == 0 {
        d_append_char(dpi, ')' as i32 as libc::c_char);
    }
}
unsafe extern "C" fn d_save_scope(
    mut dpi: *mut d_print_info,
    mut container: *const demangle_component,
) {
    let mut scope: *mut d_saved_scope = 0 as *mut d_saved_scope;
    let mut src: *mut d_print_template = 0 as *mut d_print_template;
    let mut link: *mut *mut d_print_template = 0 as *mut *mut d_print_template;
    if (*dpi).next_saved_scope >= (*dpi).num_saved_scopes {
        d_print_error(dpi);
        return;
    }
    scope = &mut *((*dpi).saved_scopes).offset((*dpi).next_saved_scope as isize)
        as *mut d_saved_scope;
    (*dpi).next_saved_scope += 1;
    (*dpi).next_saved_scope;
    (*scope).container = container;
    link = &mut (*scope).templates;
    src = (*dpi).templates;
    while !src.is_null() {
        let mut dst: *mut d_print_template = 0 as *mut d_print_template;
        if (*dpi).next_copy_template >= (*dpi).num_copy_templates {
            d_print_error(dpi);
            return;
        }
        dst = &mut *((*dpi).copy_templates).offset((*dpi).next_copy_template as isize)
            as *mut d_print_template;
        (*dpi).next_copy_template += 1;
        (*dpi).next_copy_template;
        (*dst).template_decl = (*src).template_decl;
        *link = dst;
        link = &mut (*dst).next;
        src = (*src).next;
    }
    *link = 0 as *mut d_print_template;
}
unsafe extern "C" fn d_get_saved_scope(
    mut dpi: *mut d_print_info,
    mut container: *const demangle_component,
) -> *mut d_saved_scope {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*dpi).next_saved_scope {
        if (*((*dpi).saved_scopes).offset(i as isize)).container == container {
            return &mut *((*dpi).saved_scopes).offset(i as isize) as *mut d_saved_scope;
        }
        i += 1;
        i;
    }
    return 0 as *mut d_saved_scope;
}
unsafe extern "C" fn d_maybe_print_fold_expression(
    mut dpi: *mut d_print_info,
    mut options: libc::c_int,
    mut dc: *mut demangle_component,
) -> libc::c_int {
    let mut ops: *mut demangle_component = 0 as *mut demangle_component;
    let mut operator_: *mut demangle_component = 0 as *mut demangle_component;
    let mut op1: *mut demangle_component = 0 as *mut demangle_component;
    let mut op2: *mut demangle_component = 0 as *mut demangle_component;
    let mut save_idx: libc::c_int = 0;
    let mut fold_code: *const libc::c_char = (*(*(*dc).u.s_binary.left).u.s_operator.op)
        .code;
    if *fold_code.offset(0 as libc::c_int as isize) as libc::c_int != 'f' as i32 {
        return 0 as libc::c_int;
    }
    ops = (*dc).u.s_binary.right;
    operator_ = (*ops).u.s_binary.left;
    op1 = (*ops).u.s_binary.right;
    op2 = 0 as *mut demangle_component;
    if (*op1).type_0 as libc::c_uint
        == DEMANGLE_COMPONENT_TRINARY_ARG2 as libc::c_int as libc::c_uint
    {
        op2 = (*op1).u.s_binary.right;
        op1 = (*op1).u.s_binary.left;
    }
    save_idx = (*dpi).pack_index;
    (*dpi).pack_index = -(1 as libc::c_int);
    match *fold_code.offset(1 as libc::c_int as isize) as libc::c_int {
        108 => {
            d_append_string(dpi, b"(...\0" as *const u8 as *const libc::c_char);
            d_print_expr_op(dpi, options, operator_);
            d_print_subexpr(dpi, options, op1);
            d_append_char(dpi, ')' as i32 as libc::c_char);
        }
        114 => {
            d_append_char(dpi, '(' as i32 as libc::c_char);
            d_print_subexpr(dpi, options, op1);
            d_print_expr_op(dpi, options, operator_);
            d_append_string(dpi, b"...)\0" as *const u8 as *const libc::c_char);
        }
        76 | 82 => {
            d_append_char(dpi, '(' as i32 as libc::c_char);
            d_print_subexpr(dpi, options, op1);
            d_print_expr_op(dpi, options, operator_);
            d_append_string(dpi, b"...\0" as *const u8 as *const libc::c_char);
            d_print_expr_op(dpi, options, operator_);
            d_print_subexpr(dpi, options, op2);
            d_append_char(dpi, ')' as i32 as libc::c_char);
        }
        _ => {}
    }
    (*dpi).pack_index = save_idx;
    return 1 as libc::c_int;
}
unsafe extern "C" fn is_designated_init(mut dc: *mut demangle_component) -> libc::c_int {
    if (*dc).type_0 as libc::c_uint
        != DEMANGLE_COMPONENT_BINARY as libc::c_int as libc::c_uint
        && (*dc).type_0 as libc::c_uint
            != DEMANGLE_COMPONENT_TRINARY as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    let mut op: *mut demangle_component = (*dc).u.s_binary.left;
    let mut code: *const libc::c_char = (*(*op).u.s_operator.op).code;
    return (*code.offset(0 as libc::c_int as isize) as libc::c_int == 'd' as i32
        && (*code.offset(1 as libc::c_int as isize) as libc::c_int == 'i' as i32
            || *code.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
            || *code.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32))
        as libc::c_int;
}
unsafe extern "C" fn d_maybe_print_designated_init(
    mut dpi: *mut d_print_info,
    mut options: libc::c_int,
    mut dc: *mut demangle_component,
) -> libc::c_int {
    if is_designated_init(dc) == 0 {
        return 0 as libc::c_int;
    }
    let mut code: *const libc::c_char = (*(*(*dc).u.s_binary.left).u.s_operator.op).code;
    let mut operands: *mut demangle_component = (*dc).u.s_binary.right;
    let mut op1: *mut demangle_component = (*operands).u.s_binary.left;
    let mut op2: *mut demangle_component = (*operands).u.s_binary.right;
    if *code.offset(1 as libc::c_int as isize) as libc::c_int == 'i' as i32 {
        d_append_char(dpi, '.' as i32 as libc::c_char);
    } else {
        d_append_char(dpi, '[' as i32 as libc::c_char);
    }
    d_print_comp(dpi, options, op1);
    if *code.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32 {
        d_append_string(dpi, b" ... \0" as *const u8 as *const libc::c_char);
        d_print_comp(dpi, options, (*op2).u.s_binary.left);
        op2 = (*op2).u.s_binary.right;
    }
    if *code.offset(1 as libc::c_int as isize) as libc::c_int != 'i' as i32 {
        d_append_char(dpi, ']' as i32 as libc::c_char);
    }
    if is_designated_init(op2) != 0 {
        d_print_comp(dpi, options, op2);
    } else {
        d_append_char(dpi, '=' as i32 as libc::c_char);
        d_print_subexpr(dpi, options, op2);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn d_print_comp_inner(
    mut dpi: *mut d_print_info,
    mut options: libc::c_int,
    mut dc: *mut demangle_component,
) {
    let mut mod_inner: *mut demangle_component = 0 as *mut demangle_component;
    let mut saved_templates: *mut d_print_template = 0 as *mut d_print_template;
    let mut need_template_restore: libc::c_int = 0 as libc::c_int;
    if dc.is_null() {
        d_print_error(dpi);
        return;
    }
    if d_print_saw_error(dpi) != 0 {
        return;
    }
    match (*dc).type_0 as libc::c_uint {
        0 => {
            if options & (1 as libc::c_int) << 2 as libc::c_int == 0 as libc::c_int {
                d_append_buffer(dpi, (*dc).u.s_name.s, (*dc).u.s_name.len as size_t);
            } else {
                d_print_java_identifier(dpi, (*dc).u.s_name.s, (*dc).u.s_name.len);
            }
            return;
        }
        77 => {
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            d_append_string(dpi, b"[abi:\0" as *const u8 as *const libc::c_char);
            d_print_comp(dpi, options, (*dc).u.s_binary.right);
            d_append_char(dpi, ']' as i32 as libc::c_char);
            return;
        }
        1 | 2 => {
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            if options & (1 as libc::c_int) << 2 as libc::c_int == 0 as libc::c_int {
                d_append_string(dpi, b"::\0" as *const u8 as *const libc::c_char);
            } else {
                d_append_char(dpi, '.' as i32 as libc::c_char);
            }
            let mut local_name: *mut demangle_component = (*dc).u.s_binary.right;
            if (*local_name).type_0 as libc::c_uint
                == DEMANGLE_COMPONENT_DEFAULT_ARG as libc::c_int as libc::c_uint
            {
                d_append_string(
                    dpi,
                    b"{default arg#\0" as *const u8 as *const libc::c_char,
                );
                d_append_num(dpi, (*local_name).u.s_unary_num.num + 1 as libc::c_int);
                d_append_string(dpi, b"}::\0" as *const u8 as *const libc::c_char);
                local_name = (*local_name).u.s_unary_num.sub;
            }
            d_print_comp(dpi, options, local_name);
            return;
        }
        3 => {
            let mut hold_modifiers: *mut d_print_mod = 0 as *mut d_print_mod;
            let mut typed_name: *mut demangle_component = 0 as *mut demangle_component;
            let mut adpm: [d_print_mod; 4] = [d_print_mod {
                next: 0 as *mut d_print_mod,
                mod_0: 0 as *mut demangle_component,
                printed: 0,
                templates: 0 as *mut d_print_template,
            }; 4];
            let mut i: libc::c_uint = 0;
            let mut dpt: d_print_template = d_print_template {
                next: 0 as *mut d_print_template,
                template_decl: 0 as *const demangle_component,
            };
            hold_modifiers = (*dpi).modifiers;
            (*dpi).modifiers = 0 as *mut d_print_mod;
            i = 0 as libc::c_int as libc::c_uint;
            typed_name = (*dc).u.s_binary.left;
            while !typed_name.is_null() {
                if i as libc::c_ulong
                    >= (::core::mem::size_of::<[d_print_mod; 4]>() as libc::c_ulong)
                        .wrapping_div(
                            ::core::mem::size_of::<d_print_mod>() as libc::c_ulong,
                        )
                {
                    d_print_error(dpi);
                    return;
                }
                adpm[i as usize].next = (*dpi).modifiers;
                (*dpi)
                    .modifiers = &mut *adpm.as_mut_ptr().offset(i as isize)
                    as *mut d_print_mod;
                adpm[i as usize].mod_0 = typed_name;
                adpm[i as usize].printed = 0 as libc::c_int;
                adpm[i as usize].templates = (*dpi).templates;
                i = i.wrapping_add(1);
                i;
                if is_fnqual_component_type((*typed_name).type_0) == 0 {
                    break;
                }
                typed_name = (*typed_name).u.s_binary.left;
            }
            if typed_name.is_null() {
                d_print_error(dpi);
                return;
            }
            if (*typed_name).type_0 as libc::c_uint
                == DEMANGLE_COMPONENT_LOCAL_NAME as libc::c_int as libc::c_uint
            {
                typed_name = (*typed_name).u.s_binary.right;
                if (*typed_name).type_0 as libc::c_uint
                    == DEMANGLE_COMPONENT_DEFAULT_ARG as libc::c_int as libc::c_uint
                {
                    typed_name = (*typed_name).u.s_unary_num.sub;
                }
                while !typed_name.is_null()
                    && is_fnqual_component_type((*typed_name).type_0) != 0
                {
                    if i as libc::c_ulong
                        >= (::core::mem::size_of::<[d_print_mod; 4]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<d_print_mod>() as libc::c_ulong,
                            )
                    {
                        d_print_error(dpi);
                        return;
                    }
                    adpm[i
                        as usize] = adpm[i.wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as usize];
                    adpm[i as usize]
                        .next = &mut *adpm
                        .as_mut_ptr()
                        .offset(
                            i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as *mut d_print_mod;
                    (*dpi)
                        .modifiers = &mut *adpm.as_mut_ptr().offset(i as isize)
                        as *mut d_print_mod;
                    adpm[i.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                        .mod_0 = typed_name;
                    adpm[i.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                        .printed = 0 as libc::c_int;
                    adpm[i.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                        .templates = (*dpi).templates;
                    i = i.wrapping_add(1);
                    i;
                    typed_name = (*typed_name).u.s_binary.left;
                }
                if typed_name.is_null() {
                    d_print_error(dpi);
                    return;
                }
            }
            if (*typed_name).type_0 as libc::c_uint
                == DEMANGLE_COMPONENT_TEMPLATE as libc::c_int as libc::c_uint
            {
                dpt.next = (*dpi).templates;
                (*dpi).templates = &mut dpt;
                dpt.template_decl = typed_name;
            }
            d_print_comp(dpi, options, (*dc).u.s_binary.right);
            if (*typed_name).type_0 as libc::c_uint
                == DEMANGLE_COMPONENT_TEMPLATE as libc::c_int as libc::c_uint
            {
                (*dpi).templates = dpt.next;
            }
            while i > 0 as libc::c_int as libc::c_uint {
                i = i.wrapping_sub(1);
                i;
                if adpm[i as usize].printed == 0 {
                    d_append_char(dpi, ' ' as i32 as libc::c_char);
                    d_print_mod(dpi, options, adpm[i as usize].mod_0);
                }
            }
            (*dpi).modifiers = hold_modifiers;
            return;
        }
        4 => {
            let mut hold_dpm: *mut d_print_mod = 0 as *mut d_print_mod;
            let mut dcl: *mut demangle_component = 0 as *mut demangle_component;
            let mut hold_current: *const demangle_component = 0
                as *const demangle_component;
            hold_current = (*dpi).current_template;
            (*dpi).current_template = dc;
            hold_dpm = (*dpi).modifiers;
            (*dpi).modifiers = 0 as *mut d_print_mod;
            dcl = (*dc).u.s_binary.left;
            if options & (1 as libc::c_int) << 2 as libc::c_int != 0 as libc::c_int
                && (*dcl).type_0 as libc::c_uint
                    == DEMANGLE_COMPONENT_NAME as libc::c_int as libc::c_uint
                && (*dcl).u.s_name.len == 6 as libc::c_int
                && strncmp(
                    (*dcl).u.s_name.s,
                    b"JArray\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                d_print_comp(dpi, options, (*dc).u.s_binary.right);
                d_append_string(dpi, b"[]\0" as *const u8 as *const libc::c_char);
            } else {
                d_print_comp(dpi, options, dcl);
                if d_last_char(dpi) as libc::c_int == '<' as i32 {
                    d_append_char(dpi, ' ' as i32 as libc::c_char);
                }
                d_append_char(dpi, '<' as i32 as libc::c_char);
                d_print_comp(dpi, options, (*dc).u.s_binary.right);
                if d_last_char(dpi) as libc::c_int == '>' as i32 {
                    d_append_char(dpi, ' ' as i32 as libc::c_char);
                }
                d_append_char(dpi, '>' as i32 as libc::c_char);
            }
            (*dpi).modifiers = hold_dpm;
            (*dpi).current_template = hold_current;
            return;
        }
        5 => {
            if (*dpi).is_lambda_arg != 0 {
                d_append_buffer(
                    dpi,
                    b"auto:\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as size_t,
                );
                d_append_num(
                    dpi,
                    ((*dc).u.s_number.number + 1 as libc::c_int as libc::c_long)
                        as libc::c_int,
                );
            } else {
                let mut hold_dpt: *mut d_print_template = 0 as *mut d_print_template;
                let mut a: *mut demangle_component = d_lookup_template_argument(dpi, dc);
                if !a.is_null()
                    && (*a).type_0 as libc::c_uint
                        == DEMANGLE_COMPONENT_TEMPLATE_ARGLIST as libc::c_int
                            as libc::c_uint
                {
                    a = d_index_template_argument(a, (*dpi).pack_index);
                }
                if a.is_null() {
                    d_print_error(dpi);
                    return;
                }
                hold_dpt = (*dpi).templates;
                (*dpi).templates = (*hold_dpt).next;
                d_print_comp(dpi, options, a);
                (*dpi).templates = hold_dpt;
            }
            return;
        }
        48 => {
            d_append_string(
                dpi,
                b"template parameter object for \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        7 => {
            d_print_comp(dpi, options, (*dc).u.s_ctor.name);
            return;
        }
        8 => {
            d_append_char(dpi, '~' as i32 as libc::c_char);
            d_print_comp(dpi, options, (*dc).u.s_dtor.name);
            return;
        }
        9 => {
            d_append_string(dpi, b"vtable for \0" as *const u8 as *const libc::c_char);
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        10 => {
            d_append_string(dpi, b"VTT for \0" as *const u8 as *const libc::c_char);
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        11 => {
            d_append_string(
                dpi,
                b"construction vtable for \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            d_append_string(dpi, b"-in-\0" as *const u8 as *const libc::c_char);
            d_print_comp(dpi, options, (*dc).u.s_binary.right);
            return;
        }
        12 => {
            d_append_string(dpi, b"typeinfo for \0" as *const u8 as *const libc::c_char);
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        13 => {
            d_append_string(
                dpi,
                b"typeinfo name for \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        14 => {
            d_append_string(
                dpi,
                b"typeinfo fn for \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        15 => {
            d_append_string(
                dpi,
                b"non-virtual thunk to \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        16 => {
            d_append_string(
                dpi,
                b"virtual thunk to \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        17 => {
            d_append_string(
                dpi,
                b"covariant return thunk to \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        18 => {
            d_append_string(
                dpi,
                b"java Class for \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        19 => {
            d_append_string(
                dpi,
                b"guard variable for \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        20 => {
            d_append_string(
                dpi,
                b"TLS init function for \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        21 => {
            d_append_string(
                dpi,
                b"TLS wrapper function for \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        22 => {
            d_append_string(
                dpi,
                b"reference temporary #\0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.right);
            d_append_string(dpi, b" for \0" as *const u8 as *const libc::c_char);
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        23 => {
            d_append_string(
                dpi,
                b"hidden alias for \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        74 => {
            d_append_string(
                dpi,
                b"transaction clone for \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        75 => {
            d_append_string(
                dpi,
                b"non-transaction clone for \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        24 => {
            d_append_buffer(
                dpi,
                (*dc).u.s_string.string,
                (*dc).u.s_string.len as size_t,
            );
            return;
        }
        25 | 26 | 27 => {
            let mut pdpm: *mut d_print_mod = 0 as *mut d_print_mod;
            pdpm = (*dpi).modifiers;
            while !pdpm.is_null() {
                if (*pdpm).printed == 0 {
                    if (*(*pdpm).mod_0).type_0 as libc::c_uint
                        != DEMANGLE_COMPONENT_RESTRICT as libc::c_int as libc::c_uint
                        && (*(*pdpm).mod_0).type_0 as libc::c_uint
                            != DEMANGLE_COMPONENT_VOLATILE as libc::c_int as libc::c_uint
                        && (*(*pdpm).mod_0).type_0 as libc::c_uint
                            != DEMANGLE_COMPONENT_CONST as libc::c_int as libc::c_uint
                    {
                        break;
                    }
                    if (*(*pdpm).mod_0).type_0 as libc::c_uint
                        == (*dc).type_0 as libc::c_uint
                    {
                        d_print_comp(dpi, options, (*dc).u.s_binary.left);
                        return;
                    }
                }
                pdpm = (*pdpm).next;
            }
        }
        35 | 36 => {
            let mut sub: *mut demangle_component = (*dc).u.s_binary.left;
            if (*dpi).is_lambda_arg == 0
                && (*sub).type_0 as libc::c_uint
                    == DEMANGLE_COMPONENT_TEMPLATE_PARAM as libc::c_int as libc::c_uint
            {
                let mut scope: *mut d_saved_scope = d_get_saved_scope(dpi, sub);
                let mut a_0: *mut demangle_component = 0 as *mut demangle_component;
                if scope.is_null() {
                    d_save_scope(dpi, sub);
                    if d_print_saw_error(dpi) != 0 {
                        return;
                    }
                } else {
                    let mut dcse: *const d_component_stack = 0
                        as *const d_component_stack;
                    let mut found_self_or_parent: libc::c_int = 0 as libc::c_int;
                    dcse = (*dpi).component_stack;
                    while !dcse.is_null() {
                        if (*dcse).dc == sub as *const demangle_component
                            || (*dcse).dc == dc as *const demangle_component
                                && dcse != (*dpi).component_stack
                        {
                            found_self_or_parent = 1 as libc::c_int;
                            break;
                        } else {
                            dcse = (*dcse).parent;
                        }
                    }
                    if found_self_or_parent == 0 {
                        saved_templates = (*dpi).templates;
                        (*dpi).templates = (*scope).templates;
                        need_template_restore = 1 as libc::c_int;
                    }
                }
                a_0 = d_lookup_template_argument(dpi, sub);
                if !a_0.is_null()
                    && (*a_0).type_0 as libc::c_uint
                        == DEMANGLE_COMPONENT_TEMPLATE_ARGLIST as libc::c_int
                            as libc::c_uint
                {
                    a_0 = d_index_template_argument(a_0, (*dpi).pack_index);
                }
                if a_0.is_null() {
                    if need_template_restore != 0 {
                        (*dpi).templates = saved_templates;
                    }
                    d_print_error(dpi);
                    return;
                }
                sub = a_0;
            }
            if (*sub).type_0 as libc::c_uint
                == DEMANGLE_COMPONENT_REFERENCE as libc::c_int as libc::c_uint
                || (*sub).type_0 as libc::c_uint == (*dc).type_0 as libc::c_uint
            {
                dc = sub;
            } else if (*sub).type_0 as libc::c_uint
                == DEMANGLE_COMPONENT_RVALUE_REFERENCE as libc::c_int as libc::c_uint
            {
                mod_inner = (*sub).u.s_binary.left;
            }
        }
        33 | 34 | 37 | 38 | 28 | 29 | 30 | 31 | 32 | 78 | 80 | 81 => {}
        39 => {
            if options & (1 as libc::c_int) << 2 as libc::c_int == 0 as libc::c_int {
                d_append_buffer(
                    dpi,
                    (*(*dc).u.s_builtin.type_0).name,
                    (*(*dc).u.s_builtin.type_0).len as size_t,
                );
            } else {
                d_append_buffer(
                    dpi,
                    (*(*dc).u.s_builtin.type_0).java_name,
                    (*(*dc).u.s_builtin.type_0).java_len as size_t,
                );
            }
            return;
        }
        40 => {
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        41 => {
            if options & (1 as libc::c_int) << 5 as libc::c_int != 0 as libc::c_int {
                d_print_function_type(
                    dpi,
                    options
                        & !((1 as libc::c_int) << 5 as libc::c_int
                            | (1 as libc::c_int) << 6 as libc::c_int),
                    dc,
                    (*dpi).modifiers,
                );
            }
            if !((*dc).u.s_binary.left).is_null()
                && options & (1 as libc::c_int) << 5 as libc::c_int != 0 as libc::c_int
            {
                d_print_comp(
                    dpi,
                    options
                        & !((1 as libc::c_int) << 5 as libc::c_int
                            | (1 as libc::c_int) << 6 as libc::c_int),
                    (*dc).u.s_binary.left,
                );
            } else if !((*dc).u.s_binary.left).is_null()
                && options & (1 as libc::c_int) << 6 as libc::c_int == 0 as libc::c_int
            {
                let mut dpm_0: d_print_mod = d_print_mod {
                    next: 0 as *mut d_print_mod,
                    mod_0: 0 as *mut demangle_component,
                    printed: 0,
                    templates: 0 as *mut d_print_template,
                };
                dpm_0.next = (*dpi).modifiers;
                (*dpi).modifiers = &mut dpm_0;
                dpm_0.mod_0 = dc;
                dpm_0.printed = 0 as libc::c_int;
                dpm_0.templates = (*dpi).templates;
                d_print_comp(
                    dpi,
                    options
                        & !((1 as libc::c_int) << 5 as libc::c_int
                            | (1 as libc::c_int) << 6 as libc::c_int),
                    (*dc).u.s_binary.left,
                );
                (*dpi).modifiers = dpm_0.next;
                if dpm_0.printed != 0 {
                    return;
                }
                if options & (1 as libc::c_int) << 5 as libc::c_int == 0 as libc::c_int {
                    d_append_char(dpi, ' ' as i32 as libc::c_char);
                }
            }
            if options & (1 as libc::c_int) << 5 as libc::c_int == 0 as libc::c_int {
                d_print_function_type(
                    dpi,
                    options
                        & !((1 as libc::c_int) << 5 as libc::c_int
                            | (1 as libc::c_int) << 6 as libc::c_int),
                    dc,
                    (*dpi).modifiers,
                );
            }
            return;
        }
        42 => {
            let mut hold_modifiers_0: *mut d_print_mod = 0 as *mut d_print_mod;
            let mut adpm_0: [d_print_mod; 4] = [d_print_mod {
                next: 0 as *mut d_print_mod,
                mod_0: 0 as *mut demangle_component,
                printed: 0,
                templates: 0 as *mut d_print_template,
            }; 4];
            let mut i_0: libc::c_uint = 0;
            let mut pdpm_0: *mut d_print_mod = 0 as *mut d_print_mod;
            hold_modifiers_0 = (*dpi).modifiers;
            adpm_0[0 as libc::c_int as usize].next = hold_modifiers_0;
            (*dpi)
                .modifiers = &mut *adpm_0.as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut d_print_mod;
            adpm_0[0 as libc::c_int as usize].mod_0 = dc;
            adpm_0[0 as libc::c_int as usize].printed = 0 as libc::c_int;
            adpm_0[0 as libc::c_int as usize].templates = (*dpi).templates;
            i_0 = 1 as libc::c_int as libc::c_uint;
            pdpm_0 = hold_modifiers_0;
            while !pdpm_0.is_null()
                && ((*(*pdpm_0).mod_0).type_0 as libc::c_uint
                    == DEMANGLE_COMPONENT_RESTRICT as libc::c_int as libc::c_uint
                    || (*(*pdpm_0).mod_0).type_0 as libc::c_uint
                        == DEMANGLE_COMPONENT_VOLATILE as libc::c_int as libc::c_uint
                    || (*(*pdpm_0).mod_0).type_0 as libc::c_uint
                        == DEMANGLE_COMPONENT_CONST as libc::c_int as libc::c_uint)
            {
                if (*pdpm_0).printed == 0 {
                    if i_0 as libc::c_ulong
                        >= (::core::mem::size_of::<[d_print_mod; 4]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<d_print_mod>() as libc::c_ulong,
                            )
                    {
                        d_print_error(dpi);
                        return;
                    }
                    adpm_0[i_0 as usize] = *pdpm_0;
                    adpm_0[i_0 as usize].next = (*dpi).modifiers;
                    (*dpi)
                        .modifiers = &mut *adpm_0.as_mut_ptr().offset(i_0 as isize)
                        as *mut d_print_mod;
                    (*pdpm_0).printed = 1 as libc::c_int;
                    i_0 = i_0.wrapping_add(1);
                    i_0;
                }
                pdpm_0 = (*pdpm_0).next;
            }
            d_print_comp(dpi, options, (*dc).u.s_binary.right);
            (*dpi).modifiers = hold_modifiers_0;
            if adpm_0[0 as libc::c_int as usize].printed != 0 {
                return;
            }
            while i_0 > 1 as libc::c_int as libc::c_uint {
                i_0 = i_0.wrapping_sub(1);
                i_0;
                d_print_mod(dpi, options, adpm_0[i_0 as usize].mod_0);
            }
            d_print_array_type(dpi, options, dc, (*dpi).modifiers);
            return;
        }
        43 | 45 => {
            let mut dpm_1: d_print_mod = d_print_mod {
                next: 0 as *mut d_print_mod,
                mod_0: 0 as *mut demangle_component,
                printed: 0,
                templates: 0 as *mut d_print_template,
            };
            dpm_1.next = (*dpi).modifiers;
            (*dpi).modifiers = &mut dpm_1;
            dpm_1.mod_0 = dc;
            dpm_1.printed = 0 as libc::c_int;
            dpm_1.templates = (*dpi).templates;
            d_print_comp(dpi, options, (*dc).u.s_binary.right);
            if dpm_1.printed == 0 {
                d_print_mod(dpi, options, dc);
            }
            (*dpi).modifiers = dpm_1.next;
            return;
        }
        44 => {
            if (*dc).u.s_fixed.sat != 0 {
                d_append_string(dpi, b"_Sat \0" as *const u8 as *const libc::c_char);
            }
            if (*(*dc).u.s_fixed.length).u.s_builtin.type_0
                != &*cplus_demangle_builtin_types
                    .as_ptr()
                    .offset(('i' as i32 - 'a' as i32) as isize)
                    as *const demangle_builtin_type_info
            {
                d_print_comp(dpi, options, (*dc).u.s_fixed.length);
                d_append_char(dpi, ' ' as i32 as libc::c_char);
            }
            if (*dc).u.s_fixed.accum != 0 {
                d_append_string(dpi, b"_Accum\0" as *const u8 as *const libc::c_char);
            } else {
                d_append_string(dpi, b"_Fract\0" as *const u8 as *const libc::c_char);
            }
            return;
        }
        46 | 47 => {
            if !((*dc).u.s_binary.left).is_null() {
                d_print_comp(dpi, options, (*dc).u.s_binary.left);
            }
            if !((*dc).u.s_binary.right).is_null() {
                let mut len: size_t = 0;
                let mut flush_count: libc::c_ulong = 0;
                if (*dpi).len
                    >= (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                {
                    d_print_flush(dpi);
                }
                d_append_string(dpi, b", \0" as *const u8 as *const libc::c_char);
                len = (*dpi).len;
                flush_count = (*dpi).flush_count;
                d_print_comp(dpi, options, (*dc).u.s_binary.right);
                if (*dpi).flush_count == flush_count && (*dpi).len == len {
                    (*dpi)
                        .len = ((*dpi).len as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
            }
            return;
        }
        49 => {
            let mut type_0: *mut demangle_component = (*dc).u.s_binary.left;
            let mut list: *mut demangle_component = (*dc).u.s_binary.right;
            if !type_0.is_null() {
                d_print_comp(dpi, options, type_0);
            }
            d_append_char(dpi, '{' as i32 as libc::c_char);
            d_print_comp(dpi, options, list);
            d_append_char(dpi, '}' as i32 as libc::c_char);
            return;
        }
        50 => {
            let mut op: *const demangle_operator_info = (*dc).u.s_operator.op;
            let mut len_0: libc::c_int = (*op).len;
            d_append_string(dpi, b"operator\0" as *const u8 as *const libc::c_char);
            if *((*op).name).offset(0 as libc::c_int as isize) as libc::c_int
                >= 'a' as i32
                && *((*op).name).offset(0 as libc::c_int as isize) as libc::c_int
                    <= 'z' as i32
            {
                d_append_char(dpi, ' ' as i32 as libc::c_char);
            }
            if *((*op).name).offset((len_0 - 1 as libc::c_int) as isize) as libc::c_int
                == ' ' as i32
            {
                len_0 -= 1;
                len_0;
            }
            d_append_buffer(dpi, (*op).name, len_0 as size_t);
            return;
        }
        51 => {
            d_append_string(dpi, b"operator \0" as *const u8 as *const libc::c_char);
            d_print_comp(dpi, options, (*dc).u.s_extended_operator.name);
            return;
        }
        53 => {
            d_append_string(dpi, b"operator \0" as *const u8 as *const libc::c_char);
            d_print_conversion(dpi, options, dc);
            return;
        }
        54 => {
            d_print_expr_op(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        55 => {
            let mut op_0: *mut demangle_component = (*dc).u.s_binary.left;
            let mut operand: *mut demangle_component = (*dc).u.s_binary.right;
            let mut code: *const libc::c_char = 0 as *const libc::c_char;
            if (*op_0).type_0 as libc::c_uint
                == DEMANGLE_COMPONENT_OPERATOR as libc::c_int as libc::c_uint
            {
                code = (*(*op_0).u.s_operator.op).code;
                if strcmp(code, b"ad\0" as *const u8 as *const libc::c_char) == 0 {
                    if (*operand).type_0 as libc::c_uint
                        == DEMANGLE_COMPONENT_TYPED_NAME as libc::c_int as libc::c_uint
                        && (*(*operand).u.s_binary.left).type_0 as libc::c_uint
                            == DEMANGLE_COMPONENT_QUAL_NAME as libc::c_int
                                as libc::c_uint
                        && (*(*operand).u.s_binary.right).type_0 as libc::c_uint
                            == DEMANGLE_COMPONENT_FUNCTION_TYPE as libc::c_int
                                as libc::c_uint
                    {
                        operand = (*operand).u.s_binary.left;
                    }
                }
                if (*operand).type_0 as libc::c_uint
                    == DEMANGLE_COMPONENT_BINARY_ARGS as libc::c_int as libc::c_uint
                {
                    operand = (*operand).u.s_binary.left;
                    d_print_subexpr(dpi, options, operand);
                    d_print_expr_op(dpi, options, op_0);
                    return;
                }
            }
            if !code.is_null()
                && strcmp(code, b"sZ\0" as *const u8 as *const libc::c_char) == 0
            {
                let mut a_1: *mut demangle_component = d_find_pack(dpi, operand);
                let mut len_1: libc::c_int = d_pack_length(a_1);
                d_append_num(dpi, len_1);
                return;
            } else if !code.is_null()
                && strcmp(code, b"sP\0" as *const u8 as *const libc::c_char) == 0
            {
                let mut len_2: libc::c_int = d_args_length(dpi, operand);
                d_append_num(dpi, len_2);
                return;
            }
            if (*op_0).type_0 as libc::c_uint
                != DEMANGLE_COMPONENT_CAST as libc::c_int as libc::c_uint
            {
                d_print_expr_op(dpi, options, op_0);
            } else {
                d_append_char(dpi, '(' as i32 as libc::c_char);
                d_print_cast(dpi, options, op_0);
                d_append_char(dpi, ')' as i32 as libc::c_char);
            }
            if !code.is_null()
                && strcmp(code, b"gs\0" as *const u8 as *const libc::c_char) == 0
            {
                d_print_comp(dpi, options, operand);
            } else if !code.is_null()
                && strcmp(code, b"st\0" as *const u8 as *const libc::c_char) == 0
            {
                d_append_char(dpi, '(' as i32 as libc::c_char);
                d_print_comp(dpi, options, operand);
                d_append_char(dpi, ')' as i32 as libc::c_char);
            } else {
                d_print_subexpr(dpi, options, operand);
            }
            return;
        }
        56 => {
            if (*(*dc).u.s_binary.right).type_0 as libc::c_uint
                != DEMANGLE_COMPONENT_BINARY_ARGS as libc::c_int as libc::c_uint
            {
                d_print_error(dpi);
                return;
            }
            if op_is_new_cast((*dc).u.s_binary.left) != 0 {
                d_print_expr_op(dpi, options, (*dc).u.s_binary.left);
                d_append_char(dpi, '<' as i32 as libc::c_char);
                d_print_comp(dpi, options, (*(*dc).u.s_binary.right).u.s_binary.left);
                d_append_string(dpi, b">(\0" as *const u8 as *const libc::c_char);
                d_print_comp(dpi, options, (*(*dc).u.s_binary.right).u.s_binary.right);
                d_append_char(dpi, ')' as i32 as libc::c_char);
                return;
            }
            if d_maybe_print_fold_expression(dpi, options, dc) != 0 {
                return;
            }
            if d_maybe_print_designated_init(dpi, options, dc) != 0 {
                return;
            }
            if (*(*dc).u.s_binary.left).type_0 as libc::c_uint
                == DEMANGLE_COMPONENT_OPERATOR as libc::c_int as libc::c_uint
                && (*(*(*dc).u.s_binary.left).u.s_operator.op).len == 1 as libc::c_int
                && *((*(*(*dc).u.s_binary.left).u.s_operator.op).name)
                    .offset(0 as libc::c_int as isize) as libc::c_int == '>' as i32
            {
                d_append_char(dpi, '(' as i32 as libc::c_char);
            }
            if strcmp(
                (*(*(*dc).u.s_binary.left).u.s_operator.op).code,
                b"cl\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                && (*(*(*dc).u.s_binary.right).u.s_binary.left).type_0 as libc::c_uint
                    == DEMANGLE_COMPONENT_TYPED_NAME as libc::c_int as libc::c_uint
            {
                let mut func: *const demangle_component = (*(*dc).u.s_binary.right)
                    .u
                    .s_binary
                    .left;
                if (*(*func).u.s_binary.right).type_0 as libc::c_uint
                    != DEMANGLE_COMPONENT_FUNCTION_TYPE as libc::c_int as libc::c_uint
                {
                    d_print_error(dpi);
                }
                d_print_subexpr(dpi, options, (*func).u.s_binary.left);
            } else {
                d_print_subexpr(dpi, options, (*(*dc).u.s_binary.right).u.s_binary.left);
            }
            if strcmp(
                (*(*(*dc).u.s_binary.left).u.s_operator.op).code,
                b"ix\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                d_append_char(dpi, '[' as i32 as libc::c_char);
                d_print_comp(dpi, options, (*(*dc).u.s_binary.right).u.s_binary.right);
                d_append_char(dpi, ']' as i32 as libc::c_char);
            } else {
                if strcmp(
                    (*(*(*dc).u.s_binary.left).u.s_operator.op).code,
                    b"cl\0" as *const u8 as *const libc::c_char,
                ) != 0 as libc::c_int
                {
                    d_print_expr_op(dpi, options, (*dc).u.s_binary.left);
                }
                d_print_subexpr(
                    dpi,
                    options,
                    (*(*dc).u.s_binary.right).u.s_binary.right,
                );
            }
            if (*(*dc).u.s_binary.left).type_0 as libc::c_uint
                == DEMANGLE_COMPONENT_OPERATOR as libc::c_int as libc::c_uint
                && (*(*(*dc).u.s_binary.left).u.s_operator.op).len == 1 as libc::c_int
                && *((*(*(*dc).u.s_binary.left).u.s_operator.op).name)
                    .offset(0 as libc::c_int as isize) as libc::c_int == '>' as i32
            {
                d_append_char(dpi, ')' as i32 as libc::c_char);
            }
            return;
        }
        57 => {
            d_print_error(dpi);
            return;
        }
        58 => {
            if (*(*dc).u.s_binary.right).type_0 as libc::c_uint
                != DEMANGLE_COMPONENT_TRINARY_ARG1 as libc::c_int as libc::c_uint
                || (*(*(*dc).u.s_binary.right).u.s_binary.right).type_0 as libc::c_uint
                    != DEMANGLE_COMPONENT_TRINARY_ARG2 as libc::c_int as libc::c_uint
            {
                d_print_error(dpi);
                return;
            }
            if d_maybe_print_fold_expression(dpi, options, dc) != 0 {
                return;
            }
            if d_maybe_print_designated_init(dpi, options, dc) != 0 {
                return;
            }
            let mut op_1: *mut demangle_component = (*dc).u.s_binary.left;
            let mut first: *mut demangle_component = (*(*dc).u.s_binary.right)
                .u
                .s_binary
                .left;
            let mut second: *mut demangle_component = (*(*(*dc).u.s_binary.right)
                .u
                .s_binary
                .right)
                .u
                .s_binary
                .left;
            let mut third: *mut demangle_component = (*(*(*dc).u.s_binary.right)
                .u
                .s_binary
                .right)
                .u
                .s_binary
                .right;
            if strcmp(
                (*(*op_1).u.s_operator.op).code,
                b"qu\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                d_print_subexpr(dpi, options, first);
                d_print_expr_op(dpi, options, op_1);
                d_print_subexpr(dpi, options, second);
                d_append_string(dpi, b" : \0" as *const u8 as *const libc::c_char);
                d_print_subexpr(dpi, options, third);
            } else {
                d_append_string(dpi, b"new \0" as *const u8 as *const libc::c_char);
                if !((*first).u.s_binary.left).is_null() {
                    d_print_subexpr(dpi, options, first);
                    d_append_char(dpi, ' ' as i32 as libc::c_char);
                }
                d_print_comp(dpi, options, second);
                if !third.is_null() {
                    d_print_subexpr(dpi, options, third);
                }
            }
            return;
        }
        59 | 60 => {
            d_print_error(dpi);
            return;
        }
        61 | 62 => {
            let mut tp: d_builtin_type_print = D_PRINT_DEFAULT;
            tp = D_PRINT_DEFAULT;
            if (*(*dc).u.s_binary.left).type_0 as libc::c_uint
                == DEMANGLE_COMPONENT_BUILTIN_TYPE as libc::c_int as libc::c_uint
            {
                tp = (*(*(*dc).u.s_binary.left).u.s_builtin.type_0).print;
                match tp as libc::c_uint {
                    1 | 2 | 3 | 4 | 5 | 6 => {
                        if (*(*dc).u.s_binary.right).type_0 as libc::c_uint
                            == DEMANGLE_COMPONENT_NAME as libc::c_int as libc::c_uint
                        {
                            if (*dc).type_0 as libc::c_uint
                                == DEMANGLE_COMPONENT_LITERAL_NEG as libc::c_int
                                    as libc::c_uint
                            {
                                d_append_char(dpi, '-' as i32 as libc::c_char);
                            }
                            d_print_comp(dpi, options, (*dc).u.s_binary.right);
                            match tp as libc::c_uint {
                                2 => {
                                    d_append_char(dpi, 'u' as i32 as libc::c_char);
                                }
                                3 => {
                                    d_append_char(dpi, 'l' as i32 as libc::c_char);
                                }
                                4 => {
                                    d_append_string(
                                        dpi,
                                        b"ul\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                5 => {
                                    d_append_string(
                                        dpi,
                                        b"ll\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                6 => {
                                    d_append_string(
                                        dpi,
                                        b"ull\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                _ => {}
                            }
                            return;
                        }
                    }
                    7 => {
                        if (*(*dc).u.s_binary.right).type_0 as libc::c_uint
                            == DEMANGLE_COMPONENT_NAME as libc::c_int as libc::c_uint
                            && (*(*dc).u.s_binary.right).u.s_name.len == 1 as libc::c_int
                            && (*dc).type_0 as libc::c_uint
                                == DEMANGLE_COMPONENT_LITERAL as libc::c_int as libc::c_uint
                        {
                            match *((*(*dc).u.s_binary.right).u.s_name.s)
                                .offset(0 as libc::c_int as isize) as libc::c_int
                            {
                                48 => {
                                    d_append_string(
                                        dpi,
                                        b"false\0" as *const u8 as *const libc::c_char,
                                    );
                                    return;
                                }
                                49 => {
                                    d_append_string(
                                        dpi,
                                        b"true\0" as *const u8 as *const libc::c_char,
                                    );
                                    return;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            d_append_char(dpi, '(' as i32 as libc::c_char);
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            d_append_char(dpi, ')' as i32 as libc::c_char);
            if (*dc).type_0 as libc::c_uint
                == DEMANGLE_COMPONENT_LITERAL_NEG as libc::c_int as libc::c_uint
            {
                d_append_char(dpi, '-' as i32 as libc::c_char);
            }
            if tp as libc::c_uint == D_PRINT_FLOAT as libc::c_int as libc::c_uint {
                d_append_char(dpi, '[' as i32 as libc::c_char);
            }
            d_print_comp(dpi, options, (*dc).u.s_binary.right);
            if tp as libc::c_uint == D_PRINT_FLOAT as libc::c_int as libc::c_uint {
                d_append_char(dpi, ']' as i32 as libc::c_char);
            }
            return;
        }
        63 => {
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            d_append_char(dpi, '(' as i32 as libc::c_char);
            d_print_comp(dpi, options, (*dc).u.s_binary.right);
            d_append_char(dpi, ')' as i32 as libc::c_char);
            return;
        }
        67 => {
            d_append_num(dpi, (*dc).u.s_number.number as libc::c_int);
            return;
        }
        64 => {
            d_append_string(
                dpi,
                b"java resource \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        65 => {
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            d_print_comp(dpi, options, (*dc).u.s_binary.right);
            return;
        }
        66 => {
            d_append_char(dpi, (*dc).u.s_character.character as libc::c_char);
            return;
        }
        68 => {
            d_append_string(dpi, b"decltype (\0" as *const u8 as *const libc::c_char);
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            d_append_char(dpi, ')' as i32 as libc::c_char);
            return;
        }
        76 => {
            let mut len_3: libc::c_int = 0;
            let mut i_1: libc::c_int = 0;
            let mut a_2: *mut demangle_component = d_find_pack(
                dpi,
                (*dc).u.s_binary.left,
            );
            if a_2.is_null() {
                d_print_subexpr(dpi, options, (*dc).u.s_binary.left);
                d_append_string(dpi, b"...\0" as *const u8 as *const libc::c_char);
                return;
            }
            len_3 = d_pack_length(a_2);
            dc = (*dc).u.s_binary.left;
            i_1 = 0 as libc::c_int;
            while i_1 < len_3 {
                (*dpi).pack_index = i_1;
                d_print_comp(dpi, options, dc);
                if i_1 < len_3 - 1 as libc::c_int {
                    d_append_string(dpi, b", \0" as *const u8 as *const libc::c_char);
                }
                i_1 += 1;
                i_1;
            }
            return;
        }
        6 => {
            let mut num: libc::c_long = (*dc).u.s_number.number;
            if num == 0 as libc::c_int as libc::c_long {
                d_append_string(dpi, b"this\0" as *const u8 as *const libc::c_char);
            } else {
                d_append_string(dpi, b"{parm#\0" as *const u8 as *const libc::c_char);
                d_append_num(dpi, num as libc::c_int);
                d_append_char(dpi, '}' as i32 as libc::c_char);
            }
            return;
        }
        69 => {
            d_append_string(
                dpi,
                b"global constructors keyed to \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        70 => {
            d_append_string(
                dpi,
                b"global destructors keyed to \0" as *const u8 as *const libc::c_char,
            );
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            return;
        }
        71 => {
            d_append_string(dpi, b"{lambda(\0" as *const u8 as *const libc::c_char);
            (*dpi).is_lambda_arg += 1;
            (*dpi).is_lambda_arg;
            d_print_comp(dpi, options, (*dc).u.s_unary_num.sub);
            (*dpi).is_lambda_arg -= 1;
            (*dpi).is_lambda_arg;
            d_append_string(dpi, b")#\0" as *const u8 as *const libc::c_char);
            d_append_num(dpi, (*dc).u.s_unary_num.num + 1 as libc::c_int);
            d_append_char(dpi, '}' as i32 as libc::c_char);
            return;
        }
        73 => {
            d_append_string(
                dpi,
                b"{unnamed type#\0" as *const u8 as *const libc::c_char,
            );
            d_append_num(
                dpi,
                ((*dc).u.s_number.number + 1 as libc::c_int as libc::c_long)
                    as libc::c_int,
            );
            d_append_char(dpi, '}' as i32 as libc::c_char);
            return;
        }
        79 => {
            d_print_comp(dpi, options, (*dc).u.s_binary.left);
            d_append_string(dpi, b" [clone \0" as *const u8 as *const libc::c_char);
            d_print_comp(dpi, options, (*dc).u.s_binary.right);
            d_append_char(dpi, ']' as i32 as libc::c_char);
            return;
        }
        _ => {
            d_print_error(dpi);
            return;
        }
    }
    let mut dpm: d_print_mod = d_print_mod {
        next: 0 as *mut d_print_mod,
        mod_0: 0 as *mut demangle_component,
        printed: 0,
        templates: 0 as *mut d_print_template,
    };
    dpm.next = (*dpi).modifiers;
    (*dpi).modifiers = &mut dpm;
    dpm.mod_0 = dc;
    dpm.printed = 0 as libc::c_int;
    dpm.templates = (*dpi).templates;
    if mod_inner.is_null() {
        mod_inner = (*dc).u.s_binary.left;
    }
    d_print_comp(dpi, options, mod_inner);
    if dpm.printed == 0 {
        d_print_mod(dpi, options, dc);
    }
    (*dpi).modifiers = dpm.next;
    if need_template_restore != 0 {
        (*dpi).templates = saved_templates;
    }
}
unsafe extern "C" fn d_print_comp(
    mut dpi: *mut d_print_info,
    mut options: libc::c_int,
    mut dc: *mut demangle_component,
) {
    let mut self_0: d_component_stack = d_component_stack {
        dc: 0 as *const demangle_component,
        parent: 0 as *const d_component_stack,
    };
    if dc.is_null() || (*dc).d_printing > 1 as libc::c_int
        || (*dpi).recursion > 1024 as libc::c_int
    {
        d_print_error(dpi);
        return;
    }
    (*dc).d_printing += 1;
    (*dc).d_printing;
    (*dpi).recursion += 1;
    (*dpi).recursion;
    self_0.dc = dc;
    self_0.parent = (*dpi).component_stack;
    (*dpi).component_stack = &mut self_0;
    d_print_comp_inner(dpi, options, dc);
    (*dpi).component_stack = self_0.parent;
    (*dc).d_printing -= 1;
    (*dc).d_printing;
    (*dpi).recursion -= 1;
    (*dpi).recursion;
}
unsafe extern "C" fn d_print_java_identifier(
    mut dpi: *mut d_print_info,
    mut name: *const libc::c_char,
    mut len: libc::c_int,
) {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    end = name.offset(len as isize);
    let mut current_block_11: u64;
    p = name;
    while p < end {
        if end.offset_from(p) as libc::c_long > 3 as libc::c_int as libc::c_long
            && *p.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == '_' as i32
            && *p.offset(2 as libc::c_int as isize) as libc::c_int == 'U' as i32
        {
            let mut c: libc::c_ulong = 0;
            let mut q: *const libc::c_char = 0 as *const libc::c_char;
            c = 0 as libc::c_int as libc::c_ulong;
            q = p.offset(3 as libc::c_int as isize);
            while q < end {
                let mut dig: libc::c_int = 0;
                if *q as libc::c_int >= '0' as i32 && *q as libc::c_int <= '9' as i32 {
                    dig = *q as libc::c_int - '0' as i32;
                } else if *q as libc::c_int >= 'A' as i32
                    && *q as libc::c_int <= 'F' as i32
                {
                    dig = *q as libc::c_int - 'A' as i32 + 10 as libc::c_int;
                } else {
                    if !(*q as libc::c_int >= 'a' as i32
                        && *q as libc::c_int <= 'f' as i32)
                    {
                        break;
                    }
                    dig = *q as libc::c_int - 'a' as i32 + 10 as libc::c_int;
                }
                c = c
                    .wrapping_mul(16 as libc::c_int as libc::c_ulong)
                    .wrapping_add(dig as libc::c_ulong);
                q = q.offset(1);
                q;
            }
            if q < end && *q as libc::c_int == '_' as i32
                && c < 256 as libc::c_int as libc::c_ulong
            {
                d_append_char(dpi, c as libc::c_char);
                p = q;
                current_block_11 = 16559507199688588974;
            } else {
                current_block_11 = 4956146061682418353;
            }
        } else {
            current_block_11 = 4956146061682418353;
        }
        match current_block_11 {
            4956146061682418353 => {
                d_append_char(dpi, *p);
            }
            _ => {}
        }
        p = p.offset(1);
        p;
    }
}
unsafe extern "C" fn d_print_mod_list(
    mut dpi: *mut d_print_info,
    mut options: libc::c_int,
    mut mods: *mut d_print_mod,
    mut suffix: libc::c_int,
) {
    let mut hold_dpt: *mut d_print_template = 0 as *mut d_print_template;
    if mods.is_null() || d_print_saw_error(dpi) != 0 {
        return;
    }
    if (*mods).printed != 0
        || suffix == 0 && is_fnqual_component_type((*(*mods).mod_0).type_0) != 0
    {
        d_print_mod_list(dpi, options, (*mods).next, suffix);
        return;
    }
    (*mods).printed = 1 as libc::c_int;
    hold_dpt = (*dpi).templates;
    (*dpi).templates = (*mods).templates;
    if (*(*mods).mod_0).type_0 as libc::c_uint
        == DEMANGLE_COMPONENT_FUNCTION_TYPE as libc::c_int as libc::c_uint
    {
        d_print_function_type(dpi, options, (*mods).mod_0, (*mods).next);
        (*dpi).templates = hold_dpt;
        return;
    } else if (*(*mods).mod_0).type_0 as libc::c_uint
        == DEMANGLE_COMPONENT_ARRAY_TYPE as libc::c_int as libc::c_uint
    {
        d_print_array_type(dpi, options, (*mods).mod_0, (*mods).next);
        (*dpi).templates = hold_dpt;
        return;
    } else if (*(*mods).mod_0).type_0 as libc::c_uint
        == DEMANGLE_COMPONENT_LOCAL_NAME as libc::c_int as libc::c_uint
    {
        let mut hold_modifiers: *mut d_print_mod = 0 as *mut d_print_mod;
        let mut dc: *mut demangle_component = 0 as *mut demangle_component;
        hold_modifiers = (*dpi).modifiers;
        (*dpi).modifiers = 0 as *mut d_print_mod;
        d_print_comp(dpi, options, (*(*mods).mod_0).u.s_binary.left);
        (*dpi).modifiers = hold_modifiers;
        if options & (1 as libc::c_int) << 2 as libc::c_int == 0 as libc::c_int {
            d_append_string(dpi, b"::\0" as *const u8 as *const libc::c_char);
        } else {
            d_append_char(dpi, '.' as i32 as libc::c_char);
        }
        dc = (*(*mods).mod_0).u.s_binary.right;
        if (*dc).type_0 as libc::c_uint
            == DEMANGLE_COMPONENT_DEFAULT_ARG as libc::c_int as libc::c_uint
        {
            d_append_string(dpi, b"{default arg#\0" as *const u8 as *const libc::c_char);
            d_append_num(dpi, (*dc).u.s_unary_num.num + 1 as libc::c_int);
            d_append_string(dpi, b"}::\0" as *const u8 as *const libc::c_char);
            dc = (*dc).u.s_unary_num.sub;
        }
        while is_fnqual_component_type((*dc).type_0) != 0 {
            dc = (*dc).u.s_binary.left;
        }
        d_print_comp(dpi, options, dc);
        (*dpi).templates = hold_dpt;
        return;
    }
    d_print_mod(dpi, options, (*mods).mod_0);
    (*dpi).templates = hold_dpt;
    d_print_mod_list(dpi, options, (*mods).next, suffix);
}
unsafe extern "C" fn d_print_mod(
    mut dpi: *mut d_print_info,
    mut options: libc::c_int,
    mut mod_0: *mut demangle_component,
) {
    let mut current_block_50: u64;
    match (*mod_0).type_0 as libc::c_uint {
        25 | 28 => {
            d_append_string(dpi, b" restrict\0" as *const u8 as *const libc::c_char);
            return;
        }
        26 | 29 => {
            d_append_string(dpi, b" volatile\0" as *const u8 as *const libc::c_char);
            return;
        }
        27 | 30 => {
            d_append_string(dpi, b" const\0" as *const u8 as *const libc::c_char);
            return;
        }
        78 => {
            d_append_string(
                dpi,
                b" transaction_safe\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        80 => {
            d_append_string(dpi, b" noexcept\0" as *const u8 as *const libc::c_char);
            if !((*mod_0).u.s_binary.right).is_null() {
                d_append_char(dpi, '(' as i32 as libc::c_char);
                d_print_comp(dpi, options, (*mod_0).u.s_binary.right);
                d_append_char(dpi, ')' as i32 as libc::c_char);
            }
            return;
        }
        81 => {
            d_append_string(dpi, b" throw\0" as *const u8 as *const libc::c_char);
            if !((*mod_0).u.s_binary.right).is_null() {
                d_append_char(dpi, '(' as i32 as libc::c_char);
                d_print_comp(dpi, options, (*mod_0).u.s_binary.right);
                d_append_char(dpi, ')' as i32 as libc::c_char);
            }
            return;
        }
        33 => {
            d_append_char(dpi, ' ' as i32 as libc::c_char);
            d_print_comp(dpi, options, (*mod_0).u.s_binary.right);
            return;
        }
        34 => {
            if options & (1 as libc::c_int) << 2 as libc::c_int == 0 as libc::c_int {
                d_append_char(dpi, '*' as i32 as libc::c_char);
            }
            return;
        }
        31 => {
            d_append_char(dpi, ' ' as i32 as libc::c_char);
            current_block_50 = 13580471147157397510;
        }
        35 => {
            current_block_50 = 13580471147157397510;
        }
        32 => {
            d_append_char(dpi, ' ' as i32 as libc::c_char);
            current_block_50 = 8433652531900264989;
        }
        36 => {
            current_block_50 = 8433652531900264989;
        }
        37 => {
            d_append_string(dpi, b" _Complex\0" as *const u8 as *const libc::c_char);
            return;
        }
        38 => {
            d_append_string(dpi, b" _Imaginary\0" as *const u8 as *const libc::c_char);
            return;
        }
        43 => {
            if d_last_char(dpi) as libc::c_int != '(' as i32 {
                d_append_char(dpi, ' ' as i32 as libc::c_char);
            }
            d_print_comp(dpi, options, (*mod_0).u.s_binary.left);
            d_append_string(dpi, b"::*\0" as *const u8 as *const libc::c_char);
            return;
        }
        3 => {
            d_print_comp(dpi, options, (*mod_0).u.s_binary.left);
            return;
        }
        45 => {
            d_append_string(dpi, b" __vector(\0" as *const u8 as *const libc::c_char);
            d_print_comp(dpi, options, (*mod_0).u.s_binary.left);
            d_append_char(dpi, ')' as i32 as libc::c_char);
            return;
        }
        _ => {
            d_print_comp(dpi, options, mod_0);
            return;
        }
    }
    match current_block_50 {
        13580471147157397510 => {
            d_append_char(dpi, '&' as i32 as libc::c_char);
            return;
        }
        _ => {
            d_append_string(dpi, b"&&\0" as *const u8 as *const libc::c_char);
            return;
        }
    };
}
unsafe extern "C" fn d_print_function_type(
    mut dpi: *mut d_print_info,
    mut options: libc::c_int,
    mut dc: *mut demangle_component,
    mut mods: *mut d_print_mod,
) {
    let mut need_paren: libc::c_int = 0;
    let mut need_space: libc::c_int = 0;
    let mut p: *mut d_print_mod = 0 as *mut d_print_mod;
    let mut hold_modifiers: *mut d_print_mod = 0 as *mut d_print_mod;
    need_paren = 0 as libc::c_int;
    need_space = 0 as libc::c_int;
    p = mods;
    while !p.is_null() {
        if (*p).printed != 0 {
            break;
        }
        match (*(*p).mod_0).type_0 as libc::c_uint {
            34 | 35 | 36 => {
                need_paren = 1 as libc::c_int;
            }
            25 | 26 | 27 | 33 | 37 | 38 | 43 => {
                need_space = 1 as libc::c_int;
                need_paren = 1 as libc::c_int;
            }
            28 | 29 | 30 | 31 | 32 | 78 | 80 | 81 | _ => {}
        }
        if need_paren != 0 {
            break;
        }
        p = (*p).next;
    }
    if need_paren != 0 {
        if need_space == 0 {
            if d_last_char(dpi) as libc::c_int != '(' as i32
                && d_last_char(dpi) as libc::c_int != '*' as i32
            {
                need_space = 1 as libc::c_int;
            }
        }
        if need_space != 0 && d_last_char(dpi) as libc::c_int != ' ' as i32 {
            d_append_char(dpi, ' ' as i32 as libc::c_char);
        }
        d_append_char(dpi, '(' as i32 as libc::c_char);
    }
    hold_modifiers = (*dpi).modifiers;
    (*dpi).modifiers = 0 as *mut d_print_mod;
    d_print_mod_list(dpi, options, mods, 0 as libc::c_int);
    if need_paren != 0 {
        d_append_char(dpi, ')' as i32 as libc::c_char);
    }
    d_append_char(dpi, '(' as i32 as libc::c_char);
    if !((*dc).u.s_binary.right).is_null() {
        d_print_comp(dpi, options, (*dc).u.s_binary.right);
    }
    d_append_char(dpi, ')' as i32 as libc::c_char);
    d_print_mod_list(dpi, options, mods, 1 as libc::c_int);
    (*dpi).modifiers = hold_modifiers;
}
unsafe extern "C" fn d_print_array_type(
    mut dpi: *mut d_print_info,
    mut options: libc::c_int,
    mut dc: *mut demangle_component,
    mut mods: *mut d_print_mod,
) {
    let mut need_space: libc::c_int = 0;
    need_space = 1 as libc::c_int;
    if !mods.is_null() {
        let mut need_paren: libc::c_int = 0;
        let mut p: *mut d_print_mod = 0 as *mut d_print_mod;
        need_paren = 0 as libc::c_int;
        p = mods;
        while !p.is_null() {
            if (*p).printed == 0 {
                if (*(*p).mod_0).type_0 as libc::c_uint
                    == DEMANGLE_COMPONENT_ARRAY_TYPE as libc::c_int as libc::c_uint
                {
                    need_space = 0 as libc::c_int;
                    break;
                } else {
                    need_paren = 1 as libc::c_int;
                    need_space = 1 as libc::c_int;
                    break;
                }
            } else {
                p = (*p).next;
            }
        }
        if need_paren != 0 {
            d_append_string(dpi, b" (\0" as *const u8 as *const libc::c_char);
        }
        d_print_mod_list(dpi, options, mods, 0 as libc::c_int);
        if need_paren != 0 {
            d_append_char(dpi, ')' as i32 as libc::c_char);
        }
    }
    if need_space != 0 {
        d_append_char(dpi, ' ' as i32 as libc::c_char);
    }
    d_append_char(dpi, '[' as i32 as libc::c_char);
    if !((*dc).u.s_binary.left).is_null() {
        d_print_comp(dpi, options, (*dc).u.s_binary.left);
    }
    d_append_char(dpi, ']' as i32 as libc::c_char);
}
unsafe extern "C" fn d_print_expr_op(
    mut dpi: *mut d_print_info,
    mut options: libc::c_int,
    mut dc: *mut demangle_component,
) {
    if (*dc).type_0 as libc::c_uint
        == DEMANGLE_COMPONENT_OPERATOR as libc::c_int as libc::c_uint
    {
        d_append_buffer(
            dpi,
            (*(*dc).u.s_operator.op).name,
            (*(*dc).u.s_operator.op).len as size_t,
        );
    } else {
        d_print_comp(dpi, options, dc);
    };
}
unsafe extern "C" fn d_print_cast(
    mut dpi: *mut d_print_info,
    mut options: libc::c_int,
    mut dc: *mut demangle_component,
) {
    d_print_comp(dpi, options, (*dc).u.s_binary.left);
}
unsafe extern "C" fn d_print_conversion(
    mut dpi: *mut d_print_info,
    mut options: libc::c_int,
    mut dc: *mut demangle_component,
) {
    let mut dpt: d_print_template = d_print_template {
        next: 0 as *mut d_print_template,
        template_decl: 0 as *const demangle_component,
    };
    if !((*dpi).current_template).is_null() {
        dpt.next = (*dpi).templates;
        (*dpi).templates = &mut dpt;
        dpt.template_decl = (*dpi).current_template;
    }
    if (*(*dc).u.s_binary.left).type_0 as libc::c_uint
        != DEMANGLE_COMPONENT_TEMPLATE as libc::c_int as libc::c_uint
    {
        d_print_comp(dpi, options, (*dc).u.s_binary.left);
        if !((*dpi).current_template).is_null() {
            (*dpi).templates = dpt.next;
        }
    } else {
        d_print_comp(dpi, options, (*(*dc).u.s_binary.left).u.s_binary.left);
        if !((*dpi).current_template).is_null() {
            (*dpi).templates = dpt.next;
        }
        if d_last_char(dpi) as libc::c_int == '<' as i32 {
            d_append_char(dpi, ' ' as i32 as libc::c_char);
        }
        d_append_char(dpi, '<' as i32 as libc::c_char);
        d_print_comp(dpi, options, (*(*dc).u.s_binary.left).u.s_binary.right);
        if d_last_char(dpi) as libc::c_int == '>' as i32 {
            d_append_char(dpi, ' ' as i32 as libc::c_char);
        }
        d_append_char(dpi, '>' as i32 as libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_init_info(
    mut mangled: *const libc::c_char,
    mut options: libc::c_int,
    mut len: size_t,
    mut di: *mut d_info,
) {
    (*di).s = mangled;
    (*di).send = mangled.offset(len as isize);
    (*di).options = options;
    (*di).n = mangled;
    (*di)
        .num_comps = (2 as libc::c_int as libc::c_ulong).wrapping_mul(len)
        as libc::c_int;
    (*di).next_comp = 0 as libc::c_int;
    (*di).num_subs = len as libc::c_int;
    (*di).next_sub = 0 as libc::c_int;
    (*di).last_name = 0 as *mut demangle_component;
    (*di).expansion = 0 as libc::c_int;
    (*di).is_expression = 0 as libc::c_int;
    (*di).is_conversion = 0 as libc::c_int;
    (*di).recursion_level = 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn d_demangle_callback(
    mut mangled: *const libc::c_char,
    mut options: libc::c_int,
    mut callback: demangle_callbackref,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    let mut comps: Vec::<demangle_component>;
    let mut subs: Vec::<*mut demangle_component>;
    let mut type_0: C2RustUnnamed_12 = DCT_TYPE;
    let mut di: d_info = d_info {
        s: 0 as *const libc::c_char,
        send: 0 as *const libc::c_char,
        options: 0,
        n: 0 as *const libc::c_char,
        comps: 0 as *mut demangle_component,
        next_comp: 0,
        num_comps: 0,
        subs: 0 as *mut *mut demangle_component,
        next_sub: 0,
        num_subs: 0,
        last_name: 0 as *mut demangle_component,
        expansion: 0,
        is_expression: 0,
        is_conversion: 0,
        unresolved_name_state: 0,
        recursion_level: 0,
    };
    let mut dc: *mut demangle_component = 0 as *mut demangle_component;
    let mut status: libc::c_int = 0;
    if *mangled.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
        && *mangled.offset(1 as libc::c_int as isize) as libc::c_int == 'Z' as i32
    {
        type_0 = DCT_MANGLED;
    } else if strncmp(
        mangled,
        b"_GLOBAL_\0" as *const u8 as *const libc::c_char,
        8 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        && (*mangled.offset(8 as libc::c_int as isize) as libc::c_int == '.' as i32
            || *mangled.offset(8 as libc::c_int as isize) as libc::c_int == '_' as i32
            || *mangled.offset(8 as libc::c_int as isize) as libc::c_int == '$' as i32)
        && (*mangled.offset(9 as libc::c_int as isize) as libc::c_int == 'D' as i32
            || *mangled.offset(9 as libc::c_int as isize) as libc::c_int == 'I' as i32)
        && *mangled.offset(10 as libc::c_int as isize) as libc::c_int == '_' as i32
    {
        type_0 = (if *mangled.offset(9 as libc::c_int as isize) as libc::c_int
            == 'I' as i32
        {
            DCT_GLOBAL_CTORS as libc::c_int
        } else {
            DCT_GLOBAL_DTORS as libc::c_int
        }) as C2RustUnnamed_12;
    } else {
        if options & (1 as libc::c_int) << 4 as libc::c_int == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        type_0 = DCT_TYPE;
    }
    di.unresolved_name_state = 1 as libc::c_int;
    loop {
        cplus_demangle_init_info(mangled, options, strlen(mangled), &mut di);
        if options & (1 as libc::c_int) << 18 as libc::c_int == 0 as libc::c_int
            && di.num_comps as libc::c_ulong > 2048 as libc::c_int as libc::c_ulong
        {
            return 0 as libc::c_int;
        }
        let vla = di.num_comps as usize;
        comps = ::std::vec::from_elem(
            demangle_component {
                type_0: DEMANGLE_COMPONENT_NAME,
                d_printing: 0,
                d_counting: 0,
                u: C2RustUnnamed {
                    s_name: C2RustUnnamed_11 {
                        s: 0 as *const libc::c_char,
                        len: 0,
                    },
                },
            },
            vla,
        );
        let vla_0 = di.num_subs as usize;
        subs = ::std::vec::from_elem(0 as *mut demangle_component, vla_0);
        di.comps = comps.as_mut_ptr();
        di.subs = subs.as_mut_ptr();
        match type_0 as libc::c_uint {
            0 => {
                dc = cplus_demangle_type(&mut di);
            }
            1 => {
                dc = cplus_demangle_mangled_name(&mut di, 1 as libc::c_int);
            }
            2 | 3 => {
                di.n = (di.n).offset(11 as libc::c_int as isize);
                dc = d_make_comp(
                    &mut di,
                    (if type_0 as libc::c_uint
                        == DCT_GLOBAL_CTORS as libc::c_int as libc::c_uint
                    {
                        DEMANGLE_COMPONENT_GLOBAL_CONSTRUCTORS as libc::c_int
                    } else {
                        DEMANGLE_COMPONENT_GLOBAL_DESTRUCTORS as libc::c_int
                    }) as demangle_component_type,
                    d_make_demangle_mangled_name(&mut di, di.n),
                    0 as *mut demangle_component,
                );
                di.n = (di.n).offset(strlen(di.n) as isize);
            }
            _ => {
                abort();
            }
        }
        if options & (1 as libc::c_int) << 0 as libc::c_int != 0 as libc::c_int
            && *di.n as libc::c_int != '\0' as i32
        {
            dc = 0 as *mut demangle_component;
        }
        if !(dc.is_null() && di.unresolved_name_state == -(1 as libc::c_int)) {
            break;
        }
        di.unresolved_name_state = 0 as libc::c_int;
    }
    status = if !dc.is_null() {
        cplus_demangle_print_callback(options, dc, callback, opaque)
    } else {
        0 as libc::c_int
    };
    return status;
}
unsafe extern "C" fn d_demangle(
    mut mangled: *const libc::c_char,
    mut options: libc::c_int,
    mut palc: *mut size_t,
) -> *mut libc::c_char {
    let mut dgs: d_growable_string = d_growable_string {
        buf: 0 as *mut libc::c_char,
        len: 0,
        alc: 0,
        allocation_failure: 0,
    };
    let mut status: libc::c_int = 0;
    d_growable_string_init(&mut dgs, 0 as libc::c_int as size_t);
    status = d_demangle_callback(
        mangled,
        options,
        Some(
            d_growable_string_callback_adapter
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    size_t,
                    *mut libc::c_void,
                ) -> (),
        ),
        &mut dgs as *mut d_growable_string as *mut libc::c_void,
    );
    if status == 0 as libc::c_int {
        free(dgs.buf as *mut libc::c_void);
        *palc = 0 as libc::c_int as size_t;
        return 0 as *mut libc::c_char;
    }
    *palc = if dgs.allocation_failure != 0 {
        1 as libc::c_int as libc::c_ulong
    } else {
        dgs.alc
    };
    return dgs.buf;
}
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_v3(
    mut mangled: *const libc::c_char,
    mut options: libc::c_int,
) -> *mut libc::c_char {
    let mut alc: size_t = 0;
    return d_demangle(mangled, options, &mut alc);
}
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_v3_callback(
    mut mangled: *const libc::c_char,
    mut options: libc::c_int,
    mut callback: demangle_callbackref,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    return d_demangle_callback(mangled, options, callback, opaque);
}
#[no_mangle]
pub unsafe extern "C" fn java_demangle_v3(
    mut mangled: *const libc::c_char,
) -> *mut libc::c_char {
    let mut alc: size_t = 0;
    return d_demangle(
        mangled,
        (1 as libc::c_int) << 2 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 5 as libc::c_int,
        &mut alc,
    );
}
#[no_mangle]
pub unsafe extern "C" fn java_demangle_v3_callback(
    mut mangled: *const libc::c_char,
    mut callback: demangle_callbackref,
    mut opaque: *mut libc::c_void,
) -> libc::c_int {
    return d_demangle_callback(
        mangled,
        (1 as libc::c_int) << 2 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 5 as libc::c_int,
        callback,
        opaque,
    );
}
unsafe extern "C" fn is_ctor_or_dtor(
    mut mangled: *const libc::c_char,
    mut ctor_kind: *mut gnu_v3_ctor_kinds,
    mut dtor_kind: *mut gnu_v3_dtor_kinds,
) -> libc::c_int {
    let mut di: d_info = d_info {
        s: 0 as *const libc::c_char,
        send: 0 as *const libc::c_char,
        options: 0,
        n: 0 as *const libc::c_char,
        comps: 0 as *mut demangle_component,
        next_comp: 0,
        num_comps: 0,
        subs: 0 as *mut *mut demangle_component,
        next_sub: 0,
        num_subs: 0,
        last_name: 0 as *mut demangle_component,
        expansion: 0,
        is_expression: 0,
        is_conversion: 0,
        unresolved_name_state: 0,
        recursion_level: 0,
    };
    let mut dc: *mut demangle_component = 0 as *mut demangle_component;
    let mut ret: libc::c_int = 0;
    *ctor_kind = 0 as gnu_v3_ctor_kinds;
    *dtor_kind = 0 as gnu_v3_dtor_kinds;
    cplus_demangle_init_info(
        mangled,
        (1 as libc::c_int) << 14 as libc::c_int,
        strlen(mangled),
        &mut di,
    );
    let vla = di.num_comps as usize;
    let mut comps: Vec::<demangle_component> = ::std::vec::from_elem(
        demangle_component {
            type_0: DEMANGLE_COMPONENT_NAME,
            d_printing: 0,
            d_counting: 0,
            u: C2RustUnnamed {
                s_name: C2RustUnnamed_11 {
                    s: 0 as *const libc::c_char,
                    len: 0,
                },
            },
        },
        vla,
    );
    let vla_0 = di.num_subs as usize;
    let mut subs: Vec::<*mut demangle_component> = ::std::vec::from_elem(
        0 as *mut demangle_component,
        vla_0,
    );
    di.comps = comps.as_mut_ptr();
    di.subs = subs.as_mut_ptr();
    dc = cplus_demangle_mangled_name(&mut di, 1 as libc::c_int);
    ret = 0 as libc::c_int;
    while !dc.is_null() {
        match (*dc).type_0 as libc::c_uint {
            3 | 4 => {
                dc = (*dc).u.s_binary.left;
            }
            1 | 2 => {
                dc = (*dc).u.s_binary.right;
            }
            7 => {
                *ctor_kind = (*dc).u.s_ctor.kind;
                ret = 1 as libc::c_int;
                dc = 0 as *mut demangle_component;
            }
            8 => {
                *dtor_kind = (*dc).u.s_dtor.kind;
                ret = 1 as libc::c_int;
                dc = 0 as *mut demangle_component;
            }
            28 | 29 | 30 | 31 | 32 | _ => {
                dc = 0 as *mut demangle_component;
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn is_gnu_v3_mangled_ctor(
    mut name: *const libc::c_char,
) -> gnu_v3_ctor_kinds {
    let mut ctor_kind: gnu_v3_ctor_kinds = 0 as gnu_v3_ctor_kinds;
    let mut dtor_kind: gnu_v3_dtor_kinds = 0 as gnu_v3_dtor_kinds;
    if is_ctor_or_dtor(name, &mut ctor_kind, &mut dtor_kind) == 0 {
        return 0 as gnu_v3_ctor_kinds;
    }
    return ctor_kind;
}
#[no_mangle]
pub unsafe extern "C" fn is_gnu_v3_mangled_dtor(
    mut name: *const libc::c_char,
) -> gnu_v3_dtor_kinds {
    let mut ctor_kind: gnu_v3_ctor_kinds = 0 as gnu_v3_ctor_kinds;
    let mut dtor_kind: gnu_v3_dtor_kinds = 0 as gnu_v3_dtor_kinds;
    if is_ctor_or_dtor(name, &mut ctor_kind, &mut dtor_kind) == 0 {
        return 0 as gnu_v3_dtor_kinds;
    }
    return dtor_kind;
}
unsafe extern "C" fn run_static_initializers() {
    cplus_demangle_operators = [
        {
            let mut init = demangle_operator_info {
                code: b"aN\0" as *const u8 as *const libc::c_char,
                name: b"&=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"aS\0" as *const u8 as *const libc::c_char,
                name: b"=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"aa\0" as *const u8 as *const libc::c_char,
                name: b"&&\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"ad\0" as *const u8 as *const libc::c_char,
                name: b"&\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"an\0" as *const u8 as *const libc::c_char,
                name: b"&\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"at\0" as *const u8 as *const libc::c_char,
                name: b"alignof \0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"aw\0" as *const u8 as *const libc::c_char,
                name: b"co_await \0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"az\0" as *const u8 as *const libc::c_char,
                name: b"alignof \0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"cc\0" as *const u8 as *const libc::c_char,
                name: b"const_cast\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"cl\0" as *const u8 as *const libc::c_char,
                name: b"()\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"cm\0" as *const u8 as *const libc::c_char,
                name: b",\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"co\0" as *const u8 as *const libc::c_char,
                name: b"~\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"dV\0" as *const u8 as *const libc::c_char,
                name: b"/=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"dX\0" as *const u8 as *const libc::c_char,
                name: b"[...]=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"da\0" as *const u8 as *const libc::c_char,
                name: b"delete[] \0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"dc\0" as *const u8 as *const libc::c_char,
                name: b"dynamic_cast\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"de\0" as *const u8 as *const libc::c_char,
                name: b"*\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"di\0" as *const u8 as *const libc::c_char,
                name: b"=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"dl\0" as *const u8 as *const libc::c_char,
                name: b"delete \0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"ds\0" as *const u8 as *const libc::c_char,
                name: b".*\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"dt\0" as *const u8 as *const libc::c_char,
                name: b".\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"dv\0" as *const u8 as *const libc::c_char,
                name: b"/\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"dx\0" as *const u8 as *const libc::c_char,
                name: b"]=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"eO\0" as *const u8 as *const libc::c_char,
                name: b"^=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"eo\0" as *const u8 as *const libc::c_char,
                name: b"^\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"eq\0" as *const u8 as *const libc::c_char,
                name: b"==\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"fL\0" as *const u8 as *const libc::c_char,
                name: b"...\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"fR\0" as *const u8 as *const libc::c_char,
                name: b"...\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"fl\0" as *const u8 as *const libc::c_char,
                name: b"...\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"fr\0" as *const u8 as *const libc::c_char,
                name: b"...\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"ge\0" as *const u8 as *const libc::c_char,
                name: b">=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"gs\0" as *const u8 as *const libc::c_char,
                name: b"::\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"gt\0" as *const u8 as *const libc::c_char,
                name: b">\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"ix\0" as *const u8 as *const libc::c_char,
                name: b"[]\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"lS\0" as *const u8 as *const libc::c_char,
                name: b"<<=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"le\0" as *const u8 as *const libc::c_char,
                name: b"<=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"li\0" as *const u8 as *const libc::c_char,
                name: b"operator\"\" \0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"ls\0" as *const u8 as *const libc::c_char,
                name: b"<<\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"lt\0" as *const u8 as *const libc::c_char,
                name: b"<\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"mI\0" as *const u8 as *const libc::c_char,
                name: b"-=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"mL\0" as *const u8 as *const libc::c_char,
                name: b"*=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"mi\0" as *const u8 as *const libc::c_char,
                name: b"-\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"ml\0" as *const u8 as *const libc::c_char,
                name: b"*\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"mm\0" as *const u8 as *const libc::c_char,
                name: b"--\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"na\0" as *const u8 as *const libc::c_char,
                name: b"new[]\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"ne\0" as *const u8 as *const libc::c_char,
                name: b"!=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"ng\0" as *const u8 as *const libc::c_char,
                name: b"-\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"nt\0" as *const u8 as *const libc::c_char,
                name: b"!\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"nw\0" as *const u8 as *const libc::c_char,
                name: b"new\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"oR\0" as *const u8 as *const libc::c_char,
                name: b"|=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"oo\0" as *const u8 as *const libc::c_char,
                name: b"||\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"or\0" as *const u8 as *const libc::c_char,
                name: b"|\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"pL\0" as *const u8 as *const libc::c_char,
                name: b"+=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"pl\0" as *const u8 as *const libc::c_char,
                name: b"+\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"pm\0" as *const u8 as *const libc::c_char,
                name: b"->*\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"pp\0" as *const u8 as *const libc::c_char,
                name: b"++\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"ps\0" as *const u8 as *const libc::c_char,
                name: b"+\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"pt\0" as *const u8 as *const libc::c_char,
                name: b"->\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"qu\0" as *const u8 as *const libc::c_char,
                name: b"?\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"rM\0" as *const u8 as *const libc::c_char,
                name: b"%=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"rS\0" as *const u8 as *const libc::c_char,
                name: b">>=\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"rc\0" as *const u8 as *const libc::c_char,
                name: b"reinterpret_cast\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"rm\0" as *const u8 as *const libc::c_char,
                name: b"%\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"rs\0" as *const u8 as *const libc::c_char,
                name: b">>\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"sP\0" as *const u8 as *const libc::c_char,
                name: b"sizeof...\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"sZ\0" as *const u8 as *const libc::c_char,
                name: b"sizeof...\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"sc\0" as *const u8 as *const libc::c_char,
                name: b"static_cast\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"ss\0" as *const u8 as *const libc::c_char,
                name: b"<=>\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"st\0" as *const u8 as *const libc::c_char,
                name: b"sizeof \0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"sz\0" as *const u8 as *const libc::c_char,
                name: b"sizeof \0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"tr\0" as *const u8 as *const libc::c_char,
                name: b"throw\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: b"tw\0" as *const u8 as *const libc::c_char,
                name: b"throw \0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                args: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = demangle_operator_info {
                code: 0 as *const libc::c_char,
                name: 0 as *const libc::c_char,
                len: 0 as libc::c_int,
                args: 0 as libc::c_int,
            };
            init
        },
    ];
    cplus_demangle_builtin_types = [
        {
            let mut init = demangle_builtin_type_info {
                name: b"signed char\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"signed char\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"bool\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"boolean\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_BOOL,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"char\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"byte\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"double\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"double\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_FLOAT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"long double\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"long double\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_FLOAT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"float\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"float\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_FLOAT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"__float128\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"__float128\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_FLOAT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"unsigned char\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"unsigned char\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"int\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"int\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_INT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"unsigned int\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"unsigned\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_UNSIGNED,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: 0 as *const libc::c_char,
                len: 0 as libc::c_int,
                java_name: 0 as *const libc::c_char,
                java_len: 0 as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"long\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"long\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_LONG,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"unsigned long\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"unsigned long\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_UNSIGNED_LONG,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"__int128\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"__int128\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"unsigned __int128\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"unsigned __int128\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: 0 as *const libc::c_char,
                len: 0 as libc::c_int,
                java_name: 0 as *const libc::c_char,
                java_len: 0 as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: 0 as *const libc::c_char,
                len: 0 as libc::c_int,
                java_name: 0 as *const libc::c_char,
                java_len: 0 as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: 0 as *const libc::c_char,
                len: 0 as libc::c_int,
                java_name: 0 as *const libc::c_char,
                java_len: 0 as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"short\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"short\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"unsigned short\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"unsigned short\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: 0 as *const libc::c_char,
                len: 0 as libc::c_int,
                java_name: 0 as *const libc::c_char,
                java_len: 0 as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"void\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"void\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_VOID,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"wchar_t\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"char\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"long long\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"long\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_LONG_LONG,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"unsigned long long\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"unsigned long long\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_UNSIGNED_LONG_LONG,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"...\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"...\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"decimal32\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"decimal32\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"decimal64\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"decimal64\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"decimal128\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"decimal128\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"half\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"half\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_FLOAT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"char8_t\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"char8_t\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"char16_t\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"char16_t\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"char32_t\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"char32_t\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
        {
            let mut init = demangle_builtin_type_info {
                name: b"decltype(nullptr)\0" as *const u8 as *const libc::c_char,
                len: (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                java_name: b"decltype(nullptr)\0" as *const u8 as *const libc::c_char,
                java_len: (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                print: D_PRINT_DEFAULT,
            };
            init
        },
    ];
    standard_subs = [
        {
            let mut init = d_standard_sub_info {
                code: 't' as i32 as libc::c_char,
                simple_expansion: b"std\0" as *const u8 as *const libc::c_char,
                simple_len: (::core::mem::size_of::<[libc::c_char; 4]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                full_expansion: b"std\0" as *const u8 as *const libc::c_char,
                full_len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                set_last_name: 0 as *const libc::c_char,
                set_last_name_len: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = d_standard_sub_info {
                code: 'a' as i32 as libc::c_char,
                simple_expansion: b"std::allocator\0" as *const u8
                    as *const libc::c_char,
                simple_len: (::core::mem::size_of::<[libc::c_char; 15]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                full_expansion: b"std::allocator\0" as *const u8 as *const libc::c_char,
                full_len: (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                set_last_name: b"allocator\0" as *const u8 as *const libc::c_char,
                set_last_name_len: (::core::mem::size_of::<[libc::c_char; 10]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            };
            init
        },
        {
            let mut init = d_standard_sub_info {
                code: 'b' as i32 as libc::c_char,
                simple_expansion: b"std::basic_string\0" as *const u8
                    as *const libc::c_char,
                simple_len: (::core::mem::size_of::<[libc::c_char; 18]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                full_expansion: b"std::basic_string\0" as *const u8
                    as *const libc::c_char,
                full_len: (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                set_last_name: b"basic_string\0" as *const u8 as *const libc::c_char,
                set_last_name_len: (::core::mem::size_of::<[libc::c_char; 13]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            };
            init
        },
        {
            let mut init = d_standard_sub_info {
                code: 's' as i32 as libc::c_char,
                simple_expansion: b"std::string\0" as *const u8 as *const libc::c_char,
                simple_len: (::core::mem::size_of::<[libc::c_char; 12]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                full_expansion: b"std::basic_string<char, std::char_traits<char>, std::allocator<char> >\0"
                    as *const u8 as *const libc::c_char,
                full_len: (::core::mem::size_of::<[libc::c_char; 71]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                set_last_name: b"basic_string\0" as *const u8 as *const libc::c_char,
                set_last_name_len: (::core::mem::size_of::<[libc::c_char; 13]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            };
            init
        },
        {
            let mut init = d_standard_sub_info {
                code: 'i' as i32 as libc::c_char,
                simple_expansion: b"std::istream\0" as *const u8 as *const libc::c_char,
                simple_len: (::core::mem::size_of::<[libc::c_char; 13]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                full_expansion: b"std::basic_istream<char, std::char_traits<char> >\0"
                    as *const u8 as *const libc::c_char,
                full_len: (::core::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                set_last_name: b"basic_istream\0" as *const u8 as *const libc::c_char,
                set_last_name_len: (::core::mem::size_of::<[libc::c_char; 14]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            };
            init
        },
        {
            let mut init = d_standard_sub_info {
                code: 'o' as i32 as libc::c_char,
                simple_expansion: b"std::ostream\0" as *const u8 as *const libc::c_char,
                simple_len: (::core::mem::size_of::<[libc::c_char; 13]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                full_expansion: b"std::basic_ostream<char, std::char_traits<char> >\0"
                    as *const u8 as *const libc::c_char,
                full_len: (::core::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                set_last_name: b"basic_ostream\0" as *const u8 as *const libc::c_char,
                set_last_name_len: (::core::mem::size_of::<[libc::c_char; 14]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            };
            init
        },
        {
            let mut init = d_standard_sub_info {
                code: 'd' as i32 as libc::c_char,
                simple_expansion: b"std::iostream\0" as *const u8 as *const libc::c_char,
                simple_len: (::core::mem::size_of::<[libc::c_char; 14]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                full_expansion: b"std::basic_iostream<char, std::char_traits<char> >\0"
                    as *const u8 as *const libc::c_char,
                full_len: (::core::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                set_last_name: b"basic_iostream\0" as *const u8 as *const libc::c_char,
                set_last_name_len: (::core::mem::size_of::<[libc::c_char; 15]>()
                    as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
