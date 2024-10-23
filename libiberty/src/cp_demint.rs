use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static cplus_demangle_operators: [demangle_operator_info; 0];
    static cplus_demangle_builtin_types: [demangle_builtin_type_info; 34];
    fn cplus_demangle_mangled_name(
        _: *mut d_info,
        _: libc::c_int,
    ) -> *mut demangle_component;
    fn cplus_demangle_type(_: *mut d_info) -> *mut demangle_component;
    fn cplus_demangle_init_info(
        _: *const libc::c_char,
        _: libc::c_int,
        _: size_t,
        _: *mut d_info,
    );
}
pub type size_t = libc::c_ulong;
pub type gnu_v3_ctor_kinds = libc::c_uint;
pub const gnu_v3_object_ctor_group: gnu_v3_ctor_kinds = 5;
pub const gnu_v3_unified_ctor: gnu_v3_ctor_kinds = 4;
pub const gnu_v3_complete_object_allocating_ctor: gnu_v3_ctor_kinds = 3;
pub const gnu_v3_base_object_ctor: gnu_v3_ctor_kinds = 2;
pub const gnu_v3_complete_object_ctor: gnu_v3_ctor_kinds = 1;
pub type gnu_v3_dtor_kinds = libc::c_uint;
pub const gnu_v3_object_dtor_group: gnu_v3_dtor_kinds = 5;
pub const gnu_v3_unified_dtor: gnu_v3_dtor_kinds = 4;
pub const gnu_v3_base_object_dtor: gnu_v3_dtor_kinds = 3;
pub const gnu_v3_complete_object_dtor: gnu_v3_dtor_kinds = 2;
pub const gnu_v3_deleting_dtor: gnu_v3_dtor_kinds = 1;
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
pub struct demangle_operator_info {
    pub code: *const libc::c_char,
    pub name: *const libc::c_char,
    pub len: libc::c_int,
    pub args: libc::c_int,
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
pub struct C2RustUnnamed_6 {
    pub kind: gnu_v3_dtor_kinds,
    pub name: *mut demangle_component,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub kind: gnu_v3_ctor_kinds,
    pub name: *mut demangle_component,
}
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
pub struct C2RustUnnamed_11 {
    pub s: *const libc::c_char,
    pub len: libc::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_fill_component(
    mut p: *mut demangle_component,
    mut type_0: demangle_component_type,
    mut left: *mut demangle_component,
    mut right: *mut demangle_component,
) -> libc::c_int {
    if p.is_null() {
        return 0 as libc::c_int;
    }
    match type_0 as libc::c_uint {
        1 | 2 | 3 | 4 | 11 | 33 | 41 | 42 | 43 | 46 | 47 | 55 | 56 | 57 | 58 | 59 | 60
        | 61 | 62 => {}
        9 | 10 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 22 | 25 | 26 | 27 | 28 | 29 | 30
        | 34 | 35 | 36 | 37 | 38 | 40 | 52 | 53 => {
            if !right.is_null() {
                return 0 as libc::c_int;
            }
        }
        _ => return 0 as libc::c_int,
    }
    (*p).type_0 = type_0;
    (*p).u.s_binary.left = left;
    (*p).u.s_binary.right = right;
    (*p).d_printing = 0 as libc::c_int;
    (*p).d_counting = 0 as libc::c_int;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_fill_builtin_type(
    mut p: *mut demangle_component,
    mut type_name: *const libc::c_char,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    if p.is_null() || type_name.is_null() {
        return 0 as libc::c_int;
    }
    len = strlen(type_name) as libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 34 as libc::c_int as libc::c_uint {
        if len == cplus_demangle_builtin_types[i as usize].len
            && strcmp(type_name, cplus_demangle_builtin_types[i as usize].name)
                == 0 as libc::c_int
        {
            (*p).type_0 = DEMANGLE_COMPONENT_BUILTIN_TYPE;
            (*p)
                .u
                .s_builtin
                .type_0 = &*cplus_demangle_builtin_types.as_ptr().offset(i as isize)
                as *const demangle_builtin_type_info;
            (*p).d_printing = 0 as libc::c_int;
            (*p).d_counting = 0 as libc::c_int;
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_fill_operator(
    mut p: *mut demangle_component,
    mut opname: *const libc::c_char,
    mut args: libc::c_int,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    if p.is_null() || opname.is_null() {
        return 0 as libc::c_int;
    }
    len = strlen(opname) as libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while !((*cplus_demangle_operators.as_ptr().offset(i as isize)).name).is_null() {
        if len == (*cplus_demangle_operators.as_ptr().offset(i as isize)).len
            && args == (*cplus_demangle_operators.as_ptr().offset(i as isize)).args
            && strcmp(
                opname,
                (*cplus_demangle_operators.as_ptr().offset(i as isize)).name,
            ) == 0 as libc::c_int
        {
            (*p).type_0 = DEMANGLE_COMPONENT_OPERATOR;
            (*p)
                .u
                .s_operator
                .op = &*cplus_demangle_operators.as_ptr().offset(i as isize)
                as *const demangle_operator_info;
            (*p).d_printing = 0 as libc::c_int;
            (*p).d_counting = 0 as libc::c_int;
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cplus_demangle_v3_components(
    mut mangled: *const libc::c_char,
    mut options: libc::c_int,
    mut mem: *mut *mut libc::c_void,
) -> *mut demangle_component {
    let mut len: size_t = 0;
    let mut type_0: libc::c_int = 0;
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
    len = strlen(mangled);
    if *mangled.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
        && *mangled.offset(1 as libc::c_int as isize) as libc::c_int == 'Z' as i32
    {
        type_0 = 0 as libc::c_int;
    } else {
        if options & (1 as libc::c_int) << 4 as libc::c_int == 0 as libc::c_int {
            return 0 as *mut demangle_component;
        }
        type_0 = 1 as libc::c_int;
    }
    cplus_demangle_init_info(mangled, options, len, &mut di);
    di
        .comps = malloc(
        (di.num_comps as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<demangle_component>() as libc::c_ulong),
    ) as *mut demangle_component;
    di
        .subs = malloc(
        (di.num_subs as libc::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<*mut demangle_component>() as libc::c_ulong,
            ),
    ) as *mut *mut demangle_component;
    if (di.comps).is_null() || (di.subs).is_null() {
        free(di.comps as *mut libc::c_void);
        free(di.subs as *mut libc::c_void);
        return 0 as *mut demangle_component;
    }
    if type_0 == 0 {
        dc = cplus_demangle_mangled_name(&mut di, 1 as libc::c_int);
    } else {
        dc = cplus_demangle_type(&mut di);
    }
    if options & (1 as libc::c_int) << 0 as libc::c_int != 0 as libc::c_int
        && *di.n as libc::c_int != '\0' as i32
    {
        dc = 0 as *mut demangle_component;
    }
    free(di.subs as *mut libc::c_void);
    if !dc.is_null() {
        *mem = di.comps as *mut libc::c_void;
    } else {
        free(di.comps as *mut libc::c_void);
    }
    return dc;
}
