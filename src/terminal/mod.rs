pub(crate) mod ansi;
pub(crate) mod capabilities;
pub(crate) mod emulator;
pub(crate) mod image;
pub(crate) mod printer;
pub(crate) mod virt;

pub(crate) use printer::{Terminal, TerminalWrite, should_hide_cursor};

#[derive(Clone, Debug)]
pub enum GraphicsMode {
    Iterm2,
    Kitty {
        mode: image::protocols::kitty::KittyMode,
        inside_tmux: bool,
    },
    AsciiBlocks,
    Raw,
    #[cfg(feature = "sixel")]
    Sixel,
}
