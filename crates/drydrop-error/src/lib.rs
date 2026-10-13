use snafu::Snafu;

pub type DryDropResult<T> = Result<T, DryDropError>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DryDropError {
    #[cfg(feature = "server")]
    DB {
        #[snafu(source(from(toasty::Error, |e| e)))]
        source: toasty::Error,
    },
}
