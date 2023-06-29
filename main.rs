use std::ops::DerefMut;
use std::io::Write;
fn main() {
    solve();
    let mut locked_stdout = proconio::STDOUT.get_or_init(|| {
        std::sync::Mutex::new(
            std::io::BufWriter::new(std::io::stdout())
        )
    }).lock()
    .expect("failed to lock stdout.");
    locked_stdout.deref_mut().flush().unwrap();
}

#[allow(dead_code)]
mod proconio {
    // proconio.rs: https://docs.rs/proconio/latest/proconio/
    // The macro’s user interface is basically the same with tanakh's input macro: https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8

    // [Changelog by juyoung35]
    // - changed `input_interactive!` into `inputln!`.
    // - elided some fileds, struct: `AutoSource`
    // - `input!` doesn't match to `AutoSource` but to `OnceSource`.
    // - some modules are omitted, also module structure changed.

    use std::io::{BufRead, BufReader, Stdin, BufWriter, Stdout};
    use std::sync::OnceLock;
    use std::sync::Mutex;
    
    pub use source::{Source, OnceSource, LineSource};
    pub use source::Readable as __Readable;
    
    pub static STDOUT: OnceLock<Mutex<BufWriter<Stdout>>> = OnceLock::new();

    #[macro_export]
    macro_rules! print {
        ($($tt:tt)*) => {
            use std::io::Write;
            let mut locked_stdout = $crate::proconio::STDOUT.get_or_init(|| {
                std::sync::Mutex::new(
                    std::io::BufWriter::new(std::io::stdout())
                )
            }).lock()
            .expect("failed to lock stdout.");
            write!(locked_stdout, $($tt)*).unwrap();
            drop(locked_stdout);
        };
    }
    #[macro_export]
    macro_rules! println {
        ($($tt:tt)*) => {
            use std::io::Write;
            let mut locked_stdout = $crate::proconio::STDOUT.get_or_init(|| {
                std::sync::Mutex::new(
                    std::io::BufWriter::new(std::io::stdout())
                )
            }).lock()
            .expect("failed to lock stdout.");
            writeln!(locked_stdout, $($tt)*).unwrap();
            drop(locked_stdout);
        };
    }

    pub static STDIN_SOURCE: OnceLock<Mutex<StdinSource<BufReader<Stdin>>>> = OnceLock::new();

    #[macro_export]
    macro_rules! input {
        // terminator
        (@from [$source:expr] @rest) => {};
    
        // parse mutability
        (@from [$source:expr] @rest mut $($rest:tt)*) => {
            $crate::input! {
                @from [$source]
                @mut [mut]
                @rest $($rest)*
            }
        };
        (@from [$source:expr] @rest $($rest:tt)*) => {
            $crate::input! {
                @from [$source]
                @mut []
                @rest $($rest)*
            }
        };
    
        // parse variable pattern
        (@from [$source:expr] @mut [$($mut:tt)?] @rest $var:tt: $($rest:tt)*) => {
            $crate::input! {
                @from [$source]
                @mut [$($mut)*]
                @var $var
                @kind []
                @rest $($rest)*
            }
        };

        // parse kind (type)
        (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest) => {
            let $($mut)* $var = $crate::read_value!(@source [$source] @kind [$($kind)*]);
        };
        (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest, $($rest:tt)*) => {
            $crate::input!(@from [$source] @mut [$($mut)*] @var $var @kind [$($kind)*] @rest);
            $crate::input!(@from [$source] @rest $($rest)*);
        };
        (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @kind [$($kind:tt)*] @rest $tt:tt $($rest:tt)*) => {
            $crate::input!(@from [$source] @mut [$($mut)*] @var $var @kind [$($kind)* $tt] @rest $($rest)*);
        };

        (from $source:expr, $($rest:tt)*) => {
            #[allow(unused_variables, unused_mut)]
            let mut s = $source;
            $crate::input! {
                @from [&mut s]
                @rest $($rest)*
            }
        };
        ($($rest:tt)*) => {
            let mut locked_stdin = $crate::proconio::STDIN_SOURCE
                .get_or_init(|| {
                    std::sync::Mutex::new(
                        $crate::proconio::StdinSource::Once(
                            $crate::proconio::OnceSource::new(std::io::BufReader::new(std::io::stdin()))
                        )
                    )
                })
                .lock()
                .expect("failed to lock stdin.");
            $crate::input! {
                @from [&mut *locked_stdin]
                @rest $($rest)*
            }
            drop(locked_stdin);
        };
    }

    #[macro_export]
    macro_rules! inputln {
        ($($rest:tt)*) => {
            let mut locked_stdin = $crate::proconio::STDIN_SOURCE
                .get_or_init(|| {
                    std::sync::Mutex::new(
                        $crate::proconio::StdinSource::Line(
                            $crate::proconio::LineSource::new(std::io::BufReader::new(std::io::stdin()))
                        )
                    )
                })
                .lock()
                .expect("failed to lock stdin.");
            $crate::input! {
                from &mut *locked_stdin,
                $($rest)*
            }
            drop(locked_stdin);
        };
    }
    
    #[macro_export]
    macro_rules! read_value {
        // array and variable length array
        (@source [$source:expr] @kind [[$($kind:tt)*]]) => {
            $crate::read_value!(@array @source [$source] @kind [] @rest $($kind)*)
        };
        (@array @source [$source:expr] @kind [$($kind:tt)*] @rest) => {{
            let len = <usize as $crate::__Readable>::read($source);
            $crate::read_value!(@source [$source] @kind [[$($kind)*; len]])
        }};
        (@array @source [$source:expr] @kind [$($kind:tt)*] @rest ; $($rest:tt)*) => {
            $crate::read_value!(@array @source [$source] @kind [$($kind)*] @len [$($rest)*])
        };
        (@array @source [$source:expr] @kind [$($kind:tt)*] @rest $tt:tt $($rest:tt)*) => {
            $crate::read_value!(@array @source [$source] @kind [$($kind)* $tt] @rest $($rest)*)
        };
        (@array @source [$source:expr] @kind [$($kind:tt)*] @len [$($len:tt)*]) => {{
            let len = $($len)*;
            (0..len)
                .map(|_| $crate::read_value!(@source [$source] @kind [$($kind)*]))
                .collect::<Vec<_>>()
        }};
    
        // tuple
        (@source [$source:expr] @kind [($($kinds:tt)*)]) => {
            $crate::read_value!(@tuple @source [$source] @kinds [] @current [] @rest $($kinds)*)
        };
        (@tuple @source [$source:expr] @kinds [$([$($kind:tt)*])*] @current [] @rest) => {
            (
                $($crate::read_value!(@source [$source] @kind [$($kind)*]),)*
            )
        };
        (@tuple @source [$source:expr] @kinds [$($kinds:tt)*] @current [$($curr:tt)*] @rest) => {
            $crate::read_value!(@tuple @source [$source] @kinds [$($kinds)* [$($curr)*]] @current [] @rest)
        };
        (@tuple @source [$source:expr] @kinds [$($kinds:tt)*] @current [$($curr:tt)*] @rest, $($rest:tt)*) => {
            $crate::read_value!(@tuple @source [$source] @kinds [$($kinds)* [$($curr)*]] @current [] @rest $($rest)*)
        };
        (@tuple @source [$source:expr] @kinds [$($kinds:tt)*] @current [$($curr:tt)*] @rest $tt:tt $($rest:tt)*) => {
            $crate::read_value!(@tuple @source [$source] @kinds [$($kinds)*] @current [$($curr)* $tt] @rest $($rest)*)
        };
    
        // unreachable
        (@source [$source:expr] @kind []) => {
            compile_error!("reached unreachable statement while parsing macro input.");
        };
    
        // normal other
        (@source [$source:expr] @kind [$kind:ty]) => {
            <$kind as $crate::proconio::__Readable>::read($source)
        }
    }
    
    pub enum StdinSource<R: BufRead> {
        Once(OnceSource<R>),
        Line(LineSource<R>),
    }
    impl<R: BufRead> Source<R> for StdinSource<R> {
        fn next_token(&mut self) -> Option<&str> {
            match self {
                Self::Once(source) => source.next_token(),
                Self::Line(source) => source.next_token(),
            }
        }
        fn is_empty(&mut self) -> bool {
            match self {
                Self::Once(source) => source.is_empty(),
                Self::Line(source) => source.is_empty(),
            }
        }
    }

    pub mod source {
        use std::io::BufRead;
        use std::marker::PhantomData;
        use std::ptr::NonNull;
        use std::str::{FromStr, SplitAsciiWhitespace};
        use std::fmt::Debug;
        use std::any::type_name;
        use std::iter::Peekable;
    
        // Used for source of `input!` macro.
        pub trait Source<R: BufRead> {
            // Get a whitespace-splitted next token.
            fn next_token(&mut self) -> Option<&str>;
            // Check if tokens are empty.
            fn is_empty(&mut self) -> bool;
            // Coerce to get a whitespace-splitted next token.
            fn next_token_unwrap(&mut self) -> &str {
                self.next_token().expect("failed to get the next token.")
            }
        }
        impl<R: BufRead, S: Source<R>> Source<R> for &'_ mut S {
            fn next_token(&mut self) -> Option<&str> {
                (*self).next_token()
            }
            fn is_empty(&mut self) -> bool {
                (*self).is_empty()
            }
        }
    
        // Can be read from `Source`.
        pub trait Readable {
            type Output;
            fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Self::Output;
        }
        // implmentations of Readable for any `FromStr` types including primitives.
        impl<T: FromStr> Readable for T
        where T::Err: Debug,
        {
            type Output = T;
            fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Self::Output {
                let token = source.next_token_unwrap();
                match token.parse() {
                    Ok(v) => v,
                    Err(e) => panic!(
                        concat!(
                            "failed to parse the input: `{input}`",
                            "to the value of type `{ty}`: {err:?}."
                        ),
                        input = token,
                        ty = type_name::<T>(),
                        err = e,
                    ),
                }
            }
        }
    
        struct Tokens {
            tokens: Peekable<SplitAsciiWhitespace<'static>>,
        }
        impl From<String> for Tokens {
            fn from(current_context: String) -> Self {
                let b = current_context.into_boxed_str();
                let current_context = NonNull::new(Box::leak(b)).unwrap();
        
                // # Safety
                //
                // - `tokens` is dropped before `current_context`.
                // - `current_context` is not accessed directly until dropped.
                unsafe {
                    // using `split_ascii_whitespace` instead of `split_whitespace` for `'static` lifetime.
                    let tokens = current_context.as_ref().split_ascii_whitespace().peekable();
                    Self {
                        tokens,
                    }
                }
            }
        }
        impl Tokens {
            fn next_token(&mut self) -> Option<&str> {
                self.tokens.next()
            }
            fn is_empty(&mut self) -> bool {
                self.tokens.peek().is_none()
            }
        }
    
        // Source reading entire content for the first time.
        pub struct OnceSource<R: BufRead> {
            tokens: Tokens,
            _read: PhantomData<R>,
        }
        impl<R: BufRead> OnceSource<R> {
            pub fn new(mut source: R) -> OnceSource<R> {
                let mut context = String::new();
                source
                    .read_to_string(&mut context)
                    .expect("failed to read from source.");
                Self {
                    tokens: context.into(),
                    _read: PhantomData,
                }
            }
        }
        impl<R: BufRead> Source<R> for OnceSource<R> {
            fn next_token(&mut self) -> Option<&str> {
                self.tokens.next_token()
            }
            fn is_empty(&mut self) -> bool {
                self.tokens.is_empty()
            }
        }
    
        // Source reading stream line by line.
        pub struct LineSource<R: BufRead> {
            tokens: Tokens,
            reader: R,
        }
        impl<R: BufRead> LineSource<R> {
            pub fn new(reader: R) -> LineSource<R> {
                Self {
                    tokens: "".to_string().into(),
                    reader,
                }
            }
            fn prepare(&mut self) {
                while self.tokens.is_empty() {
                    let mut line = String::new();
                    let num_bytes = self.reader
                        .read_line(&mut line)
                        .expect("failed to read newline(the 0xA byte).");
                    // reached EOF
                    if num_bytes == 0 { return }
                    self.tokens = line.into();
                }
            }
        }
        impl<R: BufRead> Source<R> for LineSource<R> {
            fn next_token(&mut self) -> Option<&str> {
                self.prepare();
                self.tokens.next_token()
            }
            fn is_empty(&mut self) -> bool {
                self.prepare();
                self.tokens.is_empty()
            }
        }
    }
}

fn solve() {
    println!("Hello World!");
}
