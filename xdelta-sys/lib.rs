/* xdelta-sys - rust bindings for xdelta3 delta compression library
 * Copyright (C) 2016 Dmitry Starostin
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 */

#![allow(bad_style)]

use std::os::raw;

pub type xoff_t = u64;
pub type usize_t = u64;

pub type xd3_alloc_func = unsafe extern "C" fn(
    opaque: *mut raw::c_void,
    items: usize,
    size: usize_t)
    -> *mut raw::c_void;
pub type xd3_free_func = unsafe extern "C" fn(
    opaque: *mut raw::c_void,
    address: *mut raw::c_void);
pub type xd3_getblk_func = unsafe extern "C" fn(
    stream: *mut xd3_stream,
    source: *mut xd3_source,
    blkno: xoff_t)
    -> raw::c_int;
pub type xd3_code_table_func = extern "C" fn() -> *const xd3_dinst;


#[repr(i32)]
pub enum xd3_rvalues {
    XD3_INPUT = -17703,
    XD3_OUTPUT = -17704,
    XD3_GETSRCBLK = -17705,
    XD3_GOTHEADER = -17706,
    XD3_WINSTART = -17707,
    XD3_WINFINISH = -17708,
    XD3_TOOFARBACK = -17709,
    XD3_INTERNAL = -17710,
    XD3_INVALID = -17711,
    XD3_INVALID_INPUT = -17712,
    XD3_NOSECOND = -17713,
    XD3_UNIMPLEMENTED = -17714,
}

#[repr(u32)]
pub enum xd3_flags {
    XD3_JUST_HDR = 2,
    XD3_SKIP_WINDOW = 4,
    XD3_SKIP_EMIT = 8,
    XD3_FLUSH = 16,
    XD3_SEC_DJW = 32,
    XD3_SEC_FGK = 64,
    XD3_SEC_LZMA = 16777216,
    XD3_SEC_TYPE = 16777312,
    XD3_SEC_NODATA = 128,
    XD3_SEC_NOINST = 256,
    XD3_SEC_NOADDR = 512,
    XD3_SEC_NOALL = 896,
    XD3_ADLER32 = 1024,
    XD3_ADLER32_NOVER = 2048,
    XD3_NOCOMPRESS = 8192,
    XD3_BEGREEDY = 16384,
    XD3_ADLER32_RECODE = 32768,
    XD3_COMPLEVEL_SHIFT = 20,
    XD3_COMPLEVEL_MASK = 15728640,
    XD3_COMPLEVEL_1 = 1048576,
    XD3_COMPLEVEL_2 = 2097152,
    XD3_COMPLEVEL_3 = 3145728,
    XD3_COMPLEVEL_6 = 6291456,
    XD3_COMPLEVEL_9 = 9437184,
}

#[repr(u32)]
pub enum xd3_smatch_cfg {
    XD3_SMATCH_DEFAULT = 0,
    XD3_SMATCH_SLOW = 1,
    XD3_SMATCH_FAST = 2,
    XD3_SMATCH_FASTER = 3,
    XD3_SMATCH_FASTEST = 4,
    XD3_SMATCH_SOFT = 5,
}


#[repr(C)]
pub struct xd3_sec_cfg {
    pub data_type: raw::c_int,
    pub ngroups: usize_t,
    pub sector_size: usize_t,
    pub inefficient: raw::c_int,
}

#[repr(C)]
pub struct xd3_config {
    pub winsize: usize_t,
    pub sprevsz: usize_t,
    pub iopt_size: usize_t,
    pub getblk: *mut xd3_getblk_func,
    pub alloc: *mut xd3_alloc_func,
    pub freef: *mut xd3_free_func,
    pub opaque: *mut raw::c_void,
    pub flags: u32,
    pub sec_data: xd3_sec_cfg,
    pub sec_inst: xd3_sec_cfg,
    pub sec_addr: xd3_sec_cfg,
    pub smatch_cfg: xd3_smatch_cfg,
    smatcher_soft: xd3_smatcher,
}

#[repr(C)]
pub struct xd3_source {
    pub blksize: usize_t,
    pub name: *const raw::c_char,
    pub ioh: *mut raw::c_void,
    pub max_winsize: xoff_t,
    curblkno: xoff_t,
    onblk: usize_t,
    curblk: *const u8,
    srclen: usize_t,
    srcbase: xoff_t,
    shiftby: usize_t,
    maskby: usize_t,
    cpyoff_blocks: xoff_t,
    cpyoff_blkoff: usize_t,
    getblkno: xoff_t,
    max_blkno: xoff_t,
    onlastblk: usize_t,
    eof_known: raw::c_int,
}

#[repr(C)]
pub struct xd3_stream {
    next_in: *const u8,
    avail_in: usize_t,
    total_in: xoff_t,
    pub next_out: *mut u8,
    pub avail_out: usize_t,
    space_out: usize_t,
    current_window: xoff_t,
    total_out: xoff_t,
    msg: *const raw::c_char,
    src: *mut xd3_source,
    winsize: usize_t,
    sprevsz: usize_t,
    sprevmask: usize_t,
    iopt_size: usize_t,
    iopt_unlimited: usize_t,
    getblk: *mut xd3_getblk_func,
    alloc: *mut xd3_alloc_func,
    free: *mut xd3_free_func,
    opaque: *mut raw::c_void,
    flags: u32,
    sec_data: xd3_sec_cfg,
    sec_inst: xd3_sec_cfg,
    sec_addr: xd3_sec_cfg,
    smatcher: xd3_smatcher,
    large_table: *mut usize_t,
    large_hash: xd3_hash_cfg,
    small_table: *mut usize_t,
    small_prev: *mut xd3_slist,
    small_reset: raw::c_int,
    small_hash: xd3_hash_cfg,
    acache: xd3_addr_cache,
    enc_state: xd3_encode_state,
    taroff: usize_t,
    input_position: usize_t,
    min_match: usize_t,
    unencoded_offset: usize_t,
    srcwin_decided: raw::c_int,
    srcwin_decided_early: raw::c_int,
    srcwin_cksum_pos: xoff_t,
    match_state: xd3_match_state,
    match_srcpos: xoff_t,
    match_last_srcpos: xoff_t,
    match_minaddr: xoff_t,
    match_maxaddr: xoff_t,
    match_back: usize_t,
    match_maxback: usize_t,
    match_fwd: usize_t,
    match_maxfwd: usize_t,
    maxsrcaddr: xoff_t,
    buf_in: *mut u8,
    buf_avail: usize_t,
    buf_leftover: *const u8,
    buf_leftavail: usize_t,
    enc_current: *mut xd3_output,
    enc_free: *mut xd3_output,
    enc_heads: [*mut xd3_output; 4usize],
    enc_tails: [*mut xd3_output; 4usize],
    recode_adler32: u32,
    iopt_used: xd3_rlist,
    iopt_free: xd3_rlist,
    iout: *mut xd3_rinst,
    iopt_alloc: *mut xd3_iopt_buflist,
    enc_appheader: *const u8,
    enc_appheadsz: usize_t,
    dec_state: xd3_decode_state,
    dec_hdr_ind: usize_t,
    dec_win_ind: usize_t,
    dec_del_ind: usize_t,
    dec_magic: [u8; 4usize],
    dec_magicbytes: usize_t,
    dec_secondid: usize_t,
    dec_codetblsz: usize_t,
    dec_codetbl: *mut u8,
    dec_codetblbytes: usize_t,
    dec_appheadsz: usize_t,
    dec_appheader: *mut u8,
    dec_appheadbytes: usize_t,
    dec_cksumbytes: usize_t,
    dec_cksum: [u8; 4usize],
    dec_adler32: u32,
    dec_cpylen: usize_t,
    dec_cpyoff: xoff_t,
    dec_enclen: usize_t,
    dec_tgtlen: usize_t,
    dec_64part: u64,
    dec_winstart: xoff_t,
    dec_window_count: xoff_t,
    dec_winbytes: usize_t,
    dec_hdrsize: usize_t,
    dec_tgtaddrbase: *const u8,
    dec_cpyaddrbase: *const u8,
    dec_position: usize_t,
    dec_maxpos: usize_t,
    dec_current1: xd3_hinst,
    dec_current2: xd3_hinst,
    dec_buffer: *mut u8,
    dec_lastwin: *mut u8,
    dec_lastlen: usize_t,
    dec_laststart: xoff_t,
    dec_lastspace: usize_t,
    inst_sect: xd3_desect,
    addr_sect: xd3_desect,
    data_sect: xd3_desect,
    code_table_func: *mut xd3_code_table_func,
    code_table: *const xd3_dinst,
    code_table_desc: *const xd3_code_table_desc,
    code_table_alloc: *mut xd3_dinst,
    sec_type: *const xd3_sec_type,
    sec_stream_d: *mut xd3_sec_stream,
    sec_stream_i: *mut xd3_sec_stream,
    sec_stream_a: *mut xd3_sec_stream,
    whole_target: xd3_whole_state,
    n_scpy: xoff_t,
    n_tcpy: xoff_t,
    n_add: xoff_t,
    n_run: xoff_t,
    l_scpy: xoff_t,
    l_tcpy: xoff_t,
    l_add: xoff_t,
    l_run: xoff_t,
    i_slots_used: usize_t,
}

#[repr(C)]
pub struct xd3_addr_cache {
    s_near: usize_t,
    s_same: usize_t,
    next_slot: usize_t,
    near_array: *mut usize_t,
    same_array: *mut usize_t,
}

#[repr(C)]
pub struct xd3_dinst {
    type1: u8,
    size1: u8,
    type2: u8,
    size2: u8,
}


extern "C" {
    pub fn xd3_encode_memory(
        input: *const u8,
        input_size: usize_t,
        source: *const u8,
        source_size: usize_t,
        output_buffer: *mut u8,
        output_size: *mut usize_t,
        avail_output: usize_t,
        flags: raw::c_int)
        -> raw::c_int;
    pub fn xd3_decode_memory(
        input: *const u8,
        input_size: usize_t,
        source: *const u8,
        source_size: usize_t,
        output_buf: *mut u8,
        output_size: *mut usize_t,
        avail_output: usize_t,
        flags: raw::c_int)
        -> raw::c_int;
    pub fn xd3_encode_stream(
        stream: *mut xd3_stream,
        input: *const u8,
        input_size: usize_t,
        output: *mut u8,
        output_size: *mut usize_t,
        avail_output: usize_t)
        -> raw::c_int;
    pub fn xd3_decode_stream(
        stream: *mut xd3_stream,
        input: *const u8,
        input_size: usize_t,
        output: *mut u8,
        output_size: *mut usize_t,
        avail_size: usize_t)
        -> raw::c_int;
    pub fn xd3_decode_input(stream: *mut xd3_stream) -> raw::c_int;
    pub fn xd3_encode_input(stream: *mut xd3_stream) -> raw::c_int;
    pub fn xd3_config_stream(stream: *mut xd3_stream, config: *mut xd3_config) -> raw::c_int;
    pub fn xd3_close_stream(stream: *mut xd3_stream) -> raw::c_int;
    pub fn xd3_abort_stream(stream: *mut xd3_stream);
    pub fn xd3_free_stream(stream: *mut xd3_stream);
    pub fn xd3_set_source(stream: *mut xd3_stream, source: *mut xd3_source) -> raw::c_int;
    pub fn xd3_set_source_and_size(
        stream: *mut xd3_stream,
        source: *mut xd3_source,
        source_size: xoff_t)
        -> raw::c_int;
    pub fn xd3_set_appheader(stream: *mut xd3_stream, data: *const u8, size: usize_t);
    pub fn xd3_get_appheader(
        stream: *mut xd3_stream,
        data: *mut *mut u8,
        size: *mut usize_t)
        -> raw::c_int;
    pub fn xd3_encode_init_partial(stream: *mut xd3_stream) -> raw::c_int;
    pub fn xd3_init_cache(acache: *mut xd3_addr_cache);
    pub fn xd3_found_match(
        stream: *mut xd3_stream,
        pos: usize_t,
        size: usize_t,
        addr: xoff_t,
        is_source: raw::c_int)
        -> raw::c_int;
    pub fn xd3_strerror(ret: raw::c_int) -> *const raw::c_char;
}


// internals

enum xd3_sec_type {}
enum xd3_sec_stream {}
enum xd3_code_table_desc {}

#[allow(dead_code)]
#[repr(u32)]
enum xd3_encode_state {
    ENC_INIT = 0,
    ENC_INPUT = 1,
    ENC_SEARCH = 2,
    ENC_INSTR = 3,
    ENC_FLUSH = 4,
    ENC_POSTOUT = 5,
    ENC_POSTWIN = 6,
    ENC_ABORTED = 7,
}
#[allow(dead_code)]
#[repr(u32)]
enum xd3_decode_state {
    DEC_VCHEAD = 0,
    DEC_HDRIND = 1,
    DEC_SECONDID = 2,
    DEC_TABLEN = 3,
    DEC_NEAR = 4,
    DEC_SAME = 5,
    DEC_TABDAT = 6,
    DEC_APPLEN = 7,
    DEC_APPDAT = 8,
    DEC_WININD = 9,
    DEC_CPYLEN = 10,
    DEC_CPYOFF = 11,
    DEC_ENCLEN = 12,
    DEC_TGTLEN = 13,
    DEC_DELIND = 14,
    DEC_DATALEN = 15,
    DEC_INSTLEN = 16,
    DEC_ADDRLEN = 17,
    DEC_CKSUM = 18,
    DEC_DATA = 19,
    DEC_INST = 20,
    DEC_ADDR = 21,
    DEC_EMIT = 22,
    DEC_FINISH = 23,
    DEC_ABORTED = 24,
}
#[allow(dead_code)]
#[repr(u32)]
pub enum xd3_match_state {
    MATCH_TARGET = 0,
    MATCH_BACKWARD = 1,
    MATCH_FORWARD = 2,
    MATCH_SEARCHING = 3,
}
#[repr(C)]
struct xd3_smatcher {
    name: *const raw::c_char,
    string_match: ::std::option::Option<unsafe extern "C" fn(stream:*mut xd3_stream)
        -> raw::c_int>,
    large_look: usize_t,
    large_step: usize_t,
    small_look: usize_t,
    small_chain: usize_t,
    small_lchain: usize_t,
    max_lazy: usize_t,
    long_enough: usize_t,
}
#[repr(C)]
struct xd3_hash_cfg {
    size: usize_t,
    shift: usize_t,
    mask: usize_t,
    look: usize_t,
    multiplier: usize_t,
    powers: *mut usize_t,
}
#[repr(C)]
struct xd3_slist {
    last_pos: usize_t,
}
#[repr(C)]
struct xd3_output {
    base: *mut u8,
    next: usize_t,
    avail: usize_t,
    next_page: *mut xd3_output,
}
#[repr(C)]
struct xd3_rlist {
    next: *mut xd3_rlist,
    prev: *mut xd3_rlist,
}
#[repr(C)]
struct xd3_rinst {
    _type: u8,
    xtra: u8,
    code1: u8,
    code2: u8,
    pos: usize_t,
    size: usize_t,
    addr: xoff_t,
    link: xd3_rlist,
}
#[repr(C)]
struct xd3_iopt_buflist {
    buffer: *mut xd3_rinst,
    next: *mut xd3_iopt_buflist,
}
#[repr(C)]
struct xd3_hinst {
    _type: u8,
    size: usize_t,
    addr: usize_t,
}
#[repr(C)]
struct xd3_desect {
    buf: *const u8,
    buf_max: *const u8,
    size: usize_t,
    pos: usize_t,
    copied1: *mut u8,
    alloc1: usize_t,
    copied2: *mut u8,
    alloc2: usize_t,
}
#[repr(C)]
struct xd3_whole_state {
    addslen: usize_t,
    adds: *mut u8,
    adds_alloc: usize_t,
    instlen: usize_t,
    inst: *mut xd3_winst,
    inst_alloc: usize_t,
    wininfolen: usize_t,
    wininfo: *mut xd3_wininfo,
    wininfo_alloc: usize_t,
    length: xoff_t,
}
#[repr(C)]
struct xd3_winst {
    _type: u8,
    mode: u8,
    size: usize_t,
    addr: xoff_t,
    position: xoff_t,
}
#[repr(C)]
struct xd3_wininfo {
    offset: xoff_t,
    length: usize_t,
    adler32: u32,
}



// tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of_val;

    #[test]
    fn basic() {
        let source: &[u8] = b"source source input0 source source";
        let target: &[u8] = b"source source target source source";
        const BUF_SIZE: usize = 64;
        let mut encoded = [0u8; BUF_SIZE];
        let mut decoded = [0u8; BUF_SIZE];
        let mut enc_size: usize_t = 0;
        let mut dec_size: usize_t = 0;

        unsafe {
            let res = xd3_encode_memory(
                target.as_ptr(), size_of_val(target) as usize_t,
                source.as_ptr(), size_of_val(source) as usize_t,
                encoded.as_mut_ptr(), &mut enc_size,
                BUF_SIZE as usize_t, 0);
            assert_eq!(res, 0);
            assert!(enc_size > 0);

            let res = xd3_decode_memory(
                encoded.as_ptr(), enc_size,
                source.as_ptr(), size_of_val(source) as usize_t,
                decoded.as_mut_ptr(), &mut dec_size,
                BUF_SIZE as usize_t, 0);
            assert_eq!(res, 0);
            assert!(dec_size > 0);
        }

        assert_eq!(target, &decoded[..(dec_size as usize)]);
    }
}
