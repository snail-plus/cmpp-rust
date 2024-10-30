
#[cfg(test)]
mod tests {
    use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
    use rsa::pkcs1::{DecodeRsaPublicKey, DecodeRsaPrivateKey, EncodeRsaPrivateKey, EncodeRsaPublicKey};
    use base64::{Engine as _, engine::general_purpose};

    pub fn encrypt(message: &str, public_key_pem: &str) -> String {
        let mut rng = rand::thread_rng();
        let public_key = RsaPublicKey::from_pkcs1_pem(public_key_pem).unwrap();
        let encrypted_message = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, message.as_bytes()).unwrap();
        general_purpose::STANDARD_NO_PAD.encode(&encrypted_message)
    }

    pub fn decrypt(message: &str, private_key_pem: &str) -> String {
        let private_key = RsaPrivateKey::from_pkcs1_pem(private_key_pem).unwrap();
        let encrypted_message = private_key.decrypt(Pkcs1v15Encrypt, &general_purpose::STANDARD_NO_PAD.decode(message).unwrap()).unwrap();
        String::from_utf8_lossy(&encrypted_message).to_string()
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        let private_key_pem = "-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEAsODE3ZePViksxCIUYOr/X9Cj8GXoAByJNH3QIu88WWL6IH4l
4bTDH5J5WCp7RC6O/ENQgzvxdhmXOyBK7E3Oh6zMgHLh+d+k6575XyjE+c6yfSTG
LOUwfzL+Jjc1qW3gmXReSqnH4lqCUlqOii4v1//j0ARhXGzz/ZlDvl1dBlnebFZ0
HrlF5NxkO0gGeCFJEF8Vmty0j1YLirfdvo0o4kYgr9RG33pIgpjJUqGOX9bf+i3+
MIdACf7pOnabDPbcY/XV5FiC0IuvASNa+buibV5R1UewVRo+85vD12j3ez/HjBBy
kDnRGw6F4R7FD7poCHgF6O9HkCKll7UYt1AIzwIDAQABAoIBAGbYKl+V8lfs1QSt
tB+uRBKYI9pxxnXaIcUNqmnFpNdsf3dJIgmcqx++cSPcf5PjZmwzT6sevYUn2BEe
87F2hDHLPZUqN06sxR3jb6fu8qk1j/7H1RrhoFM1eSULUP5UzxUq3nCXS4vHiKMN
2Hdr55G2TeXzxhCRoUTBSuxzKNW5jqH78aS+vSS7i73+sJarjSYevT+0z+g3KXD3
Ejt4OHPY4YqXgnfbJTjkt2c/8Zj0UGObGsrPgY32W7TX2cTGbPohz9VoK+tTRLKN
zpYuY5D759KooLfT6owBnci5TXg3XgrVzFBVyr5zFWaFYIJkFB6nyTR8H8kM2qGG
IDUa7tECgYEA68YTpxRgX5yleAzSfQ07Gg49mo009BVxMQhDkW6nYWcQdlJJ1giS
WQI9cMtPaslWq5pWDkettF/Ai7mjk4KTldIzG1zutVPwL8bBcoz54Y9gQgjBYxa0
4sx8e01H6qeIhWOZu0Pz3pgB4CLBsdytyjXfmnBC3AmYP30UbpMO1UMCgYEAwA1C
Tmm6cncu0T62yuIQXHA24Bcxz7uW/jjfywnrNT2+VEh+HNcw1Ul/SSGE5TKTiy+t
mNSBX+8Jne9oQuJTkgX6gKaqdlRYNVSDMfYU7Nka7kIkYgNW7U399oXPDDLg/VKx
3i5xs6fdopopRYGRBuAZctiV7Q5c5+9+ByOFf4UCgYBBxjl3VATqx32V9yXFgypo
w25GLnyGV3EDd8W5zb/eOW1rRNuaXTvOnwRa8i0OomqOZvj49OAtwMSrdjd/EF9x
3XkqeguSBH+uJGmVNmUDmwcEhQLGTPBbkSZtE4srmF9KrxWVG4juIUPsrmWQ4/qL
venKYOUWE4sosxE/I8FwlwKBgHPuTQ1ag2MfkNJc4ij8Z9X1IxsIxVfZrF8P5AAD
n8lG2UK1c/Ni7yaBNGXn5voYGrqVcFxmQyau/AYrg6uEReBK3ZmVgibl6U1gE8Yv
/xeuQrR6ls3XqDydrOWIhqCdgfVJWvd838QMXB0QpENB4FLfKl2KYo5z8h9MEa6r
gaWlAoGBAIw6H98nq5QfI2K6yApLKENkn3d7sXW2Exokql/sOs9c47Hl4NknU7xa
3rCsOcNDDuzwY6tgRW06kO2pagYz5bP/eQHPD1g+nig4ilnRepuYCzOGLW9DfPcc
28SQkbujwAFqE0Jdj/Xh1+JO6J5ym1jyL+BXZG2REQMX+XCXDY9b
-----END RSA PRIVATE KEY-----";

        let private_key = RsaPrivateKey::from_pkcs1_pem(private_key_pem).unwrap();
        let public_key_pem = private_key.to_public_key().to_pkcs1_pem(rsa::pkcs1::LineEnding::LF).unwrap();
        println!("pub_key_pem: {}", public_key_pem);

        let aa = encrypt("12345", &public_key_pem);
        let bb = decrypt("d163Z/iLQEbnmBjL3X9TcLD8cEHonwePEnPhd4FFQq83fCjBu9lvWsTB0+7c+lv2RiagH1FPUAWj2pP3EVgf7WCekCpRKuk6CqS/wBCYYE/6ae0+6/rUOvlkqaAeYGXwi2Ppe1Ef3fjAj7dEHrgxvAeumF7JGwXA10NPeY02xyG9bISN8Z0W0rNDRAYSI6M0OoRoieaxTJytoxgfEEXqZCCuX06BkJf6JuPqZt6fJ78SAobvEWrWyIZ5O/zFiCB1pikSxoxTb6V0frBbrx1Qztqa9P88R4dM7xOMo6bXtObmysGiQMAES4eBxxMb/EmOu+1iwrP2iBOkWdX0mkqIGA", private_key_pem);
        println!("bb: {}", bb);

    }
}