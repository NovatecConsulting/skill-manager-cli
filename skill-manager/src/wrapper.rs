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

            impl std::str::FromStr for $name {
                type Err = anyhow::Error;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    let inner = s.parse::<$type>()?;
                    Ok(Self(inner))
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
        )+
    };
}
