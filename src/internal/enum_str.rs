macro_rules! impl_enum_str {
    ($visibility:vis $type_name:ident,
    parse_error = $err:ident,
    values = [ $(
      $(#[$docs:meta])*
      ($enum_name:ident, $display:literal)
    ),+ ]) => {

      #[derive(Clone, Copy, PartialEq, Eq, Debug)]
      #[non_exhaustive]
      $visibility enum $type_name {
        $(
          $(#[$docs])*
          $enum_name
        ),+
      }

      impl $type_name {
        pub const fn as_str(&self) -> &'static str {
          match self {
            $(
              Self::$enum_name => $display,
            )+
          }
        }
      }

      impl core::str::FromStr for $type_name {
        type Err = $err;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
          const TABLE: &[(&'static str, $type_name)] = &[$(($display, $type_name::$enum_name)),+];

          for (name, value) in TABLE {
            if name.eq_ignore_ascii_case(s) {
              return Ok(*value)
            }
          }

          Err($err)
        }
      }

      impl core::fmt::Display for $type_name {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          f.write_str(self.as_str())
        }
      }
    };
}

pub(crate) use impl_enum_str;
