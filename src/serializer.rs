use crate::parser::*;
use bytes::{BytesMut, BufMut};


/// Serialization of a SomeIP header to bytes
fn serialize_someip_header(header: &SomeIpHeader) -> Vec<u8> {
    let mut buf = BytesMut::with_capacity(144);
    buf.put_u16(header.message_id.service_id.into());
    buf.put_u16(header.message_id.method_id.into());
    buf.put_u32(header.length);
    buf.put_u16(header.request_id.client_id.into());
    buf.put_u16(header.request_id.session_id.into());
    buf.put_u8(header.protocol_version);
    buf.put_u8(header.interface_version);
    buf.put_u8(header.message_type.into());
    buf.put_u8(header.return_code.into());

    buf.to_vec()
}

/// Serialization of a SomeIP message to bytes
pub fn serialize_someip(package: &SomeIp) -> Vec<u8> {
    match package {
        SomeIp::SomeIpMessage(p)=> {
            let mut buf = serialize_someip_header(&p.header);
            buf.put(p.payload);
            buf
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{MessageID, MethodID, ServiceID, RequestID, MessageType, ReturnCode};


    #[test]
    fn check_someip_header_serializer() {
        // SOME/IP Protocol (Service ID: 0x3085, Method ID: 0x0001, Length: 8)
        //     Service ID: 0x3085
        //     Method ID: 0x0001
        //     Length: 8
        //     Client ID: 0x0000
        //     Session ID: 0x0000
        //     SOME/IP Version: 0x01
        //     Interface Version: 0x01
        //     Message Type: 0x00 (Request)
        //     Return Code: 0x00 (Ok)
        //     Payload: NONE

        let bytes =
            hex::decode("30850001000000080000000001010000").expect("invalid hex string");
        // let bytes_slice = bytes.as_slice();
        // let payload: &[u8] = &[0x01, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(
            bytes,
            serialize_someip_header(
                &SomeIpHeader {
                    message_id: MessageID {
                        method_id: MethodID(0x0001),
                        service_id: ServiceID(0x3085)
                    },
                    length: 8,
                    request_id: RequestID {
                        client_id: ClientID(0x0000),
                        session_id: SessionID(0x0000)
                    },
                    protocol_version: 0x01,
                    interface_version: 0x01,
                    message_type: MessageType::Request,
                    return_code: ReturnCode::EOk,
                }
            )
        )
    }
    #[test]
    fn check_someip_package_serializer() {
        // SOME/IP Protocol (Service ID: 0x0103, Method ID: 0x8005, Length: 13)
        //     Service ID: 0x0103
        //     Method ID: 0x8005
        //     Length: 13
        //     Client ID: 0x0000
        //     Session ID: 0x0000
        //     SOME/IP Version: 0x01
        //     Interface Version: 0x01
        //     Message Type: 0x02 (Notification)
        //         .0.. .... = Message Type Ack Flag: False
        //         ..0. .... = Message Type TP Flag: False
        //     Return Code: 0x00 (Ok)
        //     Payload: 0100000000
        let bytes =
            hex::decode("010380050000000d00000000010102000100000000").expect("invalid hex string");
        // let bytes_slice = bytes.as_slice();
        let payload: &[u8] = &[0x01, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(
            bytes,
            serialize_someip(
                &SomeIp::SomeIpMessage(
                    SomeIpMessage {
                        header:
                            SomeIpHeader {
                            message_id: MessageID {
                                method_id: MethodID(0x8005),
                                service_id: ServiceID(0x0103)
                            },
                            length: 13,
                            request_id: RequestID {
                                client_id: ClientID(0x0000),
                                session_id: SessionID(0x0000)
                            },
                            protocol_version: 0x01,
                            interface_version: 0x01,
                            message_type: MessageType::Notification,
                            return_code: ReturnCode::EOk,
                        },
                    payload
                    }
                )
            )
        )
    }

}