#[cfg(test)]
mod tests {

    use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField; 

    use lambdaworks_math::{
        field::element::FieldElement,
        polynomial::Polynomial,
    };

    const ORDER: u64 = 1613; // Defining modulo
    type F = U64PrimeField<ORDER>;
    type FE = FieldElement<F>;

    // Polynomial:

    fn polynomial_f() -> Polynomial<FE> {
        Polynomial::new(&[FE::new(1234), FE::new(166), FE::new(94)])
    }

    // Defining shares:

    fn defining_shares (a: FE) -> FE {
        let result = polynomial_f().evaluate(&a);
        return result;
    }

    // Reconstructing the secret using a subset of shares:

    fn ver (a: u64, b: u64, c: u64) -> FE {

        let subset_shares: Vec<(FE, FE)> = vec! [
            (FE::new(a), defining_shares(FE::new(a))),
            (FE::new(b), defining_shares(FE::new(b))),
            (FE::new(c), defining_shares(FE::new(c)))];    

        let mut reconstructed_secret = FE::new(0);

        for i in 0..3 {
            
            let (x_i, y_i) = subset_shares[i];

            let mut numerator: FE = FE::new(1);
            let mut denominator: FE = FE::new(1);
            for j in 0..3 {
                if i != j {
                    let (x_j, _) = subset_shares[j];
                    numerator = numerator * (- x_j);
                    denominator = denominator * (x_i - x_j);
                }
            }
            
            let pow_result = denominator.pow(1611_u64);
            reconstructed_secret = reconstructed_secret + (y_i * numerator * pow_result);        
            
        }

        return reconstructed_secret;
    }

    #[test]
    fn test() {
        assert_eq!(ver (1, 2, 3), FE::new(1234));
    }
}
