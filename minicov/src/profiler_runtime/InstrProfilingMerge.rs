use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;

use super::InstrProfData::{
    __llvm_profile_data, __llvm_profile_header, IPVK_Last, VTableProfData, ValueProfData,
    VARIANT_MASK_BYTE_COVERAGE, VARIANT_MASK_TEMPORAL_PROF,
};
use super::InstrProfiling::{
    __llvm_profile_get_magic, __llvm_profile_get_num_padding_bytes, __llvm_profile_get_version,
};
use super::InstrProfilingBuffer::{
    __llvm_profile_counter_entry_size, __llvm_profile_get_name_size,
    __llvm_profile_get_num_bitmap_bytes, __llvm_profile_get_num_counters,
    __llvm_profile_get_num_data,
};
use super::InstrProfilingMergeFile::lprofMergeValueProfData;
use super::InstrProfilingPlatformLinux::{
    __llvm_profile_begin_bitmap, __llvm_profile_begin_counters, __llvm_profile_begin_data,
    __llvm_profile_begin_names, __llvm_profile_begin_vnodes, __llvm_profile_end_bitmap,
    __llvm_profile_end_counters, __llvm_profile_end_data, __llvm_profile_end_names,
    __llvm_profile_end_vnodes,
};

pub static VPMergeHook: Option<
    unsafe extern "C" fn(*mut ValueProfData, *mut __llvm_profile_data) -> (),
> = Some(lprofMergeValueProfData);

pub unsafe fn lprofGetLoadModuleSignature() -> u64 {
    let Version = __llvm_profile_get_version();
    let NumCounters = __llvm_profile_get_num_counters(
        __llvm_profile_begin_counters(),
        __llvm_profile_end_counters(),
    );
    let NumData =
        __llvm_profile_get_num_data(__llvm_profile_begin_data(), __llvm_profile_end_data());
    let NamesSize: u64 =
        __llvm_profile_end_names().offset_from(__llvm_profile_begin_names()) as u64;
    let NumVnodes: u64 =
        __llvm_profile_end_vnodes().offset_from(__llvm_profile_begin_vnodes()) as u64;
    let FirstD = __llvm_profile_begin_data();
    (NamesSize << 40 as c_int)
        .wrapping_add(NumCounters << 30 as c_int)
        .wrapping_add(NumData << 20 as c_int)
        .wrapping_add(NumVnodes << 10 as c_int)
        .wrapping_add(if NumData > 0 { (*FirstD).0.NameRef } else { 0 })
        .wrapping_add(Version)
        .wrapping_add(__llvm_profile_get_magic())
}

pub unsafe fn __llvm_profile_check_compatibility(
    ProfileData: *const c_char,
    ProfileSize: u64,
) -> c_int {
    let Header = ProfileData as *mut __llvm_profile_header;
    let mut SrcData: *mut __llvm_profile_data;
    let mut DstData: *mut __llvm_profile_data;
    let SrcDataStart: *mut __llvm_profile_data = ProfileData
        .add(mem::size_of::<__llvm_profile_header>())
        .offset((*Header).BinaryIdsSize as isize)
        as *mut __llvm_profile_data;
    let SrcDataEnd: *mut __llvm_profile_data = SrcDataStart.offset((*Header).NumData as isize);
    if ProfileSize < mem::size_of::<__llvm_profile_header>() as u64 {
        return 1;
    }
    if (*Header).Magic != __llvm_profile_get_magic()
        || (*Header).Version != __llvm_profile_get_version()
        || (*Header).NumData
            != __llvm_profile_get_num_data(__llvm_profile_begin_data(), __llvm_profile_end_data())
        || (*Header).NumCounters
            != __llvm_profile_get_num_counters(
                __llvm_profile_begin_counters(),
                __llvm_profile_end_counters(),
            )
        || (*Header).NumBitmapBytes
            != __llvm_profile_get_num_bitmap_bytes(
                __llvm_profile_begin_bitmap(),
                __llvm_profile_end_bitmap(),
            )
        || (*Header).NamesSize
            != __llvm_profile_get_name_size(
                __llvm_profile_begin_names(),
                __llvm_profile_end_names(),
            )
        || (*Header).ValueKindLast != IPVK_Last as c_int as u64
    {
        return 1;
    }
    if ProfileSize
        < (mem::size_of::<__llvm_profile_header>() as u64)
            .wrapping_add((*Header).BinaryIdsSize)
            .wrapping_add(
                (*Header)
                    .NumData
                    .wrapping_mul(mem::size_of::<__llvm_profile_data>() as u64),
            )
            .wrapping_add((*Header).NamesSize)
            .wrapping_add(
                (*Header)
                    .NumCounters
                    .wrapping_mul(__llvm_profile_counter_entry_size() as u64),
            )
            .wrapping_add((*Header).NumBitmapBytes)
    {
        return 1;
    }
    SrcData = SrcDataStart;
    DstData = __llvm_profile_begin_data() as *mut __llvm_profile_data;
    while SrcData < SrcDataEnd {
        if (*SrcData).0.NameRef != (*DstData).0.NameRef
            || (*SrcData).0.FuncHash != (*DstData).0.FuncHash
            || (*SrcData).0.NumCounters != (*DstData).0.NumCounters
            || (*SrcData).0.NumBitmapBytes != (*DstData).0.NumBitmapBytes
        {
            return 1;
        }
        SrcData = SrcData.offset(1);
        DstData = DstData.offset(1);
    }
    0
}

unsafe fn signextIfWin64(V: *mut c_void) -> usize {
    V as usize
}

unsafe fn getDistanceFromCounterToValueProf(Header: *const __llvm_profile_header) -> u64 {
    let VTableSectionSize = (*Header)
        .NumVTables
        .wrapping_mul(mem::size_of::<VTableProfData>() as u64);
    let PaddingBytesAfterVTableSection =
        __llvm_profile_get_num_padding_bytes(VTableSectionSize) as u64;
    let VNamesSize = (*Header).VNamesSize;
    let PaddingBytesAfterVNamesSize = __llvm_profile_get_num_padding_bytes(VNamesSize) as u64;
    (*Header)
        .NamesSize
        .wrapping_add(__llvm_profile_get_num_padding_bytes((*Header).NamesSize) as u64)
        .wrapping_add(VTableSectionSize)
        .wrapping_add(PaddingBytesAfterVTableSection)
        .wrapping_add(VNamesSize)
        .wrapping_add(PaddingBytesAfterVNamesSize)
}

pub unsafe fn __llvm_profile_merge_from_buffer(
    ProfileData: *const c_char,
    ProfileSize: u64,
) -> c_int {
    #[expect(clippy::unnecessary_cast)]
    if __llvm_profile_get_version() & VARIANT_MASK_TEMPORAL_PROF as u64 != 0 {
        return 1;
    }
    let Header = ProfileData as *mut __llvm_profile_header;
    let mut CountersDelta = (*Header).CountersDelta as usize;
    let mut BitmapDelta = (*Header).BitmapDelta as usize;
    let SrcDataStart = ProfileData
        .add(mem::size_of::<__llvm_profile_header>())
        .offset((*Header).BinaryIdsSize as isize)
        as *mut __llvm_profile_data;
    let SrcDataEnd = SrcDataStart.offset((*Header).NumData as isize);
    let SrcCountersStart = SrcDataEnd as usize;
    let SrcCountersEnd = (SrcCountersStart as u64).wrapping_add(
        (*Header)
            .NumCounters
            .wrapping_mul(__llvm_profile_counter_entry_size() as u64),
    ) as usize;
    let SrcBitmapStart: usize = SrcCountersEnd.wrapping_add(__llvm_profile_get_num_padding_bytes(
        SrcCountersEnd.wrapping_sub(SrcCountersStart) as u64,
    ) as usize);
    let SrcNameStart: usize =
        (SrcBitmapStart as u64).wrapping_add((*Header).NumBitmapBytes) as usize;
    let SrcValueProfDataStart: usize =
        (SrcNameStart as u64).wrapping_add(getDistanceFromCounterToValueProf(Header)) as usize;
    if SrcNameStart < SrcCountersStart || SrcNameStart < SrcBitmapStart {
        return 1;
    }
    if (*Header).NumData == 0 {
        let mut SrcCounter = SrcCountersStart;
        let mut DstCounter = __llvm_profile_begin_counters() as usize;
        while SrcCounter < SrcCountersEnd {
            #[expect(clippy::unnecessary_cast)]
            if __llvm_profile_get_version() & VARIANT_MASK_BYTE_COVERAGE as u64 != 0 {
                let fresh0 = &mut *(DstCounter as *mut c_char);
                *fresh0 = (*fresh0 as c_int & *(SrcCounter as *const c_char) as c_int) as c_char;
            } else {
                let fresh1 = &mut *(DstCounter as *mut u64);
                *fresh1 = (*fresh1).wrapping_add(*(SrcCounter as *const u64));
            }
            SrcCounter = SrcCounter.wrapping_add(__llvm_profile_counter_entry_size());
            DstCounter = DstCounter.wrapping_add(__llvm_profile_counter_entry_size());
        }
        return 0;
    }
    let mut SrcData: *mut __llvm_profile_data;
    let mut DstData: *mut __llvm_profile_data;
    let mut SrcValueProfData: usize;
    SrcData = SrcDataStart;
    DstData = __llvm_profile_begin_data().cast_mut();
    SrcValueProfData = SrcValueProfDataStart;
    while SrcData < SrcDataEnd {
        let DstCounters = (DstData as usize).wrapping_add(signextIfWin64((*DstData).0.CounterPtr));
        let DstBitmap = (DstData as usize).wrapping_add(signextIfWin64((*DstData).0.BitmapPtr));
        let mut NVK: c_uint = 0;
        let SrcCounters = SrcCountersStart
            .wrapping_add(((*SrcData).0.CounterPtr as usize).wrapping_sub(CountersDelta));
        CountersDelta = CountersDelta.wrapping_sub(mem::size_of::<__llvm_profile_data>());
        let NC = (*SrcData).0.NumCounters as c_uint;
        if NC == 0 {
            return 1;
        }
        if SrcCounters < SrcCountersStart
            || SrcCounters >= SrcNameStart
            || SrcCounters
                .wrapping_add((__llvm_profile_counter_entry_size()).wrapping_mul(NC as usize))
                > SrcNameStart
        {
            return 1;
        }
        let mut I: c_uint = 0;
        while I < NC {
            #[expect(clippy::unnecessary_cast)]
            if __llvm_profile_get_version() & VARIANT_MASK_BYTE_COVERAGE as u64 != 0 {
                let fresh2 = &mut *(DstCounters as *mut c_char).offset(I as isize);
                *fresh2 = (*fresh2 as c_int
                    & *(SrcCounters as *const c_char).offset(I as isize) as c_int)
                    as c_char;
            } else {
                let fresh3 = &mut *(DstCounters as *mut u64).offset(I as isize);
                *fresh3 = (*fresh3).wrapping_add(*(SrcCounters as *const u64).offset(I as isize));
            }
            I = I.wrapping_add(1);
        }
        let SrcBitmap: usize = SrcBitmapStart
            .wrapping_add(((*SrcData).0.BitmapPtr as usize).wrapping_sub(BitmapDelta));
        BitmapDelta = BitmapDelta.wrapping_sub(mem::size_of::<__llvm_profile_data>());
        let NB = (*SrcData).0.NumBitmapBytes as c_uint;
        if NB != 0 {
            if SrcBitmap < SrcBitmapStart || SrcBitmap.wrapping_add(NB as usize) > SrcNameStart {
                return 1;
            }
            let mut I_0: c_uint = 0;
            while I_0 < NB {
                let fresh4 = &mut *(DstBitmap as *mut c_char).offset(I_0 as isize);
                *fresh4 = (*fresh4 as c_int
                    | *(SrcBitmap as *const c_char).offset(I_0 as isize) as c_int)
                    as c_char;
                I_0 = I_0.wrapping_add(1);
            }
        }
        if !VPMergeHook.is_none() {
            let mut I_1: c_uint = 0;
            while I_1 <= IPVK_Last as c_int as c_uint {
                NVK = NVK.wrapping_add(
                    ((*SrcData).0.NumValueSites[I_1 as usize] as c_int != 0) as c_int as c_uint,
                );
                I_1 = I_1.wrapping_add(1);
            }
            if !(NVK == 0) {
                if SrcValueProfData as u64 >= (ProfileData as u64).wrapping_add(ProfileSize) {
                    return 1;
                }
                VPMergeHook.expect("non-null function pointer")(
                    SrcValueProfData as *mut ValueProfData,
                    DstData,
                );
                SrcValueProfData = SrcValueProfData
                    .wrapping_add((*(SrcValueProfData as *mut ValueProfData)).TotalSize as usize);
            }
        }
        SrcData = SrcData.offset(1);
        DstData = DstData.offset(1);
    }
    0
}
