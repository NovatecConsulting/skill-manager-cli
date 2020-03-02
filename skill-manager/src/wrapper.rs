macro_rules! gen_wrapper {
    ( $( $name:ident ),+ ) => {
        gen_wrapper!(
            $(
                $name: String [Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash]
            ),+
            );
    };
    ( $( $name:ident: $type:ty ),+ ) => {
        gen_wrapper!(
            $(
                $name: $type [Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash]
            ),+
            );
    };
    ( $( $name:ident: $type:ty [$( $derive:ident ), +]),+ ) => {
        $(
            #[derive( $( $derive, )+ )]
            pub struct $name($type);
        )+
    };
}
