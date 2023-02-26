extern crate sstp_protocol;

use anyhow::Result;
use sstp_protocol::request::Request;
use sstp_protocol::response::{AdditionalData, Response};
use sstp_protocol::{Charset, HeaderName, Method, StatusCode, Version};

/// http://usada.sakura.vg/contents/sstp.html#notify10
///
/// ```text
/// NOTIFY SSTP/1.0
/// Sender: さくら
/// Event: OnMusicPlay
/// Reference0: 元祖高木ブー伝説
/// Reference1: 筋肉少女帯
/// Charset: Shift_JIS
///
/// [EOD]
/// ```
#[test]
fn example_notify1_0() -> Result<()> {
    let request = Request::builder()
        .notify(Version::SSTP_10)
        .header(HeaderName::SENDER, "さくら")
        .header(HeaderName::EVENT, "OnMusicPlay")
        .header(HeaderName::REFERENCE0, "元祖高木ブー伝説")
        .header(HeaderName::REFERENCE1, "筋肉少女帯")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::NOTIFY);
    assert_eq!(request.version(), Version::SSTP_10);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("さくら".to_string())
    );
    assert_eq!(
        request.event().and_then(|v| v.text().ok()),
        Some("OnMusicPlay".to_string())
    );
    assert_eq!(
        request
            .reference0()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("元祖高木ブー伝説".to_string())
    );
    assert_eq!(
        request
            .reference1()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("筋肉少女帯".to_string())
    );
    assert!(request.reference2().is_none());
    assert!(request.reference3().is_none());
    assert!(request.reference4().is_none());
    assert!(request.reference5().is_none());
    assert!(request.reference6().is_none());
    assert!(request.reference7().is_none());
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}

/// http://usada.sakura.vg/contents/sstp.html#notify11
///
/// ```text
/// NOTIFY SSTP/1.1
/// Sender: さくら
/// Event: OnMusicPlay
/// Reference0: 元祖高木ブー伝説
/// Reference1: 筋肉少女帯
/// IfGhost: なる,ゆうか
/// Script: \h\s0‥‥\w8\w8高木ブーだね。\u\s0‥‥\e
/// IfGhost: さくら,うにゅう
/// Script: \h\s0‥‥\w8\w8高木ブーだね。\u\s0‥‥\w8\w8むう。\e
/// Charset: Shift_JIS
///
/// [EOD]
/// ```
#[test]
fn example_notify1_1() -> Result<()> {
    let request = Request::builder()
        .notify(Version::SSTP_11)
        .header(HeaderName::SENDER, "さくら")
        .header(HeaderName::EVENT, "OnMusicPlay")
        .header(HeaderName::REFERENCE0, "元祖高木ブー伝説")
        .header(HeaderName::REFERENCE1, "筋肉少女帯")
        .header(HeaderName::IF_GHOST, "なる,ゆうか")
        .header(
            HeaderName::SCRIPT,
            "\\h\\s0‥‥\\w8\\w8高木ブーだね。\\u\\s0‥‥\\e",
        )
        .header(HeaderName::IF_GHOST, "さくら,うにゅう")
        .header(
            HeaderName::SCRIPT,
            "\\h\\s0‥‥\\w8\\w8高木ブーだね。\\u\\s0‥‥\\w8\\w8むう。\\e",
        )
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::NOTIFY);
    assert_eq!(request.version(), Version::SSTP_11);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("さくら".to_string())
    );
    assert_eq!(
        request
            .event()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("OnMusicPlay".to_string())
    );
    assert_eq!(
        request
            .reference0()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("元祖高木ブー伝説".to_string())
    );
    assert_eq!(
        request
            .reference1()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("筋肉少女帯".to_string())
    );
    assert!(request.reference2().is_none());
    assert!(request.reference3().is_none());
    assert!(request.reference4().is_none());
    assert!(request.reference5().is_none());
    assert!(request.reference6().is_none());
    assert!(request.reference7().is_none());
    assert_eq!(
        request
            .if_ghost()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec!["なる,ゆうか", "さくら,うにゅう"]
    );
    assert_eq!(
        request
            .script()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec![
            "\\h\\s0‥‥\\w8\\w8高木ブーだね。\\u\\s0‥‥\\e",
            "\\h\\s0‥‥\\w8\\w8高木ブーだね。\\u\\s0‥‥\\w8\\w8むう。\\e"
        ]
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}

/// http://usada.sakura.vg/contents/sstp.html#send11
///
/// ```text
/// SEND SSTP/1.1
/// Sender: カードキャプター
/// Script: \h\s0汝のあるべき姿に戻れ。\e
/// Option: nodescript,notranslate
/// Charset: Shift_JIS
///
/// [EOD]
/// ```
#[test]
fn example_send_1_1() -> Result<()> {
    let request = Request::builder()
        .send(Version::SSTP_11)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::SCRIPT, "\\h\\s0汝のあるべき姿に戻れ。\\e")
        .header(HeaderName::OPTION, "nodescript,notranslate")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::SEND);
    assert_eq!(request.version(), Version::SSTP_11);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request
            .script()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec!["\\h\\s0汝のあるべき姿に戻れ。\\e"]
    );
    assert_eq!(
        request.option().and_then(|v| v.text().ok()),
        Some("nodescript,notranslate".to_string())
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}

/// http://usada.sakura.vg/contents/sstp.html#send12
///
/// ```text
/// SEND SSTP/1.2
/// Sender: カードキャプター
/// Script: \h\s0どんな感じ？\n\n\q0[#temp0][まあまあ]\q1[#temp1][今ひとつ]\z
/// Entry: #temp0,\h\s0ふーん。\e
/// Entry: #temp1,\h\s0酒に逃げるなヨ！\e
/// Charset: Shift_JIS
///
/// [EOD]
/// ```
#[test]
fn example_send_1_2() -> Result<()> {
    let request = Request::builder()
        .send(Version::SSTP_12)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(
            HeaderName::SCRIPT,
            "\\h\\s0どんな感じ？\\n\\n\\q0[#temp0][まあまあ]\\q1[#temp1][今ひとつ]\\z",
        )
        .header(HeaderName::ENTRY, "#temp0,\\h\\s0ふーん。\\e")
        .header(HeaderName::ENTRY, "#temp1,\\h\\s0酒に逃げるなヨ！\\e")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::SEND);
    assert_eq!(request.version(), Version::SSTP_12);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request
            .script()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec!["\\h\\s0どんな感じ？\\n\\n\\q0[#temp0][まあまあ]\\q1[#temp1][今ひとつ]\\z"]
    );
    assert_eq!(
        request
            .entry()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec![
            "#temp0,\\h\\s0ふーん。\\e",
            "#temp1,\\h\\s0酒に逃げるなヨ！\\e"
        ]
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}

/// http://usada.sakura.vg/contents/sstp.html#send13
///
/// ```text
/// SEND SSTP/1.3
/// Sender: カードキャプター
/// HWnd: 1024
/// Script: \h\s0どんな感じ？\n\n\q0[#temp0][まあまあ]\q1[#temp1][今ひとつ]\z
/// Entry: #temp0,\m[1025,0,0]\h\s0ふーん。\m[1025,0,1]\e
/// Entry: #temp1,\m[1025,1,0]\h\s0酒に逃げるなヨ！\m[1025,1,1]\e
/// Charset: Shift_JIS
///
/// [EOD]
/// ```
#[test]
fn example_send_1_3() -> Result<()> {
    let request = Request::builder()
        .send(Version::SSTP_13)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::HWND, "1024")
        .header(
            HeaderName::SCRIPT,
            "\\h\\s0どんな感じ？\\n\\n\\q0[#temp0][まあまあ]\\q1[#temp1][今ひとつ]\\z",
        )
        .header(
            HeaderName::ENTRY,
            "#temp0,\\m[1025,0,0]\\h\\s0ふーん。\\m[1025,0,1]\\e",
        )
        .header(
            HeaderName::ENTRY,
            "#temp1,\\m[1025,1,0]\\h\\s0酒に逃げるなヨ！\\m[1025,1,1]\\e",
        )
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::SEND);
    assert_eq!(request.version(), Version::SSTP_13);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request.hwnd().and_then(|v| v.text().ok()),
        Some("1024".to_string())
    );
    assert_eq!(
        request
            .script()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec!["\\h\\s0どんな感じ？\\n\\n\\q0[#temp0][まあまあ]\\q1[#temp1][今ひとつ]\\z"]
    );
    assert_eq!(
        request
            .entry()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec![
            "#temp0,\\m[1025,0,0]\\h\\s0ふーん。\\m[1025,0,1]\\e",
            "#temp1,\\m[1025,1,0]\\h\\s0酒に逃げるなヨ！\\m[1025,1,1]\\e"
        ]
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}

/// http://usada.sakura.vg/contents/sstp.html#send14
///
/// ```text
/// SEND SSTP/1.4
/// Sender: カードキャプター
/// IfGhost: さくら,うにゅう
/// Script: \h\s0さくらだー。\w8\n\n%j[#mainblock]
/// IfGhost: せりこ,まるちい
/// Script: \h\s0せりこだー。\w8\n\n%j[#mainblock]
/// IfGhost: さくら,ケロ
/// Script: \u\s0わいのはモダン焼きにしてや～。\w8\h\s0はいはい。\e
/// Entry: #mainblock,\s7寝言は寝てから言えっ！\w8\u\s0落ち着けっ！\e
/// Charset: Shift_JIS
///
/// [EOD]
/// ```
#[test]
fn example_send_1_4() -> Result<()> {
    let request = Request::builder()
        .send(Version::SSTP_14)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::IF_GHOST, "さくら,うにゅう")
        .header(
            HeaderName::SCRIPT,
            "\\h\\s0さくらだー。\\w8\\n\\n%j[#mainblock]",
        )
        .header(HeaderName::IF_GHOST, "せりこ,まるちい")
        .header(
            HeaderName::SCRIPT,
            "\\h\\s0せりこだー。\\w8\\n\\n%j[#mainblock]",
        )
        .header(HeaderName::IF_GHOST, "さくら,ケロ")
        .header(
            HeaderName::SCRIPT,
            "\\u\\s0わいのはモダン焼きにしてや～。\\w8\\h\\s0はいはい。\\e",
        )
        .header(
            HeaderName::ENTRY,
            "#mainblock,\\s7寝言は寝てから言えっ！\\w8\\u\\s0落ち着けっ！\\e",
        )
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::SEND);
    assert_eq!(request.version(), Version::SSTP_14);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request
            .if_ghost()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec!["さくら,うにゅう", "せりこ,まるちい", "さくら,ケロ"]
    );
    assert_eq!(
        request
            .script()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec![
            "\\h\\s0さくらだー。\\w8\\n\\n%j[#mainblock]",
            "\\h\\s0せりこだー。\\w8\\n\\n%j[#mainblock]",
            "\\u\\s0わいのはモダン焼きにしてや～。\\w8\\h\\s0はいはい。\\e"
        ]
    );
    assert_eq!(
        request
            .entry()
            .iter()
            .map(|v| v.text_with_charset(request.charset()).unwrap())
            .collect::<Vec<String>>(),
        vec!["#mainblock,\\s7寝言は寝てから言えっ！\\w8\\u\\s0落ち着けっ！\\e"]
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}

/// http://usada.sakura.vg/contents/sstp.html#execute10
///
/// ```text
/// EXECUTE SSTP/1.0
/// Sender: サンプルプログラム
/// Command: GetName
/// Charset: Shift_JIS
///
/// [EOD]
/// ```
#[test]
fn example_execute_1_0() -> Result<()> {
    let request = Request::builder()
        .execute(Version::SSTP_10)
        .header(HeaderName::SENDER, "サンプルプログラム")
        .header(HeaderName::COMMAND, "GetName")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::EXECUTE);
    assert_eq!(request.version(), Version::SSTP_10);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("サンプルプログラム".to_string())
    );
    assert_eq!(
        request.command().and_then(|v| v.text().ok()),
        Some("GetName".to_string())
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}

/// http://usada.sakura.vg/contents/sstp.html#execute11
///
/// ```text
/// EXECUTE SSTP/1.1
/// Sender: カードキャプター
/// Command: SetCookie[visitcount,1]
/// Charset: Shift_JIS
///
/// [EOD]
///
/// EXECUTE SSTP/1.1
/// Sender: カードキャプター
/// Command: GetCookie[visitcount]
/// Charset: Shift_JIS
///
/// [EOD]
/// ```
#[test]
fn example_execute_1_1() -> Result<()> {
    let request = Request::builder()
        .execute(Version::SSTP_11)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::COMMAND, "SetCookie[visitcount,1]")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::EXECUTE);
    assert_eq!(request.version(), Version::SSTP_11);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request.command().and_then(|v| v.text().ok()),
        Some("SetCookie[visitcount,1]".to_string())
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    let request = Request::builder()
        .execute(Version::SSTP_11)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::COMMAND, "GetCookie[visitcount]")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::EXECUTE);
    assert_eq!(request.version(), Version::SSTP_11);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request.command().and_then(|v| v.text().ok()),
        Some("GetCookie[visitcount]".to_string())
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}

/// http://usada.sakura.vg/contents/sstp.html#execute12
///
/// ```text
/// EXECUTE SSTP/1.2
/// Sender: カードキャプター
/// Command: GetVersion
/// Charset: Shift_JIS
///
/// [EOD]
/// ```
#[test]
fn example_execute_1_2() -> Result<()> {
    let request = Request::builder()
        .execute(Version::SSTP_12)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::COMMAND, "GetVersion")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::EXECUTE);
    assert_eq!(request.version(), Version::SSTP_12);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request.command().and_then(|v| v.text().ok()),
        Some("GetVersion".to_string())
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}

/// http://usada.sakura.vg/contents/sstp.html#execute13
///
/// ```text
/// EXECUTE SSTP/1.3
/// Sender: カードキャプター
/// Command: Quiet
/// Charset: Shift_JIS
///
/// [EOD]
/// ```
#[test]
fn example_execute_1_3() -> Result<()> {
    let request = Request::builder()
        .execute(Version::SSTP_13)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::COMMAND, "Quiet")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::EXECUTE);
    assert_eq!(request.version(), Version::SSTP_13);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request.command().and_then(|v| v.text().ok()),
        Some("Quiet".to_string())
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}

/// http://usada.sakura.vg/contents/sstp.html#give11
///
/// ```text
/// GIVE SSTP/1.1
/// Sender: カードキャプター
/// Document: こんにちはさくらです。闇の力を秘めし鍵よ真の姿を我の前に示せレリーズ。汝のあるべき姿に戻れクロウカード。
/// Charset: Shift_JIS
///
/// [EOD]
/// ```
#[test]
fn example_give_1_1() -> Result<()> {
    let request = Request::builder()
        .give(Version::SSTP_11)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::DOCUMENT, "こんにちはさくらです。闇の力を秘めし鍵よ真の姿を我の前に示せレリーズ。汝のあるべき姿に戻れクロウカード。")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::GIVE);
    assert_eq!(request.version(), Version::SSTP_11);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request
            .document()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("こんにちはさくらです。闇の力を秘めし鍵よ真の姿を我の前に示せレリーズ。汝のあるべき姿に戻れクロウカード。".to_string()));
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}

/// http://usada.sakura.vg/contents/sstp.html#communicate11
///
/// ```text
/// COMMUNICATE SSTP/1.1
/// Sender: カードキャプター
/// Sentence: 今日は寒いなー。
/// Option: substitute
/// Charset: Shift_JIS
///
/// [EOD]
/// ```
#[test]
fn example_communicate_1_1() -> Result<()> {
    let request = Request::builder()
        .communicate(Version::SSTP_11)
        .header(HeaderName::SENDER, "カードキャプター")
        .header(HeaderName::SENTENCE, "今日は寒いなー。")
        .header(HeaderName::OPTION, "substitute")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::COMMUNICATE);
    assert_eq!(request.version(), Version::SSTP_11);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("カードキャプター".to_string())
    );
    assert_eq!(
        request
            .sentence()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("今日は寒いなー。".to_string())
    );
    assert_eq!(
        request
            .option()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("substitute".to_string())
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}

/// http://usada.sakura.vg/contents/sstp.html#communicate12
///
/// ```text
/// COMMUNICATE SSTP/1.2
/// Sender: 双葉
/// HWnd: 0
/// Sentence: \0\s0どうも。\e
/// Surface: 0,10
/// Reference0: N/A
/// Charset: Shift_JIS
///
/// [EOD]
/// ```
#[test]
fn example_communicate_1_2() -> Result<()> {
    let request = Request::builder()
        .communicate(Version::SSTP_12)
        .header(HeaderName::SENDER, "双葉")
        .header(HeaderName::HWND, "0")
        .header(HeaderName::SENTENCE, "\\0\\s0どうも。\\e")
        .header(HeaderName::SURFACE, "0,10")
        .header(HeaderName::REFERENCE0, "N/A")
        .charset(Charset::SHIFT_JIS)
        .build()?;
    let input = request.as_bytes();

    let request = Request::parse(&input)?;
    assert_eq!(request.method(), Method::COMMUNICATE);
    assert_eq!(request.version(), Version::SSTP_12);
    assert_eq!(
        request
            .sender()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("双葉".to_string())
    );
    assert_eq!(
        request.hwnd().and_then(|v| v.text().ok()),
        Some("0".to_string())
    );
    assert_eq!(
        request
            .sentence()
            .and_then(|v| v.text_with_charset(request.charset()).ok()),
        Some("\\0\\s0どうも。\\e".to_string())
    );
    assert_eq!(
        request.surface().and_then(|v| v.text().ok()),
        Some("0,10".to_string())
    );
    assert_eq!(
        request.reference0().and_then(|v| v.text().ok()),
        Some("N/A".to_string())
    );
    assert_eq!(request.charset(), Charset::SHIFT_JIS);

    assert_eq!(request.as_bytes(), input);

    Ok(())
}

/// http://ssp.shillest.net/ukadoc/manual/spec_sstp.html#req_res
/// ```test
///
/// SSTP/1.4 200 OK
/// Charset: UTF-8
/// Script: \h\s0テストー。\u\s[10]テストやな。
///
/// 追加データはここ
///
/// [EOD]
/// ```
#[test]
fn example_response_no_additional() -> Result<()> {
    let response = Response::builder()
        .version(Version::SSTP_14)
        .status_code(StatusCode::OK)
        .charset(Charset::UTF8)
        .header(
            HeaderName::SCRIPT,
            "\\h\\s0テストー。\\u\\s[10]テストやな。",
        )
        .build()?;
    let input = response.as_bytes();

    let response = Response::parse(&input)?;
    assert_eq!(response.version(), Version::SSTP_14);
    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(response.charset(), Charset::UTF8);
    assert_eq!(
        response
            .headers()
            .get(HeaderName::SCRIPT)
            .and_then(|v| v.text_with_charset(response.charset()).ok()),
        Some("\\h\\s0テストー。\\u\\s[10]テストやな。".to_string())
    );
    matches!(response.additional(), AdditionalData::Empty);
    assert_eq!(response.additional().text()?, "");

    assert_eq!(response.as_bytes(), input);

    Ok(())
}

/// http://ssp.shillest.net/ukadoc/manual/spec_sstp.html#req_res
/// ```test
///
/// SSTP/1.4 200 OK
/// Charset: UTF-8
/// Script: \h\s0テストー。\u\s[10]テストやな。
///
/// 追加データはここ
///
/// [EOD]
/// ```
#[test]
fn example_response_use_additional() -> Result<()> {
    let response = Response::builder()
        .version(Version::SSTP_14)
        .status_code(StatusCode::OK)
        .charset(Charset::UTF8)
        .header(
            HeaderName::SCRIPT,
            "\\h\\s0テストー。\\u\\s[10]テストやな。",
        )
        .additional("追加データはここ")
        .build()?;
    let input = response.as_bytes();

    let response = Response::parse(&input)?;
    assert_eq!(response.version(), Version::SSTP_14);
    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(response.charset(), Charset::UTF8);
    assert_eq!(
        response
            .headers()
            .get(HeaderName::SCRIPT)
            .and_then(|v| v.text_with_charset(response.charset()).ok()),
        Some("\\h\\s0テストー。\\u\\s[10]テストやな。".to_string())
    );
    assert_eq!(
        response
            .additional()
            .text_with_charset(response.charset())?,
        "追加データはここ"
    );

    assert_eq!(response.as_bytes(), input);

    Ok(())
}
