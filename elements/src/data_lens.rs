pub trait DataLens {
    type Data;

    fn data_lens(&self) -> Self::Data;
}
