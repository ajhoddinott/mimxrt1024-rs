# mimxrt1024

This crate provides a peripheral access API for NXP i.MX RT1024 series microcontrollers.

The API is generated by svd2rust from the manufacturer SVD file MIMXRT1024.xml, modified to eliminate duplicate TRISTATE and ENABLED enum variants.