/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}

impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }

    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];

        let bit_index = index % 8;
        let mask = 1 << bit_index;

        byte & mask == mask
    }

    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());

        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];

        let bit_index = index % 8;
        let mask = 1 << bit_index;

        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }

    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());

        let mut val = 0;

        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                val |= 1 << i;
            }
        }

        val
    }

    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());

        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            self.set_bit(i + bit_offset, val_bit_is_set);
        }
    }
}
pub const BPF_LD: u32 = 0;
pub const BPF_LDX: u32 = 1;
pub const BPF_ST: u32 = 2;
pub const BPF_STX: u32 = 3;
pub const BPF_ALU: u32 = 4;
pub const BPF_JMP: u32 = 5;
pub const BPF_RET: u32 = 6;
pub const BPF_MISC: u32 = 7;
pub const BPF_W: u32 = 0;
pub const BPF_H: u32 = 8;
pub const BPF_B: u32 = 16;
pub const BPF_IMM: u32 = 0;
pub const BPF_ABS: u32 = 32;
pub const BPF_IND: u32 = 64;
pub const BPF_MEM: u32 = 96;
pub const BPF_LEN: u32 = 128;
pub const BPF_MSH: u32 = 160;
pub const BPF_ADD: u32 = 0;
pub const BPF_SUB: u32 = 16;
pub const BPF_MUL: u32 = 32;
pub const BPF_DIV: u32 = 48;
pub const BPF_OR: u32 = 64;
pub const BPF_AND: u32 = 80;
pub const BPF_LSH: u32 = 96;
pub const BPF_RSH: u32 = 112;
pub const BPF_NEG: u32 = 128;
pub const BPF_MOD: u32 = 144;
pub const BPF_XOR: u32 = 160;
pub const BPF_JA: u32 = 0;
pub const BPF_JEQ: u32 = 16;
pub const BPF_JGT: u32 = 32;
pub const BPF_JGE: u32 = 48;
pub const BPF_JSET: u32 = 64;
pub const BPF_K: u32 = 0;
pub const BPF_X: u32 = 8;
pub const BPF_MAXINSNS: u32 = 4096;
pub const BPF_ALU64: u32 = 7;
pub const BPF_DW: u32 = 24;
pub const BPF_XADD: u32 = 192;
pub const BPF_MOV: u32 = 176;
pub const BPF_ARSH: u32 = 192;
pub const BPF_END: u32 = 208;
pub const BPF_TO_LE: u32 = 0;
pub const BPF_TO_BE: u32 = 8;
pub const BPF_FROM_LE: u32 = 0;
pub const BPF_FROM_BE: u32 = 8;
pub const BPF_JNE: u32 = 80;
pub const BPF_JSGT: u32 = 96;
pub const BPF_JSGE: u32 = 112;
pub const BPF_CALL: u32 = 128;
pub const BPF_EXIT: u32 = 144;
pub const BPF_PSEUDO_MAP_FD: u32 = 1;
pub const BPF_ANY: u32 = 0;
pub const BPF_NOEXIST: u32 = 1;
pub const BPF_EXIST: u32 = 2;
pub const BPF_F_NO_PREALLOC: u32 = 1;
pub const BPF_F_NO_COMMON_LRU: u32 = 2;
pub const BPF_F_RECOMPUTE_CSUM: u32 = 1;
pub const BPF_F_INVALIDATE_HASH: u32 = 2;
pub const BPF_F_HDR_FIELD_MASK: u32 = 15;
pub const BPF_F_PSEUDO_HDR: u32 = 16;
pub const BPF_F_MARK_MANGLED_0: u32 = 32;
pub const BPF_F_INGRESS: u32 = 1;
pub const BPF_F_TUNINFO_IPV6: u32 = 1;
pub const BPF_F_SKIP_FIELD_MASK: u32 = 255;
pub const BPF_F_USER_STACK: u32 = 256;
pub const BPF_F_FAST_STACK_CMP: u32 = 512;
pub const BPF_F_REUSE_STACKID: u32 = 1024;
pub const BPF_F_ZERO_CSUM_TX: u32 = 2;
pub const BPF_F_DONT_FRAGMENT: u32 = 4;
pub const BPF_F_INDEX_MASK: u32 = 4294967295;
pub const BPF_F_CURRENT_CPU: u32 = 4294967295;
pub const BPF_F_CTXLEN_MASK: u64 = 4503595332403200;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __s16 = ::std::os::raw::c_short;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __u64 = ::std::os::raw::c_ulonglong;
pub const BPF_REG_0: _bindgen_ty_1 = 0;
pub const BPF_REG_1: _bindgen_ty_1 = 1;
pub const BPF_REG_2: _bindgen_ty_1 = 2;
pub const BPF_REG_3: _bindgen_ty_1 = 3;
pub const BPF_REG_4: _bindgen_ty_1 = 4;
pub const BPF_REG_5: _bindgen_ty_1 = 5;
pub const BPF_REG_6: _bindgen_ty_1 = 6;
pub const BPF_REG_7: _bindgen_ty_1 = 7;
pub const BPF_REG_8: _bindgen_ty_1 = 8;
pub const BPF_REG_9: _bindgen_ty_1 = 9;
pub const BPF_REG_10: _bindgen_ty_1 = 10;
pub const __MAX_BPF_REG: _bindgen_ty_1 = 11;
pub type _bindgen_ty_1 = u32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct bpf_insn {
    pub code: __u8,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub off: __s16,
    pub imm: __s32,
}
#[test]
fn bindgen_test_layout_bpf_insn() {
    assert_eq!(
        ::std::mem::size_of::<bpf_insn>(),
        8usize,
        concat!("Size of: ", stringify!(bpf_insn))
    );
    assert_eq!(
        ::std::mem::align_of::<bpf_insn>(),
        4usize,
        concat!("Alignment of ", stringify!(bpf_insn))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_insn>())).code as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_insn),
            "::",
            stringify!(code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_insn>())).off as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_insn),
            "::",
            stringify!(off)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_insn>())).imm as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_insn),
            "::",
            stringify!(imm)
        )
    );
}
impl bpf_insn {
    #[inline]
    pub fn dst_reg(&self) -> __u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_dst_reg(&mut self, val: __u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn src_reg(&self) -> __u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_src_reg(&mut self, val: __u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(dst_reg: __u8, src_reg: __u8) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let dst_reg: u8 = unsafe { ::std::mem::transmute(dst_reg) };
            dst_reg as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let src_reg: u8 = unsafe { ::std::mem::transmute(src_reg) };
            src_reg as u64
        });
        __bindgen_bitfield_unit
    }
}
pub const bpf_cmd_BPF_MAP_CREATE: bpf_cmd = 0;
pub const bpf_cmd_BPF_MAP_LOOKUP_ELEM: bpf_cmd = 1;
pub const bpf_cmd_BPF_MAP_UPDATE_ELEM: bpf_cmd = 2;
pub const bpf_cmd_BPF_MAP_DELETE_ELEM: bpf_cmd = 3;
pub const bpf_cmd_BPF_MAP_GET_NEXT_KEY: bpf_cmd = 4;
pub const bpf_cmd_BPF_PROG_LOAD: bpf_cmd = 5;
pub const bpf_cmd_BPF_OBJ_PIN: bpf_cmd = 6;
pub const bpf_cmd_BPF_OBJ_GET: bpf_cmd = 7;
pub const bpf_cmd_BPF_PROG_ATTACH: bpf_cmd = 8;
pub const bpf_cmd_BPF_PROG_DETACH: bpf_cmd = 9;
pub type bpf_cmd = u32;
pub const bpf_map_type_BPF_MAP_TYPE_UNSPEC: bpf_map_type = 0;
pub const bpf_map_type_BPF_MAP_TYPE_HASH: bpf_map_type = 1;
pub const bpf_map_type_BPF_MAP_TYPE_ARRAY: bpf_map_type = 2;
pub const bpf_map_type_BPF_MAP_TYPE_PROG_ARRAY: bpf_map_type = 3;
pub const bpf_map_type_BPF_MAP_TYPE_PERF_EVENT_ARRAY: bpf_map_type = 4;
pub const bpf_map_type_BPF_MAP_TYPE_PERCPU_HASH: bpf_map_type = 5;
pub const bpf_map_type_BPF_MAP_TYPE_PERCPU_ARRAY: bpf_map_type = 6;
pub const bpf_map_type_BPF_MAP_TYPE_STACK_TRACE: bpf_map_type = 7;
pub const bpf_map_type_BPF_MAP_TYPE_CGROUP_ARRAY: bpf_map_type = 8;
pub const bpf_map_type_BPF_MAP_TYPE_LRU_HASH: bpf_map_type = 9;
pub const bpf_map_type_BPF_MAP_TYPE_LRU_PERCPU_HASH: bpf_map_type = 10;
pub type bpf_map_type = u32;
pub const bpf_prog_type_BPF_PROG_TYPE_UNSPEC: bpf_prog_type = 0;
pub const bpf_prog_type_BPF_PROG_TYPE_SOCKET_FILTER: bpf_prog_type = 1;
pub const bpf_prog_type_BPF_PROG_TYPE_KPROBE: bpf_prog_type = 2;
pub const bpf_prog_type_BPF_PROG_TYPE_SCHED_CLS: bpf_prog_type = 3;
pub const bpf_prog_type_BPF_PROG_TYPE_SCHED_ACT: bpf_prog_type = 4;
pub const bpf_prog_type_BPF_PROG_TYPE_TRACEPOINT: bpf_prog_type = 5;
pub const bpf_prog_type_BPF_PROG_TYPE_XDP: bpf_prog_type = 6;
pub const bpf_prog_type_BPF_PROG_TYPE_PERF_EVENT: bpf_prog_type = 7;
pub const bpf_prog_type_BPF_PROG_TYPE_CGROUP_SKB: bpf_prog_type = 8;
pub const bpf_prog_type_BPF_PROG_TYPE_CGROUP_SOCK: bpf_prog_type = 9;
pub const bpf_prog_type_BPF_PROG_TYPE_LWT_IN: bpf_prog_type = 10;
pub const bpf_prog_type_BPF_PROG_TYPE_LWT_OUT: bpf_prog_type = 11;
pub const bpf_prog_type_BPF_PROG_TYPE_LWT_XMIT: bpf_prog_type = 12;
pub type bpf_prog_type = u32;
pub const bpf_attach_type_BPF_CGROUP_INET_INGRESS: bpf_attach_type = 0;
pub const bpf_attach_type_BPF_CGROUP_INET_EGRESS: bpf_attach_type = 1;
pub const bpf_attach_type_BPF_CGROUP_INET_SOCK_CREATE: bpf_attach_type = 2;
pub const bpf_attach_type___MAX_BPF_ATTACH_TYPE: bpf_attach_type = 3;
pub type bpf_attach_type = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_attr {
    pub __bindgen_anon_1: bpf_attr__bindgen_ty_1,
    pub __bindgen_anon_2: bpf_attr__bindgen_ty_2,
    pub __bindgen_anon_3: bpf_attr__bindgen_ty_3,
    pub __bindgen_anon_4: bpf_attr__bindgen_ty_4,
    pub __bindgen_anon_5: bpf_attr__bindgen_ty_5,
    _bindgen_union_align: [u64; 6usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct bpf_attr__bindgen_ty_1 {
    pub map_type: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub map_flags: __u32,
}
#[test]
fn bindgen_test_layout_bpf_attr__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<bpf_attr__bindgen_ty_1>(),
        20usize,
        concat!("Size of: ", stringify!(bpf_attr__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<bpf_attr__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(bpf_attr__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_attr__bindgen_ty_1>())).map_type as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_1),
            "::",
            stringify!(map_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_attr__bindgen_ty_1>())).key_size as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_1),
            "::",
            stringify!(key_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bpf_attr__bindgen_ty_1>())).value_size as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_1),
            "::",
            stringify!(value_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bpf_attr__bindgen_ty_1>())).max_entries as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_1),
            "::",
            stringify!(max_entries)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bpf_attr__bindgen_ty_1>())).map_flags as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_1),
            "::",
            stringify!(map_flags)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_attr__bindgen_ty_2 {
    pub map_fd: __u32,
    pub key: __u64,
    pub __bindgen_anon_1: bpf_attr__bindgen_ty_2__bindgen_ty_1,
    pub flags: __u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_attr__bindgen_ty_2__bindgen_ty_1 {
    pub value: __u64,
    pub next_key: __u64,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_bpf_attr__bindgen_ty_2__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<bpf_attr__bindgen_ty_2__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(bpf_attr__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<bpf_attr__bindgen_ty_2__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(bpf_attr__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bpf_attr__bindgen_ty_2__bindgen_ty_1>())).value as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(value)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bpf_attr__bindgen_ty_2__bindgen_ty_1>())).next_key as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_2__bindgen_ty_1),
            "::",
            stringify!(next_key)
        )
    );
}
impl Default for bpf_attr__bindgen_ty_2__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for bpf_attr__bindgen_ty_2__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "bpf_attr__bindgen_ty_2__bindgen_ty_1 {{ union }}")
    }
}
#[test]
fn bindgen_test_layout_bpf_attr__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<bpf_attr__bindgen_ty_2>(),
        32usize,
        concat!("Size of: ", stringify!(bpf_attr__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<bpf_attr__bindgen_ty_2>(),
        8usize,
        concat!("Alignment of ", stringify!(bpf_attr__bindgen_ty_2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_attr__bindgen_ty_2>())).map_fd as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_2),
            "::",
            stringify!(map_fd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_attr__bindgen_ty_2>())).key as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_2),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_attr__bindgen_ty_2>())).flags as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_2),
            "::",
            stringify!(flags)
        )
    );
}
impl Default for bpf_attr__bindgen_ty_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for bpf_attr__bindgen_ty_2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write ! ( f , "bpf_attr__bindgen_ty_2 {{ map_fd: {:?}, key: {:?}, __bindgen_anon_1: {:?}, flags: {:?} }}" , self . map_fd , self . key , self . __bindgen_anon_1 , self . flags )
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct bpf_attr__bindgen_ty_3 {
    pub prog_type: __u32,
    pub insn_cnt: __u32,
    pub insns: __u64,
    pub license: __u64,
    pub log_level: __u32,
    pub log_size: __u32,
    pub log_buf: __u64,
    pub kern_version: __u32,
}
#[test]
fn bindgen_test_layout_bpf_attr__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<bpf_attr__bindgen_ty_3>(),
        48usize,
        concat!("Size of: ", stringify!(bpf_attr__bindgen_ty_3))
    );
    assert_eq!(
        ::std::mem::align_of::<bpf_attr__bindgen_ty_3>(),
        8usize,
        concat!("Alignment of ", stringify!(bpf_attr__bindgen_ty_3))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bpf_attr__bindgen_ty_3>())).prog_type as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_3),
            "::",
            stringify!(prog_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_attr__bindgen_ty_3>())).insn_cnt as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_3),
            "::",
            stringify!(insn_cnt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_attr__bindgen_ty_3>())).insns as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_3),
            "::",
            stringify!(insns)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_attr__bindgen_ty_3>())).license as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_3),
            "::",
            stringify!(license)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bpf_attr__bindgen_ty_3>())).log_level as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_3),
            "::",
            stringify!(log_level)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_attr__bindgen_ty_3>())).log_size as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_3),
            "::",
            stringify!(log_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_attr__bindgen_ty_3>())).log_buf as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_3),
            "::",
            stringify!(log_buf)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bpf_attr__bindgen_ty_3>())).kern_version as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_3),
            "::",
            stringify!(kern_version)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct bpf_attr__bindgen_ty_4 {
    pub pathname: __u64,
    pub bpf_fd: __u32,
}
#[test]
fn bindgen_test_layout_bpf_attr__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<bpf_attr__bindgen_ty_4>(),
        16usize,
        concat!("Size of: ", stringify!(bpf_attr__bindgen_ty_4))
    );
    assert_eq!(
        ::std::mem::align_of::<bpf_attr__bindgen_ty_4>(),
        8usize,
        concat!("Alignment of ", stringify!(bpf_attr__bindgen_ty_4))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_attr__bindgen_ty_4>())).pathname as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_4),
            "::",
            stringify!(pathname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_attr__bindgen_ty_4>())).bpf_fd as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_4),
            "::",
            stringify!(bpf_fd)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct bpf_attr__bindgen_ty_5 {
    pub target_fd: __u32,
    pub attach_bpf_fd: __u32,
    pub attach_type: __u32,
}
#[test]
fn bindgen_test_layout_bpf_attr__bindgen_ty_5() {
    assert_eq!(
        ::std::mem::size_of::<bpf_attr__bindgen_ty_5>(),
        12usize,
        concat!("Size of: ", stringify!(bpf_attr__bindgen_ty_5))
    );
    assert_eq!(
        ::std::mem::align_of::<bpf_attr__bindgen_ty_5>(),
        4usize,
        concat!("Alignment of ", stringify!(bpf_attr__bindgen_ty_5))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bpf_attr__bindgen_ty_5>())).target_fd as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_5),
            "::",
            stringify!(target_fd)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bpf_attr__bindgen_ty_5>())).attach_bpf_fd as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_5),
            "::",
            stringify!(attach_bpf_fd)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bpf_attr__bindgen_ty_5>())).attach_type as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_attr__bindgen_ty_5),
            "::",
            stringify!(attach_type)
        )
    );
}
#[test]
fn bindgen_test_layout_bpf_attr() {
    assert_eq!(
        ::std::mem::size_of::<bpf_attr>(),
        48usize,
        concat!("Size of: ", stringify!(bpf_attr))
    );
    assert_eq!(
        ::std::mem::align_of::<bpf_attr>(),
        8usize,
        concat!("Alignment of ", stringify!(bpf_attr))
    );
}
impl Default for bpf_attr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for bpf_attr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "bpf_attr {{ union }}")
    }
}
pub const bpf_func_id_BPF_FUNC_unspec: bpf_func_id = 0;
pub const bpf_func_id_BPF_FUNC_map_lookup_elem: bpf_func_id = 1;
pub const bpf_func_id_BPF_FUNC_map_update_elem: bpf_func_id = 2;
pub const bpf_func_id_BPF_FUNC_map_delete_elem: bpf_func_id = 3;
pub const bpf_func_id_BPF_FUNC_probe_read: bpf_func_id = 4;
pub const bpf_func_id_BPF_FUNC_ktime_get_ns: bpf_func_id = 5;
pub const bpf_func_id_BPF_FUNC_trace_printk: bpf_func_id = 6;
pub const bpf_func_id_BPF_FUNC_get_prandom_u32: bpf_func_id = 7;
pub const bpf_func_id_BPF_FUNC_get_smp_processor_id: bpf_func_id = 8;
pub const bpf_func_id_BPF_FUNC_skb_store_bytes: bpf_func_id = 9;
pub const bpf_func_id_BPF_FUNC_l3_csum_replace: bpf_func_id = 10;
pub const bpf_func_id_BPF_FUNC_l4_csum_replace: bpf_func_id = 11;
pub const bpf_func_id_BPF_FUNC_tail_call: bpf_func_id = 12;
pub const bpf_func_id_BPF_FUNC_clone_redirect: bpf_func_id = 13;
pub const bpf_func_id_BPF_FUNC_get_current_pid_tgid: bpf_func_id = 14;
pub const bpf_func_id_BPF_FUNC_get_current_uid_gid: bpf_func_id = 15;
pub const bpf_func_id_BPF_FUNC_get_current_comm: bpf_func_id = 16;
pub const bpf_func_id_BPF_FUNC_get_cgroup_classid: bpf_func_id = 17;
pub const bpf_func_id_BPF_FUNC_skb_vlan_push: bpf_func_id = 18;
pub const bpf_func_id_BPF_FUNC_skb_vlan_pop: bpf_func_id = 19;
pub const bpf_func_id_BPF_FUNC_skb_get_tunnel_key: bpf_func_id = 20;
pub const bpf_func_id_BPF_FUNC_skb_set_tunnel_key: bpf_func_id = 21;
pub const bpf_func_id_BPF_FUNC_perf_event_read: bpf_func_id = 22;
pub const bpf_func_id_BPF_FUNC_redirect: bpf_func_id = 23;
pub const bpf_func_id_BPF_FUNC_get_route_realm: bpf_func_id = 24;
pub const bpf_func_id_BPF_FUNC_perf_event_output: bpf_func_id = 25;
pub const bpf_func_id_BPF_FUNC_skb_load_bytes: bpf_func_id = 26;
pub const bpf_func_id_BPF_FUNC_get_stackid: bpf_func_id = 27;
pub const bpf_func_id_BPF_FUNC_csum_diff: bpf_func_id = 28;
pub const bpf_func_id_BPF_FUNC_skb_get_tunnel_opt: bpf_func_id = 29;
pub const bpf_func_id_BPF_FUNC_skb_set_tunnel_opt: bpf_func_id = 30;
pub const bpf_func_id_BPF_FUNC_skb_change_proto: bpf_func_id = 31;
pub const bpf_func_id_BPF_FUNC_skb_change_type: bpf_func_id = 32;
pub const bpf_func_id_BPF_FUNC_skb_under_cgroup: bpf_func_id = 33;
pub const bpf_func_id_BPF_FUNC_get_hash_recalc: bpf_func_id = 34;
pub const bpf_func_id_BPF_FUNC_get_current_task: bpf_func_id = 35;
pub const bpf_func_id_BPF_FUNC_probe_write_user: bpf_func_id = 36;
pub const bpf_func_id_BPF_FUNC_current_task_under_cgroup: bpf_func_id = 37;
pub const bpf_func_id_BPF_FUNC_skb_change_tail: bpf_func_id = 38;
pub const bpf_func_id_BPF_FUNC_skb_pull_data: bpf_func_id = 39;
pub const bpf_func_id_BPF_FUNC_csum_update: bpf_func_id = 40;
pub const bpf_func_id_BPF_FUNC_set_hash_invalid: bpf_func_id = 41;
pub const bpf_func_id_BPF_FUNC_get_numa_node_id: bpf_func_id = 42;
pub const bpf_func_id_BPF_FUNC_skb_change_head: bpf_func_id = 43;
pub const bpf_func_id_BPF_FUNC_xdp_adjust_head: bpf_func_id = 44;
pub const bpf_func_id___BPF_FUNC_MAX_ID: bpf_func_id = 45;
pub type bpf_func_id = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tunnel_key {
    pub tunnel_id: __u32,
    pub __bindgen_anon_1: bpf_tunnel_key__bindgen_ty_1,
    pub tunnel_tos: __u8,
    pub tunnel_ttl: __u8,
    pub tunnel_ext: __u16,
    pub tunnel_label: __u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_tunnel_key__bindgen_ty_1 {
    pub remote_ipv4: __u32,
    pub remote_ipv6: [__u32; 4usize],
    _bindgen_union_align: [u32; 4usize],
}
#[test]
fn bindgen_test_layout_bpf_tunnel_key__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<bpf_tunnel_key__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(bpf_tunnel_key__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<bpf_tunnel_key__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(bpf_tunnel_key__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bpf_tunnel_key__bindgen_ty_1>())).remote_ipv4 as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_tunnel_key__bindgen_ty_1),
            "::",
            stringify!(remote_ipv4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bpf_tunnel_key__bindgen_ty_1>())).remote_ipv6 as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_tunnel_key__bindgen_ty_1),
            "::",
            stringify!(remote_ipv6)
        )
    );
}
impl Default for bpf_tunnel_key__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for bpf_tunnel_key__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "bpf_tunnel_key__bindgen_ty_1 {{ union }}")
    }
}
#[test]
fn bindgen_test_layout_bpf_tunnel_key() {
    assert_eq!(
        ::std::mem::size_of::<bpf_tunnel_key>(),
        28usize,
        concat!("Size of: ", stringify!(bpf_tunnel_key))
    );
    assert_eq!(
        ::std::mem::align_of::<bpf_tunnel_key>(),
        4usize,
        concat!("Alignment of ", stringify!(bpf_tunnel_key))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_tunnel_key>())).tunnel_id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_tunnel_key),
            "::",
            stringify!(tunnel_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_tunnel_key>())).tunnel_tos as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_tunnel_key),
            "::",
            stringify!(tunnel_tos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_tunnel_key>())).tunnel_ttl as *const _ as usize },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_tunnel_key),
            "::",
            stringify!(tunnel_ttl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_tunnel_key>())).tunnel_ext as *const _ as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_tunnel_key),
            "::",
            stringify!(tunnel_ext)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_tunnel_key>())).tunnel_label as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_tunnel_key),
            "::",
            stringify!(tunnel_label)
        )
    );
}
impl Default for bpf_tunnel_key {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for bpf_tunnel_key {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write ! ( f , "bpf_tunnel_key {{ tunnel_id: {:?}, __bindgen_anon_1: {:?}, tunnel_tos: {:?}, tunnel_ttl: {:?}, tunnel_ext: {:?}, tunnel_label: {:?} }}" , self . tunnel_id , self . __bindgen_anon_1 , self . tunnel_tos , self . tunnel_ttl , self . tunnel_ext , self . tunnel_label )
    }
}
pub const bpf_ret_code_BPF_OK: bpf_ret_code = 0;
pub const bpf_ret_code_BPF_DROP: bpf_ret_code = 2;
pub const bpf_ret_code_BPF_REDIRECT: bpf_ret_code = 7;
pub type bpf_ret_code = u32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct bpf_sock {
    pub bound_dev_if: __u32,
    pub family: __u32,
    pub type_: __u32,
    pub protocol: __u32,
}
#[test]
fn bindgen_test_layout_bpf_sock() {
    assert_eq!(
        ::std::mem::size_of::<bpf_sock>(),
        16usize,
        concat!("Size of: ", stringify!(bpf_sock))
    );
    assert_eq!(
        ::std::mem::align_of::<bpf_sock>(),
        4usize,
        concat!("Alignment of ", stringify!(bpf_sock))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_sock>())).bound_dev_if as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_sock),
            "::",
            stringify!(bound_dev_if)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_sock>())).family as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_sock),
            "::",
            stringify!(family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_sock>())).type_ as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_sock),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_sock>())).protocol as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_sock),
            "::",
            stringify!(protocol)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map_def {
    pub type_: ::std::os::raw::c_uint,
    pub key_size: ::std::os::raw::c_uint,
    pub value_size: ::std::os::raw::c_uint,
    pub max_entries: ::std::os::raw::c_uint,
    pub map_flags: ::std::os::raw::c_uint,
    pub pinning: ::std::os::raw::c_uint,
    pub namespace: [::std::os::raw::c_char; 256usize],
}
#[test]
fn bindgen_test_layout_bpf_map_def() {
    assert_eq!(
        ::std::mem::size_of::<bpf_map_def>(),
        280usize,
        concat!("Size of: ", stringify!(bpf_map_def))
    );
    assert_eq!(
        ::std::mem::align_of::<bpf_map_def>(),
        4usize,
        concat!("Alignment of ", stringify!(bpf_map_def))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_map_def>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_map_def),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_map_def>())).key_size as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_map_def),
            "::",
            stringify!(key_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_map_def>())).value_size as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_map_def),
            "::",
            stringify!(value_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_map_def>())).max_entries as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_map_def),
            "::",
            stringify!(max_entries)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_map_def>())).map_flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_map_def),
            "::",
            stringify!(map_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_map_def>())).pinning as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_map_def),
            "::",
            stringify!(pinning)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bpf_map_def>())).namespace as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(bpf_map_def),
            "::",
            stringify!(namespace)
        )
    );
}
impl Default for bpf_map_def {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for bpf_map_def {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write ! ( f , "bpf_map_def {{ type: {:?}, key_size: {:?}, value_size: {:?}, max_entries: {:?}, map_flags: {:?}, pinning: {:?}, namespace: [{}] }}" , self . type_ , self . key_size , self . value_size , self . max_entries , self . map_flags , self . pinning , self . namespace . iter ( ) . enumerate ( ) . map ( | ( i , v ) | format ! ( "{}{:?}" , if i > 0 { ", " } else { "" } , v ) ) . collect :: < String > ( ) )
    }
}
impl ::std::cmp::PartialEq for bpf_map_def {
    fn eq(&self, other: &bpf_map_def) -> bool {
        self.type_ == other.type_ && self.key_size == other.key_size
            && self.value_size == other.value_size && self.max_entries == other.max_entries
            && self.map_flags == other.map_flags && self.pinning == other.pinning
            && &self.namespace[..] == &other.namespace[..]
    }
}
pub const bpf_pin_type_PIN_NONE: bpf_pin_type = 0;
pub const bpf_pin_type_PIN_OBJECT_NS: bpf_pin_type = 1;
pub const bpf_pin_type_PIN_GLOBAL_NS: bpf_pin_type = 2;
pub const bpf_pin_type_PIN_CUSTOM_NS: bpf_pin_type = 3;
pub type bpf_pin_type = u32;