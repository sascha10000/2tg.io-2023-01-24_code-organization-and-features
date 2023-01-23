#[cfg(feature = "f1")]
pub fn service() {
    println!("Iam a Service w f1");
}

#[cfg(not(feature = "f1"))]
pub fn service() {
    println!("Iam a Service wo f1");
}