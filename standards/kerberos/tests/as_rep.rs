use rasn::prelude::*;
use rasn_kerberos::*;

use pretty_assertions::assert_eq;

const _: () = assert!(AsRep::TAG.const_eq(&Tag::new(Class::Application, 11)));

#[test]
fn as_rep() {
    let as_rep = AsRep(KdcRep {
        pvno: Integer::from(5),
        msg_type: Integer::from(11),
        padata: Some(vec![PaData {
            r#type: 19,
            value: OctetString::from_static(&[
                0x30, 0x1d, 0x30, 0x1b, 0xa0, 0x03, 0x02, 0x01, 0x12, 0xa1, 0x14, 0x1b, 0x12, 0x43,
                0x4f, 0x4d, 0x50, 0x41, 0x4e, 0x59, 0x2e, 0x49, 0x4e, 0x54, 0x75, 0x73, 0x65, 0x72,
            ]),
        }]),
        crealm: KerberosString::try_from("COMPANY.INT".to_string()).unwrap(),
        cname: PrincipalName {
            r#type: 1,
            string: vec![KerberosString::try_from(String::from("user")).unwrap()],
        },
        ticket: Ticket {
            tkt_vno: Integer::from(5),
            realm: KerberosString::try_from("COMPANY.INT".to_string()).unwrap(),
            sname: PrincipalName {
                r#type: 2,
                string: vec![
                    KerberosString::try_from(String::from("krbtgt")).unwrap(),
                    KerberosString::try_from(String::from("COMPANY.INT")).unwrap(),
                ],
            },
            enc_part: EncryptedData {
                etype: 18,
                kvno: Some(2),
                cipher: OctetString::from_static(&[0xde, 0xad, 0xbe, 0xef]),
            },
        },
        enc_part: EncryptedData {
            etype: 18,
            kvno: Some(13),
            cipher: OctetString::from_static(&[0xde, 0xad, 0xbe, 0xef]),
        },
    });
    let data: &[u8] = &[
        0x6b, 0x81, 0xc2, 0x30, 0x81, 0xbf, 0xa0, 0x03, 0x02, 0x01, 0x05, 0xa1, 0x03, 0x02, 0x01,
        0x0b, 0xa2, 0x29, 0x30, 0x27, 0x30, 0x25, 0xa1, 0x03, 0x02, 0x01, 0x13, 0xa2, 0x1e, 0x04,
        0x1c, 0x30, 0x1d, 0x30, 0x1b, 0xa0, 0x03, 0x02, 0x01, 0x12, 0xa1, 0x14, 0x1b, 0x12, 0x43,
        0x4f, 0x4d, 0x50, 0x41, 0x4e, 0x59, 0x2e, 0x49, 0x4e, 0x54, 0x75, 0x73, 0x65, 0x72, 0xa3,
        0x0d, 0x1b, 0x0b, 0x43, 0x4f, 0x4d, 0x50, 0x41, 0x4e, 0x59, 0x2e, 0x49, 0x4e, 0x54, 0xa4,
        0x11, 0x30, 0x0f, 0xa0, 0x03, 0x02, 0x01, 0x01, 0xa1, 0x08, 0x30, 0x06, 0x1b, 0x04, 0x75,
        0x73, 0x65, 0x72, 0xa5, 0x50, 0x61, 0x4e, 0x30, 0x4c, 0xa0, 0x03, 0x02, 0x01, 0x05, 0xa1,
        0x0d, 0x1b, 0x0b, 0x43, 0x4f, 0x4d, 0x50, 0x41, 0x4e, 0x59, 0x2e, 0x49, 0x4e, 0x54, 0xa2,
        0x20, 0x30, 0x1e, 0xa0, 0x03, 0x02, 0x01, 0x02, 0xa1, 0x17, 0x30, 0x15, 0x1b, 0x06, 0x6b,
        0x72, 0x62, 0x74, 0x67, 0x74, 0x1b, 0x0b, 0x43, 0x4f, 0x4d, 0x50, 0x41, 0x4e, 0x59, 0x2e,
        0x49, 0x4e, 0x54, 0xa3, 0x14, 0x30, 0x12, 0xa0, 0x03, 0x02, 0x01, 0x12, 0xa1, 0x03, 0x02,
        0x01, 0x02, 0xa2, 0x06, 0x04, 0x04, 0xde, 0xad, 0xbe, 0xef, 0xa6, 0x14, 0x30, 0x12, 0xa0,
        0x03, 0x02, 0x01, 0x12, 0xa1, 0x03, 0x02, 0x01, 0x0d, 0xa2, 0x06, 0x04, 0x04, 0xde, 0xad,
        0xbe, 0xef,
    ];

    let enc = rasn::der::encode(&as_rep).unwrap();
    assert_eq!(data, &enc);
    assert_eq!(as_rep, rasn::der::decode(&enc).unwrap());
}
