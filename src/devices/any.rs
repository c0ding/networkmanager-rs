use super::{EthernetDevice, WiFiDevice};
use crate::config::Config;
use crate::connection::Connection;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDevice;
use crate::types::{Capability, ConnectivityState, DeviceInterfaceFlag, DeviceType};
use num_traits::FromPrimitive;

pub trait Any {
    fn reapply(
        &self,
        connection: ::std::collections::HashMap<
            &str,
            ::std::collections::HashMap<&str, dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>>,
        >,
        version_id: u64,
        flags: u32,
    ) -> Result<(), Error>;
    fn get_applied_connection(
        &self,
        flags: u32,
    ) -> Result<
        (
            ::std::collections::HashMap<
                String,
                ::std::collections::HashMap<
                    String,
                    dbus::arg::Variant<Box<dyn dbus::arg::RefArg + 'static>>,
                >,
            >,
            u64,
        ),
        Error,
    >;
    fn disconnect(&self) -> Result<(), Error>;
    fn delete(&self) -> Result<(), Error>;
    fn udi(&self) -> Result<String, Error>;
    fn interface(&self) -> Result<String, Error>;
    fn ip_interface(&self) -> Result<String, Error>;
    fn driver(&self) -> Result<String, Error>;
    fn driver_version(&self) -> Result<String, Error>;
    fn firmware_version(&self) -> Result<String, Error>;
    fn capabilities(&self) -> Result<Capability, Error>;
    fn ip4_address(&self) -> Result<u32, Error>;
    fn state(&self) -> Result<u32, Error>;
    fn state_reason(&self) -> Result<(u32, u32), Error>;
    fn active_connection(&self) -> Result<Connection, Error>;
    fn ip4_config(&self) -> Result<Config, Error>;
    fn dhcp4_config(&self) -> Result<Config, Error>;
    fn ip6_config(&self) -> Result<Config, Error>;
    fn dhcp6_config(&self) -> Result<Config, Error>;
    fn managed(&self) -> Result<bool, Error>;
    fn set_managed(&self, value: bool) -> Result<(), Error>;
    fn autoconnect(&self) -> Result<bool, Error>;
    fn set_autoconnect(&self, value: bool) -> Result<(), Error>;
    fn firmware_missing(&self) -> Result<bool, Error>;
    fn nm_plugin_missing(&self) -> Result<bool, Error>;
    fn device_type(&self) -> Result<DeviceType, Error>;
    fn available_connections(&self) -> Result<Vec<Connection>, Error>;
    fn physical_port_id(&self) -> Result<String, Error>;
    fn mtu(&self) -> Result<u32, Error>;
    fn metered(&self) -> Result<u32, Error>;
    fn lldp_neighbors(
        &self,
    ) -> Result<
        Vec<
            ::std::collections::HashMap<
                String,
                dbus::arg::Variant<Box<dyn dbus::arg::RefArg + 'static>>,
            >,
        >,
        Error,
    >;
    fn real(&self) -> Result<bool, Error>;
    fn ip4_connectivity(&self) -> Result<ConnectivityState, Error>;
    fn ip6_connectivity(&self) -> Result<ConnectivityState, Error>;
    fn interface_flags(&self) -> Result<DeviceInterfaceFlag, Error>;
    fn hw_address(&self) -> Result<String, Error>;
}

macro_rules! impl_any {
    ($name:ty, $lifetime:tt) => {
        impl<$lifetime> Any for $name {
            fn reapply(
                &self,
                _connection: std::collections::HashMap<
                    &str,
                    std::collections::HashMap<&str, dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>>,
                >,
                _version_id: u64,
                _flags: u32,
            ) -> Result<(), Error> {
                todo!()
            }
            fn get_applied_connection(
                &self,
                _flags: u32,
            ) -> Result<
                (
                    std::collections::HashMap<
                        String,
                        std::collections::HashMap<
                            String,
                            dbus::arg::Variant<Box<dyn dbus::arg::RefArg + 'static>>,
                        >,
                    >,
                    u64,
                ),
                Error,
            > {
                todo!()
            }
            fn disconnect(&self) -> Result<(), Error> {
                Ok(proxy!(self).disconnect()?)
            }
            fn delete(&self) -> Result<(), Error> {
                Ok(proxy!(self).delete()?)
            }
            fn udi(&self) -> Result<String, Error> {
                Ok(proxy!(self).udi()?)
            }
            fn interface(&self) -> Result<String, Error> {
                Ok(proxy!(self).interface()?)
            }
            fn ip_interface(&self) -> Result<String, Error> {
                Ok(proxy!(self).ip_interface()?)
            }
            fn driver(&self) -> Result<String, Error> {
                Ok(proxy!(self).driver()?)
            }
            fn driver_version(&self) -> Result<String, Error> {
                Ok(proxy!(self).driver_version()?)
            }
            fn firmware_version(&self) -> Result<String, Error> {
                Ok(proxy!(self).firmware_version()?)
            }
            fn capabilities(&self) -> Result<Capability, Error> {
                let cap = proxy!(self).capabilities()?;
                match FromPrimitive::from_u32(cap) {
                    Some(x) => Ok(x),
                    None => Err(Error::UnsupportedType),
                }
            }
            fn ip4_address(&self) -> Result<u32, Error> {
                Ok(proxy!(self).ip4_address()?)
            }
            fn state(&self) -> Result<u32, Error> {
                Ok(proxy!(self).state()?)
            }
            fn state_reason(&self) -> Result<(u32, u32), Error> {
                Ok(proxy!(self).state_reason()?)
            }
            fn active_connection(&self) -> Result<Connection, Error> {
                todo!()
            }
            fn ip4_config(&self) -> Result<Config, Error> {
                todo!()
            }
            fn dhcp4_config(&self) -> Result<Config, Error> {
                todo!()
            }
            fn ip6_config(&self) -> Result<Config, Error> {
                todo!()
            }
            fn dhcp6_config(&self) -> Result<Config, Error> {
                todo!()
            }
            fn managed(&self) -> Result<bool, Error> {
                Ok(proxy!(self).managed()?)
            }
            fn set_managed(&self, value: bool) -> Result<(), Error> {
                Ok(proxy!(self).set_managed(value)?)
            }
            fn autoconnect(&self) -> Result<bool, Error> {
                Ok(proxy!(self).autoconnect()?)
            }
            fn set_autoconnect(&self, value: bool) -> Result<(), Error> {
                Ok(proxy!(self).set_autoconnect(value)?)
            }
            fn firmware_missing(&self) -> Result<bool, Error> {
                Ok(proxy!(self).firmware_missing()?)
            }
            fn nm_plugin_missing(&self) -> Result<bool, Error> {
                Ok(proxy!(self).nm_plugin_missing()?)
            }
            fn device_type(&self) -> Result<DeviceType, Error> {
                let dev_type = proxy!(self).device_type()?;
                match FromPrimitive::from_u32(dev_type) {
                    Some(x) => Ok(x),
                    None => Err(Error::UnsupportedType),
                }
            }
            fn available_connections(&self) -> Result<Vec<Connection>, Error> {
                todo!()
            }
            fn physical_port_id(&self) -> Result<String, Error> {
                Ok(proxy!(self).physical_port_id()?)
            }
            fn mtu(&self) -> Result<u32, Error> {
                Ok(proxy!(self).mtu()?)
            }
            fn lldp_neighbors(
                &self,
            ) -> Result<
                Vec<
                    std::collections::HashMap<
                        String,
                        dbus::arg::Variant<Box<dyn dbus::arg::RefArg + 'static>>,
                    >,
                >,
                Error,
            > {
                todo!()
            }
            fn metered(&self) -> Result<u32, Error> {
                Ok(proxy!(self).metered()?)
            }
            fn real(&self) -> Result<bool, Error> {
                Ok(proxy!(self).real()?)
            }
            fn ip4_connectivity(&self) -> Result<ConnectivityState, Error> {
                let con = proxy!(self).ip4_connectivity()?;
                match FromPrimitive::from_u32(con) {
                    Some(x) => Ok(x),
                    None => Err(Error::UnsupportedType),
                }
            }
            fn ip6_connectivity(&self) -> Result<ConnectivityState, Error> {
                let con = proxy!(self).ip6_connectivity()?;
                match FromPrimitive::from_u32(con) {
                    Some(x) => Ok(x),
                    None => Err(Error::UnsupportedType),
                }
            }
            fn interface_flags(&self) -> Result<DeviceInterfaceFlag, Error> {
                let interface_flag = proxy!(self).interface_flags()?;
                match FromPrimitive::from_u32(interface_flag) {
                    Some(x) => Ok(x),
                    None => Err(Error::UnsupportedType),
                }
            }
            fn hw_address(&self) -> Result<String, Error> {
                Ok(proxy!(self).hw_address()?)
            }
        }
    };
}

impl_any!(WiFiDevice<'a>, 'a);
impl_any!(EthernetDevice<'a>, 'a);