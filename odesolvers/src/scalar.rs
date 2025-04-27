pub trait Float {
    fn float(value: f64) -> Self;
}

// this needs nightly to work
// impl Float for f16 {
//     fn float(value: f64) -> Self {
//         value as f16
//     }
// }

impl Float for f32 {
    fn float(value: f64) -> Self {
        value as f32
    }
}

impl Float for f64 {
    fn float(value: f64) -> Self {
        value
    }
}

// this needs nightly to work
// impl Float for f128 {
//     fn float(value: f64) -> Self {
//         value as f128
//     }
// }
