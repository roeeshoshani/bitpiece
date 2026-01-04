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
    ($t: ty) => {{
        type Converter = <$t as $crate::BitPiece>::Converter;

        let max_val = Converter::to_bits(<$t as $crate::BitPiece>::ONES) as u64;

        &[
            <$t as $crate::BitPiece>::ZEROES,
            <$t as $crate::BitPiece>::ONES,
            Converter::from_bits(
                (0x31d6b601fb4faeb8u64 & max_val) as <$t as $crate::BitPiece>::Bits,
            ),
            Converter::from_bits(
                (0xe9bd79bf8ca99263u64 & max_val) as <$t as $crate::BitPiece>::Bits,
            ),
        ]
    }};
}

#[macro_export]
macro_rules! bitpiece_check_do_for_each_value {
    {$t: ty, $value_var_name: ident, $body: block} => {
        const _: () = {
            type Converter = <$t as $crate::BitPiece>::Converter;
            let values_to_check = $crate::bitpiece_check_gen_values_to_check!{$t};
            // can't use qualified path for `const_for`, must instead import it, since the `const_for` macro calls itself
            // in a non-hygiene way inside of its body.
            use $crate::const_for;
            const_for!(i in 0..values_to_check.len() => {
                let $value_var_name = values_to_check[i];
                $body
            });
        };
    };
}
#[macro_export]
macro_rules! bitpiece_check_base_impl {
    {$t: ty} => {
        const _: () = {
            $crate::bitpiece_check_do_for_each_value!($t, value, {
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
    {$t: ty} => {
        const _: () = {
            $crate::bitpiece_check_do_for_each_value!($t, value, {
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
    {$t: ty} => {
        const _: () = {
            type _Mut = <$t as $crate::BitPieceHasMutRef>::MutRef<'static>;
        };
    };
}

#[macro_export]
macro_rules! bitpiece_check_full_impl {
    {$t: ty} => {
        #[cfg(test)]
        const _: () = {
            bitpiece_check_base_impl!{$t}
            bitpiece_check_fields_impl!{$t}
            bitpiece_check_mut_impl!{$t}
        };
    };
}
