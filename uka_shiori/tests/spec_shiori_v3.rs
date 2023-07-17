use uka_shiori::types::v3::HeaderValue;
use uka_shiori::types::{v3, Request, Response};

/// http://usada.sakura.vg/contents/specification2.html#shioriprotocol
/// > リクエスト文字列は以下の形式で構成される。
/// > ```
/// > GET SHIORI/3.0
/// > Sender: Materia
/// > ID: hoge
/// > Reference0: uge
/// > Reference1: ....
/// > ....
/// > ..
/// >
/// > ```
/// > 行単位処理の発想であり、CR+LF で各行がセパレートされ、空行でターミネートされる。
/// > 第1行目はコマンド行であり、コマンド文字列とこのリクエストのバージョンナンバがセットされる。
/// > 第2行目以降はヘッダ行であり、Key: Value の形式で任意の数のヘッダが続く。
/// > このヘッダの最大数は無限であり、順不同である。
#[test]
fn spec_shiori_request() -> anyhow::Result<()> {
    let input = [
        b"GET SHIORI/3.0\r\n".to_vec(),
        b"Sender: Materia\r\n".to_vec(),
        b"ID: hoge\r\n".to_vec(),
        b"Reference0: uge\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();
    match Request::parse(&input)? {
        Request::V3(request) => {
            assert_eq!(request.method(), v3::Method::GET);
            assert_eq!(request.version(), v3::Version::SHIORI_30);
            assert_eq!(
                request.sender(),
                Some(&HeaderValue::from(b"Materia".as_slice()))
            );
            assert_eq!(request.id(), Some(&HeaderValue::from(b"hoge".as_slice())));
            assert_eq!(
                request.reference0(),
                Some(&HeaderValue::from(b"uge".as_slice()))
            );

            assert_eq!(
                request.as_bytes(),
                input,
                "\nassertion failed: `(left == right)\n  left: `{:?}`,\n right: `{:?}`",
                String::from_utf8_lossy(&request.as_bytes()),
                String::from_utf8_lossy(&input)
            );
        }
    };

    Ok(())
}

/// http://usada.sakura.vg/contents/specification2.html#shioriprotocol
/// > レスポンス文字列は以下の形式で構成される。
/// > ```
/// > SHIORI/3.0 200 OK
/// > Sender: F.I.R.S.T
/// > Value: hoge
/// > ....
/// > ..
/// >
/// > ```
/// > 1行目はリザルトコード行であり、このレスポンスのバージョンナンバとリザルトコードがセットされる。他はリクエスト文字列と全く同じ。
#[test]
fn spec_shiori_response() -> anyhow::Result<()> {
    let input = [
        b"SHIORI/3.0 200 OK\r\n".to_vec(),
        b"Sender: F.I.R.S.T\r\n".to_vec(),
        b"Value: hoge\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();

    match Response::parse(&input)? {
        Response::V3(response) => {
            assert_eq!(response.version(), v3::Version::SHIORI_30);
            assert_eq!(response.status_code(), v3::StatusCode::OK);
            assert_eq!(
                response.sender(),
                Some(&HeaderValue::from(b"F.I.R.S.T".as_slice()))
            );
            assert_eq!(
                response.value(),
                Some(&HeaderValue::from(b"hoge".as_slice()))
            );

            assert_eq!(
                response.as_bytes(),
                input,
                "\nassertion failed: `(left == right)\n  left: `{:?}`,\n right: `{:?}`",
                String::from_utf8_lossy(&response.as_bytes()),
                String::from_utf8_lossy(&input)
            );
        }
    };

    Ok(())
}

/// http://usada.sakura.vg/contents/specification2.html#shioriprotocol
/// > レスポンスの Reference0 が話し掛ける相手の名前を表す。
/// >
/// > 異常な仕様だが他によい方法が思いつかないので暫定的にこの位置に置く。
#[test]
fn spec_shiori_response_with_name_of_person_to_talk_to() -> anyhow::Result<()> {
    let input = [
        b"SHIORI/3.0 200 OK\r\n".to_vec(),
        b"Sender: F.I.R.S.T\r\n".to_vec(),
        b"Value: hoge\r\n".to_vec(),
        b"Reference0: Sakura\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]
    .concat();

    match Response::parse(&input)? {
        Response::V3(response) => {
            assert_eq!(response.version(), v3::Version::SHIORI_30);
            assert_eq!(response.status_code(), v3::StatusCode::OK);
            assert_eq!(
                response.sender(),
                Some(&HeaderValue::from(b"F.I.R.S.T".as_slice()))
            );
            assert_eq!(
                response.value(),
                Some(&HeaderValue::from(b"hoge".as_slice()))
            );
            assert_eq!(
                response.reference0(),
                Some(&HeaderValue::from(b"Sakura".as_slice()))
            );

            assert_eq!(
                response.as_bytes(),
                input,
                "\nassertion failed: `(left == right)\n  left: `{:?}`,\n right: `{:?}`",
                String::from_utf8_lossy(&response.as_bytes()),
                String::from_utf8_lossy(&input)
            );
        }
    };

    Ok(())
}
