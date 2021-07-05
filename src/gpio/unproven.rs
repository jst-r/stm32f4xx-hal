use super::*;

pub trait OutPort {
    type Target;
    fn outport(self) -> Self::Target;
}

macro_rules! out_port {
    ( $name:ident => ( $($i:literal),+ ), ( $($N:ident),+ ), ( $($d:ident),* )) => {
        pub struct $name<const P: char $(, const $N: u8)+> {
            $(pub $d: Pin<Output<PushPull>, P, $N>,)+
        }

        impl<const P: char $(, const $N: u8)+> OutPort for ($(Pin<Output<PushPull>, P, $N>),+) {
            type Target = $name<P $(, $N)+>;
            fn outport(self) -> Self::Target {
                let ($($d),+) = self;
                Self::Target { $($d),+ }
            }
        }

        impl<const P: char $(, const $N: u8)+> $name<P $(, $N)+> {
            pub const fn new(
                $($d: Pin<Output<PushPull>, P, $N>,)+
            ) -> Self {
                Self { $($d),+ }
            }
            const fn value_for_write_bsrr(val: u32) -> u32 {
                $(let $d = ((val >> $i) & 0b1) != 0;)+
                let r = 0;
                $(let r = r | (1 << (if $d { $N } else { $N + 16 }));)+
                r
            }
            pub fn write_u8(&mut self, word: u8) {
                unsafe {
                    (*Gpio::<P>::ptr())
                        .bsrr
                        .write(|w| w.bits(Self::value_for_write_bsrr(word as u32)))
                }
            }
        }
    }
}

out_port!(OutPort2 => (0, 1), (N0, N1), (d0, d1));
out_port!(OutPort3 => (0, 1, 2), (N0, N1, N2), (d0, d1, d2));
out_port!(OutPort4 => (0, 1, 2, 3), (N0, N1, N2, N3), (d0, d1, d2, d3));
out_port!(OutPort5 => (0, 1, 2, 3, 4), (N0, N1, N2, N3, N4), (d0, d1, d2, d3, d4));
out_port!(OutPort6 => (0, 1, 2, 3, 4, 5), (N0, N1, N2, N3, N4, N5), (d0, d1, d2, d3, d4, d5));
out_port!(OutPort7 => (0, 1, 2, 3, 4, 5, 6), (N0, N1, N2, N3, N4, N5, N6), (d0, d1, d2, d3, d4, d5, d6));
out_port!(OutPort8 => (0, 1, 2, 3, 4, 5, 6, 7), (N0, N1, N2, N3, N4, N5, N6, N7), (d0, d1, d2, d3, d4, d5, d6, d7));
