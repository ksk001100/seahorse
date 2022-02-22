pub(crate) trait Help {
    fn help_text(&self) -> String;

    fn help(&self);
}
