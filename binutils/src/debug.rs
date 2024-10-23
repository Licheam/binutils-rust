extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn abort() -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn filename_cmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type bfd_vma = libc::c_ulong;
pub type bfd_signed_vma = libc::c_long;
pub type debug_type_kind = libc::c_uint;
pub const DEBUG_KIND_TAGGED: debug_type_kind = 23;
pub const DEBUG_KIND_NAMED: debug_type_kind = 22;
pub const DEBUG_KIND_VOLATILE: debug_type_kind = 21;
pub const DEBUG_KIND_CONST: debug_type_kind = 20;
pub const DEBUG_KIND_METHOD: debug_type_kind = 19;
pub const DEBUG_KIND_OFFSET: debug_type_kind = 18;
pub const DEBUG_KIND_SET: debug_type_kind = 17;
pub const DEBUG_KIND_ARRAY: debug_type_kind = 16;
pub const DEBUG_KIND_RANGE: debug_type_kind = 15;
pub const DEBUG_KIND_REFERENCE: debug_type_kind = 14;
pub const DEBUG_KIND_FUNCTION: debug_type_kind = 13;
pub const DEBUG_KIND_POINTER: debug_type_kind = 12;
pub const DEBUG_KIND_ENUM: debug_type_kind = 11;
pub const DEBUG_KIND_UNION_CLASS: debug_type_kind = 10;
pub const DEBUG_KIND_CLASS: debug_type_kind = 9;
pub const DEBUG_KIND_UNION: debug_type_kind = 8;
pub const DEBUG_KIND_STRUCT: debug_type_kind = 7;
pub const DEBUG_KIND_BOOL: debug_type_kind = 6;
pub const DEBUG_KIND_COMPLEX: debug_type_kind = 5;
pub const DEBUG_KIND_FLOAT: debug_type_kind = 4;
pub const DEBUG_KIND_INT: debug_type_kind = 3;
pub const DEBUG_KIND_VOID: debug_type_kind = 2;
pub const DEBUG_KIND_INDIRECT: debug_type_kind = 1;
pub const DEBUG_KIND_ILLEGAL: debug_type_kind = 0;
pub type debug_var_kind = libc::c_uint;
pub const DEBUG_REGISTER: debug_var_kind = 5;
pub const DEBUG_LOCAL: debug_var_kind = 4;
pub const DEBUG_LOCAL_STATIC: debug_var_kind = 3;
pub const DEBUG_STATIC: debug_var_kind = 2;
pub const DEBUG_GLOBAL: debug_var_kind = 1;
pub const DEBUG_VAR_ILLEGAL: debug_var_kind = 0;
pub type debug_parm_kind = libc::c_uint;
pub const DEBUG_PARM_REF_REG: debug_parm_kind = 4;
pub const DEBUG_PARM_REFERENCE: debug_parm_kind = 3;
pub const DEBUG_PARM_REG: debug_parm_kind = 2;
pub const DEBUG_PARM_STACK: debug_parm_kind = 1;
pub const DEBUG_PARM_ILLEGAL: debug_parm_kind = 0;
pub type debug_visibility = libc::c_uint;
pub const DEBUG_VISIBILITY_IGNORE: debug_visibility = 3;
pub const DEBUG_VISIBILITY_PRIVATE: debug_visibility = 2;
pub const DEBUG_VISIBILITY_PROTECTED: debug_visibility = 1;
pub const DEBUG_VISIBILITY_PUBLIC: debug_visibility = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_type_s {
    pub kind: debug_type_kind,
    pub size: libc::c_uint,
    pub pointer: debug_type,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub kindirect: *mut debug_indirect_type,
    pub kint: bool,
    pub kclass: *mut debug_class_type,
    pub kenum: *mut debug_enum_type,
    pub kpointer: *mut debug_type_s,
    pub kfunction: *mut debug_function_type,
    pub kreference: *mut debug_type_s,
    pub krange: *mut debug_range_type,
    pub karray: *mut debug_array_type,
    pub kset: *mut debug_set_type,
    pub koffset: *mut debug_offset_type,
    pub kmethod: *mut debug_method_type,
    pub kconst: *mut debug_type_s,
    pub kvolatile: *mut debug_type_s,
    pub knamed: *mut debug_named_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_named_type {
    pub name: *mut debug_name,
    pub type_0: debug_type,
}
pub type debug_type = *mut debug_type_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_name {
    pub next: *mut debug_name,
    pub name: *const libc::c_char,
    pub mark: libc::c_uint,
    pub kind: debug_object_kind,
    pub linkage: debug_object_linkage,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub type_0: *mut debug_type_s,
    pub tag: *mut debug_type_s,
    pub variable: *mut debug_variable,
    pub function: *mut debug_function,
    pub int_constant: bfd_vma,
    pub float_constant: libc::c_double,
    pub typed_constant: *mut debug_typed_constant,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_typed_constant {
    pub type_0: debug_type,
    pub val: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_function {
    pub return_type: debug_type,
    pub parameters: *mut debug_parameter,
    pub blocks: *mut debug_block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_block {
    pub next: *mut debug_block,
    pub parent: *mut debug_block,
    pub children: *mut debug_block,
    pub start: bfd_vma,
    pub end: bfd_vma,
    pub locals: *mut debug_namespace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_namespace {
    pub list: *mut debug_name,
    pub tail: *mut *mut debug_name,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_parameter {
    pub next: *mut debug_parameter,
    pub name: *const libc::c_char,
    pub type_0: debug_type,
    pub kind: debug_parm_kind,
    pub val: bfd_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_variable {
    pub kind: debug_var_kind,
    pub type_0: debug_type,
    pub val: bfd_vma,
}
pub type debug_object_linkage = libc::c_uint;
pub const DEBUG_LINKAGE_NONE: debug_object_linkage = 3;
pub const DEBUG_LINKAGE_GLOBAL: debug_object_linkage = 2;
pub const DEBUG_LINKAGE_STATIC: debug_object_linkage = 1;
pub const DEBUG_LINKAGE_AUTOMATIC: debug_object_linkage = 0;
pub type debug_object_kind = libc::c_uint;
pub const DEBUG_OBJECT_TYPED_CONSTANT: debug_object_kind = 6;
pub const DEBUG_OBJECT_FLOAT_CONSTANT: debug_object_kind = 5;
pub const DEBUG_OBJECT_INT_CONSTANT: debug_object_kind = 4;
pub const DEBUG_OBJECT_FUNCTION: debug_object_kind = 3;
pub const DEBUG_OBJECT_VARIABLE: debug_object_kind = 2;
pub const DEBUG_OBJECT_TAG: debug_object_kind = 1;
pub const DEBUG_OBJECT_TYPE: debug_object_kind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_method_type {
    pub return_type: debug_type,
    pub domain_type: debug_type,
    pub arg_types: *mut debug_type,
    pub varargs: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_offset_type {
    pub base_type: debug_type,
    pub target_type: debug_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_set_type {
    pub type_0: debug_type,
    pub bitstringp: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_array_type {
    pub element_type: debug_type,
    pub range_type: debug_type,
    pub lower: bfd_signed_vma,
    pub upper: bfd_signed_vma,
    pub stringp: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_range_type {
    pub type_0: debug_type,
    pub lower: bfd_signed_vma,
    pub upper: bfd_signed_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_function_type {
    pub return_type: debug_type,
    pub arg_types: *mut debug_type,
    pub varargs: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_enum_type {
    pub names: *mut *const libc::c_char,
    pub values: *mut bfd_signed_vma,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_class_type {
    pub fields: *mut debug_field,
    pub mark: libc::c_uint,
    pub id: libc::c_uint,
    pub baseclasses: *mut debug_baseclass,
    pub methods: *mut debug_method,
    pub vptrbase: debug_type,
}
pub type debug_method = *mut debug_method_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_method_s {
    pub name: *const libc::c_char,
    pub variants: *mut *mut debug_method_variant_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_method_variant_s {
    pub physname: *const libc::c_char,
    pub type_0: *mut debug_type_s,
    pub visibility: debug_visibility,
    pub constp: bool,
    pub volatilep: bool,
    pub voffset: bfd_vma,
    pub context: *mut debug_type_s,
}
pub type debug_baseclass = *mut debug_baseclass_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_baseclass_s {
    pub type_0: *mut debug_type_s,
    pub bitpos: libc::c_uint,
    pub is_virtual: bool,
    pub visibility: debug_visibility,
}
pub type debug_field = *mut debug_field_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_field_s {
    pub name: *const libc::c_char,
    pub type_0: *mut debug_type_s,
    pub visibility: debug_visibility,
    pub static_member: bool,
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub f: C2RustUnnamed_3,
    pub s: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub physname: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub bitpos: libc::c_uint,
    pub bitsize: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_indirect_type {
    pub slot: *mut debug_type,
    pub tag: *const libc::c_char,
}
pub type debug_method_variant = *mut debug_method_variant_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_write_fns {
    pub start_compilation_unit: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> bool,
    >,
    pub start_source: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> bool,
    >,
    pub empty_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub void_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub int_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_uint, bool) -> bool,
    >,
    pub float_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> bool,
    >,
    pub complex_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> bool,
    >,
    pub bool_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> bool,
    >,
    pub enum_type: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *mut *const libc::c_char,
            *mut bfd_signed_vma,
        ) -> bool,
    >,
    pub pointer_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub function_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, bool) -> bool,
    >,
    pub reference_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub range_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, bfd_signed_vma, bfd_signed_vma) -> bool,
    >,
    pub array_type: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            bfd_signed_vma,
            bfd_signed_vma,
            bool,
        ) -> bool,
    >,
    pub set_type: Option::<unsafe extern "C" fn(*mut libc::c_void, bool) -> bool>,
    pub offset_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub method_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, bool, libc::c_int, bool) -> bool,
    >,
    pub const_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub volatile_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub start_struct_type: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_uint,
            bool,
            libc::c_uint,
        ) -> bool,
    >,
    pub struct_field: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            bfd_vma,
            bfd_vma,
            debug_visibility,
        ) -> bool,
    >,
    pub end_struct_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub start_class_type: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_uint,
            bool,
            libc::c_uint,
            bool,
            bool,
        ) -> bool,
    >,
    pub class_static_member: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const libc::c_char,
            debug_visibility,
        ) -> bool,
    >,
    pub class_baseclass: Option::<
        unsafe extern "C" fn(*mut libc::c_void, bfd_vma, bool, debug_visibility) -> bool,
    >,
    pub class_start_method: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> bool,
    >,
    pub class_method_variant: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            debug_visibility,
            bool,
            bool,
            bfd_vma,
            bool,
        ) -> bool,
    >,
    pub class_static_method_variant: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            debug_visibility,
            bool,
            bool,
        ) -> bool,
    >,
    pub class_end_method: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub end_class_type: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub typedef_type: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> bool,
    >,
    pub tag_type: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_uint,
            debug_type_kind,
        ) -> bool,
    >,
    pub typdef: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> bool,
    >,
    pub tag: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> bool,
    >,
    pub int_constant: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, bfd_vma) -> bool,
    >,
    pub float_constant: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_double,
        ) -> bool,
    >,
    pub typed_constant: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, bfd_vma) -> bool,
    >,
    pub variable: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            debug_var_kind,
            bfd_vma,
        ) -> bool,
    >,
    pub start_function: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, bool) -> bool,
    >,
    pub function_parameter: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            debug_parm_kind,
            bfd_vma,
        ) -> bool,
    >,
    pub start_block: Option::<unsafe extern "C" fn(*mut libc::c_void, bfd_vma) -> bool>,
    pub end_block: Option::<unsafe extern "C" fn(*mut libc::c_void, bfd_vma) -> bool>,
    pub end_function: Option::<unsafe extern "C" fn(*mut libc::c_void) -> bool>,
    pub lineno: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_ulong,
            bfd_vma,
        ) -> bool,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_handle {
    pub units: *mut debug_unit,
    pub current_unit: *mut debug_unit,
    pub current_file: *mut debug_file,
    pub current_function: *mut debug_function,
    pub current_block: *mut debug_block,
    pub current_lineno: *mut debug_lineno,
    pub mark: libc::c_uint,
    pub class_id: libc::c_uint,
    pub base_id: libc::c_uint,
    pub current_write_lineno: *mut debug_lineno,
    pub current_write_lineno_index: libc::c_uint,
    pub id_list: *mut debug_class_id,
    pub compare_list: *mut debug_type_compare_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_type_compare_list {
    pub next: *mut debug_type_compare_list,
    pub t1: *mut debug_type_s,
    pub t2: *mut debug_type_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_class_id {
    pub next: *mut debug_class_id,
    pub type_0: *mut debug_type_s,
    pub tag: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_lineno {
    pub next: *mut debug_lineno,
    pub file: *mut debug_file,
    pub linenos: [libc::c_ulong; 10],
    pub addrs: [bfd_vma; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_file {
    pub next: *mut debug_file,
    pub filename: *const libc::c_char,
    pub globals: *mut debug_namespace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_unit {
    pub next: *mut debug_unit,
    pub files: *mut debug_file,
    pub linenos: *mut debug_lineno,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct debug_type_real_list {
    pub next: *mut debug_type_real_list,
    pub t: *mut debug_type_s,
}
#[no_mangle]
pub unsafe extern "C" fn debug_init() -> *mut libc::c_void {
    let mut ret: *mut debug_handle = 0 as *mut debug_handle;
    ret = xmalloc(::core::mem::size_of::<debug_handle>() as libc::c_ulong)
        as *mut debug_handle;
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_handle>() as libc::c_ulong,
    );
    return ret as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn debug_set_filename(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
) -> bool {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut nfile: *mut debug_file = 0 as *mut debug_file;
    let mut nunit: *mut debug_unit = 0 as *mut debug_unit;
    if name.is_null() {
        name = b"\0" as *const u8 as *const libc::c_char;
    }
    nfile = xmalloc(::core::mem::size_of::<debug_file>() as libc::c_ulong)
        as *mut debug_file;
    memset(
        nfile as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_file>() as libc::c_ulong,
    );
    (*nfile).filename = name;
    nunit = xmalloc(::core::mem::size_of::<debug_unit>() as libc::c_ulong)
        as *mut debug_unit;
    memset(
        nunit as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_unit>() as libc::c_ulong,
    );
    (*nunit).files = nfile;
    (*info).current_file = nfile;
    if !((*info).current_unit).is_null() {
        (*(*info).current_unit).next = nunit;
    } else {
        if ((*info).units).is_null() {} else {
            __assert_fail(
                b"info->units == NULL\0" as *const u8 as *const libc::c_char,
                b"debug.c\0" as *const u8 as *const libc::c_char,
                699 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"_Bool debug_set_filename(void *, const char *)\0"))
                    .as_ptr(),
            );
        }
        'c_10266: {
            if ((*info).units).is_null() {} else {
                __assert_fail(
                    b"info->units == NULL\0" as *const u8 as *const libc::c_char,
                    b"debug.c\0" as *const u8 as *const libc::c_char,
                    699 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 47],
                        &[libc::c_char; 47],
                    >(b"_Bool debug_set_filename(void *, const char *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*info).units = nunit;
    }
    (*info).current_unit = nunit;
    (*info).current_function = 0 as *mut debug_function;
    (*info).current_block = 0 as *mut debug_block;
    (*info).current_lineno = 0 as *mut debug_lineno;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_start_source(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
) -> bool {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut f: *mut debug_file = 0 as *mut debug_file;
    let mut pf: *mut *mut debug_file = 0 as *mut *mut debug_file;
    if name.is_null() {
        name = b"\0" as *const u8 as *const libc::c_char;
    }
    if ((*info).current_unit).is_null() {
        debug_error(
            dcgettext(
                0 as *const libc::c_char,
                b"debug_start_source: no debug_set_filename call\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    f = (*(*info).current_unit).files;
    while !f.is_null() {
        if filename_cmp((*f).filename, name) == 0 as libc::c_int {
            (*info).current_file = f;
            return 1 as libc::c_int != 0;
        }
        f = (*f).next;
    }
    f = xmalloc(::core::mem::size_of::<debug_file>() as libc::c_ulong)
        as *mut debug_file;
    memset(
        f as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_file>() as libc::c_ulong,
    );
    (*f).filename = name;
    pf = &mut (*(*info).current_file).next;
    while !(*pf).is_null() {
        pf = &mut (**pf).next;
    }
    *pf = f;
    (*info).current_file = f;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_record_function(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut return_type: debug_type,
    mut global: bool,
    mut addr: bfd_vma,
) -> bool {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut f: *mut debug_function = 0 as *mut debug_function;
    let mut b: *mut debug_block = 0 as *mut debug_block;
    let mut n: *mut debug_name = 0 as *mut debug_name;
    if name.is_null() {
        name = b"\0" as *const u8 as *const libc::c_char;
    }
    if return_type.is_null() {
        return 0 as libc::c_int != 0;
    }
    if ((*info).current_unit).is_null() {
        debug_error(
            dcgettext(
                0 as *const libc::c_char,
                b"debug_record_function: no debug_set_filename call\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    f = xmalloc(::core::mem::size_of::<debug_function>() as libc::c_ulong)
        as *mut debug_function;
    memset(
        f as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_function>() as libc::c_ulong,
    );
    (*f).return_type = return_type;
    b = xmalloc(::core::mem::size_of::<debug_block>() as libc::c_ulong)
        as *mut debug_block;
    memset(
        b as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_block>() as libc::c_ulong,
    );
    (*b).start = addr;
    (*b).end = -(1 as libc::c_int) as bfd_vma;
    (*f).blocks = b;
    (*info).current_function = f;
    (*info).current_block = b;
    n = debug_add_to_namespace(
        info,
        &mut (*(*info).current_file).globals,
        name,
        DEBUG_OBJECT_FUNCTION,
        (if global as libc::c_int != 0 {
            DEBUG_LINKAGE_GLOBAL as libc::c_int
        } else {
            DEBUG_LINKAGE_STATIC as libc::c_int
        }) as debug_object_linkage,
    );
    if n.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*n).u.function = f;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_record_parameter(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut type_0: debug_type,
    mut kind: debug_parm_kind,
    mut val: bfd_vma,
) -> bool {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut p: *mut debug_parameter = 0 as *mut debug_parameter;
    let mut pp: *mut *mut debug_parameter = 0 as *mut *mut debug_parameter;
    if name.is_null() || type_0.is_null() {
        return 0 as libc::c_int != 0;
    }
    if ((*info).current_unit).is_null() || ((*info).current_function).is_null() {
        debug_error(
            dcgettext(
                0 as *const libc::c_char,
                b"debug_record_parameter: no current function\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    p = xmalloc(::core::mem::size_of::<debug_parameter>() as libc::c_ulong)
        as *mut debug_parameter;
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_parameter>() as libc::c_ulong,
    );
    (*p).name = name;
    (*p).type_0 = type_0;
    (*p).kind = kind;
    (*p).val = val;
    pp = &mut (*(*info).current_function).parameters;
    while !(*pp).is_null() {
        pp = &mut (**pp).next;
    }
    *pp = p;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_end_function(
    mut handle: *mut libc::c_void,
    mut addr: bfd_vma,
) -> bool {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    if ((*info).current_unit).is_null() || ((*info).current_block).is_null()
        || ((*info).current_function).is_null()
    {
        debug_error(
            dcgettext(
                0 as *const libc::c_char,
                b"debug_end_function: no current function\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    if !((*(*info).current_block).parent).is_null() {
        debug_error(
            dcgettext(
                0 as *const libc::c_char,
                b"debug_end_function: some blocks were not closed\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    (*(*info).current_block).end = addr;
    (*info).current_function = 0 as *mut debug_function;
    (*info).current_block = 0 as *mut debug_block;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_start_block(
    mut handle: *mut libc::c_void,
    mut addr: bfd_vma,
) -> bool {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut b: *mut debug_block = 0 as *mut debug_block;
    let mut pb: *mut *mut debug_block = 0 as *mut *mut debug_block;
    if ((*info).current_unit).is_null() || ((*info).current_block).is_null() {
        debug_error(
            dcgettext(
                0 as *const libc::c_char,
                b"debug_start_block: no current block\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    b = xmalloc(::core::mem::size_of::<debug_block>() as libc::c_ulong)
        as *mut debug_block;
    memset(
        b as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_block>() as libc::c_ulong,
    );
    (*b).parent = (*info).current_block;
    (*b).start = addr;
    (*b).end = -(1 as libc::c_int) as bfd_vma;
    pb = &mut (*(*info).current_block).children;
    while !(*pb).is_null() {
        pb = &mut (**pb).next;
    }
    *pb = b;
    (*info).current_block = b;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_end_block(
    mut handle: *mut libc::c_void,
    mut addr: bfd_vma,
) -> bool {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut parent: *mut debug_block = 0 as *mut debug_block;
    if ((*info).current_unit).is_null() || ((*info).current_block).is_null() {
        debug_error(
            dcgettext(
                0 as *const libc::c_char,
                b"debug_end_block: no current block\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    parent = (*(*info).current_block).parent;
    if parent.is_null() {
        debug_error(
            dcgettext(
                0 as *const libc::c_char,
                b"debug_end_block: attempt to close top level block\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    (*(*info).current_block).end = addr;
    (*info).current_block = parent;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_record_line(
    mut handle: *mut libc::c_void,
    mut lineno: libc::c_ulong,
    mut addr: bfd_vma,
) -> bool {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut l: *mut debug_lineno = 0 as *mut debug_lineno;
    let mut i: libc::c_uint = 0;
    if ((*info).current_unit).is_null() {
        debug_error(
            dcgettext(
                0 as *const libc::c_char,
                b"debug_record_line: no current unit\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    l = (*info).current_lineno;
    if !l.is_null() && (*l).file == (*info).current_file {
        i = 0 as libc::c_int as libc::c_uint;
        while i < 10 as libc::c_int as libc::c_uint {
            if (*l).linenos[i as usize] == -(1 as libc::c_int) as libc::c_ulong {
                (*l).linenos[i as usize] = lineno;
                (*l).addrs[i as usize] = addr;
                return 1 as libc::c_int != 0;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    l = xmalloc(::core::mem::size_of::<debug_lineno>() as libc::c_ulong)
        as *mut debug_lineno;
    memset(
        l as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_lineno>() as libc::c_ulong,
    );
    (*l).file = (*info).current_file;
    (*l).linenos[0 as libc::c_int as usize] = lineno;
    (*l).addrs[0 as libc::c_int as usize] = addr;
    i = 1 as libc::c_int as libc::c_uint;
    while i < 10 as libc::c_int as libc::c_uint {
        (*l).linenos[i as usize] = -(1 as libc::c_int) as libc::c_ulong;
        i = i.wrapping_add(1);
        i;
    }
    if !((*info).current_lineno).is_null() {
        (*(*info).current_lineno).next = l;
    } else {
        (*(*info).current_unit).linenos = l;
    }
    (*info).current_lineno = l;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_start_common_block(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
) -> bool {
    debug_error(
        dcgettext(
            0 as *const libc::c_char,
            b"debug_start_common_block: not implemented\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_end_common_block(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
) -> bool {
    debug_error(
        dcgettext(
            0 as *const libc::c_char,
            b"debug_end_common_block: not implemented\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_record_int_const(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut val: bfd_vma,
) -> bool {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut n: *mut debug_name = 0 as *mut debug_name;
    if name.is_null() {
        return 0 as libc::c_int != 0;
    }
    n = debug_add_to_current_namespace(
        info,
        name,
        DEBUG_OBJECT_INT_CONSTANT,
        DEBUG_LINKAGE_NONE,
    );
    if n.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*n).u.int_constant = val;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_record_float_const(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut val: libc::c_double,
) -> bool {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut n: *mut debug_name = 0 as *mut debug_name;
    if name.is_null() {
        return 0 as libc::c_int != 0;
    }
    n = debug_add_to_current_namespace(
        info,
        name,
        DEBUG_OBJECT_FLOAT_CONSTANT,
        DEBUG_LINKAGE_NONE,
    );
    if n.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*n).u.float_constant = val;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_record_typed_const(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut type_0: debug_type,
    mut val: bfd_vma,
) -> bool {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut n: *mut debug_name = 0 as *mut debug_name;
    let mut tc: *mut debug_typed_constant = 0 as *mut debug_typed_constant;
    if name.is_null() || type_0.is_null() {
        return 0 as libc::c_int != 0;
    }
    n = debug_add_to_current_namespace(
        info,
        name,
        DEBUG_OBJECT_TYPED_CONSTANT,
        DEBUG_LINKAGE_NONE,
    );
    if n.is_null() {
        return 0 as libc::c_int != 0;
    }
    tc = xmalloc(::core::mem::size_of::<debug_typed_constant>() as libc::c_ulong)
        as *mut debug_typed_constant;
    memset(
        tc as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_typed_constant>() as libc::c_ulong,
    );
    (*tc).type_0 = type_0;
    (*tc).val = val;
    (*n).u.typed_constant = tc;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_record_label(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut type_0: debug_type,
    mut addr: bfd_vma,
) -> bool {
    debug_error(
        dcgettext(
            0 as *const libc::c_char,
            b"debug_record_label: not implemented\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_record_variable(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut type_0: debug_type,
    mut kind: debug_var_kind,
    mut val: bfd_vma,
) -> bool {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut nsp: *mut *mut debug_namespace = 0 as *mut *mut debug_namespace;
    let mut linkage: debug_object_linkage = DEBUG_LINKAGE_AUTOMATIC;
    let mut n: *mut debug_name = 0 as *mut debug_name;
    let mut v: *mut debug_variable = 0 as *mut debug_variable;
    if name.is_null() || type_0.is_null() {
        return 0 as libc::c_int != 0;
    }
    if ((*info).current_unit).is_null() || ((*info).current_file).is_null() {
        debug_error(
            dcgettext(
                0 as *const libc::c_char,
                b"debug_record_variable: no current file\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int != 0;
    }
    if kind as libc::c_uint == DEBUG_GLOBAL as libc::c_int as libc::c_uint
        || kind as libc::c_uint == DEBUG_STATIC as libc::c_int as libc::c_uint
    {
        nsp = &mut (*(*info).current_file).globals;
        if kind as libc::c_uint == DEBUG_GLOBAL as libc::c_int as libc::c_uint {
            linkage = DEBUG_LINKAGE_GLOBAL;
        } else {
            linkage = DEBUG_LINKAGE_STATIC;
        }
    } else {
        if ((*info).current_block).is_null() {
            nsp = &mut (*(*info).current_file).globals;
        } else {
            nsp = &mut (*(*info).current_block).locals;
        }
        linkage = DEBUG_LINKAGE_AUTOMATIC;
    }
    n = debug_add_to_namespace(info, nsp, name, DEBUG_OBJECT_VARIABLE, linkage);
    if n.is_null() {
        return 0 as libc::c_int != 0;
    }
    v = xmalloc(::core::mem::size_of::<debug_variable>() as libc::c_ulong)
        as *mut debug_variable;
    memset(
        v as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_variable>() as libc::c_ulong,
    );
    (*v).kind = kind;
    (*v).type_0 = type_0;
    (*v).val = val;
    (*n).u.variable = v;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_indirect_type(
    mut handle: *mut libc::c_void,
    mut slot: *mut debug_type,
    mut tag: *const libc::c_char,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    let mut i: *mut debug_indirect_type = 0 as *mut debug_indirect_type;
    t = debug_make_type(info, DEBUG_KIND_INDIRECT, 0 as libc::c_int as libc::c_uint);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    i = xmalloc(::core::mem::size_of::<debug_indirect_type>() as libc::c_ulong)
        as *mut debug_indirect_type;
    memset(
        i as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_indirect_type>() as libc::c_ulong,
    );
    (*i).slot = slot;
    (*i).tag = tag;
    (*t).u.kindirect = i;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_void_type(
    mut handle: *mut libc::c_void,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    return debug_make_type(info, DEBUG_KIND_VOID, 0 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_int_type(
    mut handle: *mut libc::c_void,
    mut size: libc::c_uint,
    mut unsignedp: bool,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    t = debug_make_type(info, DEBUG_KIND_INT, size);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    (*t).u.kint = unsignedp;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_float_type(
    mut handle: *mut libc::c_void,
    mut size: libc::c_uint,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    return debug_make_type(info, DEBUG_KIND_FLOAT, size);
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_bool_type(
    mut handle: *mut libc::c_void,
    mut size: libc::c_uint,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    return debug_make_type(info, DEBUG_KIND_BOOL, size);
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_complex_type(
    mut handle: *mut libc::c_void,
    mut size: libc::c_uint,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    return debug_make_type(info, DEBUG_KIND_COMPLEX, size);
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_struct_type(
    mut handle: *mut libc::c_void,
    mut structp: bool,
    mut size: bfd_vma,
    mut fields: *mut debug_field,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    let mut c: *mut debug_class_type = 0 as *mut debug_class_type;
    t = debug_make_type(
        info,
        (if structp as libc::c_int != 0 {
            DEBUG_KIND_STRUCT as libc::c_int
        } else {
            DEBUG_KIND_UNION as libc::c_int
        }) as debug_type_kind,
        size as libc::c_uint,
    );
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    c = xmalloc(::core::mem::size_of::<debug_class_type>() as libc::c_ulong)
        as *mut debug_class_type;
    memset(
        c as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_class_type>() as libc::c_ulong,
    );
    (*c).fields = fields;
    (*t).u.kclass = c;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_object_type(
    mut handle: *mut libc::c_void,
    mut structp: bool,
    mut size: bfd_vma,
    mut fields: *mut debug_field,
    mut baseclasses: *mut debug_baseclass,
    mut methods: *mut debug_method,
    mut vptrbase: debug_type,
    mut ownvptr: bool,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    let mut c: *mut debug_class_type = 0 as *mut debug_class_type;
    t = debug_make_type(
        info,
        (if structp as libc::c_int != 0 {
            DEBUG_KIND_CLASS as libc::c_int
        } else {
            DEBUG_KIND_UNION_CLASS as libc::c_int
        }) as debug_type_kind,
        size as libc::c_uint,
    );
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    c = xmalloc(::core::mem::size_of::<debug_class_type>() as libc::c_ulong)
        as *mut debug_class_type;
    memset(
        c as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_class_type>() as libc::c_ulong,
    );
    (*c).fields = fields;
    (*c).baseclasses = baseclasses;
    (*c).methods = methods;
    if ownvptr {
        (*c).vptrbase = t;
    } else {
        (*c).vptrbase = vptrbase;
    }
    (*t).u.kclass = c;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_enum_type(
    mut handle: *mut libc::c_void,
    mut names: *mut *const libc::c_char,
    mut values: *mut bfd_signed_vma,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    let mut e: *mut debug_enum_type = 0 as *mut debug_enum_type;
    t = debug_make_type(info, DEBUG_KIND_ENUM, 0 as libc::c_int as libc::c_uint);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    e = xmalloc(::core::mem::size_of::<debug_enum_type>() as libc::c_ulong)
        as *mut debug_enum_type;
    memset(
        e as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_enum_type>() as libc::c_ulong,
    );
    (*e).names = names;
    (*e).values = values;
    (*t).u.kenum = e;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_pointer_type(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    if type_0.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    if !((*type_0).pointer).is_null() {
        return (*type_0).pointer;
    }
    t = debug_make_type(info, DEBUG_KIND_POINTER, 0 as libc::c_int as libc::c_uint);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    (*t).u.kpointer = type_0;
    (*type_0).pointer = t;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_function_type(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
    mut arg_types: *mut debug_type,
    mut varargs: bool,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    let mut f: *mut debug_function_type = 0 as *mut debug_function_type;
    if type_0.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    t = debug_make_type(info, DEBUG_KIND_FUNCTION, 0 as libc::c_int as libc::c_uint);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    f = xmalloc(::core::mem::size_of::<debug_function_type>() as libc::c_ulong)
        as *mut debug_function_type;
    memset(
        f as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_function_type>() as libc::c_ulong,
    );
    (*f).return_type = type_0;
    (*f).arg_types = arg_types;
    (*f).varargs = varargs;
    (*t).u.kfunction = f;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_reference_type(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    if type_0.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    t = debug_make_type(info, DEBUG_KIND_REFERENCE, 0 as libc::c_int as libc::c_uint);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    (*t).u.kreference = type_0;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_range_type(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
    mut lower: bfd_signed_vma,
    mut upper: bfd_signed_vma,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    let mut r: *mut debug_range_type = 0 as *mut debug_range_type;
    if type_0.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    t = debug_make_type(info, DEBUG_KIND_RANGE, 0 as libc::c_int as libc::c_uint);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    r = xmalloc(::core::mem::size_of::<debug_range_type>() as libc::c_ulong)
        as *mut debug_range_type;
    memset(
        r as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_range_type>() as libc::c_ulong,
    );
    (*r).type_0 = type_0;
    (*r).lower = lower;
    (*r).upper = upper;
    (*t).u.krange = r;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_array_type(
    mut handle: *mut libc::c_void,
    mut element_type: debug_type,
    mut range_type: debug_type,
    mut lower: bfd_signed_vma,
    mut upper: bfd_signed_vma,
    mut stringp: bool,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    let mut a: *mut debug_array_type = 0 as *mut debug_array_type;
    if element_type.is_null() || range_type.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    t = debug_make_type(info, DEBUG_KIND_ARRAY, 0 as libc::c_int as libc::c_uint);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    a = xmalloc(::core::mem::size_of::<debug_array_type>() as libc::c_ulong)
        as *mut debug_array_type;
    memset(
        a as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_array_type>() as libc::c_ulong,
    );
    (*a).element_type = element_type;
    (*a).range_type = range_type;
    (*a).lower = lower;
    (*a).upper = upper;
    (*a).stringp = stringp;
    (*t).u.karray = a;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_set_type(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
    mut bitstringp: bool,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    let mut s: *mut debug_set_type = 0 as *mut debug_set_type;
    if type_0.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    t = debug_make_type(info, DEBUG_KIND_SET, 0 as libc::c_int as libc::c_uint);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    s = xmalloc(::core::mem::size_of::<debug_set_type>() as libc::c_ulong)
        as *mut debug_set_type;
    memset(
        s as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_set_type>() as libc::c_ulong,
    );
    (*s).type_0 = type_0;
    (*s).bitstringp = bitstringp;
    (*t).u.kset = s;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_offset_type(
    mut handle: *mut libc::c_void,
    mut base_type: debug_type,
    mut target_type: debug_type,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    let mut o: *mut debug_offset_type = 0 as *mut debug_offset_type;
    if base_type.is_null() || target_type.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    t = debug_make_type(info, DEBUG_KIND_OFFSET, 0 as libc::c_int as libc::c_uint);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    o = xmalloc(::core::mem::size_of::<debug_offset_type>() as libc::c_ulong)
        as *mut debug_offset_type;
    memset(
        o as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_offset_type>() as libc::c_ulong,
    );
    (*o).base_type = base_type;
    (*o).target_type = target_type;
    (*t).u.koffset = o;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_method_type(
    mut handle: *mut libc::c_void,
    mut return_type: debug_type,
    mut domain_type: debug_type,
    mut arg_types: *mut debug_type,
    mut varargs: bool,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    let mut m: *mut debug_method_type = 0 as *mut debug_method_type;
    if return_type.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    t = debug_make_type(info, DEBUG_KIND_METHOD, 0 as libc::c_int as libc::c_uint);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    m = xmalloc(::core::mem::size_of::<debug_method_type>() as libc::c_ulong)
        as *mut debug_method_type;
    memset(
        m as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_method_type>() as libc::c_ulong,
    );
    (*m).return_type = return_type;
    (*m).domain_type = domain_type;
    (*m).arg_types = arg_types;
    (*m).varargs = varargs;
    (*t).u.kmethod = m;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_const_type(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    if type_0.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    t = debug_make_type(info, DEBUG_KIND_CONST, 0 as libc::c_int as libc::c_uint);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    (*t).u.kconst = type_0;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_volatile_type(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    if type_0.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    t = debug_make_type(info, DEBUG_KIND_VOLATILE, 0 as libc::c_int as libc::c_uint);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    (*t).u.kvolatile = type_0;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_undefined_tagged_type(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut kind: debug_type_kind,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    if name.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    match kind as libc::c_uint {
        7 | 8 | 9 | 10 | 11 => {}
        _ => {
            debug_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"debug_make_undefined_type: unsupported kind\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as *mut libc::c_void as debug_type;
        }
    }
    t = debug_make_type(info, kind, 0 as libc::c_int as libc::c_uint);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    return debug_tag_type(handle, name, t);
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_baseclass(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
    mut bitpos: bfd_vma,
    mut is_virtual: bool,
    mut visibility: debug_visibility,
) -> debug_baseclass {
    let mut b: *mut debug_baseclass_s = 0 as *mut debug_baseclass_s;
    b = xmalloc(::core::mem::size_of::<debug_baseclass_s>() as libc::c_ulong)
        as *mut debug_baseclass_s;
    memset(
        b as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_baseclass_s>() as libc::c_ulong,
    );
    (*b).type_0 = type_0;
    (*b).bitpos = bitpos as libc::c_uint;
    (*b).is_virtual = is_virtual;
    (*b).visibility = visibility;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_field(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut type_0: debug_type,
    mut bitpos: bfd_vma,
    mut bitsize: bfd_vma,
    mut visibility: debug_visibility,
) -> debug_field {
    let mut f: *mut debug_field_s = 0 as *mut debug_field_s;
    f = xmalloc(::core::mem::size_of::<debug_field_s>() as libc::c_ulong)
        as *mut debug_field_s;
    memset(
        f as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_field_s>() as libc::c_ulong,
    );
    (*f).name = name;
    (*f).type_0 = type_0;
    (*f).static_member = 0 as libc::c_int != 0;
    (*f).u.f.bitpos = bitpos as libc::c_uint;
    (*f).u.f.bitsize = bitsize as libc::c_uint;
    (*f).visibility = visibility;
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_static_member(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut type_0: debug_type,
    mut physname: *const libc::c_char,
    mut visibility: debug_visibility,
) -> debug_field {
    let mut f: *mut debug_field_s = 0 as *mut debug_field_s;
    f = xmalloc(::core::mem::size_of::<debug_field_s>() as libc::c_ulong)
        as *mut debug_field_s;
    memset(
        f as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_field_s>() as libc::c_ulong,
    );
    (*f).name = name;
    (*f).type_0 = type_0;
    (*f).static_member = 1 as libc::c_int != 0;
    (*f).u.s.physname = physname;
    (*f).visibility = visibility;
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_method(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut variants: *mut debug_method_variant,
) -> debug_method {
    let mut m: *mut debug_method_s = 0 as *mut debug_method_s;
    m = xmalloc(::core::mem::size_of::<debug_method_s>() as libc::c_ulong)
        as *mut debug_method_s;
    memset(
        m as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_method_s>() as libc::c_ulong,
    );
    (*m).name = name;
    (*m).variants = variants;
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_method_variant(
    mut handle: *mut libc::c_void,
    mut physname: *const libc::c_char,
    mut type_0: debug_type,
    mut visibility: debug_visibility,
    mut constp: bool,
    mut volatilep: bool,
    mut voffset: bfd_vma,
    mut context: debug_type,
) -> debug_method_variant {
    let mut m: *mut debug_method_variant_s = 0 as *mut debug_method_variant_s;
    m = xmalloc(::core::mem::size_of::<debug_method_variant_s>() as libc::c_ulong)
        as *mut debug_method_variant_s;
    memset(
        m as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_method_variant_s>() as libc::c_ulong,
    );
    (*m).physname = physname;
    (*m).type_0 = type_0;
    (*m).visibility = visibility;
    (*m).constp = constp;
    (*m).volatilep = volatilep;
    (*m).voffset = voffset;
    (*m).context = context;
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn debug_make_static_method_variant(
    mut handle: *mut libc::c_void,
    mut physname: *const libc::c_char,
    mut type_0: debug_type,
    mut visibility: debug_visibility,
    mut constp: bool,
    mut volatilep: bool,
) -> debug_method_variant {
    let mut m: *mut debug_method_variant_s = 0 as *mut debug_method_variant_s;
    m = xmalloc(::core::mem::size_of::<debug_method_variant_s>() as libc::c_ulong)
        as *mut debug_method_variant_s;
    memset(
        m as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_method_variant_s>() as libc::c_ulong,
    );
    (*m).physname = physname;
    (*m).type_0 = type_0;
    (*m).visibility = visibility;
    (*m).constp = constp;
    (*m).volatilep = volatilep;
    (*m).voffset = -(1 as libc::c_int) as bfd_vma;
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn debug_name_type(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut type_0: debug_type,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    let mut n: *mut debug_named_type = 0 as *mut debug_named_type;
    let mut nm: *mut debug_name = 0 as *mut debug_name;
    if name.is_null() || type_0.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    if ((*info).current_unit).is_null() || ((*info).current_file).is_null() {
        debug_error(
            dcgettext(
                0 as *const libc::c_char,
                b"debug_name_type: no current file\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as *mut libc::c_void as debug_type;
    }
    t = debug_make_type(info, DEBUG_KIND_NAMED, 0 as libc::c_int as libc::c_uint);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    n = xmalloc(::core::mem::size_of::<debug_named_type>() as libc::c_ulong)
        as *mut debug_named_type;
    memset(
        n as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_named_type>() as libc::c_ulong,
    );
    (*n).type_0 = type_0;
    (*t).u.knamed = n;
    nm = debug_add_to_namespace(
        info,
        &mut (*(*info).current_file).globals,
        name,
        DEBUG_OBJECT_TYPE,
        DEBUG_LINKAGE_NONE,
    );
    if nm.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    (*nm).u.type_0 = t;
    (*n).name = nm;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_tag_type(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut type_0: debug_type,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    let mut n: *mut debug_named_type = 0 as *mut debug_named_type;
    let mut nm: *mut debug_name = 0 as *mut debug_name;
    if name.is_null() || type_0.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    if ((*info).current_file).is_null() {
        debug_error(
            dcgettext(
                0 as *const libc::c_char,
                b"debug_tag_type: no current file\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as *mut libc::c_void as debug_type;
    }
    if (*type_0).kind as libc::c_uint == DEBUG_KIND_TAGGED as libc::c_int as libc::c_uint
    {
        if strcmp((*(*(*type_0).u.knamed).name).name, name) == 0 as libc::c_int {
            return type_0;
        }
        debug_error(
            dcgettext(
                0 as *const libc::c_char,
                b"debug_tag_type: extra tag attempted\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as *mut libc::c_void as debug_type;
    }
    t = debug_make_type(info, DEBUG_KIND_TAGGED, 0 as libc::c_int as libc::c_uint);
    if t.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    n = xmalloc(::core::mem::size_of::<debug_named_type>() as libc::c_ulong)
        as *mut debug_named_type;
    memset(
        n as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_named_type>() as libc::c_ulong,
    );
    (*n).type_0 = type_0;
    (*t).u.knamed = n;
    nm = debug_add_to_namespace(
        info,
        &mut (*(*info).current_file).globals,
        name,
        DEBUG_OBJECT_TAG,
        DEBUG_LINKAGE_NONE,
    );
    if nm.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    (*nm).u.tag = t;
    (*n).name = nm;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn debug_record_type_size(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
    mut size: libc::c_uint,
) -> bool {
    if (*type_0).size != 0 as libc::c_int as libc::c_uint && (*type_0).size != size {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Warning: changing type size from %d to %d\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*type_0).size,
            size,
        );
    }
    (*type_0).size = size;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_find_named_type(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut b: *mut debug_block = 0 as *mut debug_block;
    let mut f: *mut debug_file = 0 as *mut debug_file;
    if ((*info).current_unit).is_null() {
        debug_error(
            dcgettext(
                0 as *const libc::c_char,
                b"debug_find_named_type: no current compilation unit\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as *mut libc::c_void as debug_type;
    }
    b = (*info).current_block;
    while !b.is_null() {
        if !((*b).locals).is_null() {
            let mut n: *mut debug_name = 0 as *mut debug_name;
            n = (*(*b).locals).list;
            while !n.is_null() {
                if (*n).kind as libc::c_uint
                    == DEBUG_OBJECT_TYPE as libc::c_int as libc::c_uint
                    && *((*n).name).offset(0 as libc::c_int as isize) as libc::c_int
                        == *name.offset(0 as libc::c_int as isize) as libc::c_int
                    && strcmp((*n).name, name) == 0 as libc::c_int
                {
                    return (*n).u.type_0;
                }
                n = (*n).next;
            }
        }
        b = (*b).parent;
    }
    f = (*(*info).current_unit).files;
    while !f.is_null() {
        if !((*f).globals).is_null() {
            let mut n_0: *mut debug_name = 0 as *mut debug_name;
            n_0 = (*(*f).globals).list;
            while !n_0.is_null() {
                if (*n_0).kind as libc::c_uint
                    == DEBUG_OBJECT_TYPE as libc::c_int as libc::c_uint
                    && *((*n_0).name).offset(0 as libc::c_int as isize) as libc::c_int
                        == *name.offset(0 as libc::c_int as isize) as libc::c_int
                    && strcmp((*n_0).name, name) == 0 as libc::c_int
                {
                    return (*n_0).u.type_0;
                }
                n_0 = (*n_0).next;
            }
        }
        f = (*f).next;
    }
    return 0 as *mut libc::c_void as debug_type;
}
#[no_mangle]
pub unsafe extern "C" fn debug_find_tagged_type(
    mut handle: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut kind: debug_type_kind,
) -> debug_type {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut u: *mut debug_unit = 0 as *mut debug_unit;
    u = (*info).units;
    while !u.is_null() {
        let mut f: *mut debug_file = 0 as *mut debug_file;
        f = (*u).files;
        while !f.is_null() {
            let mut n: *mut debug_name = 0 as *mut debug_name;
            if !((*f).globals).is_null() {
                n = (*(*f).globals).list;
                while !n.is_null() {
                    if (*n).kind as libc::c_uint
                        == DEBUG_OBJECT_TAG as libc::c_int as libc::c_uint
                        && (kind as libc::c_uint
                            == DEBUG_KIND_ILLEGAL as libc::c_int as libc::c_uint
                            || (*(*n).u.tag).kind as libc::c_uint
                                == kind as libc::c_uint)
                        && *((*n).name).offset(0 as libc::c_int as isize) as libc::c_int
                            == *name.offset(0 as libc::c_int as isize) as libc::c_int
                        && strcmp((*n).name, name) == 0 as libc::c_int
                    {
                        return (*n).u.tag;
                    }
                    n = (*n).next;
                }
            }
            f = (*f).next;
        }
        u = (*u).next;
    }
    return 0 as *mut libc::c_void as debug_type;
}
#[no_mangle]
pub unsafe extern "C" fn debug_get_type_kind(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
) -> debug_type_kind {
    if type_0.is_null() {
        return DEBUG_KIND_ILLEGAL;
    }
    type_0 = debug_get_real_type(handle, type_0, 0 as *mut debug_type_real_list);
    if type_0.is_null() {
        return DEBUG_KIND_ILLEGAL;
    }
    return (*type_0).kind;
}
#[no_mangle]
pub unsafe extern "C" fn debug_get_type_name(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
) -> *const libc::c_char {
    if (*type_0).kind as libc::c_uint
        == DEBUG_KIND_INDIRECT as libc::c_int as libc::c_uint
    {
        if !(*(*(*type_0).u.kindirect).slot).is_null() {
            return debug_get_type_name(handle, *(*(*type_0).u.kindirect).slot);
        }
        return (*(*type_0).u.kindirect).tag;
    }
    if (*type_0).kind as libc::c_uint == DEBUG_KIND_NAMED as libc::c_int as libc::c_uint
        || (*type_0).kind as libc::c_uint
            == DEBUG_KIND_TAGGED as libc::c_int as libc::c_uint
    {
        return (*(*(*type_0).u.knamed).name).name;
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn debug_get_type_size(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
) -> bfd_vma {
    if type_0.is_null() {
        return 0 as libc::c_int as bfd_vma;
    }
    if (*type_0).size != 0 as libc::c_int as libc::c_uint {
        return (*type_0).size as bfd_vma;
    }
    match (*type_0).kind as libc::c_uint {
        1 => {
            if !(*(*(*type_0).u.kindirect).slot).is_null() {
                return debug_get_type_size(handle, *(*(*type_0).u.kindirect).slot);
            }
            return 0 as libc::c_int as bfd_vma;
        }
        22 | 23 => return debug_get_type_size(handle, (*(*type_0).u.knamed).type_0),
        _ => return 0 as libc::c_int as bfd_vma,
    };
}
#[no_mangle]
pub unsafe extern "C" fn debug_get_return_type(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
) -> debug_type {
    if type_0.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    type_0 = debug_get_real_type(handle, type_0, 0 as *mut debug_type_real_list);
    if type_0.is_null() {
        return 0 as *mut libc::c_void as debug_type;
    }
    match (*type_0).kind as libc::c_uint {
        13 => return (*(*type_0).u.kfunction).return_type,
        19 => return (*(*type_0).u.kmethod).return_type,
        _ => return 0 as *mut libc::c_void as debug_type,
    };
}
#[no_mangle]
pub unsafe extern "C" fn debug_get_parameter_types(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
    mut pvarargs: *mut bool,
) -> *const debug_type {
    if type_0.is_null() {
        return 0 as *const debug_type;
    }
    type_0 = debug_get_real_type(handle, type_0, 0 as *mut debug_type_real_list);
    if type_0.is_null() {
        return 0 as *const debug_type;
    }
    match (*type_0).kind as libc::c_uint {
        13 => {
            *pvarargs = (*(*type_0).u.kfunction).varargs;
            return (*(*type_0).u.kfunction).arg_types;
        }
        19 => {
            *pvarargs = (*(*type_0).u.kmethod).varargs;
            return (*(*type_0).u.kmethod).arg_types;
        }
        _ => return 0 as *const debug_type,
    };
}
#[no_mangle]
pub unsafe extern "C" fn debug_get_target_type(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
) -> debug_type {
    if type_0.is_null() {
        return 0 as debug_type;
    }
    type_0 = debug_get_real_type(handle, type_0, 0 as *mut debug_type_real_list);
    if type_0.is_null() {
        return 0 as debug_type;
    }
    match (*type_0).kind as libc::c_uint {
        12 => return (*type_0).u.kpointer,
        14 => return (*type_0).u.kreference,
        20 => return (*type_0).u.kconst,
        21 => return (*type_0).u.kvolatile,
        _ => return 0 as debug_type,
    };
}
#[no_mangle]
pub unsafe extern "C" fn debug_get_fields(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
) -> *const debug_field {
    if type_0.is_null() {
        return 0 as *const debug_field;
    }
    type_0 = debug_get_real_type(handle, type_0, 0 as *mut debug_type_real_list);
    if type_0.is_null() {
        return 0 as *const debug_field;
    }
    match (*type_0).kind as libc::c_uint {
        7 | 8 | 9 | 10 => return (*(*type_0).u.kclass).fields,
        _ => return 0 as *const debug_field,
    };
}
#[no_mangle]
pub unsafe extern "C" fn debug_get_field_type(
    mut handle: *mut libc::c_void,
    mut field: debug_field,
) -> debug_type {
    if field.is_null() {
        return 0 as debug_type;
    }
    return (*field).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn debug_get_field_name(
    mut handle: *mut libc::c_void,
    mut field: debug_field,
) -> *const libc::c_char {
    if field.is_null() {
        return 0 as *const libc::c_char;
    }
    return (*field).name;
}
#[no_mangle]
pub unsafe extern "C" fn debug_get_field_bitpos(
    mut handle: *mut libc::c_void,
    mut field: debug_field,
) -> bfd_vma {
    if field.is_null() || (*field).static_member as libc::c_int != 0 {
        return -(1 as libc::c_int) as bfd_vma;
    }
    return (*field).u.f.bitpos as bfd_vma;
}
#[no_mangle]
pub unsafe extern "C" fn debug_get_field_bitsize(
    mut handle: *mut libc::c_void,
    mut field: debug_field,
) -> bfd_vma {
    if field.is_null() || (*field).static_member as libc::c_int != 0 {
        return -(1 as libc::c_int) as bfd_vma;
    }
    return (*field).u.f.bitsize as bfd_vma;
}
#[no_mangle]
pub unsafe extern "C" fn debug_get_field_visibility(
    mut handle: *mut libc::c_void,
    mut field: debug_field,
) -> debug_visibility {
    if field.is_null() {
        return DEBUG_VISIBILITY_IGNORE;
    }
    return (*field).visibility;
}
#[no_mangle]
pub unsafe extern "C" fn debug_get_field_physname(
    mut handle: *mut libc::c_void,
    mut field: debug_field,
) -> *const libc::c_char {
    if field.is_null() || !(*field).static_member {
        return 0 as *const libc::c_char;
    }
    return (*field).u.s.physname;
}
#[no_mangle]
pub unsafe extern "C" fn debug_write(
    mut handle: *mut libc::c_void,
    mut fns: *const debug_write_fns,
    mut fhandle: *mut libc::c_void,
) -> bool {
    let mut info: *mut debug_handle = handle as *mut debug_handle;
    let mut u: *mut debug_unit = 0 as *mut debug_unit;
    (*info).mark = ((*info).mark).wrapping_add(1);
    (*info).mark;
    (*info).base_id = (*info).class_id;
    (*info).id_list = 0 as *mut debug_class_id;
    u = (*info).units;
    while !u.is_null() {
        let mut f: *mut debug_file = 0 as *mut debug_file;
        let mut first_file: bool = false;
        (*info).current_write_lineno = (*u).linenos;
        (*info).current_write_lineno_index = 0 as libc::c_int as libc::c_uint;
        if !(Some(((*fns).start_compilation_unit).expect("non-null function pointer")))
            .expect("non-null function pointer")(fhandle, (*(*u).files).filename)
        {
            return 0 as libc::c_int != 0;
        }
        first_file = 1 as libc::c_int != 0;
        f = (*u).files;
        while !f.is_null() {
            let mut n: *mut debug_name = 0 as *mut debug_name;
            if first_file {
                first_file = 0 as libc::c_int != 0;
            } else if !(Some(((*fns).start_source).expect("non-null function pointer")))
                .expect("non-null function pointer")(fhandle, (*f).filename)
            {
                return 0 as libc::c_int != 0
            }
            if !((*f).globals).is_null() {
                n = (*(*f).globals).list;
                while !n.is_null() {
                    if !debug_write_name(info, fns, fhandle, n) {
                        return 0 as libc::c_int != 0;
                    }
                    n = (*n).next;
                }
            }
            f = (*f).next;
        }
        if !debug_write_linenos(info, fns, fhandle, -(1 as libc::c_int) as bfd_vma) {
            return 0 as libc::c_int != 0;
        }
        u = (*u).next;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn debug_error(mut message: *const libc::c_char) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, message);
}
unsafe extern "C" fn debug_add_to_namespace(
    mut info: *mut debug_handle,
    mut nsp: *mut *mut debug_namespace,
    mut name: *const libc::c_char,
    mut kind: debug_object_kind,
    mut linkage: debug_object_linkage,
) -> *mut debug_name {
    let mut n: *mut debug_name = 0 as *mut debug_name;
    let mut ns: *mut debug_namespace = 0 as *mut debug_namespace;
    n = xmalloc(::core::mem::size_of::<debug_name>() as libc::c_ulong)
        as *mut debug_name;
    memset(
        n as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_name>() as libc::c_ulong,
    );
    (*n).name = name;
    (*n).kind = kind;
    (*n).linkage = linkage;
    ns = *nsp;
    if ns.is_null() {
        ns = xmalloc(::core::mem::size_of::<debug_namespace>() as libc::c_ulong)
            as *mut debug_namespace;
        memset(
            ns as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<debug_namespace>() as libc::c_ulong,
        );
        (*ns).tail = &mut (*ns).list;
        *nsp = ns;
    }
    *(*ns).tail = n;
    (*ns).tail = &mut (*n).next;
    return n;
}
unsafe extern "C" fn debug_add_to_current_namespace(
    mut info: *mut debug_handle,
    mut name: *const libc::c_char,
    mut kind: debug_object_kind,
    mut linkage: debug_object_linkage,
) -> *mut debug_name {
    let mut nsp: *mut *mut debug_namespace = 0 as *mut *mut debug_namespace;
    if ((*info).current_unit).is_null() || ((*info).current_file).is_null() {
        debug_error(
            dcgettext(
                0 as *const libc::c_char,
                b"debug_add_to_current_namespace: no current file\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as *mut debug_name;
    }
    if !((*info).current_block).is_null() {
        nsp = &mut (*(*info).current_block).locals;
    } else {
        nsp = &mut (*(*info).current_file).globals;
    }
    return debug_add_to_namespace(info, nsp, name, kind, linkage);
}
unsafe extern "C" fn debug_make_type(
    mut info: *mut debug_handle,
    mut kind: debug_type_kind,
    mut size: libc::c_uint,
) -> *mut debug_type_s {
    let mut t: *mut debug_type_s = 0 as *mut debug_type_s;
    t = xmalloc(::core::mem::size_of::<debug_type_s>() as libc::c_ulong)
        as *mut debug_type_s;
    memset(
        t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_type_s>() as libc::c_ulong,
    );
    (*t).kind = kind;
    (*t).size = size;
    return t;
}
unsafe extern "C" fn debug_get_real_type(
    mut handle: *mut libc::c_void,
    mut type_0: debug_type,
    mut list: *mut debug_type_real_list,
) -> *mut debug_type_s {
    let mut l: *mut debug_type_real_list = 0 as *mut debug_type_real_list;
    let mut rl: debug_type_real_list = debug_type_real_list {
        next: 0 as *mut debug_type_real_list,
        t: 0 as *mut debug_type_s,
    };
    match (*type_0).kind as libc::c_uint {
        1 | 22 | 23 => {}
        _ => return type_0,
    }
    l = list;
    while !l.is_null() {
        if (*l).t == type_0 || l == (*l).next {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"debug_get_real_type: circular debug information for %s\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                debug_get_type_name(handle, type_0),
            );
            return 0 as *mut debug_type_s;
        }
        l = (*l).next;
    }
    rl.next = list;
    rl.t = type_0;
    match (*type_0).kind as libc::c_uint {
        22 | 23 => {
            return debug_get_real_type(handle, (*(*type_0).u.knamed).type_0, &mut rl);
        }
        1 | _ => {
            if !(*(*(*type_0).u.kindirect).slot).is_null() {
                return debug_get_real_type(
                    handle,
                    *(*(*type_0).u.kindirect).slot,
                    &mut rl,
                );
            }
            return type_0;
        }
    };
}
unsafe extern "C" fn debug_write_name(
    mut info: *mut debug_handle,
    mut fns: *const debug_write_fns,
    mut fhandle: *mut libc::c_void,
    mut n: *mut debug_name,
) -> bool {
    match (*n).kind as libc::c_uint {
        0 => {
            if !debug_write_type(info, fns, fhandle, (*n).u.type_0, n)
                || !(Some(((*fns).typdef).expect("non-null function pointer")))
                    .expect("non-null function pointer")(fhandle, (*n).name)
            {
                return 0 as libc::c_int != 0;
            }
            return 1 as libc::c_int != 0;
        }
        1 => {
            if !debug_write_type(info, fns, fhandle, (*n).u.tag, n) {
                return 0 as libc::c_int != 0;
            }
            return (Some(((*fns).tag).expect("non-null function pointer")))
                .expect("non-null function pointer")(fhandle, (*n).name);
        }
        2 => {
            if !debug_write_type(
                info,
                fns,
                fhandle,
                (*(*n).u.variable).type_0,
                0 as *mut libc::c_void as *mut debug_name,
            ) {
                return 0 as libc::c_int != 0;
            }
            return (Some(((*fns).variable).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(fhandle, (*n).name, (*(*n).u.variable).kind, (*(*n).u.variable).val);
        }
        3 => {
            return debug_write_function(
                info,
                fns,
                fhandle,
                (*n).name,
                (*n).linkage,
                (*n).u.function,
            );
        }
        4 => {
            return (Some(((*fns).int_constant).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(fhandle, (*n).name, (*n).u.int_constant);
        }
        5 => {
            return (Some(((*fns).float_constant).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(fhandle, (*n).name, (*n).u.float_constant);
        }
        6 => {
            if !debug_write_type(
                info,
                fns,
                fhandle,
                (*(*n).u.typed_constant).type_0,
                0 as *mut libc::c_void as *mut debug_name,
            ) {
                return 0 as libc::c_int != 0;
            }
            return (Some(((*fns).typed_constant).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(fhandle, (*n).name, (*(*n).u.typed_constant).val);
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn debug_write_type(
    mut info: *mut debug_handle,
    mut fns: *const debug_write_fns,
    mut fhandle: *mut libc::c_void,
    mut type_0: *mut debug_type_s,
    mut name: *mut debug_name,
) -> bool {
    let mut i: libc::c_uint = 0;
    let mut is: libc::c_int = 0;
    let mut tag: *const libc::c_char = 0 as *const libc::c_char;
    if type_0.is_null() {
        return (Some(((*fns).empty_type).expect("non-null function pointer")))
            .expect("non-null function pointer")(fhandle);
    }
    if ((*type_0).kind as libc::c_uint == DEBUG_KIND_NAMED as libc::c_int as libc::c_uint
        || (*type_0).kind as libc::c_uint
            == DEBUG_KIND_TAGGED as libc::c_int as libc::c_uint)
        && ((*(*(*type_0).u.knamed).name).mark == (*info).mark
            || (*type_0).kind as libc::c_uint
                == DEBUG_KIND_TAGGED as libc::c_int as libc::c_uint
                && (*(*type_0).u.knamed).name != name)
    {
        if (*type_0).kind as libc::c_uint
            == DEBUG_KIND_NAMED as libc::c_int as libc::c_uint
        {
            return (Some(((*fns).typedef_type).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(fhandle, (*(*(*type_0).u.knamed).name).name)
        } else {
            let mut real: *mut debug_type_s = 0 as *mut debug_type_s;
            let mut id: libc::c_uint = 0;
            real = debug_get_real_type(
                info as *mut libc::c_void,
                type_0,
                0 as *mut debug_type_real_list,
            );
            if real.is_null() {
                return (Some(((*fns).empty_type).expect("non-null function pointer")))
                    .expect("non-null function pointer")(fhandle);
            }
            id = 0 as libc::c_int as libc::c_uint;
            if ((*real).kind as libc::c_uint
                == DEBUG_KIND_STRUCT as libc::c_int as libc::c_uint
                || (*real).kind as libc::c_uint
                    == DEBUG_KIND_UNION as libc::c_int as libc::c_uint
                || (*real).kind as libc::c_uint
                    == DEBUG_KIND_CLASS as libc::c_int as libc::c_uint
                || (*real).kind as libc::c_uint
                    == DEBUG_KIND_UNION_CLASS as libc::c_int as libc::c_uint)
                && !((*real).u.kclass).is_null()
            {
                if (*(*real).u.kclass).id <= (*info).base_id {
                    if !debug_set_class_id(
                        info,
                        (*(*(*type_0).u.knamed).name).name,
                        real,
                    ) {
                        return 0 as libc::c_int != 0;
                    }
                }
                id = (*(*real).u.kclass).id;
            }
            return (Some(((*fns).tag_type).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(fhandle, (*(*(*type_0).u.knamed).name).name, id, (*real).kind);
        }
    }
    if !name.is_null() {
        (*name).mark = (*info).mark;
    }
    if !name.is_null()
        && (*type_0).kind as libc::c_uint
            != DEBUG_KIND_NAMED as libc::c_int as libc::c_uint
        && (*type_0).kind as libc::c_uint
            != DEBUG_KIND_TAGGED as libc::c_int as libc::c_uint
    {
        if (*name).kind as libc::c_uint
            == DEBUG_OBJECT_TAG as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"name->kind == DEBUG_OBJECT_TAG\0" as *const u8 as *const libc::c_char,
                b"debug.c\0" as *const u8 as *const libc::c_char,
                2477 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 130],
                    &[libc::c_char; 130],
                >(
                    b"_Bool debug_write_type(struct debug_handle *, const struct debug_write_fns *, void *, struct debug_type_s *, struct debug_name *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_21528: {
            if (*name).kind as libc::c_uint
                == DEBUG_OBJECT_TAG as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"name->kind == DEBUG_OBJECT_TAG\0" as *const u8
                        as *const libc::c_char,
                    b"debug.c\0" as *const u8 as *const libc::c_char,
                    2477 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 130],
                        &[libc::c_char; 130],
                    >(
                        b"_Bool debug_write_type(struct debug_handle *, const struct debug_write_fns *, void *, struct debug_type_s *, struct debug_name *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        tag = (*name).name;
    }
    match (*type_0).kind as libc::c_uint {
        0 => {
            debug_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"debug_write_type: illegal type encountered\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return 0 as libc::c_int != 0;
        }
        1 => {
            return debug_write_type(
                info,
                fns,
                fhandle,
                *(*(*type_0).u.kindirect).slot,
                name,
            );
        }
        2 => {
            return (Some(((*fns).void_type).expect("non-null function pointer")))
                .expect("non-null function pointer")(fhandle);
        }
        3 => {
            return (Some(((*fns).int_type).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(fhandle, (*type_0).size, (*type_0).u.kint);
        }
        4 => {
            return (Some(((*fns).float_type).expect("non-null function pointer")))
                .expect("non-null function pointer")(fhandle, (*type_0).size);
        }
        5 => {
            return (Some(((*fns).complex_type).expect("non-null function pointer")))
                .expect("non-null function pointer")(fhandle, (*type_0).size);
        }
        6 => {
            return (Some(((*fns).bool_type).expect("non-null function pointer")))
                .expect("non-null function pointer")(fhandle, (*type_0).size);
        }
        7 | 8 => {
            if !((*type_0).u.kclass).is_null() {
                if (*(*type_0).u.kclass).id <= (*info).base_id {
                    if !debug_set_class_id(info, tag, type_0) {
                        return 0 as libc::c_int != 0;
                    }
                }
                if (*info).mark == (*(*type_0).u.kclass).mark {
                    if (*(*type_0).u.kclass).id > (*info).base_id {} else {
                        __assert_fail(
                            b"type->u.kclass->id > info->base_id\0" as *const u8
                                as *const libc::c_char,
                            b"debug.c\0" as *const u8 as *const libc::c_char,
                            2514 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 130],
                                &[libc::c_char; 130],
                            >(
                                b"_Bool debug_write_type(struct debug_handle *, const struct debug_write_fns *, void *, struct debug_type_s *, struct debug_name *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_21275: {
                        if (*(*type_0).u.kclass).id > (*info).base_id {} else {
                            __assert_fail(
                                b"type->u.kclass->id > info->base_id\0" as *const u8
                                    as *const libc::c_char,
                                b"debug.c\0" as *const u8 as *const libc::c_char,
                                2514 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 130],
                                    &[libc::c_char; 130],
                                >(
                                    b"_Bool debug_write_type(struct debug_handle *, const struct debug_write_fns *, void *, struct debug_type_s *, struct debug_name *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    return (Some(((*fns).tag_type).expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(fhandle, tag, (*(*type_0).u.kclass).id, (*type_0).kind);
                }
                (*(*type_0).u.kclass).mark = (*info).mark;
            }
            if !(Some(((*fns).start_struct_type).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                fhandle,
                tag,
                if !((*type_0).u.kclass).is_null() {
                    (*(*type_0).u.kclass).id
                } else {
                    0 as libc::c_int as libc::c_uint
                },
                (*type_0).kind as libc::c_uint
                    == DEBUG_KIND_STRUCT as libc::c_int as libc::c_uint,
                (*type_0).size,
            )
            {
                return 0 as libc::c_int != 0;
            }
            if !((*type_0).u.kclass).is_null()
                && !((*(*type_0).u.kclass).fields).is_null()
            {
                i = 0 as libc::c_int as libc::c_uint;
                while !(*((*(*type_0).u.kclass).fields).offset(i as isize)).is_null() {
                    let mut f: *mut debug_field_s = 0 as *mut debug_field_s;
                    f = *((*(*type_0).u.kclass).fields).offset(i as isize);
                    if !debug_write_type(
                        info,
                        fns,
                        fhandle,
                        (*f).type_0,
                        0 as *mut libc::c_void as *mut debug_name,
                    )
                        || !(Some(
                            ((*fns).struct_field).expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            fhandle,
                            (*f).name,
                            (*f).u.f.bitpos as bfd_vma,
                            (*f).u.f.bitsize as bfd_vma,
                            (*f).visibility,
                        )
                    {
                        return 0 as libc::c_int != 0;
                    }
                    i = i.wrapping_add(1);
                    i;
                }
            }
            return (Some(((*fns).end_struct_type).expect("non-null function pointer")))
                .expect("non-null function pointer")(fhandle);
        }
        9 | 10 => return debug_write_class_type(info, fns, fhandle, type_0, tag),
        11 => {
            if ((*type_0).u.kenum).is_null() {
                return (Some(((*fns).enum_type).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    fhandle,
                    tag,
                    0 as *mut libc::c_void as *mut *const libc::c_char,
                    0 as *mut libc::c_void as *mut bfd_signed_vma,
                );
            }
            return (Some(((*fns).enum_type).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(fhandle, tag, (*(*type_0).u.kenum).names, (*(*type_0).u.kenum).values);
        }
        12 => {
            if !debug_write_type(
                info,
                fns,
                fhandle,
                (*type_0).u.kpointer,
                0 as *mut libc::c_void as *mut debug_name,
            ) {
                return 0 as libc::c_int != 0;
            }
            return (Some(((*fns).pointer_type).expect("non-null function pointer")))
                .expect("non-null function pointer")(fhandle);
        }
        13 => {
            if !debug_write_type(
                info,
                fns,
                fhandle,
                (*(*type_0).u.kfunction).return_type,
                0 as *mut libc::c_void as *mut debug_name,
            ) {
                return 0 as libc::c_int != 0;
            }
            if ((*(*type_0).u.kfunction).arg_types).is_null() {
                is = -(1 as libc::c_int);
            } else {
                is = 0 as libc::c_int;
                while !(*((*(*type_0).u.kfunction).arg_types).offset(is as isize))
                    .is_null()
                {
                    if !debug_write_type(
                        info,
                        fns,
                        fhandle,
                        *((*(*type_0).u.kfunction).arg_types).offset(is as isize),
                        0 as *mut libc::c_void as *mut debug_name,
                    ) {
                        return 0 as libc::c_int != 0;
                    }
                    is += 1;
                    is;
                }
            }
            return (Some(((*fns).function_type).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(fhandle, is, (*(*type_0).u.kfunction).varargs);
        }
        14 => {
            if !debug_write_type(
                info,
                fns,
                fhandle,
                (*type_0).u.kreference,
                0 as *mut libc::c_void as *mut debug_name,
            ) {
                return 0 as libc::c_int != 0;
            }
            return (Some(((*fns).reference_type).expect("non-null function pointer")))
                .expect("non-null function pointer")(fhandle);
        }
        15 => {
            if !debug_write_type(
                info,
                fns,
                fhandle,
                (*(*type_0).u.krange).type_0,
                0 as *mut libc::c_void as *mut debug_name,
            ) {
                return 0 as libc::c_int != 0;
            }
            return (Some(((*fns).range_type).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(fhandle, (*(*type_0).u.krange).lower, (*(*type_0).u.krange).upper);
        }
        16 => {
            if !debug_write_type(
                info,
                fns,
                fhandle,
                (*(*type_0).u.karray).element_type,
                0 as *mut libc::c_void as *mut debug_name,
            )
                || !debug_write_type(
                    info,
                    fns,
                    fhandle,
                    (*(*type_0).u.karray).range_type,
                    0 as *mut libc::c_void as *mut debug_name,
                )
            {
                return 0 as libc::c_int != 0;
            }
            return (Some(((*fns).array_type).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                fhandle,
                (*(*type_0).u.karray).lower,
                (*(*type_0).u.karray).upper,
                (*(*type_0).u.karray).stringp,
            );
        }
        17 => {
            if !debug_write_type(
                info,
                fns,
                fhandle,
                (*(*type_0).u.kset).type_0,
                0 as *mut libc::c_void as *mut debug_name,
            ) {
                return 0 as libc::c_int != 0;
            }
            return (Some(((*fns).set_type).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(fhandle, (*(*type_0).u.kset).bitstringp);
        }
        18 => {
            if !debug_write_type(
                info,
                fns,
                fhandle,
                (*(*type_0).u.koffset).base_type,
                0 as *mut libc::c_void as *mut debug_name,
            )
                || !debug_write_type(
                    info,
                    fns,
                    fhandle,
                    (*(*type_0).u.koffset).target_type,
                    0 as *mut libc::c_void as *mut debug_name,
                )
            {
                return 0 as libc::c_int != 0;
            }
            return (Some(((*fns).offset_type).expect("non-null function pointer")))
                .expect("non-null function pointer")(fhandle);
        }
        19 => {
            if !debug_write_type(
                info,
                fns,
                fhandle,
                (*(*type_0).u.kmethod).return_type,
                0 as *mut libc::c_void as *mut debug_name,
            ) {
                return 0 as libc::c_int != 0;
            }
            if ((*(*type_0).u.kmethod).arg_types).is_null() {
                is = -(1 as libc::c_int);
            } else {
                is = 0 as libc::c_int;
                while !(*((*(*type_0).u.kmethod).arg_types).offset(is as isize))
                    .is_null()
                {
                    if !debug_write_type(
                        info,
                        fns,
                        fhandle,
                        *((*(*type_0).u.kmethod).arg_types).offset(is as isize),
                        0 as *mut libc::c_void as *mut debug_name,
                    ) {
                        return 0 as libc::c_int != 0;
                    }
                    is += 1;
                    is;
                }
            }
            if !((*(*type_0).u.kmethod).domain_type).is_null() {
                if !debug_write_type(
                    info,
                    fns,
                    fhandle,
                    (*(*type_0).u.kmethod).domain_type,
                    0 as *mut libc::c_void as *mut debug_name,
                ) {
                    return 0 as libc::c_int != 0;
                }
            }
            return (Some(((*fns).method_type).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                fhandle,
                !((*(*type_0).u.kmethod).domain_type).is_null(),
                is,
                (*(*type_0).u.kmethod).varargs,
            );
        }
        20 => {
            if !debug_write_type(
                info,
                fns,
                fhandle,
                (*type_0).u.kconst,
                0 as *mut libc::c_void as *mut debug_name,
            ) {
                return 0 as libc::c_int != 0;
            }
            return (Some(((*fns).const_type).expect("non-null function pointer")))
                .expect("non-null function pointer")(fhandle);
        }
        21 => {
            if !debug_write_type(
                info,
                fns,
                fhandle,
                (*type_0).u.kvolatile,
                0 as *mut libc::c_void as *mut debug_name,
            ) {
                return 0 as libc::c_int != 0;
            }
            return (Some(((*fns).volatile_type).expect("non-null function pointer")))
                .expect("non-null function pointer")(fhandle);
        }
        22 => {
            return debug_write_type(
                info,
                fns,
                fhandle,
                (*(*type_0).u.knamed).type_0,
                0 as *mut libc::c_void as *mut debug_name,
            );
        }
        23 => {
            return debug_write_type(
                info,
                fns,
                fhandle,
                (*(*type_0).u.knamed).type_0,
                (*(*type_0).u.knamed).name,
            );
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn debug_write_class_type(
    mut info: *mut debug_handle,
    mut fns: *const debug_write_fns,
    mut fhandle: *mut libc::c_void,
    mut type_0: *mut debug_type_s,
    mut tag: *const libc::c_char,
) -> bool {
    let mut i: libc::c_uint = 0;
    let mut id: libc::c_uint = 0;
    let mut vptrbase: *mut debug_type_s = 0 as *mut debug_type_s;
    if ((*type_0).u.kclass).is_null() {
        id = 0 as libc::c_int as libc::c_uint;
        vptrbase = 0 as *mut debug_type_s;
    } else {
        if (*(*type_0).u.kclass).id <= (*info).base_id {
            if !debug_set_class_id(info, tag, type_0) {
                return 0 as libc::c_int != 0;
            }
        }
        if (*info).mark == (*(*type_0).u.kclass).mark {
            if (*(*type_0).u.kclass).id > (*info).base_id {} else {
                __assert_fail(
                    b"type->u.kclass->id > info->base_id\0" as *const u8
                        as *const libc::c_char,
                    b"debug.c\0" as *const u8 as *const libc::c_char,
                    2686 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 129],
                        &[libc::c_char; 129],
                    >(
                        b"_Bool debug_write_class_type(struct debug_handle *, const struct debug_write_fns *, void *, struct debug_type_s *, const char *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_18212: {
                if (*(*type_0).u.kclass).id > (*info).base_id {} else {
                    __assert_fail(
                        b"type->u.kclass->id > info->base_id\0" as *const u8
                            as *const libc::c_char,
                        b"debug.c\0" as *const u8 as *const libc::c_char,
                        2686 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 129],
                            &[libc::c_char; 129],
                        >(
                            b"_Bool debug_write_class_type(struct debug_handle *, const struct debug_write_fns *, void *, struct debug_type_s *, const char *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            return (Some(((*fns).tag_type).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(fhandle, tag, (*(*type_0).u.kclass).id, (*type_0).kind);
        }
        (*(*type_0).u.kclass).mark = (*info).mark;
        id = (*(*type_0).u.kclass).id;
        vptrbase = (*(*type_0).u.kclass).vptrbase;
        if !vptrbase.is_null() && vptrbase != type_0 {
            if !debug_write_type(
                info,
                fns,
                fhandle,
                vptrbase,
                0 as *mut libc::c_void as *mut debug_name,
            ) {
                return 0 as libc::c_int != 0;
            }
        }
    }
    if !(Some(((*fns).start_class_type).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        fhandle,
        tag,
        id,
        (*type_0).kind as libc::c_uint
            == DEBUG_KIND_CLASS as libc::c_int as libc::c_uint,
        (*type_0).size,
        !vptrbase.is_null(),
        vptrbase == type_0,
    )
    {
        return 0 as libc::c_int != 0;
    }
    if !((*type_0).u.kclass).is_null() {
        if !((*(*type_0).u.kclass).fields).is_null() {
            i = 0 as libc::c_int as libc::c_uint;
            while !(*((*(*type_0).u.kclass).fields).offset(i as isize)).is_null() {
                let mut f: *mut debug_field_s = 0 as *mut debug_field_s;
                f = *((*(*type_0).u.kclass).fields).offset(i as isize);
                if !debug_write_type(
                    info,
                    fns,
                    fhandle,
                    (*f).type_0,
                    0 as *mut libc::c_void as *mut debug_name,
                ) {
                    return 0 as libc::c_int != 0;
                }
                if (*f).static_member {
                    if !(Some(
                        ((*fns).class_static_member).expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(fhandle, (*f).name, (*f).u.s.physname, (*f).visibility)
                    {
                        return 0 as libc::c_int != 0;
                    }
                } else if !(Some(
                    ((*fns).struct_field).expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    fhandle,
                    (*f).name,
                    (*f).u.f.bitpos as bfd_vma,
                    (*f).u.f.bitsize as bfd_vma,
                    (*f).visibility,
                )
                {
                    return 0 as libc::c_int != 0
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        if !((*(*type_0).u.kclass).baseclasses).is_null() {
            i = 0 as libc::c_int as libc::c_uint;
            while !(*((*(*type_0).u.kclass).baseclasses).offset(i as isize)).is_null() {
                let mut b: *mut debug_baseclass_s = 0 as *mut debug_baseclass_s;
                b = *((*(*type_0).u.kclass).baseclasses).offset(i as isize);
                if !debug_write_type(
                    info,
                    fns,
                    fhandle,
                    (*b).type_0,
                    0 as *mut libc::c_void as *mut debug_name,
                ) {
                    return 0 as libc::c_int != 0;
                }
                if !(Some(((*fns).class_baseclass).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(fhandle, (*b).bitpos as bfd_vma, (*b).is_virtual, (*b).visibility)
                {
                    return 0 as libc::c_int != 0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        if !((*(*type_0).u.kclass).methods).is_null() {
            i = 0 as libc::c_int as libc::c_uint;
            while !(*((*(*type_0).u.kclass).methods).offset(i as isize)).is_null() {
                let mut m: *mut debug_method_s = 0 as *mut debug_method_s;
                let mut j: libc::c_uint = 0;
                m = *((*(*type_0).u.kclass).methods).offset(i as isize);
                if !(Some(
                    ((*fns).class_start_method).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(fhandle, (*m).name)
                {
                    return 0 as libc::c_int != 0;
                }
                j = 0 as libc::c_int as libc::c_uint;
                while !(*((*m).variants).offset(j as isize)).is_null() {
                    let mut v: *mut debug_method_variant_s = 0
                        as *mut debug_method_variant_s;
                    v = *((*m).variants).offset(j as isize);
                    if !((*v).context).is_null() {
                        if !debug_write_type(
                            info,
                            fns,
                            fhandle,
                            (*v).context,
                            0 as *mut libc::c_void as *mut debug_name,
                        ) {
                            return 0 as libc::c_int != 0;
                        }
                    }
                    if !debug_write_type(
                        info,
                        fns,
                        fhandle,
                        (*v).type_0,
                        0 as *mut libc::c_void as *mut debug_name,
                    ) {
                        return 0 as libc::c_int != 0;
                    }
                    if (*v).voffset != -(1 as libc::c_int) as bfd_vma {
                        if !(Some(
                            ((*fns).class_method_variant)
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            fhandle,
                            (*v).physname,
                            (*v).visibility,
                            (*v).constp,
                            (*v).volatilep,
                            (*v).voffset,
                            !((*v).context).is_null(),
                        )
                        {
                            return 0 as libc::c_int != 0;
                        }
                    } else if !(Some(
                        ((*fns).class_static_method_variant)
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        fhandle,
                        (*v).physname,
                        (*v).visibility,
                        (*v).constp,
                        (*v).volatilep,
                    )
                    {
                        return 0 as libc::c_int != 0
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                if !(Some(((*fns).class_end_method).expect("non-null function pointer")))
                    .expect("non-null function pointer")(fhandle)
                {
                    return 0 as libc::c_int != 0;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    return (Some(((*fns).end_class_type).expect("non-null function pointer")))
        .expect("non-null function pointer")(fhandle);
}
unsafe extern "C" fn debug_write_function(
    mut info: *mut debug_handle,
    mut fns: *const debug_write_fns,
    mut fhandle: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut linkage: debug_object_linkage,
    mut function: *mut debug_function,
) -> bool {
    let mut p: *mut debug_parameter = 0 as *mut debug_parameter;
    let mut b: *mut debug_block = 0 as *mut debug_block;
    if !debug_write_linenos(info, fns, fhandle, (*(*function).blocks).start) {
        return 0 as libc::c_int != 0;
    }
    if !debug_write_type(
        info,
        fns,
        fhandle,
        (*function).return_type,
        0 as *mut libc::c_void as *mut debug_name,
    ) {
        return 0 as libc::c_int != 0;
    }
    if !(Some(((*fns).start_function).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        fhandle,
        name,
        linkage as libc::c_uint == DEBUG_LINKAGE_GLOBAL as libc::c_int as libc::c_uint,
    )
    {
        return 0 as libc::c_int != 0;
    }
    p = (*function).parameters;
    while !p.is_null() {
        if !debug_write_type(
            info,
            fns,
            fhandle,
            (*p).type_0,
            0 as *mut libc::c_void as *mut debug_name,
        )
            || !(Some(((*fns).function_parameter).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(fhandle, (*p).name, (*p).kind, (*p).val)
        {
            return 0 as libc::c_int != 0;
        }
        p = (*p).next;
    }
    b = (*function).blocks;
    while !b.is_null() {
        if !debug_write_block(info, fns, fhandle, b) {
            return 0 as libc::c_int != 0;
        }
        b = (*b).next;
    }
    return (Some(((*fns).end_function).expect("non-null function pointer")))
        .expect("non-null function pointer")(fhandle);
}
unsafe extern "C" fn debug_write_block(
    mut info: *mut debug_handle,
    mut fns: *const debug_write_fns,
    mut fhandle: *mut libc::c_void,
    mut block: *mut debug_block,
) -> bool {
    let mut n: *mut debug_name = 0 as *mut debug_name;
    let mut b: *mut debug_block = 0 as *mut debug_block;
    if !debug_write_linenos(info, fns, fhandle, (*block).start) {
        return 0 as libc::c_int != 0;
    }
    if !((*block).locals).is_null() || ((*block).parent).is_null() {
        if !(Some(((*fns).start_block).expect("non-null function pointer")))
            .expect("non-null function pointer")(fhandle, (*block).start)
        {
            return 0 as libc::c_int != 0;
        }
    }
    if !((*block).locals).is_null() {
        n = (*(*block).locals).list;
        while !n.is_null() {
            if !debug_write_name(info, fns, fhandle, n) {
                return 0 as libc::c_int != 0;
            }
            n = (*n).next;
        }
    }
    b = (*block).children;
    while !b.is_null() {
        if !debug_write_block(info, fns, fhandle, b) {
            return 0 as libc::c_int != 0;
        }
        b = (*b).next;
    }
    if !debug_write_linenos(info, fns, fhandle, (*block).end) {
        return 0 as libc::c_int != 0;
    }
    if !((*block).locals).is_null() || ((*block).parent).is_null() {
        if !(Some(((*fns).end_block).expect("non-null function pointer")))
            .expect("non-null function pointer")(fhandle, (*block).end)
        {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn debug_write_linenos(
    mut info: *mut debug_handle,
    mut fns: *const debug_write_fns,
    mut fhandle: *mut libc::c_void,
    mut address: bfd_vma,
) -> bool {
    while !((*info).current_write_lineno).is_null() {
        let mut l: *mut debug_lineno = 0 as *mut debug_lineno;
        l = (*info).current_write_lineno;
        while (*info).current_write_lineno_index < 10 as libc::c_int as libc::c_uint {
            if (*l).linenos[(*info).current_write_lineno_index as usize]
                == -(1 as libc::c_int) as libc::c_ulong
            {
                break;
            }
            if (*l).addrs[(*info).current_write_lineno_index as usize] >= address {
                return 1 as libc::c_int != 0;
            }
            if !(Some(((*fns).lineno).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                fhandle,
                (*(*l).file).filename,
                (*l).linenos[(*info).current_write_lineno_index as usize],
                (*l).addrs[(*info).current_write_lineno_index as usize],
            )
            {
                return 0 as libc::c_int != 0;
            }
            (*info)
                .current_write_lineno_index = ((*info).current_write_lineno_index)
                .wrapping_add(1);
            (*info).current_write_lineno_index;
        }
        (*info).current_write_lineno = (*l).next;
        (*info).current_write_lineno_index = 0 as libc::c_int as libc::c_uint;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn debug_set_class_id(
    mut info: *mut debug_handle,
    mut tag: *const libc::c_char,
    mut type_0: *mut debug_type_s,
) -> bool {
    let mut c: *mut debug_class_type = 0 as *mut debug_class_type;
    let mut l: *mut debug_class_id = 0 as *mut debug_class_id;
    if (*type_0).kind as libc::c_uint == DEBUG_KIND_STRUCT as libc::c_int as libc::c_uint
        || (*type_0).kind as libc::c_uint
            == DEBUG_KIND_UNION as libc::c_int as libc::c_uint
        || (*type_0).kind as libc::c_uint
            == DEBUG_KIND_CLASS as libc::c_int as libc::c_uint
        || (*type_0).kind as libc::c_uint
            == DEBUG_KIND_UNION_CLASS as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"type->kind == DEBUG_KIND_STRUCT || type->kind == DEBUG_KIND_UNION || type->kind == DEBUG_KIND_CLASS || type->kind == DEBUG_KIND_UNION_CLASS\0"
                as *const u8 as *const libc::c_char,
            b"debug.c\0" as *const u8 as *const libc::c_char,
            2945 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 85],
                &[libc::c_char; 85],
            >(
                b"_Bool debug_set_class_id(struct debug_handle *, const char *, struct debug_type_s *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_20910: {
        if (*type_0).kind as libc::c_uint
            == DEBUG_KIND_STRUCT as libc::c_int as libc::c_uint
            || (*type_0).kind as libc::c_uint
                == DEBUG_KIND_UNION as libc::c_int as libc::c_uint
            || (*type_0).kind as libc::c_uint
                == DEBUG_KIND_CLASS as libc::c_int as libc::c_uint
            || (*type_0).kind as libc::c_uint
                == DEBUG_KIND_UNION_CLASS as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"type->kind == DEBUG_KIND_STRUCT || type->kind == DEBUG_KIND_UNION || type->kind == DEBUG_KIND_CLASS || type->kind == DEBUG_KIND_UNION_CLASS\0"
                    as *const u8 as *const libc::c_char,
                b"debug.c\0" as *const u8 as *const libc::c_char,
                2945 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 85],
                    &[libc::c_char; 85],
                >(
                    b"_Bool debug_set_class_id(struct debug_handle *, const char *, struct debug_type_s *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    c = (*type_0).u.kclass;
    if (*c).id > (*info).base_id {
        return 1 as libc::c_int != 0;
    }
    let mut current_block_8: u64;
    l = (*info).id_list;
    while !l.is_null() {
        if !((*(*l).type_0).kind as libc::c_uint != (*type_0).kind as libc::c_uint) {
            if tag.is_null() {
                if !((*l).tag).is_null() {
                    current_block_8 = 17778012151635330486;
                } else {
                    current_block_8 = 1917311967535052937;
                }
            } else if ((*l).tag).is_null()
                || *((*l).tag).offset(0 as libc::c_int as isize) as libc::c_int
                    != *tag.offset(0 as libc::c_int as isize) as libc::c_int
                || strcmp((*l).tag, tag) != 0 as libc::c_int
            {
                current_block_8 = 17778012151635330486;
            } else {
                current_block_8 = 1917311967535052937;
            }
            match current_block_8 {
                17778012151635330486 => {}
                _ => {
                    if debug_type_samep(info, (*l).type_0, type_0) {
                        (*c).id = (*(*(*l).type_0).u.kclass).id;
                        return 1 as libc::c_int != 0;
                    }
                }
            }
        }
        l = (*l).next;
    }
    (*info).class_id = ((*info).class_id).wrapping_add(1);
    (*info).class_id;
    (*c).id = (*info).class_id;
    l = xmalloc(::core::mem::size_of::<debug_class_id>() as libc::c_ulong)
        as *mut debug_class_id;
    memset(
        l as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<debug_class_id>() as libc::c_ulong,
    );
    (*l).type_0 = type_0;
    (*l).tag = tag;
    (*l).next = (*info).id_list;
    (*info).id_list = l;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn debug_type_samep(
    mut info: *mut debug_handle,
    mut t1: *mut debug_type_s,
    mut t2: *mut debug_type_s,
) -> bool {
    let mut l: *mut debug_type_compare_list = 0 as *mut debug_type_compare_list;
    let mut top: debug_type_compare_list = debug_type_compare_list {
        next: 0 as *mut debug_type_compare_list,
        t1: 0 as *mut debug_type_s,
        t2: 0 as *mut debug_type_s,
    };
    let mut ret: bool = false;
    if t1.is_null() {
        return t2.is_null();
    }
    if t2.is_null() {
        return 0 as libc::c_int != 0;
    }
    while (*t1).kind as libc::c_uint
        == DEBUG_KIND_INDIRECT as libc::c_int as libc::c_uint
    {
        t1 = *(*(*t1).u.kindirect).slot;
        if t1.is_null() {
            return 0 as libc::c_int != 0;
        }
    }
    while (*t2).kind as libc::c_uint
        == DEBUG_KIND_INDIRECT as libc::c_int as libc::c_uint
    {
        t2 = *(*(*t2).u.kindirect).slot;
        if t2.is_null() {
            return 0 as libc::c_int != 0;
        }
    }
    if t1 == t2 {
        return 1 as libc::c_int != 0;
    }
    if (*t1).kind as libc::c_uint == DEBUG_KIND_NAMED as libc::c_int as libc::c_uint
        && (*t2).kind as libc::c_uint == DEBUG_KIND_TAGGED as libc::c_int as libc::c_uint
    {
        return debug_type_samep(info, (*(*t1).u.knamed).type_0, t2)
    } else if (*t1).kind as libc::c_uint
        == DEBUG_KIND_TAGGED as libc::c_int as libc::c_uint
        && (*t2).kind as libc::c_uint == DEBUG_KIND_NAMED as libc::c_int as libc::c_uint
    {
        return debug_type_samep(info, t1, (*(*t2).u.knamed).type_0)
    }
    if (*t1).kind as libc::c_uint != (*t2).kind as libc::c_uint
        || (*t1).size != (*t2).size
    {
        return 0 as libc::c_int != 0;
    }
    match (*t1).kind as libc::c_uint {
        2 | 4 | 5 | 6 => return 1 as libc::c_int != 0,
        3 => return (*t1).u.kint as libc::c_int == (*t2).u.kint as libc::c_int,
        _ => {}
    }
    l = (*info).compare_list;
    while !l.is_null() {
        if (*l).t1 == t1 && (*l).t2 == t2 {
            return 1 as libc::c_int != 0;
        }
        l = (*l).next;
    }
    top.t1 = t1;
    top.t2 = t2;
    top.next = (*info).compare_list;
    (*info).compare_list = &mut top;
    match (*t1).kind as libc::c_uint {
        7 | 8 | 9 | 10 => {
            if ((*t1).u.kclass).is_null() {
                ret = ((*t2).u.kclass).is_null();
            } else if ((*t2).u.kclass).is_null() {
                ret = 0 as libc::c_int != 0;
            } else if (*(*t1).u.kclass).id > (*info).base_id
                && (*(*t1).u.kclass).id == (*(*t2).u.kclass).id
            {
                ret = 1 as libc::c_int != 0;
            } else {
                ret = debug_class_type_samep(info, t1, t2);
            }
        }
        11 => {
            if ((*t1).u.kenum).is_null() {
                ret = ((*t2).u.kenum).is_null();
            } else if ((*t2).u.kenum).is_null() {
                ret = 0 as libc::c_int != 0;
            } else {
                let mut pn1: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
                let mut pn2: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
                let mut pv1: *mut bfd_signed_vma = 0 as *mut bfd_signed_vma;
                let mut pv2: *mut bfd_signed_vma = 0 as *mut bfd_signed_vma;
                pn1 = (*(*t1).u.kenum).names;
                pn2 = (*(*t2).u.kenum).names;
                pv1 = (*(*t1).u.kenum).values;
                pv2 = (*(*t2).u.kenum).values;
                while !(*pn1).is_null() && !(*pn2).is_null() {
                    if **pn1 as libc::c_int != **pn2 as libc::c_int || *pv1 != *pv2
                        || strcmp(*pn1, *pn2) != 0 as libc::c_int
                    {
                        break;
                    }
                    pn1 = pn1.offset(1);
                    pn1;
                    pn2 = pn2.offset(1);
                    pn2;
                    pv1 = pv1.offset(1);
                    pv1;
                    pv2 = pv2.offset(1);
                    pv2;
                }
                ret = (*pn1).is_null() && (*pn2).is_null();
            }
        }
        12 => {
            ret = debug_type_samep(info, (*t1).u.kpointer, (*t2).u.kpointer);
        }
        13 => {
            if (*(*t1).u.kfunction).varargs as libc::c_int
                != (*(*t2).u.kfunction).varargs as libc::c_int
                || !debug_type_samep(
                    info,
                    (*(*t1).u.kfunction).return_type,
                    (*(*t2).u.kfunction).return_type,
                )
                || ((*(*t1).u.kfunction).arg_types
                    == 0 as *mut libc::c_void as *mut debug_type) as libc::c_int
                    != ((*(*t2).u.kfunction).arg_types
                        == 0 as *mut libc::c_void as *mut debug_type) as libc::c_int
            {
                ret = 0 as libc::c_int != 0;
            } else if ((*(*t1).u.kfunction).arg_types).is_null() {
                ret = 1 as libc::c_int != 0;
            } else {
                let mut a1: *mut *mut debug_type_s = 0 as *mut *mut debug_type_s;
                let mut a2: *mut *mut debug_type_s = 0 as *mut *mut debug_type_s;
                a1 = (*(*t1).u.kfunction).arg_types;
                a2 = (*(*t2).u.kfunction).arg_types;
                while !(*a1).is_null() && !(*a2).is_null() {
                    if !debug_type_samep(info, *a1, *a2) {
                        break;
                    }
                    a1 = a1.offset(1);
                    a1;
                    a2 = a2.offset(1);
                    a2;
                }
                ret = (*a1).is_null() && (*a2).is_null();
            }
        }
        14 => {
            ret = debug_type_samep(info, (*t1).u.kreference, (*t2).u.kreference);
        }
        15 => {
            ret = (*(*t1).u.krange).lower == (*(*t2).u.krange).lower
                && (*(*t1).u.krange).upper == (*(*t2).u.krange).upper
                && debug_type_samep(
                    info,
                    (*(*t1).u.krange).type_0,
                    (*(*t2).u.krange).type_0,
                ) as libc::c_int != 0;
        }
        16 => {
            ret = (*(*t1).u.karray).lower == (*(*t2).u.karray).lower
                && (*(*t1).u.karray).upper == (*(*t2).u.karray).upper
                && (*(*t1).u.karray).stringp as libc::c_int
                    == (*(*t2).u.karray).stringp as libc::c_int
                && debug_type_samep(
                    info,
                    (*(*t1).u.karray).element_type,
                    (*(*t2).u.karray).element_type,
                ) as libc::c_int != 0;
        }
        17 => {
            ret = (*(*t1).u.kset).bitstringp as libc::c_int
                == (*(*t2).u.kset).bitstringp as libc::c_int
                && debug_type_samep(info, (*(*t1).u.kset).type_0, (*(*t2).u.kset).type_0)
                    as libc::c_int != 0;
        }
        18 => {
            ret = debug_type_samep(
                info,
                (*(*t1).u.koffset).base_type,
                (*(*t2).u.koffset).base_type,
            ) as libc::c_int != 0
                && debug_type_samep(
                    info,
                    (*(*t1).u.koffset).target_type,
                    (*(*t2).u.koffset).target_type,
                ) as libc::c_int != 0;
        }
        19 => {
            if (*(*t1).u.kmethod).varargs as libc::c_int
                != (*(*t2).u.kmethod).varargs as libc::c_int
                || !debug_type_samep(
                    info,
                    (*(*t1).u.kmethod).return_type,
                    (*(*t2).u.kmethod).return_type,
                )
                || !debug_type_samep(
                    info,
                    (*(*t1).u.kmethod).domain_type,
                    (*(*t2).u.kmethod).domain_type,
                )
                || ((*(*t1).u.kmethod).arg_types
                    == 0 as *mut libc::c_void as *mut debug_type) as libc::c_int
                    != ((*(*t2).u.kmethod).arg_types
                        == 0 as *mut libc::c_void as *mut debug_type) as libc::c_int
            {
                ret = 0 as libc::c_int != 0;
            } else if ((*(*t1).u.kmethod).arg_types).is_null() {
                ret = 1 as libc::c_int != 0;
            } else {
                let mut a1_0: *mut *mut debug_type_s = 0 as *mut *mut debug_type_s;
                let mut a2_0: *mut *mut debug_type_s = 0 as *mut *mut debug_type_s;
                a1_0 = (*(*t1).u.kmethod).arg_types;
                a2_0 = (*(*t2).u.kmethod).arg_types;
                while !(*a1_0).is_null() && !(*a2_0).is_null() {
                    if !debug_type_samep(info, *a1_0, *a2_0) {
                        break;
                    }
                    a1_0 = a1_0.offset(1);
                    a1_0;
                    a2_0 = a2_0.offset(1);
                    a2_0;
                }
                ret = (*a1_0).is_null() && (*a2_0).is_null();
            }
        }
        20 => {
            ret = debug_type_samep(info, (*t1).u.kconst, (*t2).u.kconst);
        }
        21 => {
            ret = debug_type_samep(info, (*t1).u.kvolatile, (*t2).u.kvolatile);
        }
        22 | 23 => {
            ret = strcmp((*(*(*t1).u.knamed).name).name, (*(*(*t2).u.knamed).name).name)
                == 0 as libc::c_int
                && debug_type_samep(
                    info,
                    (*(*t1).u.knamed).type_0,
                    (*(*t2).u.knamed).type_0,
                ) as libc::c_int != 0;
        }
        _ => {
            abort();
        }
    }
    (*info).compare_list = top.next;
    return ret;
}
unsafe extern "C" fn debug_class_type_samep(
    mut info: *mut debug_handle,
    mut t1: *mut debug_type_s,
    mut t2: *mut debug_type_s,
) -> bool {
    let mut c1: *mut debug_class_type = 0 as *mut debug_class_type;
    let mut c2: *mut debug_class_type = 0 as *mut debug_class_type;
    c1 = (*t1).u.kclass;
    c2 = (*t2).u.kclass;
    if ((*c1).fields == 0 as *mut libc::c_void as *mut debug_field) as libc::c_int
        != ((*c2).fields == 0 as *mut libc::c_void as *mut debug_field) as libc::c_int
        || ((*c1).baseclasses == 0 as *mut libc::c_void as *mut debug_baseclass)
            as libc::c_int
            != ((*c2).baseclasses == 0 as *mut libc::c_void as *mut debug_baseclass)
                as libc::c_int
        || ((*c1).methods == 0 as *mut libc::c_void as *mut debug_method) as libc::c_int
            != ((*c2).methods == 0 as *mut libc::c_void as *mut debug_method)
                as libc::c_int
        || ((*c1).vptrbase == 0 as *mut libc::c_void as debug_type) as libc::c_int
            != ((*c2).vptrbase == 0 as *mut libc::c_void as debug_type) as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    if !((*c1).fields).is_null() {
        let mut pf1: *mut *mut debug_field_s = 0 as *mut *mut debug_field_s;
        let mut pf2: *mut *mut debug_field_s = 0 as *mut *mut debug_field_s;
        pf1 = (*c1).fields;
        pf2 = (*c2).fields;
        while !(*pf1).is_null() && !(*pf2).is_null() {
            let mut f1: *mut debug_field_s = 0 as *mut debug_field_s;
            let mut f2: *mut debug_field_s = 0 as *mut debug_field_s;
            f1 = *pf1;
            f2 = *pf2;
            if *((*f1).name).offset(0 as libc::c_int as isize) as libc::c_int
                != *((*f2).name).offset(0 as libc::c_int as isize) as libc::c_int
                || (*f1).visibility as libc::c_uint != (*f2).visibility as libc::c_uint
                || (*f1).static_member as libc::c_int
                    != (*f2).static_member as libc::c_int
            {
                return 0 as libc::c_int != 0;
            }
            if (*f1).static_member {
                if strcmp((*f1).u.s.physname, (*f2).u.s.physname) != 0 as libc::c_int {
                    return 0 as libc::c_int != 0;
                }
            } else if (*f1).u.f.bitpos != (*f2).u.f.bitpos
                || (*f1).u.f.bitsize != (*f2).u.f.bitsize
            {
                return 0 as libc::c_int != 0
            }
            if strcmp((*f1).name, (*f2).name) != 0 as libc::c_int
                || ((*f1).type_0).is_null() || ((*f2).type_0).is_null()
                || !debug_type_samep(
                    info,
                    debug_get_real_type(
                        info as *mut libc::c_void,
                        (*f1).type_0,
                        0 as *mut debug_type_real_list,
                    ),
                    debug_get_real_type(
                        info as *mut libc::c_void,
                        (*f2).type_0,
                        0 as *mut debug_type_real_list,
                    ),
                )
            {
                return 0 as libc::c_int != 0;
            }
            pf1 = pf1.offset(1);
            pf1;
            pf2 = pf2.offset(1);
            pf2;
        }
        if !(*pf1).is_null() || !(*pf2).is_null() {
            return 0 as libc::c_int != 0;
        }
    }
    if !((*c1).vptrbase).is_null() {
        if !debug_type_samep(info, (*c1).vptrbase, (*c2).vptrbase) {
            return 0 as libc::c_int != 0;
        }
    }
    if !((*c1).baseclasses).is_null() {
        let mut pb1: *mut *mut debug_baseclass_s = 0 as *mut *mut debug_baseclass_s;
        let mut pb2: *mut *mut debug_baseclass_s = 0 as *mut *mut debug_baseclass_s;
        pb1 = (*c1).baseclasses;
        pb2 = (*c2).baseclasses;
        while !(*pb1).is_null() && !(*pb2).is_null() {
            let mut b1: *mut debug_baseclass_s = 0 as *mut debug_baseclass_s;
            let mut b2: *mut debug_baseclass_s = 0 as *mut debug_baseclass_s;
            b1 = *pb1;
            b2 = *pb2;
            if (*b1).bitpos != (*b2).bitpos
                || (*b1).is_virtual as libc::c_int != (*b2).is_virtual as libc::c_int
                || (*b1).visibility as libc::c_uint != (*b2).visibility as libc::c_uint
                || !debug_type_samep(info, (*b1).type_0, (*b2).type_0)
            {
                return 0 as libc::c_int != 0;
            }
            pb1 = pb1.offset(1);
            pb1;
            pb2 = pb2.offset(1);
            pb2;
        }
        if !(*pb1).is_null() || !(*pb2).is_null() {
            return 0 as libc::c_int != 0;
        }
    }
    if !((*c1).methods).is_null() {
        let mut pm1: *mut *mut debug_method_s = 0 as *mut *mut debug_method_s;
        let mut pm2: *mut *mut debug_method_s = 0 as *mut *mut debug_method_s;
        pm1 = (*c1).methods;
        pm2 = (*c2).methods;
        while !(*pm1).is_null() && !(*pm2).is_null() {
            let mut m1: *mut debug_method_s = 0 as *mut debug_method_s;
            let mut m2: *mut debug_method_s = 0 as *mut debug_method_s;
            m1 = *pm1;
            m2 = *pm2;
            if *((*m1).name).offset(0 as libc::c_int as isize) as libc::c_int
                != *((*m2).name).offset(0 as libc::c_int as isize) as libc::c_int
                || strcmp((*m1).name, (*m2).name) != 0 as libc::c_int
                || ((*m1).variants
                    == 0 as *mut libc::c_void as *mut *mut debug_method_variant_s)
                    as libc::c_int
                    != ((*m2).variants
                        == 0 as *mut libc::c_void as *mut *mut debug_method_variant_s)
                        as libc::c_int
            {
                return 0 as libc::c_int != 0;
            }
            if ((*m1).variants).is_null() {
                let mut pv1: *mut *mut debug_method_variant_s = 0
                    as *mut *mut debug_method_variant_s;
                let mut pv2: *mut *mut debug_method_variant_s = 0
                    as *mut *mut debug_method_variant_s;
                pv1 = (*m1).variants;
                pv2 = (*m2).variants;
                while !(*pv1).is_null() && !(*pv2).is_null() {
                    let mut v1: *mut debug_method_variant_s = 0
                        as *mut debug_method_variant_s;
                    let mut v2: *mut debug_method_variant_s = 0
                        as *mut debug_method_variant_s;
                    v1 = *pv1;
                    v2 = *pv2;
                    if *((*v1).physname).offset(0 as libc::c_int as isize) as libc::c_int
                        != *((*v2).physname).offset(0 as libc::c_int as isize)
                            as libc::c_int
                        || (*v1).visibility as libc::c_uint
                            != (*v2).visibility as libc::c_uint
                        || (*v1).constp as libc::c_int != (*v2).constp as libc::c_int
                        || (*v1).volatilep as libc::c_int
                            != (*v2).volatilep as libc::c_int
                        || (*v1).voffset != (*v2).voffset
                        || ((*v1).context == 0 as *mut libc::c_void as *mut debug_type_s)
                            as libc::c_int
                            != ((*v2).context
                                == 0 as *mut libc::c_void as *mut debug_type_s)
                                as libc::c_int
                        || strcmp((*v1).physname, (*v2).physname) != 0 as libc::c_int
                        || !debug_type_samep(info, (*v1).type_0, (*v2).type_0)
                    {
                        return 0 as libc::c_int != 0;
                    }
                    if !((*v1).context).is_null() {
                        if !debug_type_samep(info, (*v1).context, (*v2).context) {
                            return 0 as libc::c_int != 0;
                        }
                    }
                    pv1 = pv1.offset(1);
                    pv1;
                    pv2 = pv2.offset(1);
                    pv2;
                }
                if !(*pv1).is_null() || !(*pv2).is_null() {
                    return 0 as libc::c_int != 0;
                }
            }
            pm1 = pm1.offset(1);
            pm1;
            pm2 = pm2.offset(1);
            pm2;
        }
        if !(*pm1).is_null() || !(*pm2).is_null() {
            return 0 as libc::c_int != 0;
        }
    }
    return 1 as libc::c_int != 0;
}
