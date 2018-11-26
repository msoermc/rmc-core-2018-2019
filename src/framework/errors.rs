pub enum SubsystemStatus<D, I, W, E, F> {
    Debug(D),
    Info(I),
    Warning(W),
    Error(E),
    Fatal(F)
}