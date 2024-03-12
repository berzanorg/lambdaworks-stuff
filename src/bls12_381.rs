use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        short_weierstrass::curves::bls12_381::{
            curve::BLS12381Curve, field_extension::BLS12381PrimeField,
        },
        traits::IsEllipticCurve,
    },
    field::traits::IsPrimeField,
};

#[test]
fn compute_public_key() {
    let pk = BLS12381PrimeField::from_hex("0x6C616D6264617370").unwrap();

    let generator = BLS12381Curve::generator();

    println!("{}", pk);

    assert_eq!(true, false);
}
