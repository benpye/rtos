#[macro_export]
macro_rules! rpc_interface {
    (@num_padding_bytes $val:ty) => {
        {
            const REMAINDER_BYTES: usize = ::core::mem::size_of::<$val>() % ::core::mem::size_of::<usize>();
            if (REMAINDER_BYTES == 0) {
                0
            } else {
                ::core::mem::size_of::<usize>() - REMAINDER_BYTES
            }
        }
    };

    (@impl_struct_len $struct_type:ty) => {
        const _: () = assert!(0 == ::core::mem::size_of::<$struct_type>() % ::core::mem::size_of::<usize>());

        impl $struct_type {
            pub const LEN: usize = ::core::mem::size_of::<Self>() / ::core::mem::size_of::<usize>();

            pub const fn into_data(self) -> [usize; Self::LEN] {
                zerocopy::transmute!(self)
            }
        }
    };

    (@in_struct $fn_name:ident $($(#[padding])? $arg_name:ident : $arg_type:ty,)* $(,)*) => {
        #[repr(C)]
        #[derive(zerocopy::AsBytes, zerocopy::FromBytes, zerocopy::FromZeroes)]
        pub struct Input {
            $(pub $arg_name: $arg_type),*
        }

        #[repr(C)]
        #[derive(zerocopy::AsBytes, zerocopy::FromBytes, zerocopy::FromZeroes)]
        pub struct InputOuter {
            pub header: super::InputHeader,
            pub value: Input,
            pub _pad: [u8; Self::NUM_PADDING_BYTES],
        }

        impl InputOuter {
            pub const NUM_PADDING_BYTES: usize = $crate::rpc_interface!(@num_padding_bytes Input);
        }

        $crate::rpc_interface! { @impl_struct_len InputOuter }
    };

    (@out_struct $fn_name:ident $ret:ty) => {
        #[repr(transparent)]
        #[derive(zerocopy::AsBytes, zerocopy::FromBytes, zerocopy::FromZeroes)]
        pub struct Output(pub $ret);

        #[repr(C)]
        #[derive(zerocopy::AsBytes, zerocopy::FromBytes, zerocopy::FromZeroes)]
        pub struct OutputOuter {
            pub header: super::OutputHeader,
            pub value: $ret,
            pub _pad: [u8; Self::NUM_PADDING_BYTES],
        }

        impl OutputOuter {
            pub const NUM_PADDING_BYTES: usize = $crate::rpc_interface!(@num_padding_bytes $ret);
        }

        $crate::rpc_interface! { @impl_struct_len OutputOuter }
    };

    (@trait_fn $(#[$inner:meta])* fn $fn_name:ident( $( $(#[padding] $pad_name:ident : $pad_type:ty,)* $arg_name:ident : $arg_type:ty, )* $(,)*) -> $ret:ty) => {
        $(#[$inner])*
        fn $fn_name(&mut self, $($arg_name: $arg_type),*) -> Result<$ret, T>;
    };

    (@client_fn $fn_name:ident( $( $(#[padding] $pad_name:ident : $pad_type:ty,)* $arg_name:ident : $arg_type:ty, )* $(,)*) -> $ret:ty) => {
        fn $fn_name(&mut self, $($arg_name: $arg_type),*) -> Result<$ret, $crate::CallStatus> {
            use zerocopy::{AsBytes, FromZeroes};

            const IN_LEN: usize = $fn_name::InputOuter::LEN;
            const OUT_LEN: usize = $fn_name::OutputOuter::LEN;

            let in_message = $fn_name::InputOuter {
                header: InputHeader::new(CallCode::$fn_name),
                value: $fn_name::Input {
                    $($arg_name,)*
                    ..$fn_name::Input::new_zeroed()
                },
                _pad: [0; $fn_name::InputOuter::NUM_PADDING_BYTES],
            };

            let $crate::macro_util::syscall::CallResult { data, len: out_len } = $crate::macro_util::syscall::sys_call::<IN_LEN, OUT_LEN>(self.task_id(), IN_LEN as u8, in_message.into_data());
            let out_len = out_len as usize;

            if (out_len < OutputHeader::LEN) {
                return Err($crate::CallStatus::InvalidOutput);
            }

            let (out_header, out_message) = zerocopy::Ref::<_, OutputHeader>::new_from_prefix(data.as_bytes()).unwrap();
            if (out_header.status != $crate::CallStatus::Success) {
                return Err(out_header.status);
            }

            if (out_len != OUT_LEN) {
                return Err($crate::CallStatus::InvalidOutput);
            }

            let (out_message, _) = zerocopy::Ref::<_, $fn_name::Output>::new_from_prefix(out_message).unwrap();
            Ok(out_message.0)
        }
    };

    (@server_fn_call $in_message:ident $self:ident.$fn_name:ident( $( $(#[padding] $pad_name:ident : $pad_type:ty,)* $arg_name:ident : $arg_type:ty, )* $(,)? )) => {
        $self.$fn_name($($in_message.$arg_name),*)
    };

    (@max_call_code $head:ident) => { CallCode::$head };
    (@max_call_code $head:ident, $($tail:ident),+) => { $crate::rpc_interface!(@max_call_code $($tail),+) };

    (
        $(#[$outer:meta])*
        $v:vis trait $iface_name:ident {
            $(
                $(#[$inner:meta])*
                fn $fn_name:ident($($args:tt)*) -> $ret:ty;
            )+
        }
    ) => {
        const _: () = {
            mod internal {
                use $crate::macro_util::zerocopy;

                #[$crate::macro_util::open_enum::open_enum]
                #[repr(u8)]
                #[derive(zerocopy::AsBytes, zerocopy::FromBytes, zerocopy::FromZeroes)]
                enum CallCode {
                    $($fn_name),+
                }

                const MAX_CALL_CODE: CallCode = $crate::rpc_interface!(@max_call_code $($fn_name),+);
                const NUM_CALLS: usize = (MAX_CALL_CODE.0 as usize) + 1;

                $(mod $fn_name {
                    use $crate::macro_util::zerocopy;

                    $crate::rpc_interface! { @out_struct $fn_name $ret }
                    $crate::rpc_interface! { @in_struct $fn_name $($args)* , }
                })+

                #[repr(C)]
                #[derive(zerocopy::AsBytes, zerocopy::FromBytes, zerocopy::FromZeroes)]
                struct InputHeader {
                    pub code: CallCode,
                    _pad: [u8; 3],
                }

                impl InputHeader {
                    pub const fn new(code: CallCode) -> Self {
                        Self {
                            code,
                            _pad: [0; 3],
                        }
                    }
                }

                $crate::rpc_interface! { @impl_struct_len InputHeader }

                #[repr(C)]
                #[derive(zerocopy::AsBytes, zerocopy::FromBytes, zerocopy::FromZeroes)]
                struct OutputHeader {
                    pub status: $crate::CallStatus,
                    _pad: [u8; 3],
                }

                impl OutputHeader {
                    pub const fn new(status: $crate::CallStatus) -> Self {
                        Self {
                            status,
                            _pad: [0; 3],
                        }
                    }
                }

                $crate::rpc_interface! { @impl_struct_len OutputHeader }

                #[repr(C)]
                union InputUnion {
                    $(
                        $fn_name: ::core::mem::ManuallyDrop<$fn_name::InputOuter>
                    ),*
                }

                const MAX_INPUT_LEN: usize = ::core::mem::size_of::<InputUnion>() / ::core::mem::size_of::<usize>();

                impl<T> super::DispatchImpl for T
                where
                    T : super::$iface_name<$crate::CallStatus>
                {
                    type Data = [usize; MAX_INPUT_LEN];

                    fn dispatch_call_impl(&mut self, sender: u8, data: [usize; MAX_INPUT_LEN], len: usize) {
                        use zerocopy::AsBytes;

                        let (in_header, in_message) = zerocopy::Ref::<_, InputHeader>::new_from_prefix(data.as_bytes()).unwrap();

                        match in_header.code {
                            $(
                                CallCode::$fn_name => {
                                    if (len == $fn_name::InputOuter::LEN) {
                                        #[allow(unused_variables)]
                                        let (in_message, _) = zerocopy::Ref::<_, $fn_name::Input>::new_from_prefix(in_message).unwrap();
                                        match $crate::rpc_interface!(@server_fn_call in_message self.$fn_name( $($args)* , )) {
                                            Ok(value) => {
                                                let out_message = $fn_name::OutputOuter {
                                                    header: OutputHeader::new($crate::CallStatus::Success),
                                                    value,
                                                    _pad: [0; $fn_name::OutputOuter::NUM_PADDING_BYTES],
                                                };
                                                $crate::macro_util::syscall::sys_send(sender, $fn_name::OutputOuter::LEN as u8, out_message.into_data());
                                            },
                                            Err(status) => {
                                                let out_header = OutputHeader::new(status);
                                                $crate::macro_util::syscall::sys_send(sender, OutputHeader::LEN as u8, out_header.into_data());
                                            }
                                        }
                                    } else {
                                        let out_header = OutputHeader::new($crate::CallStatus::InvalidInput);
                                        $crate::macro_util::syscall::sys_send(sender, OutputHeader::LEN as u8, out_header.into_data());
                                    }
                                }
                            ),*
                            _ => {
                                let out_header = OutputHeader::new($crate::CallStatus::InvalidCallCode);
                                $crate::macro_util::syscall::sys_send(sender, OutputHeader::LEN as u8, out_header.into_data());
                            },
                        }
                    }
                }

                impl super::$iface_name<$crate::CallStatus> for super::Client {
                    $($crate::rpc_interface! { @client_fn $fn_name( $($args)* , ) -> $ret } )+
                }
            }
        };

        #[doc(hidden)]
        $v trait DispatchImpl {
            type Data;

            fn dispatch_call_impl(
                &mut self,
                sender: u8,
                data: Self::Data,
                len: usize,
            );
        }

        #[doc(hidden)]
        pub enum Interface { }
        pub type Client = $crate::Client<Interface>;

        $(#[$outer])*
        $v trait $iface_name<T> {
            $(
                $crate::rpc_interface! { @trait_fn $(#[$inner])* fn $fn_name( $($args)* , ) -> $ret }
            )+
        }

        pub type CallStatus = $crate::CallStatus;
    };
}

#[macro_export]
macro_rules! rpc_impl_dispatch_for {
    ($impl_type:ty as $dispatch_impl:ty) => {
        impl
            $crate::Dispatch<
                {
                    ::core::mem::size_of::<<$impl_type as $dispatch_impl>::Data>()
                        / ::core::mem::size_of::<usize>()
                },
            > for $impl_type
        {
            #[inline(always)]
            fn dispatch_call(
                &mut self,
                sender: u8,
                data: <$impl_type as $dispatch_impl>::Data,
                len: usize,
            ) {
                <$impl_type as $dispatch_impl>::dispatch_call_impl(self, sender, data, len)
            }
        }
    };
}
