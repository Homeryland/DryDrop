use snafu::Snafu;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum AppError {
    DB {
        #[snafu(source(from(toasty::Error, |e| e)))]
        source: toasty::Error,
    },
}
