#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum Either<A, B> {
	A(A),
	B(B),
}
