//! Tests for mutable references (MutRef).

use bitpiece::*;

// =============================================================================
// Basic MutRef tests
// =============================================================================

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
struct BasicStruct {
    a: B3,
    b: B5,
}
bitpiece_check_full_impl! {BasicStruct, true}

#[test]
fn mut_ref_get() {
    let mut val = BasicStruct::from_bits(0b11111_010);
    let a_ref = val.a_mut();
    assert_eq!(a_ref.get(), B3::new(2));
}

#[test]
fn mut_ref_set() {
    let mut val = BasicStruct::ZEROES;
    {
        let mut a_ref = val.a_mut();
        a_ref.set(B3::new(5));
    }
    assert_eq!(val.a(), B3::new(5));
    assert_eq!(val.b(), B5::new(0)); // unchanged
}

#[test]
fn mut_ref_multiple_fields() {
    let mut val = BasicStruct::ZEROES;

    {
        let mut a_ref = val.a_mut();
        a_ref.set(B3::new(3));
    }
    {
        let mut b_ref = val.b_mut();
        b_ref.set(B5::new(20));
    }

    assert_eq!(val.a(), B3::new(3));
    assert_eq!(val.b(), B5::new(20));
}

#[test]
fn mut_ref_overwrite() {
    let mut val = BasicStruct::from_bits(0xFF);

    {
        let mut a_ref = val.a_mut();
        assert_eq!(a_ref.get(), B3::new(7));
        a_ref.set(B3::new(0));
    }

    assert_eq!(val.a(), B3::new(0));
    assert_eq!(val.b(), B5::new(31)); // unchanged
}

// =============================================================================
// MutRef with bool field
// =============================================================================

#[bitpiece(4, all)]
#[derive(Debug, PartialEq, Eq)]
struct StructWithBool {
    flag: bool,
    value: B3,
}
bitpiece_check_full_impl! {StructWithBool, true}

#[test]
fn mut_ref_bool_field() {
    let mut val = StructWithBool::ZEROES;

    {
        let mut flag_ref = val.flag_mut();
        assert_eq!(flag_ref.get(), false);
        flag_ref.set(true);
    }

    assert_eq!(val.flag(), true);
    assert_eq!(val.value(), B3::new(0));
}

// =============================================================================
// MutRef with signed fields
// =============================================================================

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq)]
struct StructWithSigned {
    a: SB5,
    b: i8,
    c: B3,
}
bitpiece_check_full_impl! {StructWithSigned, true}

#[test]
fn mut_ref_signed_field() {
    let mut val = StructWithSigned::ZEROES;

    {
        let mut a_ref = val.a_mut();
        a_ref.set(SB5::new(-10));
    }
    {
        let mut b_ref = val.b_mut();
        b_ref.set(-50);
    }

    assert_eq!(val.a(), SB5::new(-10));
    assert_eq!(val.b(), -50i8);
}

// =============================================================================
// MutRef with enum field
// =============================================================================

#[bitpiece(2, all)]
#[derive(Debug, PartialEq, Eq)]
enum Status {
    Idle = 0,
    Running = 1,
    Paused = 2,
    Stopped = 3,
}
bitpiece_check_full_impl! {Status, true}

#[bitpiece(5, all)]
#[derive(Debug, PartialEq, Eq)]
struct StructWithEnum {
    status: Status,
    count: B3,
}
bitpiece_check_full_impl! {StructWithEnum, true}

#[test]
fn mut_ref_enum_field() {
    let mut val = StructWithEnum::ZEROES;

    {
        let mut status_ref = val.status_mut();
        assert_eq!(status_ref.get(), Status::Idle);
        status_ref.set(Status::Running);
    }

    assert_eq!(val.status(), Status::Running);
}

// =============================================================================
// MutRef with nested struct
// =============================================================================

#[bitpiece(4, all)]
#[derive(Debug, PartialEq, Eq)]
struct Inner {
    x: B2,
    y: B2,
}
bitpiece_check_full_impl! {Inner, true}

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
struct Outer {
    inner: Inner,
    extra: B4,
}
bitpiece_check_full_impl! {Outer, true}

#[test]
fn mut_ref_nested_struct() {
    let mut val = Outer::ZEROES;

    {
        let mut inner_ref = val.inner_mut();
        inner_ref.set(Inner::from_bits(0b1111));
    }

    assert_eq!(val.inner().x(), B2::new(3));
    assert_eq!(val.inner().y(), B2::new(3));
}

#[test]
fn mut_ref_nested_get_set() {
    let mut val = Outer::from_bits(0b1010_0011);

    {
        let inner_ref = val.inner_mut();
        let inner_val = inner_ref.get();
        assert_eq!(inner_val.x(), B2::new(3));
        assert_eq!(inner_val.y(), B2::new(0));
    }

    {
        let mut inner_ref = val.inner_mut();
        let new_inner = Inner::from_bits(0b0110);
        inner_ref.set(new_inner);
    }

    assert_eq!(val.inner().x(), B2::new(2));
    assert_eq!(val.inner().y(), B2::new(1));
    assert_eq!(val.extra(), B4::new(10)); // unchanged
}

// =============================================================================
// MutRef on MutStruct (nested mutable access)
// =============================================================================

#[test]
fn mut_struct_field_access() {
    let mut val = Outer::from_bits(0b1010_0011);

    {
        let mut outer_ref = OuterMutRef::new(BitPieceStorageMutRef::U8(&mut val.storage), 0);

        // Access inner through MutRef
        let inner_val = outer_ref.inner();
        assert_eq!(inner_val.x(), B2::new(3));

        // Modify inner through MutRef
        outer_ref.set_inner(Inner::from_bits(0b1100));
    }

    assert_eq!(val.inner().x(), B2::new(0));
    assert_eq!(val.inner().y(), B2::new(3));
}

#[test]
fn mut_struct_nested_mut() {
    let mut val = Outer::ZEROES;

    {
        let mut outer_ref = OuterMutRef::new(BitPieceStorageMutRef::U8(&mut val.storage), 0);

        // Get nested mutable reference
        let mut inner_ref = outer_ref.inner_mut();
        inner_ref.set(Inner::from_bits(0b1001));
    }

    assert_eq!(val.inner().x(), B2::new(1));
    assert_eq!(val.inner().y(), B2::new(2));
}

// =============================================================================
// MutRef with different storage types
// =============================================================================

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq)]
struct Storage16 {
    a: B8,
    b: B8,
}
bitpiece_check_full_impl! {Storage16, true}

#[bitpiece(32, all)]
#[derive(Debug, PartialEq, Eq)]
struct Storage32 {
    a: B16,
    b: B16,
}
bitpiece_check_full_impl! {Storage32, true}

#[bitpiece(64, all)]
#[derive(Debug, PartialEq, Eq)]
struct Storage64 {
    a: B32,
    b: B32,
}
bitpiece_check_full_impl! {Storage64, true}

#[test]
fn mut_ref_storage_u16() {
    let mut val = Storage16::ZEROES;
    {
        let mut a_ref = val.a_mut();
        a_ref.set(B8::new(0xAB));
    }
    assert_eq!(val.a(), B8::new(0xAB));
}

#[test]
fn mut_ref_storage_u32() {
    let mut val = Storage32::ZEROES;
    {
        let mut a_ref = val.a_mut();
        a_ref.set(B16::new(0xABCD));
    }
    assert_eq!(val.a(), B16::new(0xABCD));
}

#[test]
fn mut_ref_storage_u64() {
    let mut val = Storage64::ZEROES;
    {
        let mut a_ref = val.a_mut();
        a_ref.set(B32::new(0xDEADBEEF));
    }
    assert_eq!(val.a(), B32::new(0xDEADBEEF));
}

// =============================================================================
// BitPieceStorageMutRef tests
// =============================================================================

#[test]
fn storage_mut_ref_u8() {
    let mut storage: u8 = 0;
    let mut ref_ = BitPieceStorageMutRef::U8(&mut storage);

    assert_eq!(ref_.get(), 0);
    ref_.set(0xAB);
    assert_eq!(ref_.get(), 0xAB);
    assert_eq!(storage, 0xAB);
}

#[test]
fn storage_mut_ref_u16() {
    let mut storage: u16 = 0;
    let mut ref_ = BitPieceStorageMutRef::U16(&mut storage);

    assert_eq!(ref_.get(), 0);
    ref_.set(0xABCD);
    assert_eq!(ref_.get(), 0xABCD);
    assert_eq!(storage, 0xABCD);
}

#[test]
fn storage_mut_ref_u32() {
    let mut storage: u32 = 0;
    let mut ref_ = BitPieceStorageMutRef::U32(&mut storage);

    assert_eq!(ref_.get(), 0);
    ref_.set(0xDEADBEEF);
    assert_eq!(ref_.get(), 0xDEADBEEF);
    assert_eq!(storage, 0xDEADBEEF);
}

#[test]
fn storage_mut_ref_u64() {
    let mut storage: u64 = 0;
    let mut ref_ = BitPieceStorageMutRef::U64(&mut storage);

    assert_eq!(ref_.get(), 0);
    ref_.set(0xDEADBEEFCAFEBABE);
    assert_eq!(ref_.get(), 0xDEADBEEFCAFEBABE);
    assert_eq!(storage, 0xDEADBEEFCAFEBABE);
}

#[test]
fn storage_mut_ref_reborrow() {
    let mut storage: u32 = 0xABCD1234;
    let mut ref_ = BitPieceStorageMutRef::U32(&mut storage);

    {
        let reborrowed = ref_.reborrow();
        assert_eq!(reborrowed.get(), 0xABCD1234);
    }

    // Original ref still works
    ref_.set(0x12345678);
    assert_eq!(storage, 0x12345678);
}

// =============================================================================
// BitsMut tests
// =============================================================================

#[test]
fn bits_mut_get_bits() {
    let mut storage: u16 = 0b1010_0110_0011_1001;
    let ref_ = BitPieceStorageMutRef::U16(&mut storage);
    let bits_mut = BitsMut::new(ref_, 0);

    // Get bits at different offsets
    assert_eq!(bits_mut.get_bits(0, 4), 0b1001);
    assert_eq!(bits_mut.get_bits(4, 4), 0b0011);
    assert_eq!(bits_mut.get_bits(8, 4), 0b0110);
    assert_eq!(bits_mut.get_bits(12, 4), 0b1010);
}

#[test]
fn bits_mut_get_bits_noshift() {
    let mut storage: u16 = 0b1010_0110_0011_1001;
    let ref_ = BitPieceStorageMutRef::U16(&mut storage);
    let bits_mut = BitsMut::new(ref_, 0);

    // Get bits without shifting
    assert_eq!(bits_mut.get_bits_noshift(0, 4), 0b1001);
    assert_eq!(bits_mut.get_bits_noshift(4, 4), 0b0011_0000);
    assert_eq!(bits_mut.get_bits_noshift(8, 4), 0b0110_0000_0000);
}

#[test]
fn bits_mut_set_bits() {
    let mut storage: u16 = 0;
    {
        let ref_ = BitPieceStorageMutRef::U16(&mut storage);
        let mut bits_mut = BitsMut::new(ref_, 0);
        bits_mut.set_bits(0, 4, 0b1001);
    }
    assert_eq!(storage, 0b1001);

    {
        let ref_ = BitPieceStorageMutRef::U16(&mut storage);
        let mut bits_mut = BitsMut::new(ref_, 0);
        bits_mut.set_bits(4, 4, 0b0011);
    }
    assert_eq!(storage, 0b0011_1001);

    {
        let ref_ = BitPieceStorageMutRef::U16(&mut storage);
        let mut bits_mut = BitsMut::new(ref_, 0);
        bits_mut.set_bits(8, 4, 0b0110);
    }
    assert_eq!(storage, 0b0110_0011_1001);
}

#[test]
fn bits_mut_with_start_offset() {
    let mut storage: u16 = 0;
    {
        let ref_ = BitPieceStorageMutRef::U16(&mut storage);
        let mut bits_mut = BitsMut::new(ref_, 4); // Start at bit 4
        bits_mut.set_bits(0, 4, 0b1111); // Actually sets bits 4-7
    }
    assert_eq!(storage, 0b1111_0000);

    {
        let ref_ = BitPieceStorageMutRef::U16(&mut storage);
        let mut bits_mut = BitsMut::new(ref_, 4); // Start at bit 4
        bits_mut.set_bits(4, 4, 0b1010); // Actually sets bits 8-11
    }
    assert_eq!(storage, 0b1010_1111_0000);
}

// =============================================================================
// BitPieceMutRef trait tests
// =============================================================================

#[test]
fn bitpiece_mut_ref_trait() {
    fn test_mut_ref<'a, T: BitPieceMutRef<'a>>(_: T) {}

    let mut storage: u8 = 0;
    let ref_ = B3MutRef::new(BitPieceStorageMutRef::U8(&mut storage), 0);
    test_mut_ref(ref_);
}

#[test]
fn bitpiece_mut_ref_trait_operations() {
    let mut storage: u8 = 0b101;
    let mut ref_ = B3MutRef::new(BitPieceStorageMutRef::U8(&mut storage), 0);

    // Using trait methods
    assert_eq!(BitPieceMutRef::get(&ref_), B3::new(5));

    BitPieceMutRef::set(&mut ref_, B3::new(3));
    assert_eq!(storage, 0b011);
}

// =============================================================================
// MutRef const operations
// =============================================================================

#[test]
fn mut_ref_const_get() {
    let mut storage: u8 = 0b101;
    let ref_ = B3MutRef::new(BitPieceStorageMutRef::U8(&mut storage), 0);

    // const fn get
    let val = ref_.get();
    assert_eq!(val, B3::new(5));
}

#[test]
fn mut_ref_const_set() {
    let mut storage: u8 = 0;
    let mut ref_ = B3MutRef::new(BitPieceStorageMutRef::U8(&mut storage), 0);

    // const fn set
    ref_.set(B3::new(7));
    assert_eq!(storage, 0b111);
}

// =============================================================================
// Complex MutRef scenarios
// =============================================================================

#[bitpiece(16, all)]
#[derive(Debug, PartialEq, Eq)]
struct ComplexStruct {
    a: B4,
    inner: Inner,
    b: B4,
    c: B4,
}
bitpiece_check_full_impl! {ComplexStruct, true}

#[test]
fn mut_ref_complex_struct() {
    let mut val = ComplexStruct::ZEROES;

    // Modify multiple fields through MutRef
    {
        let mut a_ref = val.a_mut();
        a_ref.set(B4::new(0xA));
    }
    {
        let mut inner_ref = val.inner_mut();
        inner_ref.set(Inner::from_bits(0b1001));
    }
    {
        let mut b_ref = val.b_mut();
        b_ref.set(B4::new(0xB));
    }
    {
        let mut c_ref = val.c_mut();
        c_ref.set(B4::new(0xC));
    }

    assert_eq!(val.a(), B4::new(0xA));
    assert_eq!(val.inner().x(), B2::new(1));
    assert_eq!(val.inner().y(), B2::new(2));
    assert_eq!(val.b(), B4::new(0xB));
    assert_eq!(val.c(), B4::new(0xC));
}

#[test]
fn mut_ref_preserves_other_bits() {
    let mut val = ComplexStruct::from_bits(0xFFFF);

    // Modify only one field
    {
        let mut a_ref = val.a_mut();
        a_ref.set(B4::new(0));
    }

    // Other fields should be unchanged
    assert_eq!(val.a(), B4::new(0));
    assert_eq!(val.inner().x(), B2::new(3));
    assert_eq!(val.inner().y(), B2::new(3));
    assert_eq!(val.b(), B4::new(0xF));
    assert_eq!(val.c(), B4::new(0xF));
}
