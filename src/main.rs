use p256::{PublicKey, EncodedPoint};


fn main() {
    // got public key hexces from openssl command line (https://stackoverflow.com/questions/29355027/what-method-does-openssl-use-to-combine-a-public-ec-keys-coordinates)
    let public_key_uncompressed_hex = "04:9d:76:05:ba:37:1c:4e:a7:7c:2c:f0:e2:d3:ec:5b:8c:85:0a:ed:a7:79:6c:29:89:4c:13:ce:83:1e:64:cd:e2:6a:a1:60:a1:02:34:7c:7c:42:87:e7:0f:59:57:3b:63:30:29:6c:55:5f:04:86:26:80:86:40:27:8e:da:90:5e";
    let public_key_uncompressed_hex = str::replace(public_key_uncompressed_hex, ":", "");

    let public_key_compressed_hex = "02:9d:76:05:ba:37:1c:4e:a7:7c:2c:f0:e2:d3:ec:5b:8c:85:0a:ed:a7:79:6c:29:89:4c:13:ce:83:1e:64:cd:e2";
    let public_key_compressed_hex = str::replace(public_key_compressed_hex, ":", "");

    let public_key_uncompressed_bytes = hex::decode(public_key_uncompressed_hex).unwrap();
    let public_key = PublicKey::from_sec1_bytes(&public_key_uncompressed_bytes).unwrap();

    let encoded_point = EncodedPoint::from(public_key);
    let compressed_point = encoded_point.compress();
    let hex_compressed = hex::encode(compressed_point.as_bytes());
    dbg!(&hex_compressed);
    dbg!(hex_compressed == public_key_compressed_hex);

}