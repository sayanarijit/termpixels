pub enum ExitCode {
    Success = 0,
    GeneralFailiure = 1,
    GracefulStop = 130,
    ForcefulStop = 132,
    UnknownFailure = 200,
}
