#[macro_export]
macro_rules! bitpiece_check_const_assert_bits_eq {
    {$t: ty, $a: expr, $b: expr} => {
        if !<$t as $crate::BitPiece>::Converter::const_eq($a, $b) {
            panic!()
        }
    };
}

#[macro_export]
macro_rules! bitpiece_check_gen_values_to_check {
    ($t: ty, $supports_any_bit_pattern: literal) => {{
        type Converter = <$t as $crate::BitPiece>::Converter;

        const MAX_VAL: u64 = Converter::to_bits(<$t as $crate::BitPiece>::ONES) as u64;

        const VALUES_TO_CHECK: &[$t] = if $supports_any_bit_pattern {
            &[
                <$t as $crate::BitPiece>::ZEROES,
                <$t as $crate::BitPiece>::ONES,
                <$t as $crate::BitPiece>::MIN,
                <$t as $crate::BitPiece>::MAX,
                Converter::from_bits(
                    (0x31d6b601fb4faeb8u64 & MAX_VAL) as <$t as $crate::BitPiece>::Bits,
                ),
                Converter::from_bits(
                    (0xe9bd79bf8ca99263u64 & MAX_VAL) as <$t as $crate::BitPiece>::Bits,
                ),
            ]
        } else {
            &[
                <$t as $crate::BitPiece>::ZEROES,
                <$t as $crate::BitPiece>::ONES,
                <$t as $crate::BitPiece>::MIN,
                <$t as $crate::BitPiece>::MAX,
            ]
        };
        VALUES_TO_CHECK
    }};
}

#[macro_export]
macro_rules! bitpiece_check_do_for_each_value {
    {$t: ty, $value_var_name: ident, $supports_any_bit_pattern: literal, $body: block} => {
        let values_to_check = $crate::bitpiece_check_gen_values_to_check!{$t, $supports_any_bit_pattern};
        // can't use qualified path for `const_for`, must instead import it, since the `const_for` macro calls itself
        // in a non-hygiene way inside of its body.
        use $crate::const_for;
        const_for!(i in 0..values_to_check.len() => {
            let $value_var_name = values_to_check[i];
            $body
        });
    };
}
#[macro_export]
macro_rules! bitpiece_check_base_impl {
    {$t: ty, $supports_any_bit_pattern: literal} => {
        const _: () = {
            type Converter = <$t as $crate::BitPiece>::Converter;
            $crate::bitpiece_check_do_for_each_value!($t, value, $supports_any_bit_pattern, {
                $crate::bitpiece_check_const_assert_bits_eq!{
                    $t,
                    value,
                    Converter::from_bits(Converter::to_bits(value))
                };
                $crate::bitpiece_check_const_assert_bits_eq!{
                    $t,
                    value,
                    Converter::try_from_bits(Converter::to_bits(value)).unwrap()
                };
            });
        };
    };
}

#[macro_export]
macro_rules! bitpiece_check_fields_impl {
    {$t: ty, $supports_any_bit_pattern: literal} => {
        const _: () = {
            type Converter = <$t as $crate::BitPiece>::Converter;
            $crate::bitpiece_check_do_for_each_value!($t, value, $supports_any_bit_pattern, {
                $crate::bitpiece_check_const_assert_bits_eq!(
                    $t,
                    value,
                    Converter::from_fields(Converter::to_fields(value))
                );
            });
        };
    };
}

#[macro_export]
macro_rules! bitpiece_check_mut_impl {
    {$t: ty, $supports_any_bit_pattern: literal} => {
        const _: () = {
            type Converter = <$t as $crate::BitPiece>::Converter;
            type MutRefTy<'a> = <$t as $crate::BitPieceHasMutRef>::MutRef<'a>;
            $crate::bitpiece_check_do_for_each_value!($t, value, $supports_any_bit_pattern, {
                $crate::bitpiece_check_do_for_each_value!($t, value2, $supports_any_bit_pattern, {
                    let mut storage = Converter::to_bits(value) as u64;
                    let mut mut_ref = MutRefTy::new(BitPieceStorageMutRef::U64(&mut storage), 0);
                    $crate::bitpiece_check_const_assert_bits_eq!(
                        $t,
                        value,
                        mut_ref.get()
                    );
                    mut_ref.set(value2);
                    $crate::bitpiece_check_const_assert_bits_eq!(
                        $t,
                        value2,
                        mut_ref.get()
                    );
                });
            });
        };
    };
}

#[macro_export]
macro_rules! bitpiece_check_full_impl {
    {$t: ty, $supports_any_bit_pattern: literal} => {
        #[cfg(test)]
        const _: () = {
            bitpiece_check_base_impl! { $t, $supports_any_bit_pattern }
            bitpiece_check_fields_impl! { $t, $supports_any_bit_pattern }
            bitpiece_check_mut_impl! { $t, $supports_any_bit_pattern }
        };
    };
}
