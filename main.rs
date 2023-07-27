#![no_main]
use std::io::{self, BufReader, Stdin, BufWriter, StdoutLock, Write};
use std::sync::Mutex;
#[allow(unused_imports)]
use proconio::{StdinSource, OnceSource, LineSource};
static mut STDIN_SOURCE: Option<Mutex<StdinSource<BufReader<Stdin>>>> = None;
static mut WRITER: Option<BufWriter<StdoutLock>> = None;

#[no_mangle]
unsafe fn main() {
    WRITER = Some(BufWriter::new(io::stdout().lock()));
    solve();
    WRITER.as_mut().unwrap_unchecked().flush().unwrap();
}

#[allow(dead_code)]
mod proconio {
    // proconio.rs: https://docs.rs/proconio/latest/proconio/
    // The macro’s user interface is basically the same with tanakh's input macro: https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8

    // [Changelog by juyoung35]
    // - now `input!` has new mode: while statement conditional assignment to possibly infinite length of 1-dimensional vector.
    // - usage: input!(var while var != 0 => [usize])
    // - changed `input_interactive!` into `inputln!`.
    // - `inputln` doesn't split ascii whitespace.
    // - probably you can not use both OnceSource and LineSource at the same time.
    // - deleted struct: `AutoSource`
    // - `input!` doesn't match to `AutoSource` but to `OnceSource`.
    // - `print!` and `println!` macros are have been changed.
    use std::io::BufRead;
    
    pub use source::{Source, OnceSource, LineSource};
    pub use source::Readable as __Readable;
    pub use source::CheckedReadable as __CheckedReadable;

    #[macro_export]
    macro_rules! print {
        ($($tt:tt)*) => {
            write!(unsafe { WRITER.as_mut().unwrap_unchecked() }, $($tt)*).unwrap()
        };
    }
    #[macro_export]
    macro_rules! println {
        ($($tt:tt)*) => {
            writeln!(unsafe { WRITER.as_mut().unwrap_unchecked() }, $($tt)*).unwrap()
        };
    }

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

        // parse variable pattern with while statement condition
        (@from [$source:expr] @mut [$($mut:tt)?] @rest $var:tt while $condition:expr => $($rest:tt)*) => {
            $crate::input! {
                @from [$source]
                @mut [$($mut)*]
                @var $var
                @condition [$condition]
                @kind []
                @rest $($rest)*
            }
        };
        // parse variable pattern
        (@from [$source:expr] @mut [$($mut:tt)?] @rest $var:tt: $($rest:tt)*) => {
            $crate::input! {
                @from [$source]
                @mut [$($mut)*]
                @var $var
                @condition []
                @kind []
                @rest $($rest)*
            }
        };

        // parse kind (type)
        (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @condition [$($condition:expr)?] @kind [$($kind:tt)*] @rest) => {
            let $($mut)* $var = $crate::read_value!(@source [$source] @var $var @condition [$($condition)*] @kind [$($kind)*]);
        };
        // parse Adjacency List
        // usage:
        // input! {
        //     n: usize, m: usize,
        //     mut adj_list: AdjList<Usize1, usize>::new(n, m),
        // }
        (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @condition [$($condition:expr)?] @kind [$($kind:tt)*] @rest DirectedAdjList<$t:tt, $u:tt>::new($v:expr, $e:expr), $($rest:tt)*) => {
            let $($mut)* $var = vec![vec![]; $v];
            for _ in 0..$e {
                $crate::input!(u: $u, v: $u, t: $t);
                $var[u].push((v, t));
                $var[v].push((u, t));
            }
            $crate::input!(@from [$source] @rest $($rest)*);
        };
        (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @condition [$($condition:expr)?] @kind [$($kind:tt)*] @rest UnDirectedAdjList<$t:tt, $u:tt>::new($v:expr, $e:expr), $($rest:tt)*) => {
            let $($mut)* $var = vec![vec![]; $v];
            for _ in 0..$e {
                $crate::input!(u: $u, v: $u, t: $t);
                $var[u].push((v, t));
                $var[v].push((u, t));
            }
            $crate::input!(@from [$source] @rest $($rest)*);
        };
        (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @condition [$($condition:expr)?] @kind [$($kind:tt)*] @rest, $($rest:tt)*) => {
            $crate::input!(@from [$source] @mut [$($mut)*] @var $var @condition [$($condition)*] @kind [$($kind)*] @rest);
            $crate::input!(@from [$source] @rest $($rest)*);
        };
        (@from [$source:expr] @mut [$($mut:tt)?] @var $var:tt @condition [$($condition:expr)?] @kind [$($kind:tt)*] @rest $tt:tt $($rest:tt)*) => {
            $crate::input!(@from [$source] @mut [$($mut)*] @var $var @condition [$($condition)*] @kind [$($kind)* $tt] @rest $($rest)*);
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
            if STDIN_SOURCE.is_none() {
                STDIN_SOURCE = Some(Mutex::new(StdinSource::Once(
                    OnceSource::new(std::io::BufReader::new(io::stdin()))
                )));
            }
            let mut locked_stdin = STDIN_SOURCE.as_mut().unwrap_unchecked().lock().unwrap();
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
            if STDIN_SOURCE.is_none() {
                STDIN_SOURCE = Some(Mutex::new(StdinSource::Line(
                    LineSource::new(std::io::BufReader::new(io::stdin()))
                )));
            }
            let mut locked_stdin = STDIN_SOURCE.as_mut().unwrap_unchecked().lock().unwrap();
            $crate::input! {
                from &mut *locked_stdin,
                $($rest)*
            }
            drop(locked_stdin);
        };
    }
    
    #[macro_export]
    macro_rules! read_value {
        // conditionally fills vectors of undefined length
        (@source [$source:expr] @var $var:tt @condition [$condition:expr] @kind [[$kind:tt]]) => {{
            let mut res = vec![];
            while let Some($var) = <$kind as $crate::proconio::__CheckedReadable>::checked_read($source) {
                if !$condition { break }
                res.push($var);
            }
            res
        }};
        // array and variable length array
        (@source [$source:expr] @var $var:tt @condition [] @kind [[$($kind:tt)*]]) => {
            $crate::read_value!(@array @source [$source] @kind [] @rest $($kind)*)
        };
        (@source [$source:expr] @kind [[$($kind:tt)*]]) => {
            $crate::read_value!(@array @source [$source] @kind [] @rest $($kind)*)
        };
        (@array @source [$source:expr] @kind [$($kind:tt)*] @rest) => {{
            let len = <usize as $crate::proconio::__Readable>::read($source);
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
        (@source [$source:expr] @var $var:tt @condition [$($condition:expr)?] @kind [($($kinds:tt)*)]) => {
            $crate::read_value!(@tuple @source [$source] @kinds [] @current [] @rest $($kinds)*)
        };
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
    
        // undesired while statement pattern
        (@source [$source:expr] @var $var:tt @condition [$condition:expr] @kind [$kind:ty]) => {
            compile_error("giving while state pattern to anything other than a vector is an undesired pattern.")
        };

        // unreachable
        (@source [$source:expr] @var $var:tt @condition [$($condition:expr)?] @kind []) => {
            compile_error!("reached unreachable statement while parsing macro input.");
        };
    
        // normal other
        (@source [$source:expr] @var $var:tt @condition [$($condition:expr)?] @kind [AdjList<$t:tt, $u:tt>::new($v:tt, $e:tt)]) => {
            // $crate::read_value!(@source [$source] @kind [$kind])
        };
        (@source [$source:expr] @var $var:tt @condition [$($condition:expr)?] @kind [$kind:ty]) => {
            $crate::read_value!(@source [$source] @kind [$kind])
        };
        (@source [$source:expr] @kind [$kind:ty]) => {
            <$kind as $crate::proconio::__Readable>::read($source)
        }
    }
    
    #[derive(Debug)]
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
        impl Readable for String {
            type Output = Self;
            fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Self::Output {
                let token = source.next_token_unwrap();
                token.to_string()
            }
        }
        impl<T> Readable for Option<T>
        where
            T: Readable + FromStr,
            <T as FromStr>::Err: Debug,
        {
            type Output = Self;
            fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Self::Output {
                if let Some(token) = source.next_token() {
                    match token.parse() {
                        Ok(v) => Some(v),
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
                } else { None }
            }
        }
        // implmentations of Readable for any `FromStr` types including primitives.
        macro_rules! impl_readable {
            ($($t:ty) *) => {
                $(impl Readable for $t {
                    type Output = Self;
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
                                ty = type_name::<$t>(),
                                err = e,
                            ),
                        }
                    }
                })*
            };
        }
        // impl_readable!(IpAddr SocketAddr bool char f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize OsString Ipv4Addr Ipv6Addr SocketAddrV4 SocketAddrV6 NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize PathBuf TokenStream);
        impl_readable!(bool char f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
        pub trait CheckedReadable {
            type Output;
            fn checked_read<R: BufRead, S: Source<R>>(source: &mut S) -> Option<Self::Output>;
        }
        // implmentations of CheckedReadable for any `FromStr` types including primitives.
        impl<T: FromStr> CheckedReadable for T
        where T::Err: Debug,
        {
            type Output = T;
            fn checked_read<R: BufRead, S: Source<R>>(source: &mut S) -> Option<Self::Output> {
                let token = source.next_token()?;
                match token.parse() {
                    Ok(v) => Some(v),
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

        #[derive(Debug)]
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
            // fn peek(&mut self) -> Option<&str> {
            //     self.tokens.peek().copied()
            // }
        }
    
        // Source reading entire content for the first time.
        #[derive(Debug)]
        pub struct OnceSource<R: BufRead> {
            tokens: Tokens,
            _read: PhantomData<R>,
        }
        impl<R: BufRead> OnceSource<R> {
            pub fn new(mut source: R) -> OnceSource<R> {
                let mut context = String::new();
                #[cfg(target_os = "windows")]
                let _ = source
                    .read_to_string(&mut context);

                #[cfg(not(target_os = "windows"))]
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
        #[derive(Debug)]
        pub struct LineSource<R: BufRead> {
            token: Option<String>,
            tokens: Peekable<std::io::Lines<R>>,
            _read: PhantomData<R>,
        }
        impl<R: BufRead> LineSource<R> {
            // Creates a `LineSource` by specified `BufRead`.
            pub unsafe fn new(source: R) -> LineSource<R> {
                let lines = source.lines();
                let tokens = lines.peekable();
                Self {
                    token: None,
                    tokens,
                    _read: PhantomData,
                }
            }
        }
        impl<R: BufRead> Source<R> for LineSource<R> {
            fn next_token(&mut self) -> Option<&str> {
                self.token = self.tokens.next().and_then(|token| token.ok());
                self.token.as_ref().map(|token| token.as_str())
            }
            fn is_empty(&mut self) -> bool {
                self.tokens.peek().is_none()
            }
        }
    }
    pub mod marker {
        use std::io::BufRead;
        use super::source::{Source, Readable};
        // Usize1: 1-indexed usize.  Output of reading has type usize.
        pub enum Usize1 {}
        impl Readable for Usize1 {
            type Output = usize;
            fn read<R: BufRead, S: Source<R>>(source: &mut S) -> usize {
                // panic if the subtraction overflows
                usize::read(source)
                    .checked_sub(1)
                    .expect("attempted to read the value 0 as a Usize1")
            }
        }
    }
}
