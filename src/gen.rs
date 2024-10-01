pub mod variant {
    pub enum Tuple0 {}
    #[repr(usize, C)]
    pub enum Tuple1<A> {
        Variant0(A),
    }
    #[repr(usize, C)]
    pub enum Tuple2<A, B> {
        Variant0(A),
        Variant1(B),
    }
    #[repr(usize, C)]
    pub enum Tuple3<A, B, C> {
        Variant0(A),
        Variant1(B),
        Variant2(C),
    }
    #[repr(usize, C)]
    pub enum Tuple4<A, B, C, D> {
        Variant0(A),
        Variant1(B),
        Variant2(C),
        Variant3(D),
    }
    #[repr(usize, C)]
    pub enum Tuple5<A, B, C, D, E> {
        Variant0(A),
        Variant1(B),
        Variant2(C),
        Variant3(D),
        Variant4(E),
    }
    #[repr(usize, C)]
    pub enum Tuple6<A, B, C, D, E, F> {
        Variant0(A),
        Variant1(B),
        Variant2(C),
        Variant3(D),
        Variant4(E),
        Variant5(F),
    }
    #[repr(usize, C)]
    pub enum Tuple7<A, B, C, D, E, F, G> {
        Variant0(A),
        Variant1(B),
        Variant2(C),
        Variant3(D),
        Variant4(E),
        Variant5(F),
        Variant6(G),
    }
    #[repr(usize, C)]
    pub enum Tuple8<A, B, C, D, E, F, G, H> {
        Variant0(A),
        Variant1(B),
        Variant2(C),
        Variant3(D),
        Variant4(E),
        Variant5(F),
        Variant6(G),
        Variant7(H),
    }
    #[repr(usize, C)]
    pub enum Tuple9<A, B, C, D, E, F, G, H, I> {
        Variant0(A),
        Variant1(B),
        Variant2(C),
        Variant3(D),
        Variant4(E),
        Variant5(F),
        Variant6(G),
        Variant7(H),
        Variant8(I),
    }
    #[repr(usize, C)]
    pub enum Tuple10<A, B, C, D, E, F, G, H, I, J> {
        Variant0(A),
        Variant1(B),
        Variant2(C),
        Variant3(D),
        Variant4(E),
        Variant5(F),
        Variant6(G),
        Variant7(H),
        Variant8(I),
        Variant9(J),
    }
    #[repr(usize, C)]
    pub enum Tuple11<A, B, C, D, E, F, G, H, I, J, K> {
        Variant0(A),
        Variant1(B),
        Variant2(C),
        Variant3(D),
        Variant4(E),
        Variant5(F),
        Variant6(G),
        Variant7(H),
        Variant8(I),
        Variant9(J),
        Variant10(K),
    }
    #[repr(usize, C)]
    pub enum Tuple12<A, B, C, D, E, F, G, H, I, J, K, L> {
        Variant0(A),
        Variant1(B),
        Variant2(C),
        Variant3(D),
        Variant4(E),
        Variant5(F),
        Variant6(G),
        Variant7(H),
        Variant8(I),
        Variant9(J),
        Variant10(K),
        Variant11(L),
    }
    #[repr(usize, C)]
    pub enum Tuple13<A, B, C, D, E, F, G, H, I, J, K, L, M> {
        Variant0(A),
        Variant1(B),
        Variant2(C),
        Variant3(D),
        Variant4(E),
        Variant5(F),
        Variant6(G),
        Variant7(H),
        Variant8(I),
        Variant9(J),
        Variant10(K),
        Variant11(L),
        Variant12(M),
    }
    #[repr(usize, C)]
    pub enum Tuple14<A, B, C, D, E, F, G, H, I, J, K, L, M, N> {
        Variant0(A),
        Variant1(B),
        Variant2(C),
        Variant3(D),
        Variant4(E),
        Variant5(F),
        Variant6(G),
        Variant7(H),
        Variant8(I),
        Variant9(J),
        Variant10(K),
        Variant11(L),
        Variant12(M),
        Variant13(N),
    }
    #[repr(usize, C)]
    pub enum Tuple15<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> {
        Variant0(A),
        Variant1(B),
        Variant2(C),
        Variant3(D),
        Variant4(E),
        Variant5(F),
        Variant6(G),
        Variant7(H),
        Variant8(I),
        Variant9(J),
        Variant10(K),
        Variant11(L),
        Variant12(M),
        Variant13(N),
        Variant14(O),
    }
    #[repr(usize, C)]
    pub enum Tuple16<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> {
        Variant0(A),
        Variant1(B),
        Variant2(C),
        Variant3(D),
        Variant4(E),
        Variant5(F),
        Variant6(G),
        Variant7(H),
        Variant8(I),
        Variant9(J),
        Variant10(K),
        Variant11(L),
        Variant12(M),
        Variant13(N),
        Variant14(O),
        Variant15(P),
    }
}
use variant::*;
pub mod mapping {
    pub trait Map1<IA> {
        type Result0;
        fn map_0(self, value: IA) -> Self::Result0;
        #[allow(clippy::type_complexity)]
        fn apply(self, value: (IA,)) -> (Self::Result0,);
    }
    impl<IA, OA, FA> Map1<IA> for (FA,)
    where
        FA: FnMut(IA) -> OA,
    {
        type Result0 = OA;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn apply(mut self, value: (IA,)) -> (Self::Result0,) {
            ((self.0)(value.0),)
        }
    }
    pub trait Map2<IA, IB> {
        type Result0;
        type Result1;
        fn map_0(self, value: IA) -> Self::Result0;
        fn map_1(self, value: IB) -> Self::Result1;
        #[allow(clippy::type_complexity)]
        fn apply(self, value: (IA, IB)) -> (Self::Result0, Self::Result1);
    }
    impl<IA, IB, OA, OB, FA, FB> Map2<IA, IB> for (FA, FB)
    where
        FA: FnMut(IA) -> OA,
        FB: FnMut(IB) -> OB,
    {
        type Result0 = OA;
        type Result1 = OB;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn map_1(mut self, value: IB) -> Self::Result1 {
            (self.1)(value)
        }
        fn apply(mut self, value: (IA, IB)) -> (Self::Result0, Self::Result1) {
            ((self.0)(value.0), (self.1)(value.1))
        }
    }
    pub trait Map3<IA, IB, IC> {
        type Result0;
        type Result1;
        type Result2;
        fn map_0(self, value: IA) -> Self::Result0;
        fn map_1(self, value: IB) -> Self::Result1;
        fn map_2(self, value: IC) -> Self::Result2;
        #[allow(clippy::type_complexity)]
        fn apply(
            self,
            value: (IA, IB, IC),
        ) -> (Self::Result0, Self::Result1, Self::Result2);
    }
    impl<IA, IB, IC, OA, OB, OC, FA, FB, FC> Map3<IA, IB, IC> for (FA, FB, FC)
    where
        FA: FnMut(IA) -> OA,
        FB: FnMut(IB) -> OB,
        FC: FnMut(IC) -> OC,
    {
        type Result0 = OA;
        type Result1 = OB;
        type Result2 = OC;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn map_1(mut self, value: IB) -> Self::Result1 {
            (self.1)(value)
        }
        fn map_2(mut self, value: IC) -> Self::Result2 {
            (self.2)(value)
        }
        fn apply(
            mut self,
            value: (IA, IB, IC),
        ) -> (Self::Result0, Self::Result1, Self::Result2) {
            ((self.0)(value.0), (self.1)(value.1), (self.2)(value.2))
        }
    }
    pub trait Map4<IA, IB, IC, ID> {
        type Result0;
        type Result1;
        type Result2;
        type Result3;
        fn map_0(self, value: IA) -> Self::Result0;
        fn map_1(self, value: IB) -> Self::Result1;
        fn map_2(self, value: IC) -> Self::Result2;
        fn map_3(self, value: ID) -> Self::Result3;
        #[allow(clippy::type_complexity)]
        fn apply(
            self,
            value: (IA, IB, IC, ID),
        ) -> (Self::Result0, Self::Result1, Self::Result2, Self::Result3);
    }
    impl<IA, IB, IC, ID, OA, OB, OC, OD, FA, FB, FC, FD> Map4<IA, IB, IC, ID>
    for (FA, FB, FC, FD)
    where
        FA: FnMut(IA) -> OA,
        FB: FnMut(IB) -> OB,
        FC: FnMut(IC) -> OC,
        FD: FnMut(ID) -> OD,
    {
        type Result0 = OA;
        type Result1 = OB;
        type Result2 = OC;
        type Result3 = OD;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn map_1(mut self, value: IB) -> Self::Result1 {
            (self.1)(value)
        }
        fn map_2(mut self, value: IC) -> Self::Result2 {
            (self.2)(value)
        }
        fn map_3(mut self, value: ID) -> Self::Result3 {
            (self.3)(value)
        }
        fn apply(
            mut self,
            value: (IA, IB, IC, ID),
        ) -> (Self::Result0, Self::Result1, Self::Result2, Self::Result3) {
            ((self.0)(value.0), (self.1)(value.1), (self.2)(value.2), (self.3)(value.3))
        }
    }
    pub trait Map5<IA, IB, IC, ID, IE> {
        type Result0;
        type Result1;
        type Result2;
        type Result3;
        type Result4;
        fn map_0(self, value: IA) -> Self::Result0;
        fn map_1(self, value: IB) -> Self::Result1;
        fn map_2(self, value: IC) -> Self::Result2;
        fn map_3(self, value: ID) -> Self::Result3;
        fn map_4(self, value: IE) -> Self::Result4;
        #[allow(clippy::type_complexity)]
        fn apply(
            self,
            value: (IA, IB, IC, ID, IE),
        ) -> (Self::Result0, Self::Result1, Self::Result2, Self::Result3, Self::Result4);
    }
    impl<
        IA,
        IB,
        IC,
        ID,
        IE,
        OA,
        OB,
        OC,
        OD,
        OE,
        FA,
        FB,
        FC,
        FD,
        FE,
    > Map5<IA, IB, IC, ID, IE> for (FA, FB, FC, FD, FE)
    where
        FA: FnMut(IA) -> OA,
        FB: FnMut(IB) -> OB,
        FC: FnMut(IC) -> OC,
        FD: FnMut(ID) -> OD,
        FE: FnMut(IE) -> OE,
    {
        type Result0 = OA;
        type Result1 = OB;
        type Result2 = OC;
        type Result3 = OD;
        type Result4 = OE;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn map_1(mut self, value: IB) -> Self::Result1 {
            (self.1)(value)
        }
        fn map_2(mut self, value: IC) -> Self::Result2 {
            (self.2)(value)
        }
        fn map_3(mut self, value: ID) -> Self::Result3 {
            (self.3)(value)
        }
        fn map_4(mut self, value: IE) -> Self::Result4 {
            (self.4)(value)
        }
        fn apply(
            mut self,
            value: (IA, IB, IC, ID, IE),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
        ) {
            (
                (self.0)(value.0),
                (self.1)(value.1),
                (self.2)(value.2),
                (self.3)(value.3),
                (self.4)(value.4),
            )
        }
    }
    pub trait Map6<IA, IB, IC, ID, IE, IF> {
        type Result0;
        type Result1;
        type Result2;
        type Result3;
        type Result4;
        type Result5;
        fn map_0(self, value: IA) -> Self::Result0;
        fn map_1(self, value: IB) -> Self::Result1;
        fn map_2(self, value: IC) -> Self::Result2;
        fn map_3(self, value: ID) -> Self::Result3;
        fn map_4(self, value: IE) -> Self::Result4;
        fn map_5(self, value: IF) -> Self::Result5;
        #[allow(clippy::type_complexity)]
        fn apply(
            self,
            value: (IA, IB, IC, ID, IE, IF),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
        );
    }
    impl<
        IA,
        IB,
        IC,
        ID,
        IE,
        IF,
        OA,
        OB,
        OC,
        OD,
        OE,
        OF,
        FA,
        FB,
        FC,
        FD,
        FE,
        FF,
    > Map6<IA, IB, IC, ID, IE, IF> for (FA, FB, FC, FD, FE, FF)
    where
        FA: FnMut(IA) -> OA,
        FB: FnMut(IB) -> OB,
        FC: FnMut(IC) -> OC,
        FD: FnMut(ID) -> OD,
        FE: FnMut(IE) -> OE,
        FF: FnMut(IF) -> OF,
    {
        type Result0 = OA;
        type Result1 = OB;
        type Result2 = OC;
        type Result3 = OD;
        type Result4 = OE;
        type Result5 = OF;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn map_1(mut self, value: IB) -> Self::Result1 {
            (self.1)(value)
        }
        fn map_2(mut self, value: IC) -> Self::Result2 {
            (self.2)(value)
        }
        fn map_3(mut self, value: ID) -> Self::Result3 {
            (self.3)(value)
        }
        fn map_4(mut self, value: IE) -> Self::Result4 {
            (self.4)(value)
        }
        fn map_5(mut self, value: IF) -> Self::Result5 {
            (self.5)(value)
        }
        fn apply(
            mut self,
            value: (IA, IB, IC, ID, IE, IF),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
        ) {
            (
                (self.0)(value.0),
                (self.1)(value.1),
                (self.2)(value.2),
                (self.3)(value.3),
                (self.4)(value.4),
                (self.5)(value.5),
            )
        }
    }
    pub trait Map7<IA, IB, IC, ID, IE, IF, IG> {
        type Result0;
        type Result1;
        type Result2;
        type Result3;
        type Result4;
        type Result5;
        type Result6;
        fn map_0(self, value: IA) -> Self::Result0;
        fn map_1(self, value: IB) -> Self::Result1;
        fn map_2(self, value: IC) -> Self::Result2;
        fn map_3(self, value: ID) -> Self::Result3;
        fn map_4(self, value: IE) -> Self::Result4;
        fn map_5(self, value: IF) -> Self::Result5;
        fn map_6(self, value: IG) -> Self::Result6;
        #[allow(clippy::type_complexity)]
        fn apply(
            self,
            value: (IA, IB, IC, ID, IE, IF, IG),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
        );
    }
    impl<
        IA,
        IB,
        IC,
        ID,
        IE,
        IF,
        IG,
        OA,
        OB,
        OC,
        OD,
        OE,
        OF,
        OG,
        FA,
        FB,
        FC,
        FD,
        FE,
        FF,
        FG,
    > Map7<IA, IB, IC, ID, IE, IF, IG> for (FA, FB, FC, FD, FE, FF, FG)
    where
        FA: FnMut(IA) -> OA,
        FB: FnMut(IB) -> OB,
        FC: FnMut(IC) -> OC,
        FD: FnMut(ID) -> OD,
        FE: FnMut(IE) -> OE,
        FF: FnMut(IF) -> OF,
        FG: FnMut(IG) -> OG,
    {
        type Result0 = OA;
        type Result1 = OB;
        type Result2 = OC;
        type Result3 = OD;
        type Result4 = OE;
        type Result5 = OF;
        type Result6 = OG;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn map_1(mut self, value: IB) -> Self::Result1 {
            (self.1)(value)
        }
        fn map_2(mut self, value: IC) -> Self::Result2 {
            (self.2)(value)
        }
        fn map_3(mut self, value: ID) -> Self::Result3 {
            (self.3)(value)
        }
        fn map_4(mut self, value: IE) -> Self::Result4 {
            (self.4)(value)
        }
        fn map_5(mut self, value: IF) -> Self::Result5 {
            (self.5)(value)
        }
        fn map_6(mut self, value: IG) -> Self::Result6 {
            (self.6)(value)
        }
        fn apply(
            mut self,
            value: (IA, IB, IC, ID, IE, IF, IG),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
        ) {
            (
                (self.0)(value.0),
                (self.1)(value.1),
                (self.2)(value.2),
                (self.3)(value.3),
                (self.4)(value.4),
                (self.5)(value.5),
                (self.6)(value.6),
            )
        }
    }
    pub trait Map8<IA, IB, IC, ID, IE, IF, IG, IH> {
        type Result0;
        type Result1;
        type Result2;
        type Result3;
        type Result4;
        type Result5;
        type Result6;
        type Result7;
        fn map_0(self, value: IA) -> Self::Result0;
        fn map_1(self, value: IB) -> Self::Result1;
        fn map_2(self, value: IC) -> Self::Result2;
        fn map_3(self, value: ID) -> Self::Result3;
        fn map_4(self, value: IE) -> Self::Result4;
        fn map_5(self, value: IF) -> Self::Result5;
        fn map_6(self, value: IG) -> Self::Result6;
        fn map_7(self, value: IH) -> Self::Result7;
        #[allow(clippy::type_complexity)]
        fn apply(
            self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
        );
    }
    impl<
        IA,
        IB,
        IC,
        ID,
        IE,
        IF,
        IG,
        IH,
        OA,
        OB,
        OC,
        OD,
        OE,
        OF,
        OG,
        OH,
        FA,
        FB,
        FC,
        FD,
        FE,
        FF,
        FG,
        FH,
    > Map8<IA, IB, IC, ID, IE, IF, IG, IH> for (FA, FB, FC, FD, FE, FF, FG, FH)
    where
        FA: FnMut(IA) -> OA,
        FB: FnMut(IB) -> OB,
        FC: FnMut(IC) -> OC,
        FD: FnMut(ID) -> OD,
        FE: FnMut(IE) -> OE,
        FF: FnMut(IF) -> OF,
        FG: FnMut(IG) -> OG,
        FH: FnMut(IH) -> OH,
    {
        type Result0 = OA;
        type Result1 = OB;
        type Result2 = OC;
        type Result3 = OD;
        type Result4 = OE;
        type Result5 = OF;
        type Result6 = OG;
        type Result7 = OH;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn map_1(mut self, value: IB) -> Self::Result1 {
            (self.1)(value)
        }
        fn map_2(mut self, value: IC) -> Self::Result2 {
            (self.2)(value)
        }
        fn map_3(mut self, value: ID) -> Self::Result3 {
            (self.3)(value)
        }
        fn map_4(mut self, value: IE) -> Self::Result4 {
            (self.4)(value)
        }
        fn map_5(mut self, value: IF) -> Self::Result5 {
            (self.5)(value)
        }
        fn map_6(mut self, value: IG) -> Self::Result6 {
            (self.6)(value)
        }
        fn map_7(mut self, value: IH) -> Self::Result7 {
            (self.7)(value)
        }
        fn apply(
            mut self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
        ) {
            (
                (self.0)(value.0),
                (self.1)(value.1),
                (self.2)(value.2),
                (self.3)(value.3),
                (self.4)(value.4),
                (self.5)(value.5),
                (self.6)(value.6),
                (self.7)(value.7),
            )
        }
    }
    pub trait Map9<IA, IB, IC, ID, IE, IF, IG, IH, II> {
        type Result0;
        type Result1;
        type Result2;
        type Result3;
        type Result4;
        type Result5;
        type Result6;
        type Result7;
        type Result8;
        fn map_0(self, value: IA) -> Self::Result0;
        fn map_1(self, value: IB) -> Self::Result1;
        fn map_2(self, value: IC) -> Self::Result2;
        fn map_3(self, value: ID) -> Self::Result3;
        fn map_4(self, value: IE) -> Self::Result4;
        fn map_5(self, value: IF) -> Self::Result5;
        fn map_6(self, value: IG) -> Self::Result6;
        fn map_7(self, value: IH) -> Self::Result7;
        fn map_8(self, value: II) -> Self::Result8;
        #[allow(clippy::type_complexity)]
        fn apply(
            self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
        );
    }
    impl<
        IA,
        IB,
        IC,
        ID,
        IE,
        IF,
        IG,
        IH,
        II,
        OA,
        OB,
        OC,
        OD,
        OE,
        OF,
        OG,
        OH,
        OI,
        FA,
        FB,
        FC,
        FD,
        FE,
        FF,
        FG,
        FH,
        FI,
    > Map9<IA, IB, IC, ID, IE, IF, IG, IH, II> for (FA, FB, FC, FD, FE, FF, FG, FH, FI)
    where
        FA: FnMut(IA) -> OA,
        FB: FnMut(IB) -> OB,
        FC: FnMut(IC) -> OC,
        FD: FnMut(ID) -> OD,
        FE: FnMut(IE) -> OE,
        FF: FnMut(IF) -> OF,
        FG: FnMut(IG) -> OG,
        FH: FnMut(IH) -> OH,
        FI: FnMut(II) -> OI,
    {
        type Result0 = OA;
        type Result1 = OB;
        type Result2 = OC;
        type Result3 = OD;
        type Result4 = OE;
        type Result5 = OF;
        type Result6 = OG;
        type Result7 = OH;
        type Result8 = OI;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn map_1(mut self, value: IB) -> Self::Result1 {
            (self.1)(value)
        }
        fn map_2(mut self, value: IC) -> Self::Result2 {
            (self.2)(value)
        }
        fn map_3(mut self, value: ID) -> Self::Result3 {
            (self.3)(value)
        }
        fn map_4(mut self, value: IE) -> Self::Result4 {
            (self.4)(value)
        }
        fn map_5(mut self, value: IF) -> Self::Result5 {
            (self.5)(value)
        }
        fn map_6(mut self, value: IG) -> Self::Result6 {
            (self.6)(value)
        }
        fn map_7(mut self, value: IH) -> Self::Result7 {
            (self.7)(value)
        }
        fn map_8(mut self, value: II) -> Self::Result8 {
            (self.8)(value)
        }
        fn apply(
            mut self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
        ) {
            (
                (self.0)(value.0),
                (self.1)(value.1),
                (self.2)(value.2),
                (self.3)(value.3),
                (self.4)(value.4),
                (self.5)(value.5),
                (self.6)(value.6),
                (self.7)(value.7),
                (self.8)(value.8),
            )
        }
    }
    pub trait Map10<IA, IB, IC, ID, IE, IF, IG, IH, II, IJ> {
        type Result0;
        type Result1;
        type Result2;
        type Result3;
        type Result4;
        type Result5;
        type Result6;
        type Result7;
        type Result8;
        type Result9;
        fn map_0(self, value: IA) -> Self::Result0;
        fn map_1(self, value: IB) -> Self::Result1;
        fn map_2(self, value: IC) -> Self::Result2;
        fn map_3(self, value: ID) -> Self::Result3;
        fn map_4(self, value: IE) -> Self::Result4;
        fn map_5(self, value: IF) -> Self::Result5;
        fn map_6(self, value: IG) -> Self::Result6;
        fn map_7(self, value: IH) -> Self::Result7;
        fn map_8(self, value: II) -> Self::Result8;
        fn map_9(self, value: IJ) -> Self::Result9;
        #[allow(clippy::type_complexity)]
        fn apply(
            self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II, IJ),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
            Self::Result9,
        );
    }
    impl<
        IA,
        IB,
        IC,
        ID,
        IE,
        IF,
        IG,
        IH,
        II,
        IJ,
        OA,
        OB,
        OC,
        OD,
        OE,
        OF,
        OG,
        OH,
        OI,
        OJ,
        FA,
        FB,
        FC,
        FD,
        FE,
        FF,
        FG,
        FH,
        FI,
        FJ,
    > Map10<IA, IB, IC, ID, IE, IF, IG, IH, II, IJ>
    for (FA, FB, FC, FD, FE, FF, FG, FH, FI, FJ)
    where
        FA: FnMut(IA) -> OA,
        FB: FnMut(IB) -> OB,
        FC: FnMut(IC) -> OC,
        FD: FnMut(ID) -> OD,
        FE: FnMut(IE) -> OE,
        FF: FnMut(IF) -> OF,
        FG: FnMut(IG) -> OG,
        FH: FnMut(IH) -> OH,
        FI: FnMut(II) -> OI,
        FJ: FnMut(IJ) -> OJ,
    {
        type Result0 = OA;
        type Result1 = OB;
        type Result2 = OC;
        type Result3 = OD;
        type Result4 = OE;
        type Result5 = OF;
        type Result6 = OG;
        type Result7 = OH;
        type Result8 = OI;
        type Result9 = OJ;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn map_1(mut self, value: IB) -> Self::Result1 {
            (self.1)(value)
        }
        fn map_2(mut self, value: IC) -> Self::Result2 {
            (self.2)(value)
        }
        fn map_3(mut self, value: ID) -> Self::Result3 {
            (self.3)(value)
        }
        fn map_4(mut self, value: IE) -> Self::Result4 {
            (self.4)(value)
        }
        fn map_5(mut self, value: IF) -> Self::Result5 {
            (self.5)(value)
        }
        fn map_6(mut self, value: IG) -> Self::Result6 {
            (self.6)(value)
        }
        fn map_7(mut self, value: IH) -> Self::Result7 {
            (self.7)(value)
        }
        fn map_8(mut self, value: II) -> Self::Result8 {
            (self.8)(value)
        }
        fn map_9(mut self, value: IJ) -> Self::Result9 {
            (self.9)(value)
        }
        fn apply(
            mut self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II, IJ),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
            Self::Result9,
        ) {
            (
                (self.0)(value.0),
                (self.1)(value.1),
                (self.2)(value.2),
                (self.3)(value.3),
                (self.4)(value.4),
                (self.5)(value.5),
                (self.6)(value.6),
                (self.7)(value.7),
                (self.8)(value.8),
                (self.9)(value.9),
            )
        }
    }
    pub trait Map11<IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK> {
        type Result0;
        type Result1;
        type Result2;
        type Result3;
        type Result4;
        type Result5;
        type Result6;
        type Result7;
        type Result8;
        type Result9;
        type Result10;
        fn map_0(self, value: IA) -> Self::Result0;
        fn map_1(self, value: IB) -> Self::Result1;
        fn map_2(self, value: IC) -> Self::Result2;
        fn map_3(self, value: ID) -> Self::Result3;
        fn map_4(self, value: IE) -> Self::Result4;
        fn map_5(self, value: IF) -> Self::Result5;
        fn map_6(self, value: IG) -> Self::Result6;
        fn map_7(self, value: IH) -> Self::Result7;
        fn map_8(self, value: II) -> Self::Result8;
        fn map_9(self, value: IJ) -> Self::Result9;
        fn map_10(self, value: IK) -> Self::Result10;
        #[allow(clippy::type_complexity)]
        fn apply(
            self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
            Self::Result9,
            Self::Result10,
        );
    }
    impl<
        IA,
        IB,
        IC,
        ID,
        IE,
        IF,
        IG,
        IH,
        II,
        IJ,
        IK,
        OA,
        OB,
        OC,
        OD,
        OE,
        OF,
        OG,
        OH,
        OI,
        OJ,
        OK,
        FA,
        FB,
        FC,
        FD,
        FE,
        FF,
        FG,
        FH,
        FI,
        FJ,
        FK,
    > Map11<IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK>
    for (FA, FB, FC, FD, FE, FF, FG, FH, FI, FJ, FK)
    where
        FA: FnMut(IA) -> OA,
        FB: FnMut(IB) -> OB,
        FC: FnMut(IC) -> OC,
        FD: FnMut(ID) -> OD,
        FE: FnMut(IE) -> OE,
        FF: FnMut(IF) -> OF,
        FG: FnMut(IG) -> OG,
        FH: FnMut(IH) -> OH,
        FI: FnMut(II) -> OI,
        FJ: FnMut(IJ) -> OJ,
        FK: FnMut(IK) -> OK,
    {
        type Result0 = OA;
        type Result1 = OB;
        type Result2 = OC;
        type Result3 = OD;
        type Result4 = OE;
        type Result5 = OF;
        type Result6 = OG;
        type Result7 = OH;
        type Result8 = OI;
        type Result9 = OJ;
        type Result10 = OK;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn map_1(mut self, value: IB) -> Self::Result1 {
            (self.1)(value)
        }
        fn map_2(mut self, value: IC) -> Self::Result2 {
            (self.2)(value)
        }
        fn map_3(mut self, value: ID) -> Self::Result3 {
            (self.3)(value)
        }
        fn map_4(mut self, value: IE) -> Self::Result4 {
            (self.4)(value)
        }
        fn map_5(mut self, value: IF) -> Self::Result5 {
            (self.5)(value)
        }
        fn map_6(mut self, value: IG) -> Self::Result6 {
            (self.6)(value)
        }
        fn map_7(mut self, value: IH) -> Self::Result7 {
            (self.7)(value)
        }
        fn map_8(mut self, value: II) -> Self::Result8 {
            (self.8)(value)
        }
        fn map_9(mut self, value: IJ) -> Self::Result9 {
            (self.9)(value)
        }
        fn map_10(mut self, value: IK) -> Self::Result10 {
            (self.10)(value)
        }
        fn apply(
            mut self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
            Self::Result9,
            Self::Result10,
        ) {
            (
                (self.0)(value.0),
                (self.1)(value.1),
                (self.2)(value.2),
                (self.3)(value.3),
                (self.4)(value.4),
                (self.5)(value.5),
                (self.6)(value.6),
                (self.7)(value.7),
                (self.8)(value.8),
                (self.9)(value.9),
                (self.10)(value.10),
            )
        }
    }
    pub trait Map12<IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL> {
        type Result0;
        type Result1;
        type Result2;
        type Result3;
        type Result4;
        type Result5;
        type Result6;
        type Result7;
        type Result8;
        type Result9;
        type Result10;
        type Result11;
        fn map_0(self, value: IA) -> Self::Result0;
        fn map_1(self, value: IB) -> Self::Result1;
        fn map_2(self, value: IC) -> Self::Result2;
        fn map_3(self, value: ID) -> Self::Result3;
        fn map_4(self, value: IE) -> Self::Result4;
        fn map_5(self, value: IF) -> Self::Result5;
        fn map_6(self, value: IG) -> Self::Result6;
        fn map_7(self, value: IH) -> Self::Result7;
        fn map_8(self, value: II) -> Self::Result8;
        fn map_9(self, value: IJ) -> Self::Result9;
        fn map_10(self, value: IK) -> Self::Result10;
        fn map_11(self, value: IL) -> Self::Result11;
        #[allow(clippy::type_complexity)]
        fn apply(
            self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
            Self::Result9,
            Self::Result10,
            Self::Result11,
        );
    }
    impl<
        IA,
        IB,
        IC,
        ID,
        IE,
        IF,
        IG,
        IH,
        II,
        IJ,
        IK,
        IL,
        OA,
        OB,
        OC,
        OD,
        OE,
        OF,
        OG,
        OH,
        OI,
        OJ,
        OK,
        OL,
        FA,
        FB,
        FC,
        FD,
        FE,
        FF,
        FG,
        FH,
        FI,
        FJ,
        FK,
        FL,
    > Map12<IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL>
    for (FA, FB, FC, FD, FE, FF, FG, FH, FI, FJ, FK, FL)
    where
        FA: FnMut(IA) -> OA,
        FB: FnMut(IB) -> OB,
        FC: FnMut(IC) -> OC,
        FD: FnMut(ID) -> OD,
        FE: FnMut(IE) -> OE,
        FF: FnMut(IF) -> OF,
        FG: FnMut(IG) -> OG,
        FH: FnMut(IH) -> OH,
        FI: FnMut(II) -> OI,
        FJ: FnMut(IJ) -> OJ,
        FK: FnMut(IK) -> OK,
        FL: FnMut(IL) -> OL,
    {
        type Result0 = OA;
        type Result1 = OB;
        type Result2 = OC;
        type Result3 = OD;
        type Result4 = OE;
        type Result5 = OF;
        type Result6 = OG;
        type Result7 = OH;
        type Result8 = OI;
        type Result9 = OJ;
        type Result10 = OK;
        type Result11 = OL;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn map_1(mut self, value: IB) -> Self::Result1 {
            (self.1)(value)
        }
        fn map_2(mut self, value: IC) -> Self::Result2 {
            (self.2)(value)
        }
        fn map_3(mut self, value: ID) -> Self::Result3 {
            (self.3)(value)
        }
        fn map_4(mut self, value: IE) -> Self::Result4 {
            (self.4)(value)
        }
        fn map_5(mut self, value: IF) -> Self::Result5 {
            (self.5)(value)
        }
        fn map_6(mut self, value: IG) -> Self::Result6 {
            (self.6)(value)
        }
        fn map_7(mut self, value: IH) -> Self::Result7 {
            (self.7)(value)
        }
        fn map_8(mut self, value: II) -> Self::Result8 {
            (self.8)(value)
        }
        fn map_9(mut self, value: IJ) -> Self::Result9 {
            (self.9)(value)
        }
        fn map_10(mut self, value: IK) -> Self::Result10 {
            (self.10)(value)
        }
        fn map_11(mut self, value: IL) -> Self::Result11 {
            (self.11)(value)
        }
        fn apply(
            mut self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
            Self::Result9,
            Self::Result10,
            Self::Result11,
        ) {
            (
                (self.0)(value.0),
                (self.1)(value.1),
                (self.2)(value.2),
                (self.3)(value.3),
                (self.4)(value.4),
                (self.5)(value.5),
                (self.6)(value.6),
                (self.7)(value.7),
                (self.8)(value.8),
                (self.9)(value.9),
                (self.10)(value.10),
                (self.11)(value.11),
            )
        }
    }
    pub trait Map13<IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM> {
        type Result0;
        type Result1;
        type Result2;
        type Result3;
        type Result4;
        type Result5;
        type Result6;
        type Result7;
        type Result8;
        type Result9;
        type Result10;
        type Result11;
        type Result12;
        fn map_0(self, value: IA) -> Self::Result0;
        fn map_1(self, value: IB) -> Self::Result1;
        fn map_2(self, value: IC) -> Self::Result2;
        fn map_3(self, value: ID) -> Self::Result3;
        fn map_4(self, value: IE) -> Self::Result4;
        fn map_5(self, value: IF) -> Self::Result5;
        fn map_6(self, value: IG) -> Self::Result6;
        fn map_7(self, value: IH) -> Self::Result7;
        fn map_8(self, value: II) -> Self::Result8;
        fn map_9(self, value: IJ) -> Self::Result9;
        fn map_10(self, value: IK) -> Self::Result10;
        fn map_11(self, value: IL) -> Self::Result11;
        fn map_12(self, value: IM) -> Self::Result12;
        #[allow(clippy::type_complexity)]
        fn apply(
            self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
            Self::Result9,
            Self::Result10,
            Self::Result11,
            Self::Result12,
        );
    }
    impl<
        IA,
        IB,
        IC,
        ID,
        IE,
        IF,
        IG,
        IH,
        II,
        IJ,
        IK,
        IL,
        IM,
        OA,
        OB,
        OC,
        OD,
        OE,
        OF,
        OG,
        OH,
        OI,
        OJ,
        OK,
        OL,
        OM,
        FA,
        FB,
        FC,
        FD,
        FE,
        FF,
        FG,
        FH,
        FI,
        FJ,
        FK,
        FL,
        FM,
    > Map13<IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM>
    for (FA, FB, FC, FD, FE, FF, FG, FH, FI, FJ, FK, FL, FM)
    where
        FA: FnMut(IA) -> OA,
        FB: FnMut(IB) -> OB,
        FC: FnMut(IC) -> OC,
        FD: FnMut(ID) -> OD,
        FE: FnMut(IE) -> OE,
        FF: FnMut(IF) -> OF,
        FG: FnMut(IG) -> OG,
        FH: FnMut(IH) -> OH,
        FI: FnMut(II) -> OI,
        FJ: FnMut(IJ) -> OJ,
        FK: FnMut(IK) -> OK,
        FL: FnMut(IL) -> OL,
        FM: FnMut(IM) -> OM,
    {
        type Result0 = OA;
        type Result1 = OB;
        type Result2 = OC;
        type Result3 = OD;
        type Result4 = OE;
        type Result5 = OF;
        type Result6 = OG;
        type Result7 = OH;
        type Result8 = OI;
        type Result9 = OJ;
        type Result10 = OK;
        type Result11 = OL;
        type Result12 = OM;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn map_1(mut self, value: IB) -> Self::Result1 {
            (self.1)(value)
        }
        fn map_2(mut self, value: IC) -> Self::Result2 {
            (self.2)(value)
        }
        fn map_3(mut self, value: ID) -> Self::Result3 {
            (self.3)(value)
        }
        fn map_4(mut self, value: IE) -> Self::Result4 {
            (self.4)(value)
        }
        fn map_5(mut self, value: IF) -> Self::Result5 {
            (self.5)(value)
        }
        fn map_6(mut self, value: IG) -> Self::Result6 {
            (self.6)(value)
        }
        fn map_7(mut self, value: IH) -> Self::Result7 {
            (self.7)(value)
        }
        fn map_8(mut self, value: II) -> Self::Result8 {
            (self.8)(value)
        }
        fn map_9(mut self, value: IJ) -> Self::Result9 {
            (self.9)(value)
        }
        fn map_10(mut self, value: IK) -> Self::Result10 {
            (self.10)(value)
        }
        fn map_11(mut self, value: IL) -> Self::Result11 {
            (self.11)(value)
        }
        fn map_12(mut self, value: IM) -> Self::Result12 {
            (self.12)(value)
        }
        fn apply(
            mut self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
            Self::Result9,
            Self::Result10,
            Self::Result11,
            Self::Result12,
        ) {
            (
                (self.0)(value.0),
                (self.1)(value.1),
                (self.2)(value.2),
                (self.3)(value.3),
                (self.4)(value.4),
                (self.5)(value.5),
                (self.6)(value.6),
                (self.7)(value.7),
                (self.8)(value.8),
                (self.9)(value.9),
                (self.10)(value.10),
                (self.11)(value.11),
                (self.12)(value.12),
            )
        }
    }
    pub trait Map14<IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM, IN> {
        type Result0;
        type Result1;
        type Result2;
        type Result3;
        type Result4;
        type Result5;
        type Result6;
        type Result7;
        type Result8;
        type Result9;
        type Result10;
        type Result11;
        type Result12;
        type Result13;
        fn map_0(self, value: IA) -> Self::Result0;
        fn map_1(self, value: IB) -> Self::Result1;
        fn map_2(self, value: IC) -> Self::Result2;
        fn map_3(self, value: ID) -> Self::Result3;
        fn map_4(self, value: IE) -> Self::Result4;
        fn map_5(self, value: IF) -> Self::Result5;
        fn map_6(self, value: IG) -> Self::Result6;
        fn map_7(self, value: IH) -> Self::Result7;
        fn map_8(self, value: II) -> Self::Result8;
        fn map_9(self, value: IJ) -> Self::Result9;
        fn map_10(self, value: IK) -> Self::Result10;
        fn map_11(self, value: IL) -> Self::Result11;
        fn map_12(self, value: IM) -> Self::Result12;
        fn map_13(self, value: IN) -> Self::Result13;
        #[allow(clippy::type_complexity)]
        fn apply(
            self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM, IN),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
            Self::Result9,
            Self::Result10,
            Self::Result11,
            Self::Result12,
            Self::Result13,
        );
    }
    impl<
        IA,
        IB,
        IC,
        ID,
        IE,
        IF,
        IG,
        IH,
        II,
        IJ,
        IK,
        IL,
        IM,
        IN,
        OA,
        OB,
        OC,
        OD,
        OE,
        OF,
        OG,
        OH,
        OI,
        OJ,
        OK,
        OL,
        OM,
        ON,
        FA,
        FB,
        FC,
        FD,
        FE,
        FF,
        FG,
        FH,
        FI,
        FJ,
        FK,
        FL,
        FM,
        FN,
    > Map14<IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM, IN>
    for (FA, FB, FC, FD, FE, FF, FG, FH, FI, FJ, FK, FL, FM, FN)
    where
        FA: FnMut(IA) -> OA,
        FB: FnMut(IB) -> OB,
        FC: FnMut(IC) -> OC,
        FD: FnMut(ID) -> OD,
        FE: FnMut(IE) -> OE,
        FF: FnMut(IF) -> OF,
        FG: FnMut(IG) -> OG,
        FH: FnMut(IH) -> OH,
        FI: FnMut(II) -> OI,
        FJ: FnMut(IJ) -> OJ,
        FK: FnMut(IK) -> OK,
        FL: FnMut(IL) -> OL,
        FM: FnMut(IM) -> OM,
        FN: FnMut(IN) -> ON,
    {
        type Result0 = OA;
        type Result1 = OB;
        type Result2 = OC;
        type Result3 = OD;
        type Result4 = OE;
        type Result5 = OF;
        type Result6 = OG;
        type Result7 = OH;
        type Result8 = OI;
        type Result9 = OJ;
        type Result10 = OK;
        type Result11 = OL;
        type Result12 = OM;
        type Result13 = ON;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn map_1(mut self, value: IB) -> Self::Result1 {
            (self.1)(value)
        }
        fn map_2(mut self, value: IC) -> Self::Result2 {
            (self.2)(value)
        }
        fn map_3(mut self, value: ID) -> Self::Result3 {
            (self.3)(value)
        }
        fn map_4(mut self, value: IE) -> Self::Result4 {
            (self.4)(value)
        }
        fn map_5(mut self, value: IF) -> Self::Result5 {
            (self.5)(value)
        }
        fn map_6(mut self, value: IG) -> Self::Result6 {
            (self.6)(value)
        }
        fn map_7(mut self, value: IH) -> Self::Result7 {
            (self.7)(value)
        }
        fn map_8(mut self, value: II) -> Self::Result8 {
            (self.8)(value)
        }
        fn map_9(mut self, value: IJ) -> Self::Result9 {
            (self.9)(value)
        }
        fn map_10(mut self, value: IK) -> Self::Result10 {
            (self.10)(value)
        }
        fn map_11(mut self, value: IL) -> Self::Result11 {
            (self.11)(value)
        }
        fn map_12(mut self, value: IM) -> Self::Result12 {
            (self.12)(value)
        }
        fn map_13(mut self, value: IN) -> Self::Result13 {
            (self.13)(value)
        }
        fn apply(
            mut self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM, IN),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
            Self::Result9,
            Self::Result10,
            Self::Result11,
            Self::Result12,
            Self::Result13,
        ) {
            (
                (self.0)(value.0),
                (self.1)(value.1),
                (self.2)(value.2),
                (self.3)(value.3),
                (self.4)(value.4),
                (self.5)(value.5),
                (self.6)(value.6),
                (self.7)(value.7),
                (self.8)(value.8),
                (self.9)(value.9),
                (self.10)(value.10),
                (self.11)(value.11),
                (self.12)(value.12),
                (self.13)(value.13),
            )
        }
    }
    pub trait Map15<IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM, IN, IO> {
        type Result0;
        type Result1;
        type Result2;
        type Result3;
        type Result4;
        type Result5;
        type Result6;
        type Result7;
        type Result8;
        type Result9;
        type Result10;
        type Result11;
        type Result12;
        type Result13;
        type Result14;
        fn map_0(self, value: IA) -> Self::Result0;
        fn map_1(self, value: IB) -> Self::Result1;
        fn map_2(self, value: IC) -> Self::Result2;
        fn map_3(self, value: ID) -> Self::Result3;
        fn map_4(self, value: IE) -> Self::Result4;
        fn map_5(self, value: IF) -> Self::Result5;
        fn map_6(self, value: IG) -> Self::Result6;
        fn map_7(self, value: IH) -> Self::Result7;
        fn map_8(self, value: II) -> Self::Result8;
        fn map_9(self, value: IJ) -> Self::Result9;
        fn map_10(self, value: IK) -> Self::Result10;
        fn map_11(self, value: IL) -> Self::Result11;
        fn map_12(self, value: IM) -> Self::Result12;
        fn map_13(self, value: IN) -> Self::Result13;
        fn map_14(self, value: IO) -> Self::Result14;
        #[allow(clippy::type_complexity)]
        fn apply(
            self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM, IN, IO),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
            Self::Result9,
            Self::Result10,
            Self::Result11,
            Self::Result12,
            Self::Result13,
            Self::Result14,
        );
    }
    impl<
        IA,
        IB,
        IC,
        ID,
        IE,
        IF,
        IG,
        IH,
        II,
        IJ,
        IK,
        IL,
        IM,
        IN,
        IO,
        OA,
        OB,
        OC,
        OD,
        OE,
        OF,
        OG,
        OH,
        OI,
        OJ,
        OK,
        OL,
        OM,
        ON,
        OO,
        FA,
        FB,
        FC,
        FD,
        FE,
        FF,
        FG,
        FH,
        FI,
        FJ,
        FK,
        FL,
        FM,
        FN,
        FO,
    > Map15<IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM, IN, IO>
    for (FA, FB, FC, FD, FE, FF, FG, FH, FI, FJ, FK, FL, FM, FN, FO)
    where
        FA: FnMut(IA) -> OA,
        FB: FnMut(IB) -> OB,
        FC: FnMut(IC) -> OC,
        FD: FnMut(ID) -> OD,
        FE: FnMut(IE) -> OE,
        FF: FnMut(IF) -> OF,
        FG: FnMut(IG) -> OG,
        FH: FnMut(IH) -> OH,
        FI: FnMut(II) -> OI,
        FJ: FnMut(IJ) -> OJ,
        FK: FnMut(IK) -> OK,
        FL: FnMut(IL) -> OL,
        FM: FnMut(IM) -> OM,
        FN: FnMut(IN) -> ON,
        FO: FnMut(IO) -> OO,
    {
        type Result0 = OA;
        type Result1 = OB;
        type Result2 = OC;
        type Result3 = OD;
        type Result4 = OE;
        type Result5 = OF;
        type Result6 = OG;
        type Result7 = OH;
        type Result8 = OI;
        type Result9 = OJ;
        type Result10 = OK;
        type Result11 = OL;
        type Result12 = OM;
        type Result13 = ON;
        type Result14 = OO;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn map_1(mut self, value: IB) -> Self::Result1 {
            (self.1)(value)
        }
        fn map_2(mut self, value: IC) -> Self::Result2 {
            (self.2)(value)
        }
        fn map_3(mut self, value: ID) -> Self::Result3 {
            (self.3)(value)
        }
        fn map_4(mut self, value: IE) -> Self::Result4 {
            (self.4)(value)
        }
        fn map_5(mut self, value: IF) -> Self::Result5 {
            (self.5)(value)
        }
        fn map_6(mut self, value: IG) -> Self::Result6 {
            (self.6)(value)
        }
        fn map_7(mut self, value: IH) -> Self::Result7 {
            (self.7)(value)
        }
        fn map_8(mut self, value: II) -> Self::Result8 {
            (self.8)(value)
        }
        fn map_9(mut self, value: IJ) -> Self::Result9 {
            (self.9)(value)
        }
        fn map_10(mut self, value: IK) -> Self::Result10 {
            (self.10)(value)
        }
        fn map_11(mut self, value: IL) -> Self::Result11 {
            (self.11)(value)
        }
        fn map_12(mut self, value: IM) -> Self::Result12 {
            (self.12)(value)
        }
        fn map_13(mut self, value: IN) -> Self::Result13 {
            (self.13)(value)
        }
        fn map_14(mut self, value: IO) -> Self::Result14 {
            (self.14)(value)
        }
        fn apply(
            mut self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM, IN, IO),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
            Self::Result9,
            Self::Result10,
            Self::Result11,
            Self::Result12,
            Self::Result13,
            Self::Result14,
        ) {
            (
                (self.0)(value.0),
                (self.1)(value.1),
                (self.2)(value.2),
                (self.3)(value.3),
                (self.4)(value.4),
                (self.5)(value.5),
                (self.6)(value.6),
                (self.7)(value.7),
                (self.8)(value.8),
                (self.9)(value.9),
                (self.10)(value.10),
                (self.11)(value.11),
                (self.12)(value.12),
                (self.13)(value.13),
                (self.14)(value.14),
            )
        }
    }
    pub trait Map16<IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM, IN, IO, IP> {
        type Result0;
        type Result1;
        type Result2;
        type Result3;
        type Result4;
        type Result5;
        type Result6;
        type Result7;
        type Result8;
        type Result9;
        type Result10;
        type Result11;
        type Result12;
        type Result13;
        type Result14;
        type Result15;
        fn map_0(self, value: IA) -> Self::Result0;
        fn map_1(self, value: IB) -> Self::Result1;
        fn map_2(self, value: IC) -> Self::Result2;
        fn map_3(self, value: ID) -> Self::Result3;
        fn map_4(self, value: IE) -> Self::Result4;
        fn map_5(self, value: IF) -> Self::Result5;
        fn map_6(self, value: IG) -> Self::Result6;
        fn map_7(self, value: IH) -> Self::Result7;
        fn map_8(self, value: II) -> Self::Result8;
        fn map_9(self, value: IJ) -> Self::Result9;
        fn map_10(self, value: IK) -> Self::Result10;
        fn map_11(self, value: IL) -> Self::Result11;
        fn map_12(self, value: IM) -> Self::Result12;
        fn map_13(self, value: IN) -> Self::Result13;
        fn map_14(self, value: IO) -> Self::Result14;
        fn map_15(self, value: IP) -> Self::Result15;
        #[allow(clippy::type_complexity)]
        fn apply(
            self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM, IN, IO, IP),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
            Self::Result9,
            Self::Result10,
            Self::Result11,
            Self::Result12,
            Self::Result13,
            Self::Result14,
            Self::Result15,
        );
    }
    impl<
        IA,
        IB,
        IC,
        ID,
        IE,
        IF,
        IG,
        IH,
        II,
        IJ,
        IK,
        IL,
        IM,
        IN,
        IO,
        IP,
        OA,
        OB,
        OC,
        OD,
        OE,
        OF,
        OG,
        OH,
        OI,
        OJ,
        OK,
        OL,
        OM,
        ON,
        OO,
        OP,
        FA,
        FB,
        FC,
        FD,
        FE,
        FF,
        FG,
        FH,
        FI,
        FJ,
        FK,
        FL,
        FM,
        FN,
        FO,
        FP,
    > Map16<IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM, IN, IO, IP>
    for (FA, FB, FC, FD, FE, FF, FG, FH, FI, FJ, FK, FL, FM, FN, FO, FP)
    where
        FA: FnMut(IA) -> OA,
        FB: FnMut(IB) -> OB,
        FC: FnMut(IC) -> OC,
        FD: FnMut(ID) -> OD,
        FE: FnMut(IE) -> OE,
        FF: FnMut(IF) -> OF,
        FG: FnMut(IG) -> OG,
        FH: FnMut(IH) -> OH,
        FI: FnMut(II) -> OI,
        FJ: FnMut(IJ) -> OJ,
        FK: FnMut(IK) -> OK,
        FL: FnMut(IL) -> OL,
        FM: FnMut(IM) -> OM,
        FN: FnMut(IN) -> ON,
        FO: FnMut(IO) -> OO,
        FP: FnMut(IP) -> OP,
    {
        type Result0 = OA;
        type Result1 = OB;
        type Result2 = OC;
        type Result3 = OD;
        type Result4 = OE;
        type Result5 = OF;
        type Result6 = OG;
        type Result7 = OH;
        type Result8 = OI;
        type Result9 = OJ;
        type Result10 = OK;
        type Result11 = OL;
        type Result12 = OM;
        type Result13 = ON;
        type Result14 = OO;
        type Result15 = OP;
        fn map_0(mut self, value: IA) -> Self::Result0 {
            (self.0)(value)
        }
        fn map_1(mut self, value: IB) -> Self::Result1 {
            (self.1)(value)
        }
        fn map_2(mut self, value: IC) -> Self::Result2 {
            (self.2)(value)
        }
        fn map_3(mut self, value: ID) -> Self::Result3 {
            (self.3)(value)
        }
        fn map_4(mut self, value: IE) -> Self::Result4 {
            (self.4)(value)
        }
        fn map_5(mut self, value: IF) -> Self::Result5 {
            (self.5)(value)
        }
        fn map_6(mut self, value: IG) -> Self::Result6 {
            (self.6)(value)
        }
        fn map_7(mut self, value: IH) -> Self::Result7 {
            (self.7)(value)
        }
        fn map_8(mut self, value: II) -> Self::Result8 {
            (self.8)(value)
        }
        fn map_9(mut self, value: IJ) -> Self::Result9 {
            (self.9)(value)
        }
        fn map_10(mut self, value: IK) -> Self::Result10 {
            (self.10)(value)
        }
        fn map_11(mut self, value: IL) -> Self::Result11 {
            (self.11)(value)
        }
        fn map_12(mut self, value: IM) -> Self::Result12 {
            (self.12)(value)
        }
        fn map_13(mut self, value: IN) -> Self::Result13 {
            (self.13)(value)
        }
        fn map_14(mut self, value: IO) -> Self::Result14 {
            (self.14)(value)
        }
        fn map_15(mut self, value: IP) -> Self::Result15 {
            (self.15)(value)
        }
        fn apply(
            mut self,
            value: (IA, IB, IC, ID, IE, IF, IG, IH, II, IJ, IK, IL, IM, IN, IO, IP),
        ) -> (
            Self::Result0,
            Self::Result1,
            Self::Result2,
            Self::Result3,
            Self::Result4,
            Self::Result5,
            Self::Result6,
            Self::Result7,
            Self::Result8,
            Self::Result9,
            Self::Result10,
            Self::Result11,
            Self::Result12,
            Self::Result13,
            Self::Result14,
            Self::Result15,
        ) {
            (
                (self.0)(value.0),
                (self.1)(value.1),
                (self.2)(value.2),
                (self.3)(value.3),
                (self.4)(value.4),
                (self.5)(value.5),
                (self.6)(value.6),
                (self.7)(value.7),
                (self.8)(value.8),
                (self.9)(value.9),
                (self.10)(value.10),
                (self.11)(value.11),
                (self.12)(value.12),
                (self.13)(value.13),
                (self.14)(value.14),
                (self.15)(value.15),
            )
        }
    }
}
use mapping::*;
impl TypeList for () {
    type Head = crate::Never;
    type Tail = ();
    type First = crate::Never;
    type Last = crate::Never;
    type Reverse = ();
    type PushFront<Pushed> = (Pushed,);
    type PushBack<Pushed> = (Pushed,);
    type Variant = Tuple0;
    type Variants = [Self::Variant; 0];
    const LEN: usize = 0;
    const IS_EMPTY: bool = true;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (value,)
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (value,)
    }
    fn reverse(self) -> Self::Reverse {
        ()
    }
    fn into_variants(self) -> Self::Variants {
        []
    }
}
impl<A> TypeList for (A,) {
    type Head = A;
    type Tail = ();
    type First = A;
    type Last = A;
    type Reverse = (A,);
    type PushFront<Pushed> = (Pushed, A);
    type PushBack<Pushed> = (A, Pushed);
    type Variant = Tuple1<A>;
    type Variants = [Self::Variant; 1];
    const LEN: usize = 1;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (self.0, value)
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (value, self.0)
    }
    fn reverse(self) -> Self::Reverse {
        (self.0,)
    }
    fn into_variants(self) -> Self::Variants {
        [Tuple1::Variant0(self.0)]
    }
}
impl<A> NonEmpty for (A,) {
    type LTail = ();
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.0
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (self.0, ())
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (self.0, ())
    }
}
impl<A> HasElement<0> for (A,) {
    type Value = A;
    type Other = ();
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.0, ())
    }
}
impl<A, B> TypeList for (A, B) {
    type Head = A;
    type Tail = (B,);
    type First = A;
    type Last = B;
    type Reverse = (B, A);
    type PushFront<Pushed> = (Pushed, A, B);
    type PushBack<Pushed> = (A, B, Pushed);
    type Variant = Tuple2<A, B>;
    type Variants = [Self::Variant; 2];
    const LEN: usize = 2;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (self.0, self.1, value)
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (value, self.0, self.1)
    }
    fn reverse(self) -> Self::Reverse {
        (self.1, self.0)
    }
    fn into_variants(self) -> Self::Variants {
        [Tuple2::Variant0(self.0), Tuple2::Variant1(self.1)]
    }
}
impl<A, B> NonEmpty for (A, B) {
    type LTail = (A,);
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.1
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (self.1, (self.0,))
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (self.0, (self.1,))
    }
}
impl<A, B> HasElement<0> for (A, B) {
    type Value = A;
    type Other = (B,);
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.0, (self.1,))
    }
}
impl<A, B> HasElement<1> for (A, B) {
    type Value = B;
    type Other = (A,);
    fn get(&self) -> &Self::Value {
        &self.1
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.1, (self.0,))
    }
}
impl<A, B, C> TypeList for (A, B, C) {
    type Head = A;
    type Tail = (B, C);
    type First = A;
    type Last = C;
    type Reverse = (C, B, A);
    type PushFront<Pushed> = (Pushed, A, B, C);
    type PushBack<Pushed> = (A, B, C, Pushed);
    type Variant = Tuple3<A, B, C>;
    type Variants = [Self::Variant; 3];
    const LEN: usize = 3;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (self.0, self.1, self.2, value)
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (value, self.0, self.1, self.2)
    }
    fn reverse(self) -> Self::Reverse {
        (self.2, self.1, self.0)
    }
    fn into_variants(self) -> Self::Variants {
        [Tuple3::Variant0(self.0), Tuple3::Variant1(self.1), Tuple3::Variant2(self.2)]
    }
}
impl<A, B, C> NonEmpty for (A, B, C) {
    type LTail = (A, B);
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.2
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (self.2, (self.0, self.1))
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (self.0, (self.1, self.2))
    }
}
impl<A, B, C> HasElement<0> for (A, B, C) {
    type Value = A;
    type Other = (B, C);
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.0, (self.1, self.2))
    }
}
impl<A, B, C> HasElement<1> for (A, B, C) {
    type Value = B;
    type Other = (A, C);
    fn get(&self) -> &Self::Value {
        &self.1
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.1, (self.0, self.2))
    }
}
impl<A, B, C> HasElement<2> for (A, B, C) {
    type Value = C;
    type Other = (A, B);
    fn get(&self) -> &Self::Value {
        &self.2
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.2, (self.0, self.1))
    }
}
impl<A, B, C, D> TypeList for (A, B, C, D) {
    type Head = A;
    type Tail = (B, C, D);
    type First = A;
    type Last = D;
    type Reverse = (D, C, B, A);
    type PushFront<Pushed> = (Pushed, A, B, C, D);
    type PushBack<Pushed> = (A, B, C, D, Pushed);
    type Variant = Tuple4<A, B, C, D>;
    type Variants = [Self::Variant; 4];
    const LEN: usize = 4;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (self.0, self.1, self.2, self.3, value)
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (value, self.0, self.1, self.2, self.3)
    }
    fn reverse(self) -> Self::Reverse {
        (self.3, self.2, self.1, self.0)
    }
    fn into_variants(self) -> Self::Variants {
        [
            Tuple4::Variant0(self.0),
            Tuple4::Variant1(self.1),
            Tuple4::Variant2(self.2),
            Tuple4::Variant3(self.3),
        ]
    }
}
impl<A, B, C, D> NonEmpty for (A, B, C, D) {
    type LTail = (A, B, C);
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.3
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (self.3, (self.0, self.1, self.2))
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (self.0, (self.1, self.2, self.3))
    }
}
impl<A, B, C, D> HasElement<0> for (A, B, C, D) {
    type Value = A;
    type Other = (B, C, D);
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.0, (self.1, self.2, self.3))
    }
}
impl<A, B, C, D> HasElement<1> for (A, B, C, D) {
    type Value = B;
    type Other = (A, C, D);
    fn get(&self) -> &Self::Value {
        &self.1
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.1, (self.0, self.2, self.3))
    }
}
impl<A, B, C, D> HasElement<2> for (A, B, C, D) {
    type Value = C;
    type Other = (A, B, D);
    fn get(&self) -> &Self::Value {
        &self.2
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.2, (self.0, self.1, self.3))
    }
}
impl<A, B, C, D> HasElement<3> for (A, B, C, D) {
    type Value = D;
    type Other = (A, B, C);
    fn get(&self) -> &Self::Value {
        &self.3
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.3, (self.0, self.1, self.2))
    }
}
impl<A, B, C, D, E> TypeList for (A, B, C, D, E) {
    type Head = A;
    type Tail = (B, C, D, E);
    type First = A;
    type Last = E;
    type Reverse = (E, D, C, B, A);
    type PushFront<Pushed> = (Pushed, A, B, C, D, E);
    type PushBack<Pushed> = (A, B, C, D, E, Pushed);
    type Variant = Tuple5<A, B, C, D, E>;
    type Variants = [Self::Variant; 5];
    const LEN: usize = 5;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (self.0, self.1, self.2, self.3, self.4, value)
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (value, self.0, self.1, self.2, self.3, self.4)
    }
    fn reverse(self) -> Self::Reverse {
        (self.4, self.3, self.2, self.1, self.0)
    }
    fn into_variants(self) -> Self::Variants {
        [
            Tuple5::Variant0(self.0),
            Tuple5::Variant1(self.1),
            Tuple5::Variant2(self.2),
            Tuple5::Variant3(self.3),
            Tuple5::Variant4(self.4),
        ]
    }
}
impl<A, B, C, D, E> NonEmpty for (A, B, C, D, E) {
    type LTail = (A, B, C, D);
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.4
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (self.4, (self.0, self.1, self.2, self.3))
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (self.0, (self.1, self.2, self.3, self.4))
    }
}
impl<A, B, C, D, E> HasElement<0> for (A, B, C, D, E) {
    type Value = A;
    type Other = (B, C, D, E);
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.0, (self.1, self.2, self.3, self.4))
    }
}
impl<A, B, C, D, E> HasElement<1> for (A, B, C, D, E) {
    type Value = B;
    type Other = (A, C, D, E);
    fn get(&self) -> &Self::Value {
        &self.1
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.1, (self.0, self.2, self.3, self.4))
    }
}
impl<A, B, C, D, E> HasElement<2> for (A, B, C, D, E) {
    type Value = C;
    type Other = (A, B, D, E);
    fn get(&self) -> &Self::Value {
        &self.2
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.2, (self.0, self.1, self.3, self.4))
    }
}
impl<A, B, C, D, E> HasElement<3> for (A, B, C, D, E) {
    type Value = D;
    type Other = (A, B, C, E);
    fn get(&self) -> &Self::Value {
        &self.3
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.3, (self.0, self.1, self.2, self.4))
    }
}
impl<A, B, C, D, E> HasElement<4> for (A, B, C, D, E) {
    type Value = E;
    type Other = (A, B, C, D);
    fn get(&self) -> &Self::Value {
        &self.4
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.4, (self.0, self.1, self.2, self.3))
    }
}
impl<A, B, C, D, E, F> TypeList for (A, B, C, D, E, F) {
    type Head = A;
    type Tail = (B, C, D, E, F);
    type First = A;
    type Last = F;
    type Reverse = (F, E, D, C, B, A);
    type PushFront<Pushed> = (Pushed, A, B, C, D, E, F);
    type PushBack<Pushed> = (A, B, C, D, E, F, Pushed);
    type Variant = Tuple6<A, B, C, D, E, F>;
    type Variants = [Self::Variant; 6];
    const LEN: usize = 6;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (self.0, self.1, self.2, self.3, self.4, self.5, value)
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (value, self.0, self.1, self.2, self.3, self.4, self.5)
    }
    fn reverse(self) -> Self::Reverse {
        (self.5, self.4, self.3, self.2, self.1, self.0)
    }
    fn into_variants(self) -> Self::Variants {
        [
            Tuple6::Variant0(self.0),
            Tuple6::Variant1(self.1),
            Tuple6::Variant2(self.2),
            Tuple6::Variant3(self.3),
            Tuple6::Variant4(self.4),
            Tuple6::Variant5(self.5),
        ]
    }
}
impl<A, B, C, D, E, F> NonEmpty for (A, B, C, D, E, F) {
    type LTail = (A, B, C, D, E);
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.5
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (self.5, (self.0, self.1, self.2, self.3, self.4))
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (self.0, (self.1, self.2, self.3, self.4, self.5))
    }
}
impl<A, B, C, D, E, F> HasElement<0> for (A, B, C, D, E, F) {
    type Value = A;
    type Other = (B, C, D, E, F);
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.0, (self.1, self.2, self.3, self.4, self.5))
    }
}
impl<A, B, C, D, E, F> HasElement<1> for (A, B, C, D, E, F) {
    type Value = B;
    type Other = (A, C, D, E, F);
    fn get(&self) -> &Self::Value {
        &self.1
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.1, (self.0, self.2, self.3, self.4, self.5))
    }
}
impl<A, B, C, D, E, F> HasElement<2> for (A, B, C, D, E, F) {
    type Value = C;
    type Other = (A, B, D, E, F);
    fn get(&self) -> &Self::Value {
        &self.2
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.2, (self.0, self.1, self.3, self.4, self.5))
    }
}
impl<A, B, C, D, E, F> HasElement<3> for (A, B, C, D, E, F) {
    type Value = D;
    type Other = (A, B, C, E, F);
    fn get(&self) -> &Self::Value {
        &self.3
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.3, (self.0, self.1, self.2, self.4, self.5))
    }
}
impl<A, B, C, D, E, F> HasElement<4> for (A, B, C, D, E, F) {
    type Value = E;
    type Other = (A, B, C, D, F);
    fn get(&self) -> &Self::Value {
        &self.4
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.4, (self.0, self.1, self.2, self.3, self.5))
    }
}
impl<A, B, C, D, E, F> HasElement<5> for (A, B, C, D, E, F) {
    type Value = F;
    type Other = (A, B, C, D, E);
    fn get(&self) -> &Self::Value {
        &self.5
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.5, (self.0, self.1, self.2, self.3, self.4))
    }
}
impl<A, B, C, D, E, F, G> TypeList for (A, B, C, D, E, F, G) {
    type Head = A;
    type Tail = (B, C, D, E, F, G);
    type First = A;
    type Last = G;
    type Reverse = (G, F, E, D, C, B, A);
    type PushFront<Pushed> = (Pushed, A, B, C, D, E, F, G);
    type PushBack<Pushed> = (A, B, C, D, E, F, G, Pushed);
    type Variant = Tuple7<A, B, C, D, E, F, G>;
    type Variants = [Self::Variant; 7];
    const LEN: usize = 7;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, value)
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (value, self.0, self.1, self.2, self.3, self.4, self.5, self.6)
    }
    fn reverse(self) -> Self::Reverse {
        (self.6, self.5, self.4, self.3, self.2, self.1, self.0)
    }
    fn into_variants(self) -> Self::Variants {
        [
            Tuple7::Variant0(self.0),
            Tuple7::Variant1(self.1),
            Tuple7::Variant2(self.2),
            Tuple7::Variant3(self.3),
            Tuple7::Variant4(self.4),
            Tuple7::Variant5(self.5),
            Tuple7::Variant6(self.6),
        ]
    }
}
impl<A, B, C, D, E, F, G> NonEmpty for (A, B, C, D, E, F, G) {
    type LTail = (A, B, C, D, E, F);
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.6
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (self.6, (self.0, self.1, self.2, self.3, self.4, self.5))
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6))
    }
}
impl<A, B, C, D, E, F, G> HasElement<0> for (A, B, C, D, E, F, G) {
    type Value = A;
    type Other = (B, C, D, E, F, G);
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6))
    }
}
impl<A, B, C, D, E, F, G> HasElement<1> for (A, B, C, D, E, F, G) {
    type Value = B;
    type Other = (A, C, D, E, F, G);
    fn get(&self) -> &Self::Value {
        &self.1
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.1, (self.0, self.2, self.3, self.4, self.5, self.6))
    }
}
impl<A, B, C, D, E, F, G> HasElement<2> for (A, B, C, D, E, F, G) {
    type Value = C;
    type Other = (A, B, D, E, F, G);
    fn get(&self) -> &Self::Value {
        &self.2
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.2, (self.0, self.1, self.3, self.4, self.5, self.6))
    }
}
impl<A, B, C, D, E, F, G> HasElement<3> for (A, B, C, D, E, F, G) {
    type Value = D;
    type Other = (A, B, C, E, F, G);
    fn get(&self) -> &Self::Value {
        &self.3
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.3, (self.0, self.1, self.2, self.4, self.5, self.6))
    }
}
impl<A, B, C, D, E, F, G> HasElement<4> for (A, B, C, D, E, F, G) {
    type Value = E;
    type Other = (A, B, C, D, F, G);
    fn get(&self) -> &Self::Value {
        &self.4
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.4, (self.0, self.1, self.2, self.3, self.5, self.6))
    }
}
impl<A, B, C, D, E, F, G> HasElement<5> for (A, B, C, D, E, F, G) {
    type Value = F;
    type Other = (A, B, C, D, E, G);
    fn get(&self) -> &Self::Value {
        &self.5
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.5, (self.0, self.1, self.2, self.3, self.4, self.6))
    }
}
impl<A, B, C, D, E, F, G> HasElement<6> for (A, B, C, D, E, F, G) {
    type Value = G;
    type Other = (A, B, C, D, E, F);
    fn get(&self) -> &Self::Value {
        &self.6
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.6, (self.0, self.1, self.2, self.3, self.4, self.5))
    }
}
impl<A, B, C, D, E, F, G, H> TypeList for (A, B, C, D, E, F, G, H) {
    type Head = A;
    type Tail = (B, C, D, E, F, G, H);
    type First = A;
    type Last = H;
    type Reverse = (H, G, F, E, D, C, B, A);
    type PushFront<Pushed> = (Pushed, A, B, C, D, E, F, G, H);
    type PushBack<Pushed> = (A, B, C, D, E, F, G, H, Pushed);
    type Variant = Tuple8<A, B, C, D, E, F, G, H>;
    type Variants = [Self::Variant; 8];
    const LEN: usize = 8;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, value)
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (value, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7)
    }
    fn reverse(self) -> Self::Reverse {
        (self.7, self.6, self.5, self.4, self.3, self.2, self.1, self.0)
    }
    fn into_variants(self) -> Self::Variants {
        [
            Tuple8::Variant0(self.0),
            Tuple8::Variant1(self.1),
            Tuple8::Variant2(self.2),
            Tuple8::Variant3(self.3),
            Tuple8::Variant4(self.4),
            Tuple8::Variant5(self.5),
            Tuple8::Variant6(self.6),
            Tuple8::Variant7(self.7),
        ]
    }
}
impl<A, B, C, D, E, F, G, H> NonEmpty for (A, B, C, D, E, F, G, H) {
    type LTail = (A, B, C, D, E, F, G);
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.7
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (self.7, (self.0, self.1, self.2, self.3, self.4, self.5, self.6))
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7))
    }
}
impl<A, B, C, D, E, F, G, H> HasElement<0> for (A, B, C, D, E, F, G, H) {
    type Value = A;
    type Other = (B, C, D, E, F, G, H);
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7))
    }
}
impl<A, B, C, D, E, F, G, H> HasElement<1> for (A, B, C, D, E, F, G, H) {
    type Value = B;
    type Other = (A, C, D, E, F, G, H);
    fn get(&self) -> &Self::Value {
        &self.1
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.1, (self.0, self.2, self.3, self.4, self.5, self.6, self.7))
    }
}
impl<A, B, C, D, E, F, G, H> HasElement<2> for (A, B, C, D, E, F, G, H) {
    type Value = C;
    type Other = (A, B, D, E, F, G, H);
    fn get(&self) -> &Self::Value {
        &self.2
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.2, (self.0, self.1, self.3, self.4, self.5, self.6, self.7))
    }
}
impl<A, B, C, D, E, F, G, H> HasElement<3> for (A, B, C, D, E, F, G, H) {
    type Value = D;
    type Other = (A, B, C, E, F, G, H);
    fn get(&self) -> &Self::Value {
        &self.3
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.3, (self.0, self.1, self.2, self.4, self.5, self.6, self.7))
    }
}
impl<A, B, C, D, E, F, G, H> HasElement<4> for (A, B, C, D, E, F, G, H) {
    type Value = E;
    type Other = (A, B, C, D, F, G, H);
    fn get(&self) -> &Self::Value {
        &self.4
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.4, (self.0, self.1, self.2, self.3, self.5, self.6, self.7))
    }
}
impl<A, B, C, D, E, F, G, H> HasElement<5> for (A, B, C, D, E, F, G, H) {
    type Value = F;
    type Other = (A, B, C, D, E, G, H);
    fn get(&self) -> &Self::Value {
        &self.5
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.5, (self.0, self.1, self.2, self.3, self.4, self.6, self.7))
    }
}
impl<A, B, C, D, E, F, G, H> HasElement<6> for (A, B, C, D, E, F, G, H) {
    type Value = G;
    type Other = (A, B, C, D, E, F, H);
    fn get(&self) -> &Self::Value {
        &self.6
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.6, (self.0, self.1, self.2, self.3, self.4, self.5, self.7))
    }
}
impl<A, B, C, D, E, F, G, H> HasElement<7> for (A, B, C, D, E, F, G, H) {
    type Value = H;
    type Other = (A, B, C, D, E, F, G);
    fn get(&self) -> &Self::Value {
        &self.7
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.7, (self.0, self.1, self.2, self.3, self.4, self.5, self.6))
    }
}
impl<A, B, C, D, E, F, G, H, I> TypeList for (A, B, C, D, E, F, G, H, I) {
    type Head = A;
    type Tail = (B, C, D, E, F, G, H, I);
    type First = A;
    type Last = I;
    type Reverse = (I, H, G, F, E, D, C, B, A);
    type PushFront<Pushed> = (Pushed, A, B, C, D, E, F, G, H, I);
    type PushBack<Pushed> = (A, B, C, D, E, F, G, H, I, Pushed);
    type Variant = Tuple9<A, B, C, D, E, F, G, H, I>;
    type Variants = [Self::Variant; 9];
    const LEN: usize = 9;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, value)
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (value, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8)
    }
    fn reverse(self) -> Self::Reverse {
        (self.8, self.7, self.6, self.5, self.4, self.3, self.2, self.1, self.0)
    }
    fn into_variants(self) -> Self::Variants {
        [
            Tuple9::Variant0(self.0),
            Tuple9::Variant1(self.1),
            Tuple9::Variant2(self.2),
            Tuple9::Variant3(self.3),
            Tuple9::Variant4(self.4),
            Tuple9::Variant5(self.5),
            Tuple9::Variant6(self.6),
            Tuple9::Variant7(self.7),
            Tuple9::Variant8(self.8),
        ]
    }
}
impl<A, B, C, D, E, F, G, H, I> NonEmpty for (A, B, C, D, E, F, G, H, I) {
    type LTail = (A, B, C, D, E, F, G, H);
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.8
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (self.8, (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7))
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8))
    }
}
impl<A, B, C, D, E, F, G, H, I> HasElement<0> for (A, B, C, D, E, F, G, H, I) {
    type Value = A;
    type Other = (B, C, D, E, F, G, H, I);
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8))
    }
}
impl<A, B, C, D, E, F, G, H, I> HasElement<1> for (A, B, C, D, E, F, G, H, I) {
    type Value = B;
    type Other = (A, C, D, E, F, G, H, I);
    fn get(&self) -> &Self::Value {
        &self.1
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.1, (self.0, self.2, self.3, self.4, self.5, self.6, self.7, self.8))
    }
}
impl<A, B, C, D, E, F, G, H, I> HasElement<2> for (A, B, C, D, E, F, G, H, I) {
    type Value = C;
    type Other = (A, B, D, E, F, G, H, I);
    fn get(&self) -> &Self::Value {
        &self.2
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.2, (self.0, self.1, self.3, self.4, self.5, self.6, self.7, self.8))
    }
}
impl<A, B, C, D, E, F, G, H, I> HasElement<3> for (A, B, C, D, E, F, G, H, I) {
    type Value = D;
    type Other = (A, B, C, E, F, G, H, I);
    fn get(&self) -> &Self::Value {
        &self.3
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.3, (self.0, self.1, self.2, self.4, self.5, self.6, self.7, self.8))
    }
}
impl<A, B, C, D, E, F, G, H, I> HasElement<4> for (A, B, C, D, E, F, G, H, I) {
    type Value = E;
    type Other = (A, B, C, D, F, G, H, I);
    fn get(&self) -> &Self::Value {
        &self.4
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.4, (self.0, self.1, self.2, self.3, self.5, self.6, self.7, self.8))
    }
}
impl<A, B, C, D, E, F, G, H, I> HasElement<5> for (A, B, C, D, E, F, G, H, I) {
    type Value = F;
    type Other = (A, B, C, D, E, G, H, I);
    fn get(&self) -> &Self::Value {
        &self.5
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.5, (self.0, self.1, self.2, self.3, self.4, self.6, self.7, self.8))
    }
}
impl<A, B, C, D, E, F, G, H, I> HasElement<6> for (A, B, C, D, E, F, G, H, I) {
    type Value = G;
    type Other = (A, B, C, D, E, F, H, I);
    fn get(&self) -> &Self::Value {
        &self.6
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.6, (self.0, self.1, self.2, self.3, self.4, self.5, self.7, self.8))
    }
}
impl<A, B, C, D, E, F, G, H, I> HasElement<7> for (A, B, C, D, E, F, G, H, I) {
    type Value = H;
    type Other = (A, B, C, D, E, F, G, I);
    fn get(&self) -> &Self::Value {
        &self.7
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.7, (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.8))
    }
}
impl<A, B, C, D, E, F, G, H, I> HasElement<8> for (A, B, C, D, E, F, G, H, I) {
    type Value = I;
    type Other = (A, B, C, D, E, F, G, H);
    fn get(&self) -> &Self::Value {
        &self.8
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (self.8, (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7))
    }
}
impl<A, B, C, D, E, F, G, H, I, J> TypeList for (A, B, C, D, E, F, G, H, I, J) {
    type Head = A;
    type Tail = (B, C, D, E, F, G, H, I, J);
    type First = A;
    type Last = J;
    type Reverse = (J, I, H, G, F, E, D, C, B, A);
    type PushFront<Pushed> = (Pushed, A, B, C, D, E, F, G, H, I, J);
    type PushBack<Pushed> = (A, B, C, D, E, F, G, H, I, J, Pushed);
    type Variant = Tuple10<A, B, C, D, E, F, G, H, I, J>;
    type Variants = [Self::Variant; 10];
    const LEN: usize = 10;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7,
            self.8,
            self.9,
            value,
        )
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (
            value,
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7,
            self.8,
            self.9,
        )
    }
    fn reverse(self) -> Self::Reverse {
        (self.9, self.8, self.7, self.6, self.5, self.4, self.3, self.2, self.1, self.0)
    }
    fn into_variants(self) -> Self::Variants {
        [
            Tuple10::Variant0(self.0),
            Tuple10::Variant1(self.1),
            Tuple10::Variant2(self.2),
            Tuple10::Variant3(self.3),
            Tuple10::Variant4(self.4),
            Tuple10::Variant5(self.5),
            Tuple10::Variant6(self.6),
            Tuple10::Variant7(self.7),
            Tuple10::Variant8(self.8),
            Tuple10::Variant9(self.9),
        ]
    }
}
impl<A, B, C, D, E, F, G, H, I, J> NonEmpty for (A, B, C, D, E, F, G, H, I, J) {
    type LTail = (A, B, C, D, E, F, G, H, I);
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.9
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (
            self.9,
            (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8),
        )
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (
            self.0,
            (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J> HasElement<0> for (A, B, C, D, E, F, G, H, I, J) {
    type Value = A;
    type Other = (B, C, D, E, F, G, H, I, J);
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.0,
            (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J> HasElement<1> for (A, B, C, D, E, F, G, H, I, J) {
    type Value = B;
    type Other = (A, C, D, E, F, G, H, I, J);
    fn get(&self) -> &Self::Value {
        &self.1
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.1,
            (self.0, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J> HasElement<2> for (A, B, C, D, E, F, G, H, I, J) {
    type Value = C;
    type Other = (A, B, D, E, F, G, H, I, J);
    fn get(&self) -> &Self::Value {
        &self.2
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.2,
            (self.0, self.1, self.3, self.4, self.5, self.6, self.7, self.8, self.9),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J> HasElement<3> for (A, B, C, D, E, F, G, H, I, J) {
    type Value = D;
    type Other = (A, B, C, E, F, G, H, I, J);
    fn get(&self) -> &Self::Value {
        &self.3
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.3,
            (self.0, self.1, self.2, self.4, self.5, self.6, self.7, self.8, self.9),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J> HasElement<4> for (A, B, C, D, E, F, G, H, I, J) {
    type Value = E;
    type Other = (A, B, C, D, F, G, H, I, J);
    fn get(&self) -> &Self::Value {
        &self.4
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.4,
            (self.0, self.1, self.2, self.3, self.5, self.6, self.7, self.8, self.9),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J> HasElement<5> for (A, B, C, D, E, F, G, H, I, J) {
    type Value = F;
    type Other = (A, B, C, D, E, G, H, I, J);
    fn get(&self) -> &Self::Value {
        &self.5
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.5,
            (self.0, self.1, self.2, self.3, self.4, self.6, self.7, self.8, self.9),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J> HasElement<6> for (A, B, C, D, E, F, G, H, I, J) {
    type Value = G;
    type Other = (A, B, C, D, E, F, H, I, J);
    fn get(&self) -> &Self::Value {
        &self.6
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.6,
            (self.0, self.1, self.2, self.3, self.4, self.5, self.7, self.8, self.9),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J> HasElement<7> for (A, B, C, D, E, F, G, H, I, J) {
    type Value = H;
    type Other = (A, B, C, D, E, F, G, I, J);
    fn get(&self) -> &Self::Value {
        &self.7
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.7,
            (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.8, self.9),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J> HasElement<8> for (A, B, C, D, E, F, G, H, I, J) {
    type Value = I;
    type Other = (A, B, C, D, E, F, G, H, J);
    fn get(&self) -> &Self::Value {
        &self.8
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.8,
            (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.9),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J> HasElement<9> for (A, B, C, D, E, F, G, H, I, J) {
    type Value = J;
    type Other = (A, B, C, D, E, F, G, H, I);
    fn get(&self) -> &Self::Value {
        &self.9
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.9,
            (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K> TypeList for (A, B, C, D, E, F, G, H, I, J, K) {
    type Head = A;
    type Tail = (B, C, D, E, F, G, H, I, J, K);
    type First = A;
    type Last = K;
    type Reverse = (K, J, I, H, G, F, E, D, C, B, A);
    type PushFront<Pushed> = (Pushed, A, B, C, D, E, F, G, H, I, J, K);
    type PushBack<Pushed> = (A, B, C, D, E, F, G, H, I, J, K, Pushed);
    type Variant = Tuple11<A, B, C, D, E, F, G, H, I, J, K>;
    type Variants = [Self::Variant; 11];
    const LEN: usize = 11;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7,
            self.8,
            self.9,
            self.10,
            value,
        )
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (
            value,
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7,
            self.8,
            self.9,
            self.10,
        )
    }
    fn reverse(self) -> Self::Reverse {
        (
            self.10,
            self.9,
            self.8,
            self.7,
            self.6,
            self.5,
            self.4,
            self.3,
            self.2,
            self.1,
            self.0,
        )
    }
    fn into_variants(self) -> Self::Variants {
        [
            Tuple11::Variant0(self.0),
            Tuple11::Variant1(self.1),
            Tuple11::Variant2(self.2),
            Tuple11::Variant3(self.3),
            Tuple11::Variant4(self.4),
            Tuple11::Variant5(self.5),
            Tuple11::Variant6(self.6),
            Tuple11::Variant7(self.7),
            Tuple11::Variant8(self.8),
            Tuple11::Variant9(self.9),
            Tuple11::Variant10(self.10),
        ]
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K> NonEmpty for (A, B, C, D, E, F, G, H, I, J, K) {
    type LTail = (A, B, C, D, E, F, G, H, I, J);
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.10
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (
            self.10,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
            ),
        )
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (
            self.0,
            (
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K> HasElement<0>
for (A, B, C, D, E, F, G, H, I, J, K) {
    type Value = A;
    type Other = (B, C, D, E, F, G, H, I, J, K);
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.0,
            (
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K> HasElement<1>
for (A, B, C, D, E, F, G, H, I, J, K) {
    type Value = B;
    type Other = (A, C, D, E, F, G, H, I, J, K);
    fn get(&self) -> &Self::Value {
        &self.1
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.1,
            (
                self.0,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K> HasElement<2>
for (A, B, C, D, E, F, G, H, I, J, K) {
    type Value = C;
    type Other = (A, B, D, E, F, G, H, I, J, K);
    fn get(&self) -> &Self::Value {
        &self.2
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.2,
            (
                self.0,
                self.1,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K> HasElement<3>
for (A, B, C, D, E, F, G, H, I, J, K) {
    type Value = D;
    type Other = (A, B, C, E, F, G, H, I, J, K);
    fn get(&self) -> &Self::Value {
        &self.3
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.3,
            (
                self.0,
                self.1,
                self.2,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K> HasElement<4>
for (A, B, C, D, E, F, G, H, I, J, K) {
    type Value = E;
    type Other = (A, B, C, D, F, G, H, I, J, K);
    fn get(&self) -> &Self::Value {
        &self.4
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.4,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K> HasElement<5>
for (A, B, C, D, E, F, G, H, I, J, K) {
    type Value = F;
    type Other = (A, B, C, D, E, G, H, I, J, K);
    fn get(&self) -> &Self::Value {
        &self.5
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.5,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K> HasElement<6>
for (A, B, C, D, E, F, G, H, I, J, K) {
    type Value = G;
    type Other = (A, B, C, D, E, F, H, I, J, K);
    fn get(&self) -> &Self::Value {
        &self.6
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.6,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.7,
                self.8,
                self.9,
                self.10,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K> HasElement<7>
for (A, B, C, D, E, F, G, H, I, J, K) {
    type Value = H;
    type Other = (A, B, C, D, E, F, G, I, J, K);
    fn get(&self) -> &Self::Value {
        &self.7
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.7,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.8,
                self.9,
                self.10,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K> HasElement<8>
for (A, B, C, D, E, F, G, H, I, J, K) {
    type Value = I;
    type Other = (A, B, C, D, E, F, G, H, J, K);
    fn get(&self) -> &Self::Value {
        &self.8
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.8,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.9,
                self.10,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K> HasElement<9>
for (A, B, C, D, E, F, G, H, I, J, K) {
    type Value = J;
    type Other = (A, B, C, D, E, F, G, H, I, K);
    fn get(&self) -> &Self::Value {
        &self.9
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.9,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.10,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K> HasElement<10>
for (A, B, C, D, E, F, G, H, I, J, K) {
    type Value = K;
    type Other = (A, B, C, D, E, F, G, H, I, J);
    fn get(&self) -> &Self::Value {
        &self.10
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.10,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> TypeList
for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type Head = A;
    type Tail = (B, C, D, E, F, G, H, I, J, K, L);
    type First = A;
    type Last = L;
    type Reverse = (L, K, J, I, H, G, F, E, D, C, B, A);
    type PushFront<Pushed> = (Pushed, A, B, C, D, E, F, G, H, I, J, K, L);
    type PushBack<Pushed> = (A, B, C, D, E, F, G, H, I, J, K, L, Pushed);
    type Variant = Tuple12<A, B, C, D, E, F, G, H, I, J, K, L>;
    type Variants = [Self::Variant; 12];
    const LEN: usize = 12;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7,
            self.8,
            self.9,
            self.10,
            self.11,
            value,
        )
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (
            value,
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7,
            self.8,
            self.9,
            self.10,
            self.11,
        )
    }
    fn reverse(self) -> Self::Reverse {
        (
            self.11,
            self.10,
            self.9,
            self.8,
            self.7,
            self.6,
            self.5,
            self.4,
            self.3,
            self.2,
            self.1,
            self.0,
        )
    }
    fn into_variants(self) -> Self::Variants {
        [
            Tuple12::Variant0(self.0),
            Tuple12::Variant1(self.1),
            Tuple12::Variant2(self.2),
            Tuple12::Variant3(self.3),
            Tuple12::Variant4(self.4),
            Tuple12::Variant5(self.5),
            Tuple12::Variant6(self.6),
            Tuple12::Variant7(self.7),
            Tuple12::Variant8(self.8),
            Tuple12::Variant9(self.9),
            Tuple12::Variant10(self.10),
            Tuple12::Variant11(self.11),
        ]
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> NonEmpty
for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type LTail = (A, B, C, D, E, F, G, H, I, J, K);
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.11
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (
            self.11,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
            ),
        )
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (
            self.0,
            (
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> HasElement<0>
for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type Value = A;
    type Other = (B, C, D, E, F, G, H, I, J, K, L);
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.0,
            (
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> HasElement<1>
for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type Value = B;
    type Other = (A, C, D, E, F, G, H, I, J, K, L);
    fn get(&self) -> &Self::Value {
        &self.1
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.1,
            (
                self.0,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> HasElement<2>
for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type Value = C;
    type Other = (A, B, D, E, F, G, H, I, J, K, L);
    fn get(&self) -> &Self::Value {
        &self.2
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.2,
            (
                self.0,
                self.1,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> HasElement<3>
for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type Value = D;
    type Other = (A, B, C, E, F, G, H, I, J, K, L);
    fn get(&self) -> &Self::Value {
        &self.3
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.3,
            (
                self.0,
                self.1,
                self.2,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> HasElement<4>
for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type Value = E;
    type Other = (A, B, C, D, F, G, H, I, J, K, L);
    fn get(&self) -> &Self::Value {
        &self.4
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.4,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> HasElement<5>
for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type Value = F;
    type Other = (A, B, C, D, E, G, H, I, J, K, L);
    fn get(&self) -> &Self::Value {
        &self.5
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.5,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> HasElement<6>
for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type Value = G;
    type Other = (A, B, C, D, E, F, H, I, J, K, L);
    fn get(&self) -> &Self::Value {
        &self.6
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.6,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> HasElement<7>
for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type Value = H;
    type Other = (A, B, C, D, E, F, G, I, J, K, L);
    fn get(&self) -> &Self::Value {
        &self.7
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.7,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.8,
                self.9,
                self.10,
                self.11,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> HasElement<8>
for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type Value = I;
    type Other = (A, B, C, D, E, F, G, H, J, K, L);
    fn get(&self) -> &Self::Value {
        &self.8
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.8,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.9,
                self.10,
                self.11,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> HasElement<9>
for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type Value = J;
    type Other = (A, B, C, D, E, F, G, H, I, K, L);
    fn get(&self) -> &Self::Value {
        &self.9
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.9,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.10,
                self.11,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> HasElement<10>
for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type Value = K;
    type Other = (A, B, C, D, E, F, G, H, I, J, L);
    fn get(&self) -> &Self::Value {
        &self.10
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.10,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.11,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> HasElement<11>
for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type Value = L;
    type Other = (A, B, C, D, E, F, G, H, I, J, K);
    fn get(&self) -> &Self::Value {
        &self.11
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.11,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> TypeList
for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type Head = A;
    type Tail = (B, C, D, E, F, G, H, I, J, K, L, M);
    type First = A;
    type Last = M;
    type Reverse = (M, L, K, J, I, H, G, F, E, D, C, B, A);
    type PushFront<Pushed> = (Pushed, A, B, C, D, E, F, G, H, I, J, K, L, M);
    type PushBack<Pushed> = (A, B, C, D, E, F, G, H, I, J, K, L, M, Pushed);
    type Variant = Tuple13<A, B, C, D, E, F, G, H, I, J, K, L, M>;
    type Variants = [Self::Variant; 13];
    const LEN: usize = 13;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7,
            self.8,
            self.9,
            self.10,
            self.11,
            self.12,
            value,
        )
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (
            value,
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7,
            self.8,
            self.9,
            self.10,
            self.11,
            self.12,
        )
    }
    fn reverse(self) -> Self::Reverse {
        (
            self.12,
            self.11,
            self.10,
            self.9,
            self.8,
            self.7,
            self.6,
            self.5,
            self.4,
            self.3,
            self.2,
            self.1,
            self.0,
        )
    }
    fn into_variants(self) -> Self::Variants {
        [
            Tuple13::Variant0(self.0),
            Tuple13::Variant1(self.1),
            Tuple13::Variant2(self.2),
            Tuple13::Variant3(self.3),
            Tuple13::Variant4(self.4),
            Tuple13::Variant5(self.5),
            Tuple13::Variant6(self.6),
            Tuple13::Variant7(self.7),
            Tuple13::Variant8(self.8),
            Tuple13::Variant9(self.9),
            Tuple13::Variant10(self.10),
            Tuple13::Variant11(self.11),
            Tuple13::Variant12(self.12),
        ]
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> NonEmpty
for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type LTail = (A, B, C, D, E, F, G, H, I, J, K, L);
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.12
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (
            self.12,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
            ),
        )
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (
            self.0,
            (
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> HasElement<0>
for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type Value = A;
    type Other = (B, C, D, E, F, G, H, I, J, K, L, M);
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.0,
            (
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> HasElement<1>
for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type Value = B;
    type Other = (A, C, D, E, F, G, H, I, J, K, L, M);
    fn get(&self) -> &Self::Value {
        &self.1
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.1,
            (
                self.0,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> HasElement<2>
for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type Value = C;
    type Other = (A, B, D, E, F, G, H, I, J, K, L, M);
    fn get(&self) -> &Self::Value {
        &self.2
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.2,
            (
                self.0,
                self.1,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> HasElement<3>
for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type Value = D;
    type Other = (A, B, C, E, F, G, H, I, J, K, L, M);
    fn get(&self) -> &Self::Value {
        &self.3
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.3,
            (
                self.0,
                self.1,
                self.2,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> HasElement<4>
for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type Value = E;
    type Other = (A, B, C, D, F, G, H, I, J, K, L, M);
    fn get(&self) -> &Self::Value {
        &self.4
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.4,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> HasElement<5>
for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type Value = F;
    type Other = (A, B, C, D, E, G, H, I, J, K, L, M);
    fn get(&self) -> &Self::Value {
        &self.5
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.5,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> HasElement<6>
for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type Value = G;
    type Other = (A, B, C, D, E, F, H, I, J, K, L, M);
    fn get(&self) -> &Self::Value {
        &self.6
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.6,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> HasElement<7>
for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type Value = H;
    type Other = (A, B, C, D, E, F, G, I, J, K, L, M);
    fn get(&self) -> &Self::Value {
        &self.7
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.7,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> HasElement<8>
for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type Value = I;
    type Other = (A, B, C, D, E, F, G, H, J, K, L, M);
    fn get(&self) -> &Self::Value {
        &self.8
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.8,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.9,
                self.10,
                self.11,
                self.12,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> HasElement<9>
for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type Value = J;
    type Other = (A, B, C, D, E, F, G, H, I, K, L, M);
    fn get(&self) -> &Self::Value {
        &self.9
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.9,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.10,
                self.11,
                self.12,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> HasElement<10>
for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type Value = K;
    type Other = (A, B, C, D, E, F, G, H, I, J, L, M);
    fn get(&self) -> &Self::Value {
        &self.10
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.10,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.11,
                self.12,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> HasElement<11>
for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type Value = L;
    type Other = (A, B, C, D, E, F, G, H, I, J, K, M);
    fn get(&self) -> &Self::Value {
        &self.11
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.11,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.12,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> HasElement<12>
for (A, B, C, D, E, F, G, H, I, J, K, L, M) {
    type Value = M;
    type Other = (A, B, C, D, E, F, G, H, I, J, K, L);
    fn get(&self) -> &Self::Value {
        &self.12
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.12,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> TypeList
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type Head = A;
    type Tail = (B, C, D, E, F, G, H, I, J, K, L, M, N);
    type First = A;
    type Last = N;
    type Reverse = (N, M, L, K, J, I, H, G, F, E, D, C, B, A);
    type PushFront<Pushed> = (Pushed, A, B, C, D, E, F, G, H, I, J, K, L, M, N);
    type PushBack<Pushed> = (A, B, C, D, E, F, G, H, I, J, K, L, M, N, Pushed);
    type Variant = Tuple14<A, B, C, D, E, F, G, H, I, J, K, L, M, N>;
    type Variants = [Self::Variant; 14];
    const LEN: usize = 14;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7,
            self.8,
            self.9,
            self.10,
            self.11,
            self.12,
            self.13,
            value,
        )
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (
            value,
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7,
            self.8,
            self.9,
            self.10,
            self.11,
            self.12,
            self.13,
        )
    }
    fn reverse(self) -> Self::Reverse {
        (
            self.13,
            self.12,
            self.11,
            self.10,
            self.9,
            self.8,
            self.7,
            self.6,
            self.5,
            self.4,
            self.3,
            self.2,
            self.1,
            self.0,
        )
    }
    fn into_variants(self) -> Self::Variants {
        [
            Tuple14::Variant0(self.0),
            Tuple14::Variant1(self.1),
            Tuple14::Variant2(self.2),
            Tuple14::Variant3(self.3),
            Tuple14::Variant4(self.4),
            Tuple14::Variant5(self.5),
            Tuple14::Variant6(self.6),
            Tuple14::Variant7(self.7),
            Tuple14::Variant8(self.8),
            Tuple14::Variant9(self.9),
            Tuple14::Variant10(self.10),
            Tuple14::Variant11(self.11),
            Tuple14::Variant12(self.12),
            Tuple14::Variant13(self.13),
        ]
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> NonEmpty
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type LTail = (A, B, C, D, E, F, G, H, I, J, K, L, M);
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.13
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (
            self.13,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
            ),
        )
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (
            self.0,
            (
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> HasElement<0>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type Value = A;
    type Other = (B, C, D, E, F, G, H, I, J, K, L, M, N);
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.0,
            (
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> HasElement<1>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type Value = B;
    type Other = (A, C, D, E, F, G, H, I, J, K, L, M, N);
    fn get(&self) -> &Self::Value {
        &self.1
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.1,
            (
                self.0,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> HasElement<2>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type Value = C;
    type Other = (A, B, D, E, F, G, H, I, J, K, L, M, N);
    fn get(&self) -> &Self::Value {
        &self.2
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.2,
            (
                self.0,
                self.1,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> HasElement<3>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type Value = D;
    type Other = (A, B, C, E, F, G, H, I, J, K, L, M, N);
    fn get(&self) -> &Self::Value {
        &self.3
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.3,
            (
                self.0,
                self.1,
                self.2,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> HasElement<4>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type Value = E;
    type Other = (A, B, C, D, F, G, H, I, J, K, L, M, N);
    fn get(&self) -> &Self::Value {
        &self.4
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.4,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> HasElement<5>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type Value = F;
    type Other = (A, B, C, D, E, G, H, I, J, K, L, M, N);
    fn get(&self) -> &Self::Value {
        &self.5
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.5,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> HasElement<6>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type Value = G;
    type Other = (A, B, C, D, E, F, H, I, J, K, L, M, N);
    fn get(&self) -> &Self::Value {
        &self.6
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.6,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> HasElement<7>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type Value = H;
    type Other = (A, B, C, D, E, F, G, I, J, K, L, M, N);
    fn get(&self) -> &Self::Value {
        &self.7
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.7,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> HasElement<8>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type Value = I;
    type Other = (A, B, C, D, E, F, G, H, J, K, L, M, N);
    fn get(&self) -> &Self::Value {
        &self.8
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.8,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> HasElement<9>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type Value = J;
    type Other = (A, B, C, D, E, F, G, H, I, K, L, M, N);
    fn get(&self) -> &Self::Value {
        &self.9
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.9,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.10,
                self.11,
                self.12,
                self.13,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> HasElement<10>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type Value = K;
    type Other = (A, B, C, D, E, F, G, H, I, J, L, M, N);
    fn get(&self) -> &Self::Value {
        &self.10
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.10,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.11,
                self.12,
                self.13,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> HasElement<11>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type Value = L;
    type Other = (A, B, C, D, E, F, G, H, I, J, K, M, N);
    fn get(&self) -> &Self::Value {
        &self.11
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.11,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.12,
                self.13,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> HasElement<12>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type Value = M;
    type Other = (A, B, C, D, E, F, G, H, I, J, K, L, N);
    fn get(&self) -> &Self::Value {
        &self.12
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.12,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.13,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> HasElement<13>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
    type Value = N;
    type Other = (A, B, C, D, E, F, G, H, I, J, K, L, M);
    fn get(&self) -> &Self::Value {
        &self.13
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.13,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> TypeList
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Head = A;
    type Tail = (B, C, D, E, F, G, H, I, J, K, L, M, N, O);
    type First = A;
    type Last = O;
    type Reverse = (O, N, M, L, K, J, I, H, G, F, E, D, C, B, A);
    type PushFront<Pushed> = (Pushed, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
    type PushBack<Pushed> = (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, Pushed);
    type Variant = Tuple15<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O>;
    type Variants = [Self::Variant; 15];
    const LEN: usize = 15;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7,
            self.8,
            self.9,
            self.10,
            self.11,
            self.12,
            self.13,
            self.14,
            value,
        )
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (
            value,
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7,
            self.8,
            self.9,
            self.10,
            self.11,
            self.12,
            self.13,
            self.14,
        )
    }
    fn reverse(self) -> Self::Reverse {
        (
            self.14,
            self.13,
            self.12,
            self.11,
            self.10,
            self.9,
            self.8,
            self.7,
            self.6,
            self.5,
            self.4,
            self.3,
            self.2,
            self.1,
            self.0,
        )
    }
    fn into_variants(self) -> Self::Variants {
        [
            Tuple15::Variant0(self.0),
            Tuple15::Variant1(self.1),
            Tuple15::Variant2(self.2),
            Tuple15::Variant3(self.3),
            Tuple15::Variant4(self.4),
            Tuple15::Variant5(self.5),
            Tuple15::Variant6(self.6),
            Tuple15::Variant7(self.7),
            Tuple15::Variant8(self.8),
            Tuple15::Variant9(self.9),
            Tuple15::Variant10(self.10),
            Tuple15::Variant11(self.11),
            Tuple15::Variant12(self.12),
            Tuple15::Variant13(self.13),
            Tuple15::Variant14(self.14),
        ]
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> NonEmpty
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type LTail = (A, B, C, D, E, F, G, H, I, J, K, L, M, N);
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.14
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (
            self.14,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
            ),
        )
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (
            self.0,
            (
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> HasElement<0>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Value = A;
    type Other = (B, C, D, E, F, G, H, I, J, K, L, M, N, O);
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.0,
            (
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> HasElement<1>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Value = B;
    type Other = (A, C, D, E, F, G, H, I, J, K, L, M, N, O);
    fn get(&self) -> &Self::Value {
        &self.1
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.1,
            (
                self.0,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> HasElement<2>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Value = C;
    type Other = (A, B, D, E, F, G, H, I, J, K, L, M, N, O);
    fn get(&self) -> &Self::Value {
        &self.2
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.2,
            (
                self.0,
                self.1,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> HasElement<3>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Value = D;
    type Other = (A, B, C, E, F, G, H, I, J, K, L, M, N, O);
    fn get(&self) -> &Self::Value {
        &self.3
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.3,
            (
                self.0,
                self.1,
                self.2,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> HasElement<4>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Value = E;
    type Other = (A, B, C, D, F, G, H, I, J, K, L, M, N, O);
    fn get(&self) -> &Self::Value {
        &self.4
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.4,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> HasElement<5>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Value = F;
    type Other = (A, B, C, D, E, G, H, I, J, K, L, M, N, O);
    fn get(&self) -> &Self::Value {
        &self.5
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.5,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> HasElement<6>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Value = G;
    type Other = (A, B, C, D, E, F, H, I, J, K, L, M, N, O);
    fn get(&self) -> &Self::Value {
        &self.6
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.6,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> HasElement<7>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Value = H;
    type Other = (A, B, C, D, E, F, G, I, J, K, L, M, N, O);
    fn get(&self) -> &Self::Value {
        &self.7
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.7,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> HasElement<8>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Value = I;
    type Other = (A, B, C, D, E, F, G, H, J, K, L, M, N, O);
    fn get(&self) -> &Self::Value {
        &self.8
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.8,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> HasElement<9>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Value = J;
    type Other = (A, B, C, D, E, F, G, H, I, K, L, M, N, O);
    fn get(&self) -> &Self::Value {
        &self.9
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.9,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> HasElement<10>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Value = K;
    type Other = (A, B, C, D, E, F, G, H, I, J, L, M, N, O);
    fn get(&self) -> &Self::Value {
        &self.10
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.10,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.11,
                self.12,
                self.13,
                self.14,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> HasElement<11>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Value = L;
    type Other = (A, B, C, D, E, F, G, H, I, J, K, M, N, O);
    fn get(&self) -> &Self::Value {
        &self.11
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.11,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.12,
                self.13,
                self.14,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> HasElement<12>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Value = M;
    type Other = (A, B, C, D, E, F, G, H, I, J, K, L, N, O);
    fn get(&self) -> &Self::Value {
        &self.12
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.12,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.13,
                self.14,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> HasElement<13>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Value = N;
    type Other = (A, B, C, D, E, F, G, H, I, J, K, L, M, O);
    fn get(&self) -> &Self::Value {
        &self.13
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.13,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.14,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> HasElement<14>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
    type Value = O;
    type Other = (A, B, C, D, E, F, G, H, I, J, K, L, M, N);
    fn get(&self) -> &Self::Value {
        &self.14
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.14,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> TypeList
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Head = A;
    type Tail = (B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
    type First = A;
    type Last = P;
    type Reverse = (P, O, N, M, L, K, J, I, H, G, F, E, D, C, B, A);
    type PushFront<Pushed> = (Pushed, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
    type PushBack<Pushed> = (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Pushed);
    type Variant = Tuple16<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P>;
    type Variants = [Self::Variant; 16];
    const LEN: usize = 16;
    const IS_EMPTY: bool = false;
    fn push_back<Parsed>(self, value: Parsed) -> Self::PushBack<Parsed> {
        (
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7,
            self.8,
            self.9,
            self.10,
            self.11,
            self.12,
            self.13,
            self.14,
            self.15,
            value,
        )
    }
    fn push_front<Parsed>(self, value: Parsed) -> Self::PushFront<Parsed> {
        (
            value,
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7,
            self.8,
            self.9,
            self.10,
            self.11,
            self.12,
            self.13,
            self.14,
            self.15,
        )
    }
    fn reverse(self) -> Self::Reverse {
        (
            self.15,
            self.14,
            self.13,
            self.12,
            self.11,
            self.10,
            self.9,
            self.8,
            self.7,
            self.6,
            self.5,
            self.4,
            self.3,
            self.2,
            self.1,
            self.0,
        )
    }
    fn into_variants(self) -> Self::Variants {
        [
            Tuple16::Variant0(self.0),
            Tuple16::Variant1(self.1),
            Tuple16::Variant2(self.2),
            Tuple16::Variant3(self.3),
            Tuple16::Variant4(self.4),
            Tuple16::Variant5(self.5),
            Tuple16::Variant6(self.6),
            Tuple16::Variant7(self.7),
            Tuple16::Variant8(self.8),
            Tuple16::Variant9(self.9),
            Tuple16::Variant10(self.10),
            Tuple16::Variant11(self.11),
            Tuple16::Variant12(self.12),
            Tuple16::Variant13(self.13),
            Tuple16::Variant14(self.14),
            Tuple16::Variant15(self.15),
        ]
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> NonEmpty
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type LTail = (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
    fn first(&self) -> &Self::First {
        &self.0
    }
    fn last(&self) -> &Self::Last {
        &self.15
    }
    fn pop_back(self) -> (Self::Last, Self::LTail) {
        (
            self.15,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
            ),
        )
    }
    fn pop_front(self) -> (Self::Head, Self::Tail) {
        (
            self.0,
            (
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<0>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = A;
    type Other = (B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
    fn get(&self) -> &Self::Value {
        &self.0
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.0,
            (
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<1>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = B;
    type Other = (A, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
    fn get(&self) -> &Self::Value {
        &self.1
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.1,
            (
                self.0,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<2>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = C;
    type Other = (A, B, D, E, F, G, H, I, J, K, L, M, N, O, P);
    fn get(&self) -> &Self::Value {
        &self.2
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.2,
            (
                self.0,
                self.1,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<3>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = D;
    type Other = (A, B, C, E, F, G, H, I, J, K, L, M, N, O, P);
    fn get(&self) -> &Self::Value {
        &self.3
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.3,
            (
                self.0,
                self.1,
                self.2,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<4>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = E;
    type Other = (A, B, C, D, F, G, H, I, J, K, L, M, N, O, P);
    fn get(&self) -> &Self::Value {
        &self.4
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.4,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<5>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = F;
    type Other = (A, B, C, D, E, G, H, I, J, K, L, M, N, O, P);
    fn get(&self) -> &Self::Value {
        &self.5
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.5,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<6>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = G;
    type Other = (A, B, C, D, E, F, H, I, J, K, L, M, N, O, P);
    fn get(&self) -> &Self::Value {
        &self.6
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.6,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<7>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = H;
    type Other = (A, B, C, D, E, F, G, I, J, K, L, M, N, O, P);
    fn get(&self) -> &Self::Value {
        &self.7
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.7,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<8>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = I;
    type Other = (A, B, C, D, E, F, G, H, J, K, L, M, N, O, P);
    fn get(&self) -> &Self::Value {
        &self.8
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.8,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<9>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = J;
    type Other = (A, B, C, D, E, F, G, H, I, K, L, M, N, O, P);
    fn get(&self) -> &Self::Value {
        &self.9
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.9,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<10>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = K;
    type Other = (A, B, C, D, E, F, G, H, I, J, L, M, N, O, P);
    fn get(&self) -> &Self::Value {
        &self.10
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.10,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.11,
                self.12,
                self.13,
                self.14,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<11>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = L;
    type Other = (A, B, C, D, E, F, G, H, I, J, K, M, N, O, P);
    fn get(&self) -> &Self::Value {
        &self.11
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.11,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.12,
                self.13,
                self.14,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<12>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = M;
    type Other = (A, B, C, D, E, F, G, H, I, J, K, L, N, O, P);
    fn get(&self) -> &Self::Value {
        &self.12
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.12,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.13,
                self.14,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<13>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = N;
    type Other = (A, B, C, D, E, F, G, H, I, J, K, L, M, O, P);
    fn get(&self) -> &Self::Value {
        &self.13
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.13,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.14,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<14>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = O;
    type Other = (A, B, C, D, E, F, G, H, I, J, K, L, M, N, P);
    fn get(&self) -> &Self::Value {
        &self.14
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.14,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.15,
            ),
        )
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> HasElement<15>
for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
    type Value = P;
    type Other = (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
    fn get(&self) -> &Self::Value {
        &self.15
    }
    fn remove(self) -> (Self::Value, Self::Other) {
        (
            self.15,
            (
                self.0,
                self.1,
                self.2,
                self.3,
                self.4,
                self.5,
                self.6,
                self.7,
                self.8,
                self.9,
                self.10,
                self.11,
                self.12,
                self.13,
                self.14,
            ),
        )
    }
}
