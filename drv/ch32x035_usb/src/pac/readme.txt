This is a private subset of the CH32X035 PAC with a USB device only version of the USBFS peripheral.

The code can be regenerated with svd2rust:

svd2rust -g -m -s --pascal_enum_values --keep_list --target none -i ch32x035_usbfs.svd

- svd2rust will generate some device code - this should be removed.
- Find and replace 'crate::' with 'crate::pac::generic::'.
