#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate libc;

use libc::{c_int, c_long, c_char, c_void};

pub use ogg_sys::ogg_int64_t;
pub type ogg_int32_t = libc::int32_t;
pub type ogg_uint32_t = libc::uint32_t;

pub const OV_FALSE: c_int = -1;
pub const OV_EOF: c_int = -2;
pub const OV_HOLE: c_int = -3;

pub const OV_EREAD: c_int = -128;
pub const OV_EFAULT: c_int = -129;
pub const OV_EIMPL: c_int = -130;
pub const OV_EINVAL: c_int = -131;
pub const OV_ENOTVORBIS: c_int = -132;
pub const OV_EBADHEADER: c_int = -133;
pub const OV_EVERSION: c_int = -134;
pub const OV_ENOTAUDIO: c_int = -135;
pub const OV_EBADPACKET: c_int = -136;
pub const OV_EBADLINK: c_int = -137;
pub const OV_ENOSEEK: c_int = -138;

#[repr(C)]
pub struct vorbis_info {
    pub version: c_int,
    pub channels: c_int,
    pub rate: c_long,
    pub bitrate_upper: c_long,
    pub bitrate_nominal: c_long,
    pub bitrate_lower: c_long,
    pub bitrate_window: c_long,
    pub codec_setup: *mut c_void,
}

#[repr(C)]
pub struct vorbis_dsp_state {
    pub analysisp: c_int,
    pub vi: *mut vorbis_info,

    pub pcm: *mut *mut ogg_int32_t,
    pub pcmret: *mut *mut ogg_int32_t,
    pub pcm_storage: c_int,
    pub pcm_current: c_int,
    pub pcm_returned: c_int,

    pub preextrapolate: c_int,
    pub eofflag: c_int,

    pub lW: c_long,
    pub W: c_long,
    pub nW: c_long,
    pub centerW: c_long,

    pub granulepos: ogg_int64_t,
    pub sequence: ogg_int64_t,

    pub backend_state: *mut c_void,
}

#[repr(C)]
pub struct vorbis_block {
    pub pcm: *mut *mut ogg_int32_t,
    pub opb: ogg_sys::oggpack_buffer,

    pub lW: c_long,
    pub W: c_long,
    pub nW: c_long,
    pub pcmend: c_int,
    pub mode: c_int,

    pub eofflag: c_int,
    pub granulepos: ogg_int64_t,
    pub sequence: ogg_int64_t,
    pub vd: *mut vorbis_dsp_state,

    pub localstore: *mut c_void,
    pub localtop: c_long,
    pub localalloc: c_long,
    pub totaluse: c_long,
    pub reap: *mut alloc_chain,
}

#[repr(C)]
pub struct alloc_chain {
    pub ptr: *mut c_void,
    pub next: *mut alloc_chain,
}

#[repr(C)]
pub struct vorbis_comment {
    pub user_comments: *mut *mut c_char,
    pub comment_lengths: *mut c_int,
    pub comments: c_int,
    pub vendor: *mut c_char,
}

#[repr(C)]
pub struct ov_callbacks {
    pub read_func: extern fn(*mut c_void, libc::size_t, libc::size_t, *mut c_void)
        -> libc::size_t,
    pub seek_func: extern fn(*mut c_void, ogg_int64_t, c_int) -> c_int,
    pub close_func: extern fn(*mut c_void) -> c_int,
    pub tell_func: extern fn(*mut c_void) -> c_long,
}

pub const NOTOPEN: c_int = 0;
pub const PARTOPEN: c_int = 1;
pub const OPENED: c_int = 2;
pub const STREAMSET: c_int = 3;
pub const INITSET: c_int = 4;

#[repr(C)]
pub struct OggVorbis_File {
    pub datasource: *mut c_void,
    pub seekable: c_int,
    pub offset: ogg_int64_t,
    pub end: ogg_int64_t,
    pub oy: ogg_sys::ogg_sync_state,

    pub links: c_int,
    pub offsets: *mut ogg_int64_t,
    pub dataoffsets: *mut ogg_int64_t,
    pub serialnos: *mut ogg_uint32_t,
    pub pcmlengths: *mut ogg_int64_t,
    pub vi: *mut vorbis_info,
    pub vc: *mut vorbis_comment,

    pub pcm_offset: ogg_int64_t,
    pub ready_state: c_int,
    pub current_serialno: ogg_uint32_t,
    pub current_link: c_int,

    pub bittrack: ogg_int64_t,
    pub samptrack: ogg_int64_t,

    pub os: ogg_sys::ogg_stream_state,
    pub vd: vorbis_dsp_state,
    pub vb: vorbis_block,

    pub callbacks: ov_callbacks,
}

extern {
    pub fn ov_clear(vf: *mut OggVorbis_File) -> c_int;
    pub fn ov_open(f: *mut libc::FILE, vf: *mut OggVorbis_File, initial: *const c_char,
        ibytes: c_long) -> c_int;
    pub fn ov_open_callbacks(datasource: *mut c_void, vf: *mut OggVorbis_File,
        initial: *const c_char, ibytes: c_long, callbacks: ov_callbacks)
        -> c_int;

    pub fn ov_test(f: *mut libc::FILE, vf: *mut OggVorbis_File, initial: *const c_char,
        ibytes: c_long) -> c_int;
    pub fn ov_test_callbacks(datasource: *mut c_void, vf: *mut OggVorbis_File,
        initial: *const c_char, ibytes: c_long, callbacks: ov_callbacks)
        -> c_int;
    pub fn ov_test_open(vf: *mut OggVorbis_File) -> c_int;

    pub fn ov_bitrate(vf: *mut OggVorbis_File, i: c_int) -> c_long;
    pub fn ov_bitrate_instant(vf: *mut OggVorbis_File) -> c_long;
    pub fn ov_streams(vf: *mut OggVorbis_File) -> c_long;
    pub fn ov_seekable(vf: *mut OggVorbis_File) -> c_long;
    pub fn ov_serialnumber(vf: *mut OggVorbis_File, i: c_int) -> c_long;

    pub fn ov_raw_total(vf: *mut OggVorbis_File, i: c_int) -> ogg_int64_t;
    pub fn ov_pcm_total(vf: *mut OggVorbis_File, i: c_int) -> ogg_int64_t;
    pub fn ov_time_total(vf: *mut OggVorbis_File, i: c_int) -> ogg_int64_t;

    pub fn ov_raw_seek(vf: *mut OggVorbis_File, pos: ogg_int64_t) -> c_int;
    pub fn ov_pcm_seek(vf: *mut OggVorbis_File, pos: ogg_int64_t) -> c_int;
    pub fn ov_pcm_seek_page(vf: *mut OggVorbis_File, pos: ogg_int64_t) -> c_int;
    pub fn ov_time_seek(vf: *mut OggVorbis_File, pos: ogg_int64_t) -> c_int;
    pub fn ov_time_seek_page(vf: *mut OggVorbis_File, pos: ogg_int64_t) -> c_int;

    pub fn ov_raw_tell(vf: *mut OggVorbis_File) -> ogg_int64_t;
    pub fn ov_pcm_tell(vf: *mut OggVorbis_File) -> ogg_int64_t;
    pub fn ov_time_tell(vf: *mut OggVorbis_File) -> ogg_int64_t;

    pub fn ov_info(vf: *mut OggVorbis_File, link: c_int) -> *mut vorbis_info;
    pub fn ov_comment(vf: *mut OggVorbis_File, link: c_int) -> *mut vorbis_comment;

    pub fn ov_read(vf: *mut OggVorbis_File, buffer: *mut c_char, length: c_int,
        bigendianp: c_int, word: c_int, sgned: c_int,
        bitstream: *mut c_int) -> c_long;
}
