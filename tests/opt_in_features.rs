//! Tests for opt-in features.

use bitpiece::*;

// =============================================================================
// Basic preset (get, set, with) - default
// =============================================================================

#[bitpiece(8)]
#[derive(Debug, PartialEq, Eq)]
struct BasicPreset {
    a: B4,
    b: B4,
}

#[test]
fn basic_preset_has_getters() {
    let val = BasicPreset::from_bits(0xAB);
    assert_eq!(val.a(), B4::new(0xB));
    assert_eq!(val.b(), B4::new(0xA));
}

#[test]
fn basic_preset_has_setters() {
    let mut val = BasicPreset::from_bits(0);
    val.set_a(B4::new(5));
    val.set_b(B4::new(10));
    assert_eq!(val.a(), B4::new(5));
    assert_eq!(val.b(), B4::new(10));
}

#[test]
fn basic_preset_has_with() {
    let val = BasicPreset::from_bits(0)
        .with_a(B4::new(5))
        .with_b(B4::new(10));
    assert_eq!(val.a(), B4::new(5));
    assert_eq!(val.b(), B4::new(10));
}

// =============================================================================
// Get only
// =============================================================================

#[bitpiece(8, get)]
#[derive(Debug, PartialEq, Eq)]
struct GetOnly {
    a: B4,
    b: B4,
}

#[test]
fn get_only_has_getters() {
    let val = GetOnly::from_bits(0xAB);
    assert_eq!(val.a(), B4::new(0xB));
    assert_eq!(val.b(), B4::new(0xA));
}

// Note: set_* and with_* methods should not exist for GetOnly
// This is verified by the fact that the code compiles without them

// =============================================================================
// Get and Set only
// =============================================================================

#[bitpiece(8, get, set)]
#[derive(Debug, PartialEq, Eq)]
struct GetSet {
    a: B4,
    b: B4,
}

#[test]
fn get_set_has_getters() {
    let val = GetSet::from_bits(0xAB);
    assert_eq!(val.a(), B4::new(0xB));
}

#[test]
fn get_set_has_setters() {
    let mut val = GetSet::from_bits(0);
    val.set_a(B4::new(5));
    assert_eq!(val.a(), B4::new(5));
}

// =============================================================================
// Get and With only
// =============================================================================

#[bitpiece(8, get, with)]
#[derive(Debug, PartialEq, Eq)]
struct GetWith {
    a: B4,
    b: B4,
}

#[test]
fn get_with_has_getters() {
    let val = GetWith::from_bits(0xAB);
    assert_eq!(val.a(), B4::new(0xB));
}

#[test]
fn get_with_has_with() {
    let val = GetWith::from_bits(0).with_a(B4::new(5));
    assert_eq!(val.a(), B4::new(5));
}

// =============================================================================
// All features
// =============================================================================

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq)]
struct AllFeatures {
    a: B4,
    b: B4,
}
bitpiece_check_full_impl! {AllFeatures, true}

#[test]
fn all_features_has_getters() {
    let val = AllFeatures::from_bits(0xAB);
    assert_eq!(val.a(), B4::new(0xB));
}

#[test]
fn all_features_has_setters() {
    let mut val = AllFeatures::from_bits(0);
    val.set_a(B4::new(5));
    assert_eq!(val.a(), B4::new(5));
}

#[test]
fn all_features_has_with() {
    let val = AllFeatures::from_bits(0).with_a(B4::new(5));
    assert_eq!(val.a(), B4::new(5));
}

#[test]
fn all_features_has_noshift() {
    let val = AllFeatures::from_bits(0xAB);
    assert_eq!(val.a_noshift(), 0x0B);
    assert_eq!(val.b_noshift(), 0xA0);
}

#[test]
fn all_features_has_mut() {
    let mut val = AllFeatures::from_bits(0);
    {
        let mut a_ref = val.a_mut();
        a_ref.set(B4::new(5));
    }
    assert_eq!(val.a(), B4::new(5));
}

#[test]
fn all_features_has_const_eq() {
    let a = AllFeatures::from_bits(0xAB);
    let b = AllFeatures::from_bits(0xAB);
    let c = AllFeatures::from_bits(0xCD);
    assert!(AllFeatures::const_eq(a, b));
    assert!(!AllFeatures::const_eq(a, c));
}

#[test]
fn all_features_has_fields_struct() {
    let fields = AllFeaturesFields {
        a: B4::new(5),
        b: B4::new(10),
    };
    let val = AllFeatures::from_fields(fields);
    assert_eq!(val.a(), B4::new(5));
    assert_eq!(val.b(), B4::new(10));

    let extracted = val.to_fields();
    assert_eq!(extracted, fields);
}

#[test]
fn all_features_has_mut_struct() {
    let mut val = AllFeatures::from_bits(0);
    {
        let mut ref_ = AllFeaturesMutRef::new(BitPieceStorageMutRef::U8(&mut val.storage), 0);
        ref_.set_a(B4::new(5));
    }
    assert_eq!(val.a(), B4::new(5));
}

// =============================================================================
// Noshift only
// =============================================================================

#[bitpiece(8, get, get_noshift)]
#[derive(Debug, PartialEq, Eq)]
struct NoshiftOnly {
    a: B4,
    b: B4,
}

#[test]
fn noshift_only_has_noshift() {
    let val = NoshiftOnly::from_bits(0xAB);
    assert_eq!(val.a_noshift(), 0x0B);
    assert_eq!(val.b_noshift(), 0xA0);
}

// =============================================================================
// Mut only
// =============================================================================

#[bitpiece(8, get, get_mut)]
#[derive(Debug, PartialEq, Eq)]
struct MutOnly {
    a: B4,
    b: B4,
}

#[test]
fn mut_only_has_mut() {
    let mut val = MutOnly::from_bits(0);
    {
        let mut a_ref = val.a_mut();
        a_ref.set(B4::new(5));
    }
    assert_eq!(val.a(), B4::new(5));
}

// =============================================================================
// FieldsStruct only
// =============================================================================

#[bitpiece(8, get, fields_struct)]
#[derive(Debug, PartialEq, Eq)]
struct FieldsStructOnly {
    a: B4,
    b: B4,
}

#[test]
fn fields_struct_only_has_fields() {
    let fields = FieldsStructOnlyFields {
        a: B4::new(5),
        b: B4::new(10),
    };
    let val = FieldsStructOnly::from_fields(fields);
    assert_eq!(val.a(), B4::new(5));

    let extracted = val.to_fields();
    assert_eq!(extracted, fields);
}

#[test]
fn fields_struct_only_has_into() {
    let fields = FieldsStructOnlyFields {
        a: B4::new(5),
        b: B4::new(10),
    };
    let val: FieldsStructOnly = fields.into();
    assert_eq!(val.a(), B4::new(5));

    let fields2: FieldsStructOnlyFields = val.into();
    assert_eq!(fields2.a, B4::new(5));
}

// =============================================================================
// MutStruct features
// =============================================================================

#[bitpiece(8, get, mut_struct, mut_struct_field_get, mut_struct_field_set)]
#[derive(Debug, PartialEq, Eq)]
struct MutStructFeatures {
    a: B4,
    b: B4,
}

#[test]
fn mut_struct_has_field_get() {
    let mut val = MutStructFeatures::from_bits(0xAB);
    let ref_ = MutStructFeaturesMutRef::new(BitPieceStorageMutRef::U8(&mut val.storage), 0);
    assert_eq!(ref_.a(), B4::new(0xB));
    assert_eq!(ref_.b(), B4::new(0xA));
}

#[test]
fn mut_struct_has_field_set() {
    let mut val = MutStructFeatures::from_bits(0);
    {
        let mut ref_ = MutStructFeaturesMutRef::new(BitPieceStorageMutRef::U8(&mut val.storage), 0);
        ref_.set_a(B4::new(5));
        ref_.set_b(B4::new(10));
    }
    assert_eq!(val.a(), B4::new(5));
    assert_eq!(val.b(), B4::new(10));
}

// =============================================================================
// MutStructAll preset
// =============================================================================

#[bitpiece(8, get, mut_struct_all)]
#[derive(Debug, PartialEq, Eq)]
struct MutStructAll {
    a: B4,
    b: B4,
}

#[test]
fn mut_struct_all_has_all_mut_features() {
    let mut val = MutStructAll::from_bits(0xAB);

    {
        let mut ref_ = MutStructAllMutRef::new(BitPieceStorageMutRef::U8(&mut val.storage), 0);

        // field_get
        assert_eq!(ref_.a(), B4::new(0xB));

        // field_set
        ref_.set_a(B4::new(5));
        assert_eq!(ref_.a(), B4::new(5));

        // field_mut
        let mut a_ref = ref_.a_mut();
        a_ref.set(B4::new(7));
    }

    assert_eq!(val.a(), B4::new(7));
}

// =============================================================================
// ConstEq only
// =============================================================================

#[bitpiece(8, get, const_eq)]
#[derive(Debug, PartialEq, Eq)]
struct ConstEqOnly {
    a: B4,
    b: B4,
}

#[test]
fn const_eq_only_has_const_eq() {
    let a = ConstEqOnly::from_bits(0xAB);
    let b = ConstEqOnly::from_bits(0xAB);
    let c = ConstEqOnly::from_bits(0xCD);
    assert!(ConstEqOnly::const_eq(a, b));
    assert!(!ConstEqOnly::const_eq(a, c));
}

// =============================================================================
// Enum with features
// =============================================================================

#[bitpiece(2, all)]
#[derive(Debug, PartialEq, Eq)]
enum EnumAllFeatures {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}
bitpiece_check_full_impl! {EnumAllFeatures, true}

#[test]
fn enum_all_features_has_const_eq() {
    assert!(EnumAllFeatures::const_eq(
        EnumAllFeatures::A,
        EnumAllFeatures::A
    ));
    assert!(!EnumAllFeatures::const_eq(
        EnumAllFeatures::A,
        EnumAllFeatures::B
    ));
}

#[test]
fn enum_all_features_has_fields() {
    let val = EnumAllFeatures::B;
    let fields = val.to_fields();
    assert_eq!(fields, EnumAllFeatures::B);

    let reconstructed = EnumAllFeatures::from_fields(fields);
    assert_eq!(reconstructed, val);
}

#[test]
fn enum_all_features_has_mut_ref() {
    let mut storage: u8 = 0;
    let mut ref_ = EnumAllFeaturesMutRef::new(BitPieceStorageMutRef::U8(&mut storage), 0);
    ref_.set(EnumAllFeatures::C);
    assert_eq!(ref_.get(), EnumAllFeatures::C);
}

// =============================================================================
// Explicit bit length with features
// =============================================================================

#[bitpiece(16, get, set)]
#[derive(Debug, PartialEq, Eq)]
struct ExplicitLenWithFeatures {
    a: B8,
    b: B8,
}

#[test]
fn explicit_len_with_features() {
    let mut val = ExplicitLenWithFeatures::from_bits(0);
    val.set_a(B8::new(0xAB));
    val.set_b(B8::new(0xCD));
    assert_eq!(val.a(), B8::new(0xAB));
    assert_eq!(val.b(), B8::new(0xCD));
}

// =============================================================================
// Auto bit length with features
// =============================================================================

#[bitpiece(get, set, with)]
#[derive(Debug, PartialEq, Eq)]
struct AutoLenWithFeatures {
    a: B3,
    b: B5,
}

#[test]
fn auto_len_with_features() {
    assert_eq!(AutoLenWithFeatures::BITS, 8);

    let val = AutoLenWithFeatures::from_bits(0)
        .with_a(B3::new(5))
        .with_b(B5::new(20));
    assert_eq!(val.a(), B3::new(5));
    assert_eq!(val.b(), B5::new(20));
}

// =============================================================================
// Combination of specific features
// =============================================================================

#[bitpiece(8, get, with, fields_struct, const_eq)]
#[derive(Debug, PartialEq, Eq)]
struct CustomCombination {
    a: B4,
    b: B4,
}

#[test]
fn custom_combination_has_selected_features() {
    // get
    let val = CustomCombination::from_bits(0xAB);
    assert_eq!(val.a(), B4::new(0xB));

    // with
    let val2 = val.with_a(B4::new(5));
    assert_eq!(val2.a(), B4::new(5));

    // fields_struct
    let fields = CustomCombinationFields {
        a: B4::new(1),
        b: B4::new(2),
    };
    let val3 = CustomCombination::from_fields(fields);
    assert_eq!(val3.to_fields(), fields);

    // const_eq
    assert!(CustomCombination::const_eq(val, val));
    assert!(!CustomCombination::const_eq(val, val2));
}

// =============================================================================
// Verify attributes are propagated to fields struct
// =============================================================================

#[bitpiece(8, all)]
#[derive(Debug, PartialEq, Eq, Hash)]
struct DerivedWithFields {
    a: B4,
    b: B4,
}

#[test]
fn fields_struct_has_derives() {
    use std::collections::HashSet;

    // Main struct has Hash
    let mut set = HashSet::new();
    set.insert(DerivedWithFields::from_bits(0x12));
    set.insert(DerivedWithFields::from_bits(0x34));
    assert_eq!(set.len(), 2);

    // Fields struct should also have Hash
    let mut fields_set = HashSet::new();
    fields_set.insert(DerivedWithFieldsFields {
        a: B4::new(1),
        b: B4::new(2),
    });
    fields_set.insert(DerivedWithFieldsFields {
        a: B4::new(3),
        b: B4::new(4),
    });
    assert_eq!(fields_set.len(), 2);
}
