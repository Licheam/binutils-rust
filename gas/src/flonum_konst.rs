pub type LITTLENUM_TYPE = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FLONUM_STRUCT {
    pub low: *mut LITTLENUM_TYPE,
    pub high: *mut LITTLENUM_TYPE,
    pub leader: *mut LITTLENUM_TYPE,
    pub exponent: libc::c_long,
    pub sign: libc::c_char,
}
pub type FLONUM_TYPE = FLONUM_STRUCT;
#[no_mangle]
pub static mut flonum_positive_powers_of_ten: [FLONUM_TYPE; 14] = [FLONUM_TYPE {
    low: 0 as *mut LITTLENUM_TYPE,
    high: 0 as *mut LITTLENUM_TYPE,
    leader: 0 as *mut LITTLENUM_TYPE,
    exponent: 0,
    sign: 0,
}; 14];
#[no_mangle]
pub static mut flonum_negative_powers_of_ten: [FLONUM_TYPE; 14] = [FLONUM_TYPE {
    low: 0 as *mut LITTLENUM_TYPE,
    high: 0 as *mut LITTLENUM_TYPE,
    leader: 0 as *mut LITTLENUM_TYPE,
    exponent: 0,
    sign: 0,
}; 14];
#[no_mangle]
pub static mut table_size_of_flonum_powers_of_ten: libc::c_int = 13 as libc::c_int;
static mut zero: [LITTLENUM_TYPE; 1] = [1 as libc::c_int as LITTLENUM_TYPE];
static mut minus_1: [LITTLENUM_TYPE; 20] = [
    39322 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    39321 as libc::c_int as LITTLENUM_TYPE,
    6553 as libc::c_int as LITTLENUM_TYPE,
];
static mut plus_1: [LITTLENUM_TYPE; 1] = [10 as libc::c_int as LITTLENUM_TYPE];
static mut minus_2: [LITTLENUM_TYPE; 20] = [
    10486 as libc::c_int as LITTLENUM_TYPE,
    36700 as libc::c_int as LITTLENUM_TYPE,
    62914 as libc::c_int as LITTLENUM_TYPE,
    23592 as libc::c_int as LITTLENUM_TYPE,
    49807 as libc::c_int as LITTLENUM_TYPE,
    10485 as libc::c_int as LITTLENUM_TYPE,
    36700 as libc::c_int as LITTLENUM_TYPE,
    62914 as libc::c_int as LITTLENUM_TYPE,
    23592 as libc::c_int as LITTLENUM_TYPE,
    49807 as libc::c_int as LITTLENUM_TYPE,
    10485 as libc::c_int as LITTLENUM_TYPE,
    36700 as libc::c_int as LITTLENUM_TYPE,
    62914 as libc::c_int as LITTLENUM_TYPE,
    23592 as libc::c_int as LITTLENUM_TYPE,
    49807 as libc::c_int as LITTLENUM_TYPE,
    10485 as libc::c_int as LITTLENUM_TYPE,
    36700 as libc::c_int as LITTLENUM_TYPE,
    62914 as libc::c_int as LITTLENUM_TYPE,
    23592 as libc::c_int as LITTLENUM_TYPE,
    655 as libc::c_int as LITTLENUM_TYPE,
];
static mut plus_2: [LITTLENUM_TYPE; 1] = [100 as libc::c_int as LITTLENUM_TYPE];
static mut minus_3: [LITTLENUM_TYPE; 20] = [
    52534 as libc::c_int as LITTLENUM_TYPE,
    20027 as libc::c_int as LITTLENUM_TYPE,
    37329 as libc::c_int as LITTLENUM_TYPE,
    65116 as libc::c_int as LITTLENUM_TYPE,
    64067 as libc::c_int as LITTLENUM_TYPE,
    60397 as libc::c_int as LITTLENUM_TYPE,
    14784 as libc::c_int as LITTLENUM_TYPE,
    18979 as libc::c_int as LITTLENUM_TYPE,
    33659 as libc::c_int as LITTLENUM_TYPE,
    19503 as libc::c_int as LITTLENUM_TYPE,
    2726 as libc::c_int as LITTLENUM_TYPE,
    9542 as libc::c_int as LITTLENUM_TYPE,
    629 as libc::c_int as LITTLENUM_TYPE,
    2202 as libc::c_int as LITTLENUM_TYPE,
    40475 as libc::c_int as LITTLENUM_TYPE,
    10590 as libc::c_int as LITTLENUM_TYPE,
    4299 as libc::c_int as LITTLENUM_TYPE,
    47815 as libc::c_int as LITTLENUM_TYPE,
    36280 as libc::c_int as LITTLENUM_TYPE,
    6 as libc::c_int as LITTLENUM_TYPE,
];
static mut plus_3: [LITTLENUM_TYPE; 1] = [10000 as libc::c_int as LITTLENUM_TYPE];
static mut minus_4: [LITTLENUM_TYPE; 19] = [
    22517 as libc::c_int as LITTLENUM_TYPE,
    49501 as libc::c_int as LITTLENUM_TYPE,
    54293 as libc::c_int as LITTLENUM_TYPE,
    19424 as libc::c_int as LITTLENUM_TYPE,
    60699 as libc::c_int as LITTLENUM_TYPE,
    6716 as libc::c_int as LITTLENUM_TYPE,
    24348 as libc::c_int as LITTLENUM_TYPE,
    22618 as libc::c_int as LITTLENUM_TYPE,
    23904 as libc::c_int as LITTLENUM_TYPE,
    21327 as libc::c_int as LITTLENUM_TYPE,
    3919 as libc::c_int as LITTLENUM_TYPE,
    44703 as libc::c_int as LITTLENUM_TYPE,
    19149 as libc::c_int as LITTLENUM_TYPE,
    28803 as libc::c_int as LITTLENUM_TYPE,
    48959 as libc::c_int as LITTLENUM_TYPE,
    6259 as libc::c_int as LITTLENUM_TYPE,
    50273 as libc::c_int as LITTLENUM_TYPE,
    62237 as libc::c_int as LITTLENUM_TYPE,
    42 as libc::c_int as LITTLENUM_TYPE,
];
static mut plus_4: [LITTLENUM_TYPE; 2] = [
    57600 as libc::c_int as LITTLENUM_TYPE,
    1525 as libc::c_int as LITTLENUM_TYPE,
];
static mut minus_5: [LITTLENUM_TYPE; 17] = [
    22199 as libc::c_int as LITTLENUM_TYPE,
    45957 as libc::c_int as LITTLENUM_TYPE,
    17005 as libc::c_int as LITTLENUM_TYPE,
    26266 as libc::c_int as LITTLENUM_TYPE,
    10526 as libc::c_int as LITTLENUM_TYPE,
    16260 as libc::c_int as LITTLENUM_TYPE,
    55017 as libc::c_int as LITTLENUM_TYPE,
    35680 as libc::c_int as LITTLENUM_TYPE,
    40443 as libc::c_int as LITTLENUM_TYPE,
    19789 as libc::c_int as LITTLENUM_TYPE,
    17356 as libc::c_int as LITTLENUM_TYPE,
    30195 as libc::c_int as LITTLENUM_TYPE,
    55905 as libc::c_int as LITTLENUM_TYPE,
    28426 as libc::c_int as LITTLENUM_TYPE,
    63010 as libc::c_int as LITTLENUM_TYPE,
    44197 as libc::c_int as LITTLENUM_TYPE,
    1844 as libc::c_int as LITTLENUM_TYPE,
];
static mut plus_5: [LITTLENUM_TYPE; 3] = [
    28609 as libc::c_int as LITTLENUM_TYPE,
    34546 as libc::c_int as LITTLENUM_TYPE,
    35 as libc::c_int as LITTLENUM_TYPE,
];
static mut minus_6: [LITTLENUM_TYPE; 14] = [
    30926 as libc::c_int as LITTLENUM_TYPE,
    26518 as libc::c_int as LITTLENUM_TYPE,
    13110 as libc::c_int as LITTLENUM_TYPE,
    43018 as libc::c_int as LITTLENUM_TYPE,
    54982 as libc::c_int as LITTLENUM_TYPE,
    48258 as libc::c_int as LITTLENUM_TYPE,
    24658 as libc::c_int as LITTLENUM_TYPE,
    15209 as libc::c_int as LITTLENUM_TYPE,
    63366 as libc::c_int as LITTLENUM_TYPE,
    11929 as libc::c_int as LITTLENUM_TYPE,
    20069 as libc::c_int as LITTLENUM_TYPE,
    43857 as libc::c_int as LITTLENUM_TYPE,
    60487 as libc::c_int as LITTLENUM_TYPE,
    51 as libc::c_int as LITTLENUM_TYPE,
];
static mut plus_6: [LITTLENUM_TYPE; 5] = [
    61313 as libc::c_int as LITTLENUM_TYPE,
    34220 as libc::c_int as LITTLENUM_TYPE,
    16731 as libc::c_int as LITTLENUM_TYPE,
    11629 as libc::c_int as LITTLENUM_TYPE,
    1262 as libc::c_int as LITTLENUM_TYPE,
];
static mut minus_7: [LITTLENUM_TYPE; 7] = [
    29819 as libc::c_int as LITTLENUM_TYPE,
    14733 as libc::c_int as LITTLENUM_TYPE,
    21490 as libc::c_int as LITTLENUM_TYPE,
    40602 as libc::c_int as LITTLENUM_TYPE,
    31315 as libc::c_int as LITTLENUM_TYPE,
    65186 as libc::c_int as LITTLENUM_TYPE,
    2695 as libc::c_int as LITTLENUM_TYPE,
];
static mut plus_7: [LITTLENUM_TYPE; 10] = [
    7937 as libc::c_int as LITTLENUM_TYPE,
    49002 as libc::c_int as LITTLENUM_TYPE,
    60772 as libc::c_int as LITTLENUM_TYPE,
    28216 as libc::c_int as LITTLENUM_TYPE,
    38893 as libc::c_int as LITTLENUM_TYPE,
    55975 as libc::c_int as LITTLENUM_TYPE,
    63988 as libc::c_int as LITTLENUM_TYPE,
    59711 as libc::c_int as LITTLENUM_TYPE,
    20227 as libc::c_int as LITTLENUM_TYPE,
    24 as libc::c_int as LITTLENUM_TYPE,
];
static mut minus_8: [LITTLENUM_TYPE; 14] = [
    27579 as libc::c_int as LITTLENUM_TYPE,
    64807 as libc::c_int as LITTLENUM_TYPE,
    12543 as libc::c_int as LITTLENUM_TYPE,
    794 as libc::c_int as LITTLENUM_TYPE,
    13907 as libc::c_int as LITTLENUM_TYPE,
    61297 as libc::c_int as LITTLENUM_TYPE,
    12013 as libc::c_int as LITTLENUM_TYPE,
    64360 as libc::c_int as LITTLENUM_TYPE,
    15961 as libc::c_int as LITTLENUM_TYPE,
    20566 as libc::c_int as LITTLENUM_TYPE,
    24178 as libc::c_int as LITTLENUM_TYPE,
    15922 as libc::c_int as LITTLENUM_TYPE,
    59427 as libc::c_int as LITTLENUM_TYPE,
    110 as libc::c_int as LITTLENUM_TYPE,
];
static mut plus_8: [LITTLENUM_TYPE; 19] = [
    15873 as libc::c_int as LITTLENUM_TYPE,
    11925 as libc::c_int as LITTLENUM_TYPE,
    39177 as libc::c_int as LITTLENUM_TYPE,
    991 as libc::c_int as LITTLENUM_TYPE,
    14589 as libc::c_int as LITTLENUM_TYPE,
    3861 as libc::c_int as LITTLENUM_TYPE,
    58415 as libc::c_int as LITTLENUM_TYPE,
    9076 as libc::c_int as LITTLENUM_TYPE,
    62956 as libc::c_int as LITTLENUM_TYPE,
    54223 as libc::c_int as LITTLENUM_TYPE,
    56328 as libc::c_int as LITTLENUM_TYPE,
    50180 as libc::c_int as LITTLENUM_TYPE,
    45274 as libc::c_int as LITTLENUM_TYPE,
    48333 as libc::c_int as LITTLENUM_TYPE,
    32537 as libc::c_int as LITTLENUM_TYPE,
    42547 as libc::c_int as LITTLENUM_TYPE,
    9731 as libc::c_int as LITTLENUM_TYPE,
    59679 as libc::c_int as LITTLENUM_TYPE,
    590 as libc::c_int as LITTLENUM_TYPE,
];
static mut minus_9: [LITTLENUM_TYPE; 27] = [
    11042 as libc::c_int as LITTLENUM_TYPE,
    8464 as libc::c_int as LITTLENUM_TYPE,
    58971 as libc::c_int as LITTLENUM_TYPE,
    63429 as libc::c_int as LITTLENUM_TYPE,
    6022 as libc::c_int as LITTLENUM_TYPE,
    63485 as libc::c_int as LITTLENUM_TYPE,
    5500 as libc::c_int as LITTLENUM_TYPE,
    53464 as libc::c_int as LITTLENUM_TYPE,
    47545 as libc::c_int as LITTLENUM_TYPE,
    50068 as libc::c_int as LITTLENUM_TYPE,
    56988 as libc::c_int as LITTLENUM_TYPE,
    22819 as libc::c_int as LITTLENUM_TYPE,
    49708 as libc::c_int as LITTLENUM_TYPE,
    54493 as libc::c_int as LITTLENUM_TYPE,
    9920 as libc::c_int as LITTLENUM_TYPE,
    47667 as libc::c_int as LITTLENUM_TYPE,
    40409 as libc::c_int as LITTLENUM_TYPE,
    35764 as libc::c_int as LITTLENUM_TYPE,
    10383 as libc::c_int as LITTLENUM_TYPE,
    54466 as libc::c_int as LITTLENUM_TYPE,
    32702 as libc::c_int as LITTLENUM_TYPE,
    17493 as libc::c_int as LITTLENUM_TYPE,
    32420 as libc::c_int as LITTLENUM_TYPE,
    34382 as libc::c_int as LITTLENUM_TYPE,
    22750 as libc::c_int as LITTLENUM_TYPE,
    20681 as libc::c_int as LITTLENUM_TYPE,
    12300 as libc::c_int as LITTLENUM_TYPE,
];
static mut plus_9: [LITTLENUM_TYPE; 30] = [
    20678 as libc::c_int as LITTLENUM_TYPE,
    27614 as libc::c_int as LITTLENUM_TYPE,
    28272 as libc::c_int as LITTLENUM_TYPE,
    53066 as libc::c_int as LITTLENUM_TYPE,
    55311 as libc::c_int as LITTLENUM_TYPE,
    54677 as libc::c_int as LITTLENUM_TYPE,
    29038 as libc::c_int as LITTLENUM_TYPE,
    9906 as libc::c_int as LITTLENUM_TYPE,
    26288 as libc::c_int as LITTLENUM_TYPE,
    44486 as libc::c_int as LITTLENUM_TYPE,
    13860 as libc::c_int as LITTLENUM_TYPE,
    7445 as libc::c_int as LITTLENUM_TYPE,
    54106 as libc::c_int as LITTLENUM_TYPE,
    15426 as libc::c_int as LITTLENUM_TYPE,
    21518 as libc::c_int as LITTLENUM_TYPE,
    25599 as libc::c_int as LITTLENUM_TYPE,
    29632 as libc::c_int as LITTLENUM_TYPE,
    52309 as libc::c_int as LITTLENUM_TYPE,
    61207 as libc::c_int as LITTLENUM_TYPE,
    26105 as libc::c_int as LITTLENUM_TYPE,
    10482 as libc::c_int as LITTLENUM_TYPE,
    21948 as libc::c_int as LITTLENUM_TYPE,
    51191 as libc::c_int as LITTLENUM_TYPE,
    32988 as libc::c_int as LITTLENUM_TYPE,
    60892 as libc::c_int as LITTLENUM_TYPE,
    62574 as libc::c_int as LITTLENUM_TYPE,
    61390 as libc::c_int as LITTLENUM_TYPE,
    24540 as libc::c_int as LITTLENUM_TYPE,
    21495 as libc::c_int as LITTLENUM_TYPE,
    5 as libc::c_int as LITTLENUM_TYPE,
];
static mut minus_10: [LITTLENUM_TYPE; 30] = [
    6214 as libc::c_int as LITTLENUM_TYPE,
    48771 as libc::c_int as LITTLENUM_TYPE,
    23471 as libc::c_int as LITTLENUM_TYPE,
    30163 as libc::c_int as LITTLENUM_TYPE,
    31763 as libc::c_int as LITTLENUM_TYPE,
    38013 as libc::c_int as LITTLENUM_TYPE,
    57001 as libc::c_int as LITTLENUM_TYPE,
    11770 as libc::c_int as LITTLENUM_TYPE,
    18263 as libc::c_int as LITTLENUM_TYPE,
    36366 as libc::c_int as LITTLENUM_TYPE,
    20742 as libc::c_int as LITTLENUM_TYPE,
    45086 as libc::c_int as LITTLENUM_TYPE,
    56969 as libc::c_int as LITTLENUM_TYPE,
    53231 as libc::c_int as LITTLENUM_TYPE,
    37856 as libc::c_int as LITTLENUM_TYPE,
    55814 as libc::c_int as LITTLENUM_TYPE,
    38057 as libc::c_int as LITTLENUM_TYPE,
    15692 as libc::c_int as LITTLENUM_TYPE,
    46761 as libc::c_int as LITTLENUM_TYPE,
    8713 as libc::c_int as LITTLENUM_TYPE,
    6102 as libc::c_int as LITTLENUM_TYPE,
    20083 as libc::c_int as LITTLENUM_TYPE,
    8269 as libc::c_int as LITTLENUM_TYPE,
    11839 as libc::c_int as LITTLENUM_TYPE,
    11571 as libc::c_int as LITTLENUM_TYPE,
    50963 as libc::c_int as LITTLENUM_TYPE,
    15649 as libc::c_int as LITTLENUM_TYPE,
    11698 as libc::c_int as LITTLENUM_TYPE,
    40675 as libc::c_int as LITTLENUM_TYPE,
    2308 as libc::c_int as LITTLENUM_TYPE,
];
static mut plus_10: [LITTLENUM_TYPE; 30] = [
    63839 as libc::c_int as LITTLENUM_TYPE,
    36576 as libc::c_int as LITTLENUM_TYPE,
    45712 as libc::c_int as LITTLENUM_TYPE,
    44516 as libc::c_int as LITTLENUM_TYPE,
    37803 as libc::c_int as LITTLENUM_TYPE,
    29482 as libc::c_int as LITTLENUM_TYPE,
    4966 as libc::c_int as LITTLENUM_TYPE,
    30556 as libc::c_int as LITTLENUM_TYPE,
    37961 as libc::c_int as LITTLENUM_TYPE,
    23310 as libc::c_int as LITTLENUM_TYPE,
    27070 as libc::c_int as LITTLENUM_TYPE,
    44972 as libc::c_int as LITTLENUM_TYPE,
    29507 as libc::c_int as LITTLENUM_TYPE,
    48257 as libc::c_int as LITTLENUM_TYPE,
    45209 as libc::c_int as LITTLENUM_TYPE,
    7494 as libc::c_int as LITTLENUM_TYPE,
    17831 as libc::c_int as LITTLENUM_TYPE,
    38728 as libc::c_int as LITTLENUM_TYPE,
    41577 as libc::c_int as LITTLENUM_TYPE,
    29443 as libc::c_int as LITTLENUM_TYPE,
    36016 as libc::c_int as LITTLENUM_TYPE,
    7955 as libc::c_int as LITTLENUM_TYPE,
    35339 as libc::c_int as LITTLENUM_TYPE,
    35479 as libc::c_int as LITTLENUM_TYPE,
    36011 as libc::c_int as LITTLENUM_TYPE,
    14553 as libc::c_int as LITTLENUM_TYPE,
    49618 as libc::c_int as LITTLENUM_TYPE,
    5588 as libc::c_int as LITTLENUM_TYPE,
    25396 as libc::c_int as LITTLENUM_TYPE,
    28 as libc::c_int as LITTLENUM_TYPE,
];
static mut minus_11: [LITTLENUM_TYPE; 30] = [
    16663 as libc::c_int as LITTLENUM_TYPE,
    56882 as libc::c_int as LITTLENUM_TYPE,
    61983 as libc::c_int as LITTLENUM_TYPE,
    7804 as libc::c_int as LITTLENUM_TYPE,
    36555 as libc::c_int as LITTLENUM_TYPE,
    32060 as libc::c_int as LITTLENUM_TYPE,
    34502 as libc::c_int as LITTLENUM_TYPE,
    1000 as libc::c_int as LITTLENUM_TYPE,
    14356 as libc::c_int as LITTLENUM_TYPE,
    21681 as libc::c_int as LITTLENUM_TYPE,
    6605 as libc::c_int as LITTLENUM_TYPE,
    34767 as libc::c_int as LITTLENUM_TYPE,
    51411 as libc::c_int as LITTLENUM_TYPE,
    59048 as libc::c_int as LITTLENUM_TYPE,
    53614 as libc::c_int as LITTLENUM_TYPE,
    39850 as libc::c_int as LITTLENUM_TYPE,
    30079 as libc::c_int as LITTLENUM_TYPE,
    6496 as libc::c_int as LITTLENUM_TYPE,
    6846 as libc::c_int as LITTLENUM_TYPE,
    26841 as libc::c_int as LITTLENUM_TYPE,
    40778 as libc::c_int as LITTLENUM_TYPE,
    19578 as libc::c_int as LITTLENUM_TYPE,
    59899 as libc::c_int as LITTLENUM_TYPE,
    44085 as libc::c_int as LITTLENUM_TYPE,
    54016 as libc::c_int as LITTLENUM_TYPE,
    24259 as libc::c_int as LITTLENUM_TYPE,
    11232 as libc::c_int as LITTLENUM_TYPE,
    21229 as libc::c_int as LITTLENUM_TYPE,
    21313 as libc::c_int as LITTLENUM_TYPE,
    81 as libc::c_int as LITTLENUM_TYPE,
];
static mut plus_11: [LITTLENUM_TYPE; 30] = [
    92 as libc::c_int as LITTLENUM_TYPE,
    9054 as libc::c_int as LITTLENUM_TYPE,
    62707 as libc::c_int as LITTLENUM_TYPE,
    17993 as libc::c_int as LITTLENUM_TYPE,
    7821 as libc::c_int as LITTLENUM_TYPE,
    56838 as libc::c_int as LITTLENUM_TYPE,
    13992 as libc::c_int as LITTLENUM_TYPE,
    21321 as libc::c_int as LITTLENUM_TYPE,
    29637 as libc::c_int as LITTLENUM_TYPE,
    48426 as libc::c_int as LITTLENUM_TYPE,
    42982 as libc::c_int as LITTLENUM_TYPE,
    38668 as libc::c_int as LITTLENUM_TYPE,
    49574 as libc::c_int as LITTLENUM_TYPE,
    28820 as libc::c_int as LITTLENUM_TYPE,
    18200 as libc::c_int as LITTLENUM_TYPE,
    18927 as libc::c_int as LITTLENUM_TYPE,
    53979 as libc::c_int as LITTLENUM_TYPE,
    16219 as libc::c_int as LITTLENUM_TYPE,
    37484 as libc::c_int as LITTLENUM_TYPE,
    2516 as libc::c_int as LITTLENUM_TYPE,
    44642 as libc::c_int as LITTLENUM_TYPE,
    14665 as libc::c_int as LITTLENUM_TYPE,
    11587 as libc::c_int as LITTLENUM_TYPE,
    41926 as libc::c_int as LITTLENUM_TYPE,
    13556 as libc::c_int as LITTLENUM_TYPE,
    23956 as libc::c_int as LITTLENUM_TYPE,
    54320 as libc::c_int as LITTLENUM_TYPE,
    6661 as libc::c_int as LITTLENUM_TYPE,
    55766 as libc::c_int as LITTLENUM_TYPE,
    805 as libc::c_int as LITTLENUM_TYPE,
];
static mut minus_12: [LITTLENUM_TYPE; 30] = [
    33202 as libc::c_int as LITTLENUM_TYPE,
    45969 as libc::c_int as LITTLENUM_TYPE,
    58804 as libc::c_int as LITTLENUM_TYPE,
    56734 as libc::c_int as LITTLENUM_TYPE,
    16482 as libc::c_int as LITTLENUM_TYPE,
    26007 as libc::c_int as LITTLENUM_TYPE,
    44984 as libc::c_int as LITTLENUM_TYPE,
    49334 as libc::c_int as LITTLENUM_TYPE,
    31007 as libc::c_int as LITTLENUM_TYPE,
    32944 as libc::c_int as LITTLENUM_TYPE,
    44517 as libc::c_int as LITTLENUM_TYPE,
    63329 as libc::c_int as LITTLENUM_TYPE,
    47131 as libc::c_int as LITTLENUM_TYPE,
    15291 as libc::c_int as LITTLENUM_TYPE,
    59465 as libc::c_int as LITTLENUM_TYPE,
    2264 as libc::c_int as LITTLENUM_TYPE,
    23218 as libc::c_int as LITTLENUM_TYPE,
    11829 as libc::c_int as LITTLENUM_TYPE,
    59771 as libc::c_int as LITTLENUM_TYPE,
    38798 as libc::c_int as LITTLENUM_TYPE,
    31051 as libc::c_int as LITTLENUM_TYPE,
    28748 as libc::c_int as LITTLENUM_TYPE,
    23129 as libc::c_int as LITTLENUM_TYPE,
    40541 as libc::c_int as LITTLENUM_TYPE,
    41562 as libc::c_int as LITTLENUM_TYPE,
    35108 as libc::c_int as LITTLENUM_TYPE,
    50620 as libc::c_int as LITTLENUM_TYPE,
    59014 as libc::c_int as LITTLENUM_TYPE,
    51817 as libc::c_int as LITTLENUM_TYPE,
    6613 as libc::c_int as LITTLENUM_TYPE,
];
static mut plus_12: [LITTLENUM_TYPE; 30] = [
    10098 as libc::c_int as LITTLENUM_TYPE,
    37922 as libc::c_int as LITTLENUM_TYPE,
    58070 as libc::c_int as LITTLENUM_TYPE,
    7432 as libc::c_int as LITTLENUM_TYPE,
    10470 as libc::c_int as LITTLENUM_TYPE,
    63465 as libc::c_int as LITTLENUM_TYPE,
    23718 as libc::c_int as LITTLENUM_TYPE,
    62190 as libc::c_int as LITTLENUM_TYPE,
    47420 as libc::c_int as LITTLENUM_TYPE,
    7009 as libc::c_int as LITTLENUM_TYPE,
    38443 as libc::c_int as LITTLENUM_TYPE,
    4587 as libc::c_int as LITTLENUM_TYPE,
    45596 as libc::c_int as LITTLENUM_TYPE,
    38472 as libc::c_int as LITTLENUM_TYPE,
    52129 as libc::c_int as LITTLENUM_TYPE,
    52779 as libc::c_int as LITTLENUM_TYPE,
    29012 as libc::c_int as LITTLENUM_TYPE,
    13559 as libc::c_int as LITTLENUM_TYPE,
    48688 as libc::c_int as LITTLENUM_TYPE,
    31678 as libc::c_int as LITTLENUM_TYPE,
    41753 as libc::c_int as LITTLENUM_TYPE,
    58662 as libc::c_int as LITTLENUM_TYPE,
    10668 as libc::c_int as LITTLENUM_TYPE,
    36067 as libc::c_int as LITTLENUM_TYPE,
    29906 as libc::c_int as LITTLENUM_TYPE,
    56906 as libc::c_int as LITTLENUM_TYPE,
    21461 as libc::c_int as LITTLENUM_TYPE,
    46556 as libc::c_int as LITTLENUM_TYPE,
    59571 as libc::c_int as LITTLENUM_TYPE,
    9 as libc::c_int as LITTLENUM_TYPE,
];
static mut minus_13: [LITTLENUM_TYPE; 30] = [
    45309 as libc::c_int as LITTLENUM_TYPE,
    27592 as libc::c_int as LITTLENUM_TYPE,
    37144 as libc::c_int as LITTLENUM_TYPE,
    34637 as libc::c_int as LITTLENUM_TYPE,
    34328 as libc::c_int as LITTLENUM_TYPE,
    41671 as libc::c_int as LITTLENUM_TYPE,
    34620 as libc::c_int as LITTLENUM_TYPE,
    24135 as libc::c_int as LITTLENUM_TYPE,
    53401 as libc::c_int as LITTLENUM_TYPE,
    22112 as libc::c_int as LITTLENUM_TYPE,
    21576 as libc::c_int as LITTLENUM_TYPE,
    45147 as libc::c_int as LITTLENUM_TYPE,
    39310 as libc::c_int as LITTLENUM_TYPE,
    44051 as libc::c_int as LITTLENUM_TYPE,
    48572 as libc::c_int as LITTLENUM_TYPE,
    3676 as libc::c_int as LITTLENUM_TYPE,
    46544 as libc::c_int as LITTLENUM_TYPE,
    59768 as libc::c_int as LITTLENUM_TYPE,
    33350 as libc::c_int as LITTLENUM_TYPE,
    2323 as libc::c_int as LITTLENUM_TYPE,
    49524 as libc::c_int as LITTLENUM_TYPE,
    61568 as libc::c_int as LITTLENUM_TYPE,
    3903 as libc::c_int as LITTLENUM_TYPE,
    36487 as libc::c_int as LITTLENUM_TYPE,
    36356 as libc::c_int as LITTLENUM_TYPE,
    30903 as libc::c_int as LITTLENUM_TYPE,
    14975 as libc::c_int as LITTLENUM_TYPE,
    9035 as libc::c_int as LITTLENUM_TYPE,
    29715 as libc::c_int as LITTLENUM_TYPE,
    667 as libc::c_int as LITTLENUM_TYPE,
];
static mut plus_13: [LITTLENUM_TYPE; 30] = [
    18788 as libc::c_int as LITTLENUM_TYPE,
    16960 as libc::c_int as LITTLENUM_TYPE,
    6318 as libc::c_int as LITTLENUM_TYPE,
    45685 as libc::c_int as LITTLENUM_TYPE,
    55400 as libc::c_int as LITTLENUM_TYPE,
    46230 as libc::c_int as LITTLENUM_TYPE,
    35794 as libc::c_int as LITTLENUM_TYPE,
    25588 as libc::c_int as LITTLENUM_TYPE,
    7253 as libc::c_int as LITTLENUM_TYPE,
    55541 as libc::c_int as LITTLENUM_TYPE,
    49716 as libc::c_int as LITTLENUM_TYPE,
    59760 as libc::c_int as LITTLENUM_TYPE,
    63592 as libc::c_int as LITTLENUM_TYPE,
    8191 as libc::c_int as LITTLENUM_TYPE,
    63765 as libc::c_int as LITTLENUM_TYPE,
    58530 as libc::c_int as LITTLENUM_TYPE,
    44667 as libc::c_int as LITTLENUM_TYPE,
    13294 as libc::c_int as LITTLENUM_TYPE,
    10001 as libc::c_int as LITTLENUM_TYPE,
    55586 as libc::c_int as LITTLENUM_TYPE,
    47887 as libc::c_int as LITTLENUM_TYPE,
    18738 as libc::c_int as LITTLENUM_TYPE,
    9509 as libc::c_int as LITTLENUM_TYPE,
    40896 as libc::c_int as LITTLENUM_TYPE,
    42506 as libc::c_int as LITTLENUM_TYPE,
    52580 as libc::c_int as LITTLENUM_TYPE,
    4171 as libc::c_int as LITTLENUM_TYPE,
    325 as libc::c_int as LITTLENUM_TYPE,
    12329 as libc::c_int as LITTLENUM_TYPE,
    98 as libc::c_int as LITTLENUM_TYPE,
];
unsafe extern "C" fn run_static_initializers() {
    flonum_positive_powers_of_ten = [
        {
            let mut init = FLONUM_STRUCT {
                low: zero.as_ptr() as *mut LITTLENUM_TYPE,
                high: zero.as_ptr() as *mut LITTLENUM_TYPE,
                leader: zero.as_ptr() as *mut LITTLENUM_TYPE,
                exponent: 0 as libc::c_int as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: plus_1.as_ptr() as *mut LITTLENUM_TYPE,
                high: (plus_1.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(0 as libc::c_int as isize),
                leader: (plus_1.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(0 as libc::c_int as isize),
                exponent: 0 as libc::c_int as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: plus_2.as_ptr() as *mut LITTLENUM_TYPE,
                high: (plus_2.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(0 as libc::c_int as isize),
                leader: (plus_2.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(0 as libc::c_int as isize),
                exponent: 0 as libc::c_int as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: plus_3.as_ptr() as *mut LITTLENUM_TYPE,
                high: (plus_3.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(0 as libc::c_int as isize),
                leader: (plus_3.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(0 as libc::c_int as isize),
                exponent: 0 as libc::c_int as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: plus_4.as_ptr() as *mut LITTLENUM_TYPE,
                high: (plus_4.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(1 as libc::c_int as isize),
                leader: (plus_4.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(1 as libc::c_int as isize),
                exponent: 0 as libc::c_int as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: plus_5.as_ptr() as *mut LITTLENUM_TYPE,
                high: (plus_5.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(2 as libc::c_int as isize),
                leader: (plus_5.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(2 as libc::c_int as isize),
                exponent: 1 as libc::c_int as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: plus_6.as_ptr() as *mut LITTLENUM_TYPE,
                high: (plus_6.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(4 as libc::c_int as isize),
                leader: (plus_6.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(4 as libc::c_int as isize),
                exponent: 2 as libc::c_int as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: plus_7.as_ptr() as *mut LITTLENUM_TYPE,
                high: (plus_7.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(9 as libc::c_int as isize),
                leader: (plus_7.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(9 as libc::c_int as isize),
                exponent: 4 as libc::c_int as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: plus_8.as_ptr() as *mut LITTLENUM_TYPE,
                high: (plus_8.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(18 as libc::c_int as isize),
                leader: (plus_8.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(18 as libc::c_int as isize),
                exponent: 8 as libc::c_int as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: plus_9.as_ptr() as *mut LITTLENUM_TYPE,
                high: (plus_9.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                leader: (plus_9.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                exponent: 24 as libc::c_int as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: plus_10.as_ptr() as *mut LITTLENUM_TYPE,
                high: (plus_10.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                leader: (plus_10.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                exponent: 77 as libc::c_int as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: plus_11.as_ptr() as *mut LITTLENUM_TYPE,
                high: (plus_11.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                leader: (plus_11.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                exponent: 183 as libc::c_int as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: plus_12.as_ptr() as *mut LITTLENUM_TYPE,
                high: (plus_12.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                leader: (plus_12.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                exponent: 396 as libc::c_int as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: plus_13.as_ptr() as *mut LITTLENUM_TYPE,
                high: (plus_13.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                leader: (plus_13.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                exponent: 821 as libc::c_int as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
    ];
    flonum_negative_powers_of_ten = [
        {
            let mut init = FLONUM_STRUCT {
                low: zero.as_ptr() as *mut LITTLENUM_TYPE,
                high: zero.as_ptr() as *mut LITTLENUM_TYPE,
                leader: zero.as_ptr() as *mut LITTLENUM_TYPE,
                exponent: 0 as libc::c_int as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: minus_1.as_ptr() as *mut LITTLENUM_TYPE,
                high: (minus_1.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(19 as libc::c_int as isize),
                leader: (minus_1.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(19 as libc::c_int as isize),
                exponent: -(20 as libc::c_int) as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: minus_2.as_ptr() as *mut LITTLENUM_TYPE,
                high: (minus_2.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(19 as libc::c_int as isize),
                leader: (minus_2.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(19 as libc::c_int as isize),
                exponent: -(20 as libc::c_int) as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: minus_3.as_ptr() as *mut LITTLENUM_TYPE,
                high: (minus_3.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(19 as libc::c_int as isize),
                leader: (minus_3.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(19 as libc::c_int as isize),
                exponent: -(20 as libc::c_int) as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: minus_4.as_ptr() as *mut LITTLENUM_TYPE,
                high: (minus_4.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(18 as libc::c_int as isize),
                leader: (minus_4.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(18 as libc::c_int as isize),
                exponent: -(20 as libc::c_int) as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: minus_5.as_ptr() as *mut LITTLENUM_TYPE,
                high: (minus_5.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(16 as libc::c_int as isize),
                leader: (minus_5.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(16 as libc::c_int as isize),
                exponent: -(20 as libc::c_int) as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: minus_6.as_ptr() as *mut LITTLENUM_TYPE,
                high: (minus_6.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(13 as libc::c_int as isize),
                leader: (minus_6.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(13 as libc::c_int as isize),
                exponent: -(20 as libc::c_int) as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: minus_7.as_ptr() as *mut LITTLENUM_TYPE,
                high: (minus_7.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(6 as libc::c_int as isize),
                leader: (minus_7.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(6 as libc::c_int as isize),
                exponent: -(20 as libc::c_int) as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: minus_8.as_ptr() as *mut LITTLENUM_TYPE,
                high: (minus_8.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(13 as libc::c_int as isize),
                leader: (minus_8.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(13 as libc::c_int as isize),
                exponent: -(40 as libc::c_int) as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: minus_9.as_ptr() as *mut LITTLENUM_TYPE,
                high: (minus_9.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(26 as libc::c_int as isize),
                leader: (minus_9.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(26 as libc::c_int as isize),
                exponent: -(80 as libc::c_int) as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: minus_10.as_ptr() as *mut LITTLENUM_TYPE,
                high: (minus_10.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                leader: (minus_10.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                exponent: -(136 as libc::c_int) as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: minus_11.as_ptr() as *mut LITTLENUM_TYPE,
                high: (minus_11.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                leader: (minus_11.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                exponent: -(242 as libc::c_int) as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: minus_12.as_ptr() as *mut LITTLENUM_TYPE,
                high: (minus_12.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                leader: (minus_12.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                exponent: -(455 as libc::c_int) as libc::c_long,
                sign: '+' as i32 as libc::c_char,
            };
            init
        },
        {
            let mut init = FLONUM_STRUCT {
                low: minus_13.as_ptr() as *mut LITTLENUM_TYPE,
                high: (minus_13.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                leader: (minus_13.as_ptr() as *mut LITTLENUM_TYPE)
                    .offset(29 as libc::c_int as isize),
                exponent: -(880 as libc::c_int) as libc::c_long,
                sign: '+' as i32 as libc::c_char,
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
